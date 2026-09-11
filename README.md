# ReNeuro 🎓

Réapprendre les bases — les jours, l'heure, le calcul, organiser sa journée — en
questions courtes.

Ce dépôt contient deux choses :

| | |
| --- | --- |
| [`site/`](site/) | le **site de présentation**, en React + TypeScript, mobile first |
| [`ios/`](ios/) | l'**application**, native, en Swift / SwiftUI (iPhone et Apple Watch) |

## Le site

Une page, en React + TypeScript strict, pensée pour un téléphone d'abord. Elle
présente les seize thèmes, la planification, les trois écrans et les principes
d'apprentissage, puis l'offre : 4,99 € par mois, sans rien de gratuit.

```bash
cd site
npm install
npm run dev       # serveur de développement
npm run build     # site statique dans site/dist/
npm run preview   # prévisualiser le build
npm run check     # vérifier les types
```

`npm run build` produit un dossier **entièrement statique**, à chemins relatifs :
il se dépose tel quel sur n'importe quel hébergeur, à la racine d'un domaine
comme dans un sous-dossier. Le dépôt ne contient plus aucune configuration de
déploiement ni aucun serveur.

Voir [`site/README.md`](site/README.md) pour le détail.

## L'application

Native, en Swift / SwiftUI : seize thèmes, une mémoire de l'apprenant en SQLite,
une activité en direct sur l'écran verrouillé, et une application Apple Watch.
Voir [`ios/README.md`](ios/README.md).

Elle est compilée à chaque changement par une intégration continue, et peut être
envoyée sur TestFlight. Voir [`ios/CI.md`](ios/CI.md).

> Une version web du quiz a existé dans ce dépôt. Elle a été retirée : seule
> l'application native subsiste. On la retrouve dans l'historique git si besoin.

## Les icônes

Toutes les icônes viennent d'**un seul fichier vectoriel**,
[`icons/icon.svg`](icons/icon.svg), rasterisé par
[`icons/generate.ts`](icons/generate.ts) au cadrage propre à chaque plateforme
(cercle sur watchOS, squircle sur iOS). Voir [`icons/README.md`](icons/README.md).

## Un fichier à ne pas supprimer par mégarde

`site/public/sw.js` n'est pas le service worker du site — le site n'en a pas.
C'est un **service worker d'extinction** : l'ancienne version web avait laissé le
sien à la racine du domaine, et il y répond encore, de mémoire cache, chez les
personnes qui l'avaient ouverte. Ce fichier prend sa place, vide les caches et le
désinscrit. À conserver tant que le site est déposé sur le même domaine qu'elle.

## Crédits

Conçu et développé par **Maxime Nathan Lestage**.
