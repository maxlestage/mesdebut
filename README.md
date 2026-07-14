# Mes Débuts 🎓

Une petite application de quiz **React** (Vite), pensée **mobile first**, pour apprendre en s'amusant.

C'est aussi une **PWA** : une fois le site ouvert dans le navigateur du téléphone, on peut l'ajouter à l'écran d'accueil (« Ajouter à l'écran d'accueil » sur iOS, « Installer l'application » sur Android). Elle se lance alors en plein écran comme une vraie application et **fonctionne même sans connexion**.

## Démarrer

```bash
npm install
npm run dev       # serveur de développement
npm run build     # build de production dans dist/
npm run preview   # prévisualiser le build
npm start         # serveur de production (sert dist/, utilisé par Heroku)
```

## Déployer sur Heroku

[![Deploy to Heroku](https://www.herokucdn.com/deploy/button.svg)](https://heroku.com/deploy?template=https://github.com/maxlestage/mesdebut)

Ou avec la CLI Heroku :

```bash
heroku login
heroku create mesdebut        # ou le nom que tu veux
git push heroku master        # Heroku build et démarre tout seul
heroku open
```

Le buildpack Node.js de Heroku installe les dépendances, lance `heroku-postbuild`
(qui fait le `vite build`), puis démarre `npm start` (le petit serveur Express
`server.js` qui sert `dist/` avec les bons en-têtes de cache pour la PWA).

On peut aussi connecter le dépôt GitHub dans le dashboard Heroku
(Deploy → GitHub → Enable Automatic Deploys) pour déployer à chaque push.

## Ce qu'on peut apprendre

- 📅 **Les jours de la semaine** — révision de la liste, puis quiz (avant/après, position, etc.)
- 🗓️ **Les mois de l'année** — révision de la liste, puis quiz
- 🍂 **Les saisons** — révision (avec les mois de chaque saison), quiz d'ordre et d'association mois → saison
- 🔤 **L'alphabet** — révision en grille des 26 lettres, quiz (lettre d'avant/d'après, position)
- 🎨 **Les couleurs** — révision avec pastilles, reconnaissance visuelle, mélanges de peinture (bleu + jaune = vert) et association objet → couleur
- 📐 **Les formes géométriques** — 10 formes dessinées en SVG, reconnaissance visuelle, nombre de côtés et formes du quotidien (un panneau stop → octogone)
- 🔢 **Les nombres en lettres** — savoir lire et écrire les nombres de 0 à 100 (soixante-dix, quatre-vingts…), avec 3 niveaux
- ➕ **Addition**
- ➖ **Soustraction** (jamais de résultat négatif)
- ✖️ **Multiplication**
- ➗ **Division** (toujours des divisions exactes)

Les quiz de calcul ont trois niveaux : 🌱 Facile, 🌿 Moyen, 🌳 Difficile.

Chaque quiz comporte 10 questions à choix multiples, avec un score, des étoiles et des encouragements à la fin. 🌟

## Structure

```
src/
  questions.js          # données (jours, mois) et génération des questions
  App.jsx               # navigation entre les écrans
  components/
    Menu.jsx            # menu principal
    Category.jsx        # choix réviser / quiz (jours et mois)
    Learn.jsx           # écran de révision
    Levels.jsx          # choix du niveau (calcul)
    Quiz.jsx            # déroulement du quiz
    End.jsx             # écran de score
  styles.css            # styles mobile first
public/                 # icônes PWA (générées, écran d'accueil)
vite.config.js          # config Vite + manifest PWA (vite-plugin-pwa)
server.js               # serveur Express de production (Heroku)
Procfile                # commande de démarrage Heroku
app.json                # métadonnées pour le bouton « Deploy to Heroku »
```
