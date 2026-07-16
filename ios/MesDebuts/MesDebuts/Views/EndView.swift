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
                .foregroundColor(Color(hex: "#4a3f8f"))
            Text(summary.message)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
            if memory.mastered > 0 {
                Text("🧠 \(memory.mastered) notion\(memory.mastered > 1 ? "s" : "") maîtrisée\(memory.mastered > 1 ? "s" : "") au total")
                    .font(.subheadline.weight(.semibold))
                    .foregroundColor(Color(hex: "#4a3f8f"))
            }
            BigButton(title: "🔁 Rejouer", action: onReplay)
            BigButton(title: "🏠 Menu principal", secondary: true, action: onMenu)
        }
    }
}
