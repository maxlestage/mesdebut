import SwiftUI

struct EndView: View {
    let score: Int
    let total: Int
    var onReplay: () -> Void
    var onMenu: () -> Void

    var body: some View {
        let summary = endSummary(score: score, total: total)
        Group {
            ScreenTitle(text: "Quiz terminé !")
            Text(summary.stars).font(.system(size: 44))
            Text("\(score) / \(total) bonnes réponses")
                .font(.title3.bold())
                .foregroundColor(Color(hex: "#4a3f8f"))
            Text(summary.message)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
            BigButton(title: "🔁 Rejouer", action: onReplay)
            BigButton(title: "🏠 Menu principal", secondary: true, action: onMenu)
        }
    }
}
