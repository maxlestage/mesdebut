#!/usr/bin/env python3
"""Attend le verdict d'Apple sur un build envoyé à TestFlight.

« UPLOAD SUCCEEDED » ne veut dire qu'une chose : Apple a reçu le fichier. Le
traitement vient ensuite, et c'est lui qui décide si le build apparaît dans
TestFlight. Il peut :

- le rejeter (`INVALID`) : Apple envoie alors le motif par courriel, au
  titulaire du compte, avec un code ITMS-9xxxx ;
- l'accepter, mais le bloquer pour les testeurs tant que la conformité à
  l'export n'est pas renseignée (`MISSING_EXPORT_COMPLIANCE`).

Ce script interroge l'API App Store Connect jusqu'au verdict, l'affiche, et
renseigne la conformité à l'export s'il le faut (l'app n'utilise aucun
chiffrement propre). Il échoue si le build est rejeté ou n'arrive jamais.

Sans dépendance : le jeton JWT est signé par `openssl`.

Variables : ASC_KEY_ID, ASC_ISSUER_ID, ASC_KEY_PATH, BUNDLE_ID, BUILD_NUMBER.
"""
import base64
import json
import os
import subprocess
import sys
import time
import urllib.error
import urllib.parse
import urllib.request

API = 'https://api.appstoreconnect.apple.com/v1'
KEY_ID = os.environ['ASC_KEY_ID']
ISSUER = os.environ['ASC_ISSUER_ID']
KEY_PATH = os.environ['ASC_KEY_PATH']
BUNDLE_ID = os.environ.get('BUNDLE_ID', 'com.reneuro.ReNeuro')
BUILD = os.environ['BUILD_NUMBER']

# Le traitement prend d'ordinaire 5 à 15 minutes.
ATTENTE_MAX = 25 * 60
INTERVALLE = 30


def b64url(octets: bytes) -> str:
    return base64.urlsafe_b64encode(octets).rstrip(b'=').decode()


def der_vers_brut(der: bytes) -> bytes:
    """openssl signe en DER (SEQUENCE de deux INTEGER) ; JWT veut r || s."""
    assert der[0] == 0x30, 'signature DER inattendue'
    i = 2 if der[1] < 0x80 else 2 + (der[1] & 0x7F)
    morceaux = []
    for _ in range(2):
        assert der[i] == 0x02, 'signature DER inattendue'
        longueur = der[i + 1]
        entier = der[i + 2:i + 2 + longueur].lstrip(b'\0')
        morceaux.append(entier.rjust(32, b'\0'))
        i += 2 + longueur
    return b''.join(morceaux)


def jeton() -> str:
    maintenant = int(time.time())
    entete = b64url(json.dumps({'alg': 'ES256', 'kid': KEY_ID, 'typ': 'JWT'}).encode())
    charge = b64url(json.dumps({
        'iss': ISSUER,
        'iat': maintenant,
        'exp': maintenant + 15 * 60,
        'aud': 'appstoreconnect-v1',
    }).encode())
    message = f'{entete}.{charge}'.encode()
    der = subprocess.run(
        ['openssl', 'dgst', '-sha256', '-sign', KEY_PATH],
        input=message, capture_output=True, check=True,
    ).stdout
    return f'{entete}.{charge}.{b64url(der_vers_brut(der))}'


def appel(methode: str, chemin: str, corps=None):
    requete = urllib.request.Request(
        API + chemin,
        method=methode,
        data=json.dumps(corps).encode() if corps is not None else None,
        headers={
            'Authorization': f'Bearer {jeton()}',
            'Content-Type': 'application/json',
        },
    )
    try:
        with urllib.request.urlopen(requete, timeout=60) as reponse:
            corps_reponse = reponse.read()
            # un ajout de relation répond 204, sans corps
            return json.loads(corps_reponse) if corps_reponse else {}
    except urllib.error.HTTPError as erreur:
        detail = erreur.read().decode(errors='replace')
        raise SystemExit(f'::error::App Store Connect a répondu {erreur.code} sur {chemin} :\n{detail}')


def app_id() -> str:
    filtre = urllib.parse.quote(BUNDLE_ID)
    apps = appel('GET', f'/apps?filter[bundleId]={filtre}&fields[apps]=bundleId')['data']
    if not apps:
        raise SystemExit(f"::error::Aucune app « {BUNDLE_ID} » dans App Store Connect.")
    return apps[0]['id']


def chercher_build(app: str):
    chemin = (f'/builds?filter[app]={app}&filter[version]={urllib.parse.quote(BUILD)}'
              '&include=buildBetaDetail&limit=1')
    reponse = appel('GET', chemin)
    if not reponse['data']:
        return None, None
    build = reponse['data'][0]
    beta = next((i for i in reponse.get('included', []) if i['type'] == 'buildBetaDetails'), None)
    return build, beta


def main() -> int:
    app = app_id()
    print(f'App {BUNDLE_ID} : {app}. Build attendu : {BUILD}.')
    debut = time.time()
    dernier = None
    while time.time() - debut < ATTENTE_MAX:
        build, beta = chercher_build(app)
        if build is None:
            etat = 'pas encore visible'
        else:
            etat = build['attributes'].get('processingState')
        if etat != dernier:
            minutes = int(time.time() - debut) // 60
            print(f'  [{minutes} min] traitement : {etat}')
            dernier = etat
        if etat in ('INVALID', 'FAILED'):
            print(f'::error::Apple a rejeté le build {BUILD} ({etat}).')
            print('Le motif exact (code ITMS-9xxxx) est envoyé par courriel au titulaire')
            print('du compte, et se lit dans App Store Connect → TestFlight → le build.')
            return 1
        if etat == 'VALID':
            return conclure(app, build, beta)
        time.sleep(INTERVALLE)
    print(f"::error::Après {ATTENTE_MAX // 60} minutes, Apple n'a toujours pas fini de traiter le build {BUILD}.")
    print('Il peut encore aboutir : à vérifier dans App Store Connect → TestFlight.')
    return 1


# L'état TestFlight du build pour les testeurs internes. Il n'existe qu'un
# moment après la validation : juste après, Apple répond encore sans lui.
DISPONIBLE = 'IN_BETA_TESTING'
PRET = 'READY_FOR_BETA_TESTING'
ECHECS = ('PROCESSING_EXCEPTION', 'EXPIRED')
ATTENTE_BETA = 10 * 60


def etat_beta(app: str):
    _, beta = chercher_build(app)
    return (beta or {}).get('attributes', {}).get('internalBuildState')


def groupes_du_build(build_id: str) -> list:
    reponse = appel('GET', f'/builds/{build_id}/betaGroups?fields[betaGroups]=name,isInternalGroup')
    return [g['attributes']['name'] for g in reponse['data']]


def distribuer(app: str, build_id: str) -> list:
    """Ajoute le build à chaque groupe de testeurs internes de l'app."""
    groupes = appel('GET', f'/apps/{app}/betaGroups?fields[betaGroups]=name,isInternalGroup&limit=50')['data']
    internes = [g for g in groupes if g['attributes'].get('isInternalGroup')]
    for g in internes:
        appel('POST', f'/betaGroups/{g["id"]}/relationships/builds',
              {'data': [{'type': 'builds', 'id': build_id}]})
    return [g['attributes']['name'] for g in internes]


def conclure(app: str, build, beta) -> int:
    attributs = build['attributes']
    if attributs.get('expired'):
        print(f'::error::Le build {BUILD} est expiré.')
        return 1
    if attributs.get('usesNonExemptEncryption') is None:
        # L'app ne chiffre rien elle-même : on le déclare, comme le ferait le
        # questionnaire de conformité d'App Store Connect.
        declarer_conformite(build)

    # « Validé » ne suffit pas : on attend qu'Apple dise où en est le build
    # pour les testeurs, et on le leur donne s'il est prêt sans l'être.
    debut, dernier, distribue = time.time(), object(), False
    while time.time() - debut < ATTENTE_BETA:
        interne = etat_beta(app)
        if interne != dernier:
            print(f'  état TestFlight : {interne or "pas encore connu"}')
            dernier = interne
        if interne == DISPONIBLE:
            groupes = groupes_du_build(build['id'])
            print(f'Le build {BUILD} est disponible pour les testeurs internes '
                  f'({", ".join(groupes) or "groupe sans nom"}).')
            return 0
        if interne == 'MISSING_EXPORT_COMPLIANCE':
            declarer_conformite(build)
        elif interne == PRET and not distribue:
            noms = distribuer(app, build['id'])
            distribue = True
            if not noms:
                print(f'::error::Le build {BUILD} est prêt, mais l\'app n\'a aucun groupe de testeurs internes.')
                print('À créer une fois : App Store Connect → ReNeuro → TestFlight → Tests internes → +,')
                print('avec ton identifiant Apple dedans, et « Accès automatique à tous les builds ».')
                return 1
            print(f'  build ajouté aux groupes internes : {", ".join(noms)}')
        elif interne in ECHECS:
            print(f'::error::TestFlight refuse le build {BUILD} ({interne}).')
            return 1
        time.sleep(INTERVALLE)
    print(f"::error::Build {BUILD} validé, mais toujours pas disponible pour les testeurs après "
          f"{ATTENTE_BETA // 60} minutes (dernier état : {dernier}).")
    return 1


def declarer_conformite(build) -> None:
    appel('PATCH', f"/builds/{build['id']}", {
        'data': {
            'type': 'builds',
            'id': build['id'],
            'attributes': {'usesNonExemptEncryption': False},
        },
    })
    print('  conformité à l\'export renseignée : aucun chiffrement propre.')


if __name__ == '__main__':
    sys.exit(main())
