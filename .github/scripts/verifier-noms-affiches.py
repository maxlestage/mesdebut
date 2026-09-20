#!/usr/bin/env python3
"""Vérifie le nom affiché de chaque bundle d'une app iOS construite.

Le `project.pbxproj` écrit le nom de l'app sous forme échappée (`\\U2023`),
comme Xcode le fait lui-même. Lire le projet ne prouve donc rien : seul le
`Info.plist` *produit par la compilation* dit ce que l'iPhone affichera
réellement sous l'icône. C'est lui qu'on lit ici.

Usage : verifier-noms-affiches.py <chemin/vers/ReNeuro.app>
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
                contenu.get('CFBundleDisplayName'), plist.relative_to(app))

    echecs = 0
    for identifiant, voulu in ATTENDU.items():
        if identifiant not in trouves:
            if identifiant == OBLIGATOIRE:
                erreur(f'{identifiant} : aucun Info.plist dans {app.name}')
                echecs += 1
            else:
                print(f'  ·  {identifiant:34} absent de ce build')
            continue
        nom, chemin = trouves[identifiant]
        if nom == voulu:
            print(f'  ✅ {identifiant:34} → {nom!r}   ({chemin})')
        else:
            erreur(f'{identifiant} : nom affiché {nom!r} au lieu de {voulu!r}')
            echecs += 1

    print('\n✅ chaque bundle porte le nom voulu' if not echecs
          else f'\n{echecs} nom(s) incorrect(s)')
    return 1 if echecs else 0


if __name__ == '__main__':
    sys.exit(main(sys.argv))
