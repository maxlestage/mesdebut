#!/usr/bin/env python3
"""Vérifie que le texte reste lisible dans les deux thèmes.

Le fond des écrans est `Color(.systemBackground)` : blanc en clair, noir en
sombre. Une couleur de texte figée est donc lisible dans un thème et pas dans
l'autre — l'énoncé du quiz est resté un temps à 1,66:1 sur fond noir, soit
illisible, sans que rien ne le signale.

Deux contrôles, sur le code source, sans appareil ni simulateur :

1. chaque couleur de la palette adaptative atteint le seuil WCAG AA (4,5:1)
   dans son propre thème ;
2. aucune couleur figée ne s'ajoute en dehors de celles, connues, qui reposent
   sur un fond lui aussi figé.
"""
import re
import sys
from pathlib import Path

SEUIL = 4.5
BLANC, NOIR = '#ffffff', '#000000'

# Ces deux-là sont posées sur un fond clair figé (#f4f2ff, les boutons-pilules) :
# elles se lisent dans les deux thèmes, et n'ont donc pas à s'adapter.
FIGEES_ADMISES = {'#6a5fc9': 2}


def luminance(h):
    h = h.lstrip('#')
    canaux = [int(h[i:i + 2], 16) / 255 for i in (0, 2, 4)]
    canaux = [c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4
              for c in canaux]
    return 0.2126 * canaux[0] + 0.7152 * canaux[1] + 0.0722 * canaux[2]


def contraste(a, b):
    la, lb = luminance(a), luminance(b)
    return (max(la, lb) + 0.05) / (min(la, lb) + 0.05)


def erreur(m):
    print(f'::error::{m}')


def main():
    vues = Path('ios/ReNeuro/ReNeuro/Views')
    if not vues.is_dir():
        erreur(f'{vues} introuvable — lancer depuis la racine du dépôt')
        return 1
    echecs = 0

    print('— palette adaptative —')
    theme = (vues / 'Theme.swift').read_text(encoding='utf-8')
    palette = re.findall(
        r'static let (\w+) = Color\(clair: "(#[0-9a-fA-F]{6})", '
        r'sombre: "(#[0-9a-fA-F]{6})"\)', theme)
    if not palette:
        erreur('aucune couleur adaptative trouvée dans Theme.swift')
        return 1
    for nom, clair, sombre in palette:
        rc, rs = contraste(clair, BLANC), contraste(sombre, NOIR)
        if rc < SEUIL or rs < SEUIL:
            fautif = 'en thème clair' if rc < SEUIL else 'en thème sombre'
            erreur(f'{nom} : {min(rc, rs):.2f}:1 {fautif}, '
                   f'sous le seuil de {SEUIL}:1')
            echecs += 1
        else:
            print(f'  ✅ {nom:12} {clair} {rc:>5.2f}:1 sur blanc   '
                  f'{sombre} {rs:>5.2f}:1 sur noir')

    print('\n— couleurs figées —')
    trouvees = {}
    for f in sorted(vues.glob('*.swift')):
        for h in re.findall(r'foregroundColor\(Color\(hex: "(#[0-9a-fA-F]{6})"\)\)',
                            f.read_text(encoding='utf-8')):
            trouvees[h.lower()] = trouvees.get(h.lower(), 0) + 1

    for h, n in sorted(trouvees.items()):
        admis = FIGEES_ADMISES.get(h)
        if admis is None:
            erreur(f'{h} : couleur de texte figée ({n} fois). Sur le fond '
                   f'adaptatif elle sera illisible dans un des deux thèmes — '
                   f'utiliser une couleur de la palette.')
            echecs += 1
        elif n != admis:
            erreur(f'{h} : {n} usages au lieu des {admis} attendus (fond figé). '
                   f'Vérifier que les nouveaux reposent bien sur un fond figé.')
            echecs += 1
        else:
            print(f'  ✅ {h} ×{n} — sur fond clair figé, lisible partout')

    print('\n✅ le texte est lisible dans les deux thèmes' if not echecs
          else f'\n{echecs} problème(s) de lisibilité')
    return 1 if echecs else 0


if __name__ == '__main__':
    sys.exit(main())
