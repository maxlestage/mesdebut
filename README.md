# Mes Débuts 🎓

Une petite application de quiz **React** (Vite), pensée **mobile first**, pour apprendre en s'amusant.

C'est aussi une **PWA** : une fois le site ouvert dans le navigateur du téléphone, on peut l'ajouter à l'écran d'accueil (« Ajouter à l'écran d'accueil » sur iOS, « Installer l'application » sur Android). Elle se lance alors en plein écran comme une vraie application et **fonctionne même sans connexion**.

## Démarrer

```bash
npm install
npm run dev       # serveur de développement
npm run build     # build de production dans dist/
npm run preview   # prévisualiser le build
```

## Ce qu'on peut apprendre

- 📅 **Les jours de la semaine** — révision de la liste, puis quiz (avant/après, position, etc.)
- 🗓️ **Les mois de l'année** — révision de la liste, puis quiz
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
```
