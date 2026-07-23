# Mes Débuts AVC — application iOS native (SwiftUI) 🎓

Version **100 % native** de l'appli, écrite en **Swift / SwiftUI**. Toute la logique
pédagogique du quiz (génération des questions, répétition espacée, entrelacement,
sens du nombre…) a été portée depuis la version web, et l'interface est entièrement
native (billes, formes, accordéon, animations SwiftUI).

## Ouvrir et lancer

Il faut un **Mac avec Xcode 16 ou plus récent** (le projet utilise les groupes de
fichiers synchronisés, disponibles à partir de Xcode 16).

```bash
open ios/MesDebuts/MesDebuts.xcodeproj
```

1. Sélectionne un simulateur iPhone (ou ton iPhone branché) dans la barre en haut.
2. Appuie sur ▶︎ (Cmd-R) pour compiler et lancer.

Pour l'installer sur un vrai iPhone ou le publier sur l'App Store, il faut un
**compte Apple Developer** (99 €/an) et régler la signature dans
*Signing & Capabilities* (choisir ton équipe ; le bundle id par défaut est
`com.mesdebuts.MesDebuts`, à personnaliser).

## Structure

```
MesDebuts/
  MesDebutsApp.swift          # point d'entrée @main
  Models/
    Models.swift              # types (Question, catégories, révision) + utilitaires (Color hex, cap)
    QuestionEngine.swift      # port de questions.js : données + génération des questions
    CategoryData.swift        # métadonnées des catégories + contenu des écrans de révision
  Views/
    RootView.swift            # navigation entre les écrans
    MenuView.swift            # menu principal (grille de catégories)
    CategoryView.swift        # écrans « Réviser / Quiz » et choix de niveau
    LearnView.swift           # révision (liste, grille, accordéon)
    QuizView.swift            # déroulement du quiz + répétition espacée
    EndView.swift             # score et étoiles
    ProgressStatsView.swift   # écran « Mes progrès » (stats SQLite par catégorie)
    MarblesView.swift         # billes à compter (Circle + dégradés)
    GeometricShapeView.swift  # formes géométriques (Path/Shape)
    Haptics.swift             # retour tactile natif sur les réponses
    Theme.swift               # fonds, boutons, carte réutilisables
  Assets.xcassets/            # icône de l'app + couleur d'accent
```

## Fonctionnalités

Parité complète avec la version web : les 14 catégories (mélange, jours, mois,
saisons, alphabet, couleurs, formes, chiffres, jusqu'à 50, nombres, et les 4
opérations), les 3 niveaux, la révision (avec accordéon pour les nombres), la
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

- **Build automatique** à chaque push/PR touchant `ios/**` (compile l'app sur
  simulateur, sans signature — aucun secret requis).
- **Envoi TestFlight** via fastlane (manuel ou tag `ios-v*`).

Voir [`CI.md`](CI.md) pour la configuration des secrets et le déclenchement.

## Réglages du projet

- Cible de déploiement : iOS 16.0
- Orientation : portrait
- iPhone et iPad
- Icône générée (🎓 sur dégradé violet), couleur d'accent `#667eea`
