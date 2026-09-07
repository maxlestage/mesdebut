# Icônes des applications

Toutes les icônes — iPhone, Apple Watch et web (PWA) — sont produites à partir
d'**un seul fichier vectoriel**, [`icon.svg`](icon.svg).

Le vectoriel reste net à toutes les tailles, contrairement à un émoji agrandi :
la police Noto Color Emoji est une police **bitmap plafonnée à 136 px**, ce qui
rendait floues les icônes précédentes, jusqu'à 1024 px.

## Régénérer

```bash
npm install playwright-core
node icons/generate.mjs
```

Le script écrit les six fichiers d'un coup :

| Fichier | Taille | Échelle | Pourquoi |
| --- | --- | --- | --- |
| `ios/…/ReNeuro/…/icon-1024.png` | 1024 | ×1,16 | iPhone — masque en squircle |
| `ios/…/ReNeuroWatch/…/icon-1024.png` | 1024 | ×1 | Apple Watch — masque en cercle |
| `public/apple-touch-icon.png` | 180 | ×1,16 | écran d'accueil iOS |
| `public/pwa-192.png` | 192 | ×1,16 | PWA |
| `public/pwa-512.png` | 512 | ×1,16 | PWA |
| `public/pwa-maskable-512.png` | 512 | ×1 | PWA maskable — zone sûre |
| `public/favicon.svg` | — | — | copie du vectoriel |

## Le cadrage

Chaque plateforme rogne l'icône différemment, et c'est le seul réglage qui
change d'une sortie à l'autre :

- **watchOS** masque en **cercle** — c'est la contrainte la plus sévère, donc
  `icon.svg` est dessiné pour elle : à l'échelle 1, tout tient dans le cercle
  inscrit.
- **PWA maskable** garde le **cercle central de 80 %** : l'échelle 1 convient
  également.
- **iOS** applique un **squircle**, qui ne rogne que les coins : le dessin est
  agrandi de 16 % pour mieux remplir le cadre.

Le PNG produit est en **RGB sans canal alpha** — Apple refuse les icônes
d'application transparentes.
