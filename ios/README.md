# Mes Débuts — application iOS native (SwiftUI) 🎓

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
    MarblesView.swift         # billes à compter (Circle + dégradés)
    GeometricShapeView.swift  # formes géométriques (Path/Shape)
    Theme.swift               # fonds, boutons, carte réutilisables
  Assets.xcassets/            # icône de l'app + couleur d'accent
```

## Fonctionnalités

Parité complète avec la version web : les 14 catégories (mélange, jours, mois,
saisons, alphabet, couleurs, formes, chiffres, jusqu'à 50, nombres, et les 4
opérations), les 3 niveaux, la révision (avec accordéon pour les nombres), la
répétition espacée en cas d'erreur, et le mode Mélange entrelacé.

## Réglages du projet

- Cible de déploiement : iOS 16.0
- Orientation : portrait
- iPhone et iPad
- Icône générée (🎓 sur dégradé violet), couleur d'accent `#667eea`
