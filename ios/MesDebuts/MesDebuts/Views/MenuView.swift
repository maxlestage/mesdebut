import SwiftUI

struct MenuView: View {
    let onSelect: (String) -> Void

    @State private var summary = MemoryStore.Summary()

    private let columns = [GridItem(.flexible(), spacing: 12), GridItem(.flexible(), spacing: 12)]

    var body: some View {
        Group {
            ScreenTitle(text: "🎓 Mes Débuts")
            ScreenSubtitle(text: "Choisis ce que tu veux apprendre !")
            if summary.answered > 0 {
                Text(memoryLine)
                    .font(.footnote)
                    .foregroundColor(.secondary)
                    .multilineTextAlignment(.center)
            }
            LazyVGrid(columns: columns, spacing: 12) {
                ForEach(QuestionEngine.categories) { c in
                    Button { onSelect(c.key) } label: {
                        VStack(spacing: 6) {
                            Text(c.emoji).font(.system(size: 34))
                            Text(c.label)
                                .font(.subheadline.bold())
                                .foregroundColor(.white)
                                .multilineTextAlignment(.center)
                        }
                        .frame(maxWidth: .infinity, minHeight: 96)
                        .padding(8)
                        .background(LinearGradient(colors: c.gradient,
                                                   startPoint: .topLeading, endPoint: .bottomTrailing))
                        .clipShape(RoundedRectangle(cornerRadius: 16))
                    }
                    .buttonStyle(.plain)
                }
            }
        }
        .onAppear { summary = MemoryStore.shared.summary() }
    }

    private var memoryLine: String {
        var parts = ["🧠 \(summary.mastered) notion\(summary.mastered > 1 ? "s" : "") maîtrisée\(summary.mastered > 1 ? "s" : "")"]
        if summary.due > 0 {
            parts.append("\(summary.due) à réviser")
        }
        return parts.joined(separator: " · ")
    }
}
