# Icônes des applications

Les icônes — iPhone, Apple Watch et application web — sont produites à partir d'**un seul fichier
vectoriel**, [`icon.svg`](icon.svg), que le site utilise aussi tel quel.

Le vectoriel reste net à toutes les tailles, contrairement à un émoji agrandi :
la police Noto Color Emoji est une police **bitmap plafonnée à 136 px**, ce qui
rendait floues les icônes précédentes, jusqu'à 1024 px.

## Régénérer

```bash
cd icons
npm install
npm run generate
```

Le script repère ses sorties depuis la racine du dépôt, pas depuis le dossier
courant : il se lance donc d'où l'on veut.

Le script écrit tous les fichiers d'un coup :

| Fichier | Taille | Échelle | Pourquoi |
| --- | --- | --- | --- |
| `ios/…/ReNeuro/…/icon-1024.png` | 1024 | ×1,16 | iPhone — masque en squircle |
| `ios/…/ReNeuroWatch/…/icon-1024.png` | 1024 | ×1 | Apple Watch — masque en cercle |
| `site/public/app/pwa-192.png` | 192 | ×1,16 | application web installée (manifeste) |
| `site/public/app/pwa-512.png` | 512 | ×1,16 | application web installée, écran d'accueil iOS |
| `site/public/app/pwa-maskable-512.png` | 512 | ×1 | Android — masque libre, zone sûre = cercle central |
| `site/public/icon.svg` | — | — | le site utilise directement le vectoriel |

## Le cadrage

Chaque plateforme rogne l'icône différemment, et c'est le seul réglage qui
change d'une sortie à l'autre :

- **watchOS** masque en **cercle** — c'est la contrainte la plus sévère, donc
  `icon.svg` est dessiné pour elle : à l'échelle 1, tout tient dans le cercle
  inscrit.
- **iOS** applique un **squircle**, qui ne rogne que les coins : le dessin est
  agrandi de 16 % pour mieux remplir le cadre.

Les PNG produits sont en **RGB sans canal alpha** — Apple refuse les icônes
d'application transparentes.
