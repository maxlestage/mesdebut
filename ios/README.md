# ReNeuro — application iOS native (SwiftUI) 🎓

L'appli iPhone et Apple Watch, en **Swift / SwiftUI**. L'interface est entièrement
native (billes, formes, accordéon, animations SwiftUI) ; la logique pédagogique —
génération des questions, répétition espacée, entrelacement, planification —
vient du **noyau Rust** commun avec le web ([`../noyau/`](../noyau/README.md)).

## Ouvrir et lancer

Il faut un **Mac avec Xcode 16 ou plus récent** (le projet utilise les groupes de
fichiers synchronisés, disponibles à partir de Xcode 16), et **Rust**, installé
avec [rustup](https://rustup.rs) : Xcode compile le noyau à chaque build (phase
« Noyau Rust », `noyau/xcode.sh`) et ajoute tout seul les cibles Apple au premier
passage.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh   # une fois
open ios/ReNeuro/ReNeuro.xcodeproj
```

1. Sélectionne un simulateur iPhone (ou ton iPhone branché) dans la barre en haut.
2. Appuie sur ▶︎ (Cmd-R) pour compiler et lancer.

Le dossier `ReNeuro/` est un **groupe synchronisé** : un fichier Swift ajouté
dedans est repris automatiquement par le projet, sans manipulation.

Pour l'installer sur un vrai iPhone ou le publier sur l'App Store, il faut un
**compte Apple Developer** (99 €/an) et régler la signature dans
*Signing & Capabilities* (choisir ton équipe ; le bundle id par défaut est
`com.reneuro.ReNeuro`, à personnaliser).

## Le nom sous l'icône

L'app iPhone s'appelle **ReNeuro ‣**. Les deux autres cibles gardent
« ReNeuro » : le widget et l'app Apple Watch n'ont pas d'icône propre sur
l'écran d'accueil de l'iPhone.

Le caractère `‣` (U+2023, *triangular bullet*) est écrit **tel quel** dans
le `project.pbxproj`, et non sous sa forme échappée `\U2023` : le fichier
s'ouvre sur l'en-tête `// !$*UTF8*$!`, par lequel Xcode déclare lui-même que
son contenu est en UTF-8.

Ce n'est pas une question de goût : le nom rendu est vérifié à chaque build par
la CI, qui lit le `Info.plist` *produit par la compilation* — voir
`.github/scripts/verifier-noms-affiches.py`.

## Structure

Le projet contient **trois cibles** : l'app iPhone, l'extension widget qui dessine
l'activité en direct, et l'app Apple Watch. Les dossiers `ReNeuroKit/` et
`ReNeuroActivity/` sont partagés entre plusieurs cibles. Le moteur, lui, n'est pas
ici : il vit dans le noyau Rust, commun avec le web, que `ReNeuroKit/Noyau.swift`
appelle par son interface C.

```
ReNeuro.xcodeproj        # projet Xcode (versionné, 3 cibles)

ReNeuroKit/              # partagé : app iPhone + app Watch
  Noyau.swift                 # le pont vers le noyau Rust : un appel, une réponse JSON
  Models.swift                # types (Question, catégories, révision) + utilitaires (Color hex, cap)
  QuestionEngine.swift        # les quiz, demandés au noyau
  CategoryData.swift          # les thèmes et le contenu des écrans de révision, du noyau
  MemoryStore.swift           # mémoire de l'apprenant en SQLite (règle des boîtes : le noyau)
  PlanningData.swift          # activités du quotidien décomposées en étapes, du noyau
  HanoiEngine.swift           # les tours de Hanoï, jouées par le noyau
  StepsView.swift             # étapes montrées sous l'énoncé (avec le trou)
  MarblesView.swift           # billes à compter (Circle + dégradés)
  GeometricShapeView.swift    # formes géométriques (Path/Shape)
  ClockView.swift             # horloge analogique à aiguilles

ReNeuroActivity/         # partagé : app iPhone + extension widget
  QuizActivityAttributes.swift  # ce que l'écran verrouillé affiche du quiz en cours

ReNeuro/                 # app iPhone
  ReNeuroApp.swift            # point d'entrée @main
  Services/
    LiveActivity.swift        # démarre, met à jour et termine l'activité en direct
    PlanStore.swift           # « Ma journée » : stockage, et calculs demandés au noyau
  Views/
    RootView.swift            # navigation entre les écrans
    MenuView.swift            # menu principal (grille de catégories)
    CategoryView.swift        # écrans « Réviser / Quiz » et choix de niveau
    LearnView.swift           # révision (liste, grille, accordéon)
    QuizView.swift            # déroulement du quiz + répétition espacée
    EndView.swift             # score et étoiles
    ProgressStatsView.swift   # écran « Mes progrès » (stats SQLite par catégorie)
    PlannerView.swift         # « 🗓️ Ma journée » : le planificateur
    HanoiView.swift           # « 🗼 Tours de Hanoï » : le mini-jeu
    Haptics.swift             # retour tactile natif sur les réponses
    Theme.swift               # fonds, boutons, carte réutilisables
  Assets.xcassets/            # icône de l'app + couleur d'accent

ReNeuroWidget/           # extension widget (activité en direct)
  ReNeuroWidgetBundle.swift     # point d'entrée @main de l'extension
  QuizLiveActivityWidget.swift  # écran verrouillé + Dynamic Island
ReNeuroWidget-Info.plist        # déclare l'extension comme extension WidgetKit

ReNeuroWatch/            # app Apple Watch
  ReNeuroWatchApp.swift       # point d'entrée @main
  WatchRootView.swift         # liste des catégories et des niveaux
  WatchQuizView.swift         # quiz au poignet + écran de score
  Assets.xcassets/            # icône de la montre + couleur d'accent
```

La source vectorielle des icônes et le script qui les rasterise sont dans
[`../icons/`](../icons/) — un seul SVG pour l'iPhone, la montre et le web.

## Activité en direct (écran verrouillé et Dynamic Island) 🔒

Pendant un quiz, l'iPhone affiche une **activité en direct** : l'émoji et le nom
de la catégorie, le score en étoiles, une barre de progression et la question en
cours. À chaque réponse, l'activité indique brièvement si c'était juste ou faux,
puis revient au compteur « Question 4 / 10 ». Elle se termine avec le quiz et
laisse le score visible quelques secondes ; en cas d'abandon, elle disparaît
tout de suite.

Sur les iPhone à Dynamic Island, la même chose est déclinée en version compacte
(émoji + score), minimale et déployée.

Tout cela est facultatif : sur un iPhone antérieur à iOS 16.2, ou si les
activités en direct sont désactivées dans les Réglages, l'appel est sans effet
et le quiz se déroule exactement pareil.

## Application Apple Watch ⌚

Une **app watchOS autonome** (watchOS 10 minimum), avec les mêmes catégories, le
même moteur de questions et la même répétition espacée que sur l'iPhone, dans une
mise en page pensée pour un petit écran : liste des catégories, choix du niveau
quand il y en a un, questions à faire défiler, réponses en gros boutons, et un
écran de score avec les étoiles.

Les horloges, les billes, les formes et les pastilles de couleur sont dessinées
avec le même code que sur l'iPhone — apprendre à lire l'heure sur une montre est
d'ailleurs le plus naturel des exercices. Les bonnes et mauvaises réponses sont
confirmées par le **retour haptique** du poignet.

La montre tient sa propre base SQLite : elle apprend ce que l'enfant y travaille,
indépendamment de l'iPhone.

## Planifier 📋

Planifier — décomposer une activité en étapes, les ordonner, estimer le temps
qu'elles prennent — est une **fonction exécutive**, souvent touchée après un AVC,
et qui se rééduque. La catégorie a donc deux volets.

Les **exercices** portent sur 11 activités du quotidien (se faire un café, faire
une lessive, prendre le bus…) décomposées en étapes qui s'enchaînent vraiment :
chaque étape suppose la précédente, il n'y a donc qu'un ordre correct. Trois
niveaux : 🌱 les étapes, 🌿 l'ordre (étape manquante, intrus), 🌳 le temps
(durée, heure de fin, tenir dans un créneau).

**« 🗓️ Ma journée »** est le planificateur : on écrit ses tâches ou on prend une
activité connue avec sa durée, on les ordonne, on les coche. Chaque tâche démarre
quand la précédente se termine, donc l'heure de chacune s'affiche et la journée
se voit déborder. Une tâche venue d'une activité connue peut dérouler ses étapes,
pour faire le lien entre l'exercice et la vraie vie.

Chaque tâche a un **chronomètre**. On la démarre — ce qui arrête celle qui
tournait, puisqu'on fait une chose à la fois — et à l'arrêt l'app affiche
« 15 → 25 min » : le temps prévu contre le temps réellement passé, en vert si
l'estimation tombait juste à 20 % près. Un bilan totalise la journée. C'est
précisément cet écart entre l'estimation et le réel qu'on travaille, et la
formulation ne reproche jamais un dépassement : le voir suffit à progresser.

Une journée mesurée ne se perd pas : au changement de date, elle part dans un
**historique des estimations** (60 jours au plus). L'écran affiche alors les dix
derniers jours en barres, et compare les cinq récents aux cinq précédents — c'est
là que se voit la progression, bien plus que sur une journée isolée.

La journée et l'historique sont gardés dans les réglages utilisateur (de petits
documents JSON remplacés en bloc) et non dans la base SQLite, réservée à la
mémoire de l'apprenant — des lignes qu'on requête. La journée repart à zéro le
lendemain.

L'app Watch reprend automatiquement les **exercices** de planification, puisque le
moteur est partagé ; le planificateur, lui, reste sur l'iPhone.

## Fonctionnalités

Parité complète avec la version web : les 18 catégories (mélange, jours, mois,
saisons, alphabet, heure, planifier, couleurs, formes, chiffres, jusqu'à 50,
nombres, les 4 opérations, les jeux de logique et les casse-tête), les 3 niveaux, la révision (avec accordéon pour les nombres), la
répétition espacée en cas d'erreur, et le mode Mélange entrelacé.

Touches natives : **retour tactile** (haptique) sur les bonnes/mauvaises réponses,
et un écran **« 📊 Mes progrès »** (accessible depuis le menu) qui affiche, à
partir de la base SQLite, les statistiques par catégorie (vues, % de réussite,
notions maîtrisées) avec possibilité de tout réinitialiser.

## Mémoire persistée (SQLite) 🧠

`Models/MemoryStore.swift` gère la mémoire de l'apprenant dans une base **SQLite**
(via le module système `SQLite3`, sans dépendance externe — la bibliothèque est
liée par `OTHER_LDFLAGS = -lsqlite3`). La base est créée dans le dossier
*Application Support* de l'app.

- Chaque réponse est enregistrée par notion (`item_memory`) avec un système de
  **boîtes de Leitner** (1 à 5) : une bonne réponse fait monter la notion d'une
  boîte (révision plus espacée), une erreur la fait redescendre. Une date de
  prochaine révision (`due`) est calculée à chaque fois.
- C'est de la répétition espacée qui **survit d'une session à l'autre**, et même
  d'un jour à l'autre — le prolongement naturel des principes de neurosciences.
- Chaque quiz terminé est aussi historisé (`sessions`).
- Le menu et l'écran de fin affichent la progression : notions maîtrisées
  (boîte ≥ 4) et notions à réviser aujourd'hui.
- **Sélection guidée par la mémoire** : pour les catégories à ensemble fini
  (jours, mois, saisons, alphabet, couleurs, formes, chiffres, jusqu'à 50,
  nombres), le quiz fait revenir **en priorité les notions dont la date de
  révision est dépassée**, puis les notions jamais vues, enfin le reste. Le
  calcul (addition, soustraction, multiplication, division) reste aléatoire
  car son espace de questions est infini.

## Intégration continue (CI)

- **Build automatique** à chaque push/PR touchant `ios/**` : l'app iPhone (qui
  entraîne l'extension widget et l'app Watch), puis l'app Watch seule pour que
  les erreurs propres à watchOS soient bien attribuées. Sur simulateur, sans
  signature — aucun secret requis.
- **Envoi TestFlight** avec `xcodebuild` + `xcrun altool` (manuel ou tag `ios-v*`).

Voir [`CI.md`](CI.md) pour la configuration des secrets et le déclenchement.

## Le noyau Rust

Questions, thèmes, révisions, répétition espacée et calculs de « Ma journée »
viennent de [`../noyau/`](../noyau/README.md), le même code que l'application web.
Swift garde l'interface et le stockage : la base SQLite de la mémoire et les
réglages utilisateur de la journée, dont le format ne change pas.

Le pont se vérifie sans Mac : [`pont/verifier.sh`](pont/verifier.sh) compile les
fichiers sans interface de `ReNeuroKit` et `Services/PlanStore.swift` avec Swift
sous Linux, contre le noyau, et contrôle chaque appel (thèmes, révisions, quiz de
chaque niveau, boîtes de Leitner, journée). Le workflow `noyau.yml` le lance à
chaque changement.

## Réglages du projet

- Cible de déploiement : iOS 16.0 (extension widget iOS 16.2, montre watchOS 10.0)
- Orientation : portrait
- iPhone et iPad
- Icônes : 🎓 sur dégradé violet, couleur d'accent `#667eea`. Elles sont
  **dessinées en SVG** (`icons/icon.svg`) plutôt qu'agrandies depuis un émoji,
  et cadrées selon le masque de chaque plateforme — cercle sur la montre,
  squircle sur l'iPhone. Voir [`icons/README.md`](../icons/README.md).

## Crédits

Conçu et développé par **Maxime Nathan Lestage**.
