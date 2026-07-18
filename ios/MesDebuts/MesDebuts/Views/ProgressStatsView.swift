import SwiftUI

struct ProgressStatsView: View {
    var onBack: () -> Void

    @State private var stats: [String: MemoryStore.CategoryStat] = [:]
    @State private var summary = MemoryStore.Summary()
    @State private var showReset = false

    // catégories réellement pratiquées, dans l'ordre du menu
    private var rows: [LearningCategory] {
        QuestionEngine.categories.filter { (stats[$0.key]?.seen ?? 0) > 0 }
    }

    var body: some View {
        Group {
            ScreenTitle(text: "📊 Mes progrès")

            if summary.answered == 0 {
                ScreenSubtitle(text: "Fais un quiz pour voir tes progrès apparaître ici ! 🌱")
            } else {
                Text("🧠 \(summary.mastered) notion\(summary.mastered > 1 ? "s" : "") maîtrisée\(summary.mastered > 1 ? "s" : "") · \(summary.correct)/\(summary.answered) réussies")
                    .font(.subheadline.weight(.semibold))
                    .foregroundColor(Color(hex: "#4a3f8f"))
                    .multilineTextAlignment(.center)

                VStack(spacing: 8) {
                    ForEach(rows) { c in
                        CategoryStatRow(category: c, stat: stats[c.key] ?? MemoryStore.CategoryStat())
                    }
                }

                Button { showReset = true } label: {
                    Text("♻︎ Réinitialiser mes progrès")
                        .font(.callout.weight(.semibold))
                        .foregroundColor(Color(hex: "#e04545"))
                        .padding(.top, 4)
                }
                .buttonStyle(.plain)
            }

            BackButton(title: "← Retour au menu", action: onBack)
        }
        .onAppear {
            stats = MemoryStore.shared.categoryStats()
            summary = MemoryStore.shared.summary()
        }
        .confirmationDialog("Effacer toute la progression ?", isPresented: $showReset, titleVisibility: .visible) {
            Button("Tout réinitialiser", role: .destructive) {
                MemoryStore.shared.reset()
                stats = MemoryStore.shared.categoryStats()
                summary = MemoryStore.shared.summary()
            }
            Button("Annuler", role: .cancel) {}
        }
    }
}

struct CategoryStatRow: View {
    let category: LearningCategory
    let stat: MemoryStore.CategoryStat

    private var ratio: Double {
        stat.seen > 0 ? Double(stat.correct) / Double(stat.seen) : 0
    }

    var body: some View {
        HStack(spacing: 12) {
            Text(category.emoji).font(.system(size: 26))
            VStack(alignment: .leading, spacing: 4) {
                HStack {
                    Text(category.label)
                        .font(.subheadline.weight(.semibold))
                        .foregroundColor(Color(hex: "#4a3f8f"))
                    Spacer()
                    Text("\(Int((ratio * 100).rounded()))%")
                        .font(.caption.weight(.bold))
                        .foregroundColor(.secondary)
                }
                bar
                Text("\(stat.seen) vue\(stat.seen > 1 ? "s" : "") · \(stat.mastered) maîtrisée\(stat.mastered > 1 ? "s" : "")")
                    .font(.caption2)
                    .foregroundColor(.secondary)
            }
        }
        .padding(.horizontal, 14)
        .padding(.vertical, 10)
        .frame(maxWidth: .infinity, alignment: .leading)
        .background(Color(hex: "#f4f2ff"))
        .clipShape(RoundedRectangle(cornerRadius: 10))
    }

    private var bar: some View {
        GeometryReader { geo in
            ZStack(alignment: .leading) {
                Capsule().fill(Color(hex: "#e6e2f7"))
                Capsule().fill(brandGradient)
                    .frame(width: max(0, min(1, ratio)) * geo.size.width)
            }
        }
        .frame(height: 8)
    }
}
