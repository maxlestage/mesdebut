import SwiftUI

struct CategoryView: View {
    let category: LearningCategory
    var onLearn: () -> Void
    var onQuiz: () -> Void
    var onBack: () -> Void

    var body: some View {
        Group {
            ScreenTitle(text: "\(category.emoji) \(category.title)")
            BigButton(title: "📖 Réviser d'abord", action: onLearn)
            BigButton(title: "🎯 Faire le quiz", action: onQuiz)
            BackButton(title: "← Retour au menu", action: onBack)
        }
    }
}

struct LevelsView: View {
    let category: LearningCategory
    var onSelect: (Int) -> Void
    var onBack: () -> Void

    var body: some View {
        Group {
            ScreenTitle(text: "\(category.emoji) \(category.title)")
            ScreenSubtitle(text: "Choisis ton niveau")
            ForEach(LEVELS) { lvl in
                BigButton(title: "\(lvl.emoji) \(lvl.label)") { onSelect(lvl.id) }
            }
            BackButton(title: "← Retour au menu", action: onBack)
        }
    }
}
