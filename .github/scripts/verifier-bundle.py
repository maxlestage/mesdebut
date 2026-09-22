#!/usr/bin/env python3
"""Vérifie ce que contient réellement une app iOS construite.

Deux contrôles, tous deux portant sur les `Info.plist` *produits par la
compilation* — les seuls qui décrivent ce qu'Apple recevra :

1. le nom affiché de chaque bundle ;
2. les orientations iPad, qu'Apple refuse au dépôt si elles sont incomplètes.

Le `project.pbxproj` écrit le nom de l'app sous forme échappée (`\\U2023`),
comme Xcode le fait lui-même. Lire le projet ne prouve donc rien : seul le
`Info.plist` *produit par la compilation* dit ce que l'iPhone affichera
réellement sous l'icône. C'est lui qu'on lit ici.

Usage : verifier-bundle.py <chemin/vers/ReNeuro.app>
"""
import plistlib
import sys
from pathlib import Path

ATTENDU = {
    'com.reneuro.ReNeuro': 'ReNeuro ‣',
    'com.reneuro.ReNeuro.Widget': 'ReNeuro',
    'com.reneuro.ReNeuro.watchkitapp': 'ReNeuro',
}
OBLIGATOIRE = 'com.reneuro.ReNeuro'

# Une app qui se déclare compatible iPad doit proposer les quatre orientations,
# sans quoi App Store Connect refuse le dépôt :
#   « Validation failed (409) Invalid bundle. […] you need to include all of
#     the […] orientations to support iPad multitasking. »
# Apple ne le vérifie qu'à l'envoi ; on le vérifie ici, à chaque build.
FAMILLE_IPAD = 2
ORIENTATIONS_IPAD = {
    'UIInterfaceOrientationPortrait',
    'UIInterfaceOrientationPortraitUpsideDown',
    'UIInterfaceOrientationLandscapeLeft',
    'UIInterfaceOrientationLandscapeRight',
}


def erreur(message):
    # Annoté pour que GitHub le fasse remonter dans le résumé de l'exécution.
    print(f'::error::{message}')


def main(argv):
    if len(argv) != 2:
        erreur('usage : verifier-noms-affiches.py <chemin/vers/ReNeuro.app>')
        return 2
    app = Path(argv[1])
    if not app.is_dir():
        erreur(f'{app} : bundle introuvable')
        return 1

    # On parcourt tout le bundle plutôt que de coder en dur PlugIns/ et Watch/ :
    # le contrôle survit ainsi à un déplacement des cibles embarquées.
    trouves = {}
    for plist in sorted(app.rglob('Info.plist')):
        try:
            contenu = plistlib.loads(plist.read_bytes())
        except Exception as e:                       # plist illisible = défaut
            erreur(f'{plist.relative_to(app)} : illisible ({e})')
            return 1
        identifiant = contenu.get('CFBundleIdentifier')
        if identifiant in ATTENDU:
            trouves[identifiant] = (
                contenu.get('CFBundleDisplayName'), plist.relative_to(app),
                contenu)

    echecs = 0
    for identifiant, voulu in ATTENDU.items():
        if identifiant not in trouves:
            if identifiant == OBLIGATOIRE:
                erreur(f'{identifiant} : aucun Info.plist dans {app.name}')
                echecs += 1
            else:
                print(f'  ·  {identifiant:34} absent de ce build')
            continue
        nom, chemin, _ = trouves[identifiant]
        if nom == voulu:
            print(f'  ✅ {identifiant:34} → {nom!r}   ({chemin})')
        else:
            erreur(f'{identifiant} : nom affiché {nom!r} au lieu de {voulu!r}')
            echecs += 1

    if echecs:
        print(f'\n{echecs} nom(s) incorrect(s)')
    else:
        print('\n✅ chaque bundle porte le nom voulu')

    echecs += controler_orientations(trouves)
    echecs += controler_conformite(trouves)
    return 1 if echecs else 0


def controler_conformite(trouves):
    """Sans cette clé, App Store Connect réclame la conformité export à la
    main pour chaque build, et le build reste invisible aux testeurs tant
    qu'on n'a pas répondu — sans que rien, côté dépôt, ne le signale."""
    print('\n— conformité export —')
    if OBLIGATOIRE not in trouves:
        return 0
    _, chemin, plist = trouves[OBLIGATOIRE]
    valeur = plist.get('ITSAppUsesNonExemptEncryption')
    if valeur is None:
        erreur(f'{OBLIGATOIRE} ne déclare pas ITSAppUsesNonExemptEncryption : '
               'chaque build restera bloqué sur la conformité export')
        print('     Réglage : INFOPLIST_KEY_ITSAppUsesNonExemptEncryption')
        return 1
    print(f'  ✅ déclarée : {valeur!r} — aucune question au dépôt  ({chemin})')
    return 0


def controler_orientations(trouves):
    """Reproduit la validation qu'Apple n'applique qu'au moment du dépôt."""
    print('\n— orientations —')
    if OBLIGATOIRE not in trouves:
        return 0                      # déjà signalé comme nom manquant
    _, chemin, plist = trouves[OBLIGATOIRE]
    familles = plist.get('UIDeviceFamily') or []
    if FAMILLE_IPAD not in familles:
        print(f'  ·  app iPhone seule (UIDeviceFamily = {familles}) :')
        print('     les orientations iPad ne s'"'"'appliquent pas')
        return 0

    declarees = set(plist.get('UISupportedInterfaceOrientations~ipad') or [])
    absentes = ORIENTATIONS_IPAD - declarees
    if absentes:
        erreur(f'{OBLIGATOIRE} se déclare compatible iPad mais il manque '
               f'{len(absentes)} orientation(s) : {", ".join(sorted(absentes))}')
        print('     App Store Connect refuserait le dépôt (409, Invalid bundle).')
        print(f'     Déclarées : {", ".join(sorted(declarees)) or "aucune"}')
        print(f'     Réglage à corriger : '
              f'INFOPLIST_KEY_UISupportedInterfaceOrientations_iPad')
        return 1
    print(f'  ✅ compatible iPad, les quatre orientations sont déclarées  ({chemin})')
    return 0


if __name__ == '__main__':
    sys.exit(main(sys.argv))
