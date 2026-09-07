# Icônes des applications

Les icônes sont dessinées en **SVG** puis rasterisées en PNG 1024×1024 :
le vectoriel reste net à toutes les tailles, contrairement à un émoji
agrandi (la police Noto Color Emoji est une police bitmap plafonnée à
136 px, d'où le flou de l'icône iPhone historique).

- `watch-icon.svg` — icône de l'app Apple Watch. watchOS masque les icônes
  **en cercle** : tout le dessin tient dans le cercle inscrit, les coins
  ne portent rien.

Pour régénérer le PNG après avoir modifié un SVG :

```bash
npm install playwright-core
node ios/icons/render.mjs ios/icons/watch-icon.svg \
     ios/ReNeuro/ReNeuroWatch/Assets.xcassets/AppIcon.appiconset/icon-1024.png
```

Le PNG produit est en RGB **sans canal alpha** — Apple refuse les icônes
d'application transparentes.
