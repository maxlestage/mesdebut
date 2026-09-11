# Site de présentation de ReNeuro 🌐

Site vitrine de l'application, en **React + TypeScript** (Vite), pensé **mobile
first** : toute la mise en page est écrite pour un téléphone, et les écrans
larges sont traités dans les seules media queries de `src/styles.css`.

C'est un projet **indépendant** de l'application : ses propres dépendances, son
propre build, sa propre sortie statique. Il ne touche à rien de `src/`.

## Démarrer

```bash
cd site
npm install
npm run dev      # serveur de développement
npm run build    # site statique dans site/dist/
npm run preview  # prévisualiser le build
npm run check    # vérifier les types
```

## Déployer

`npm run build` produit un dossier `site/dist/` entièrement statique (aucun
serveur nécessaire). Les chemins sont relatifs (`base: './'` dans
`vite.config.ts`), donc il fonctionne aussi bien à la racine d'un domaine que
dans un sous-dossier.

`site/dist/` se dépose où tu veux. Le dépôt contient aussi de quoi le publier sur
Heroku avec le seul buildpack Node — voir le
[README principal](../README.md).

`site/public/sw.js` n'est pas le service worker du site — le site n'en a pas.
C'est un **service worker d'extinction**, servi à `/sw.js`, qui retire celui que
l'ancienne version web avait laissé à la racine du domaine. Voir le
[README principal](../README.md).

## Les thèmes

`src/donnees.ts` liste les seize thèmes présentés. Ils reprennent ceux de
l'application iOS, qui est désormais la seule implémentation : les deux listes
sont écrites séparément, donc en ajouter un ici suppose de le retrouver là-bas.

## Structure

```
index.html              # métadonnées, titre, Open Graph
src/
  main.tsx              # point d'entrée
  App.tsx               # assemblage des sections
  donnees.ts            # contenu typé (thèmes, supports, principes)
  styles.css            # styles mobile first
  components/
    Entete.tsx          # bandeau d'accueil
    Categories.tsx      # grille des seize thèmes
    Planification.tsx   # mise en avant de « Planifier » et « Ma journée »
    Supports.tsx        # web, iPhone, Apple Watch
    Principes.tsx       # les six principes d'apprentissage
    Tarif.tsx           # l'offre : prix, ce qui est compris, disponibilité
    Pied.tsx            # crédits et mention
public/icon.svg         # logo (produit par icons/generate.ts)
public/sw.js            # service worker d'extinction (voir plus haut)
```

## Crédits

Conçu et développé par **Maxime Nathan Lestage**.
