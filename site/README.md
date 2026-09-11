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
npm run check    # types + cohérence des données avec l'application
```

## Déployer

`npm run build` produit un dossier `site/dist/` entièrement statique (aucun
serveur nécessaire). Les chemins sont relatifs (`base: './'` dans
`vite.config.ts`), donc il fonctionne aussi bien à la racine d'un domaine que
dans un sous-dossier.

**En production**, il occupe **la racine** du déploiement, l'application étant
servie sous `/app` : `server.js` monte `site/dist/` à la racine, et
`heroku-postbuild` construit les deux.

`site/public/sw.js` n'est pas le service worker du site — le site n'en a pas.
C'est un **service worker d'extinction**, servi à `/sw.js`, qui retire celui que
l'application avait laissé à la racine avant son déménagement. Voir le
[README principal](../README.md#le-déménagement-de-lapplication).

## Les données restent alignées sur l'application

`src/donnees.ts` réécrit la liste des seize thèmes plutôt que d'importer celle de
l'application : le site reste ainsi un projet autonome et typé.

Le risque, c'est la dérive silencieuse — un thème ajouté ou renommé dans l'app et
oublié ici. `scripts/verifie-donnees.mjs` compare donc les deux listes (clés,
ordre, émojis, libellés, titres, niveaux) et échoue si elles divergent :

```bash
npm run check
```

## L'offre

Le site présente une offre **payante** : 4,99 € par mois, et **rien de gratuit**
— ni essai, ni accès libre. Concrètement, le site ne contient **aucun lien vers
l'application** : son seul lien mène à l'ancre des thèmes, plus bas dans la page.

Tant que l'application n'est pas publiée, l'en-tête annonce « Bientôt sur
l'App Store » sous forme de **mention, pas de bouton** : il n'y a rien à ouvrir,
et un bouton inerte ne ferait qu'induire en erreur. Le seul geste possible étant
de parcourir les thèmes, c'est lui qui garde le bouton plein.

Le prix et ce qu'il comprend sont dans `OFFRE`, dans `src/donnees.ts`.

> ⚠️ L'application reste servie sous `/app` et **joignable par son adresse
> directe** : seul le site cesse d'y mener. Réserver l'accès aux personnes
> abonnées demanderait une authentification et un paiement, qui n'existent pas
> encore.

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
public/icon.svg         # logo (copie de icons/icon.svg)
scripts/
  verifie-donnees.mjs   # garde-fou contre la dérive avec l'application
```

## Crédits

Conçu et développé par **Maxime Nathan Lestage**.
