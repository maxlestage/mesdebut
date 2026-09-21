import SwiftUI

struct EndView: View {
    let score: Int
    let total: Int
    var onReplay: () -> Void
    var onMenu: () -> Void

    var body: some View {
        let summary = endSummary(score: score, total: total)
        let memory = MemoryStore.shared.summary()
        Group {
            ScreenTitle(text: "Quiz terminé !")
            Text(summary.stars).font(.system(size: 44))
            Text("\(score) / \(total) bonnes réponses")
                .font(.title3.bold())
                .foregroundColor(.titre)
            Text(summary.message)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
            if memory.mastered > 0 {
                Text("🧠 \(memory.mastered) notion\(memory.mastered > 1 ? "s" : "") maîtrisée\(memory.mastered > 1 ? "s" : "") au total")
                    .font(.subheadline.weight(.semibold))
                    .foregroundColor(.titre)
            }
            BigButton(title: "🔁 Rejouer", action: onReplay)
            BigButton(title: "🏠 Menu principal", secondary: true, action: onMenu)
        }
    }
}
