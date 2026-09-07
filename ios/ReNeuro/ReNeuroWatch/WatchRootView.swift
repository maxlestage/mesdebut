import SwiftUI

/// Écran d'accueil de la montre : la liste des catégories, puis le niveau
/// quand la catégorie en a un.
struct WatchRootView: View {

    var body: some View {
        NavigationStack {
            List {
                Section {
                    ForEach(QuestionEngine.categories) { category in
                        NavigationLink {
                            destination(for: category)
                        } label: {
                            WatchCategoryRow(category: category)
                        }
                    }
                } header: {
                    Text("Un quiz au poignet")
                }
            }
            .navigationTitle("ReNeuro")
        }
    }

    @ViewBuilder
    private func destination(for category: LearningCategory) -> some View {
        if category.hasLevels {
            WatchLevelsView(category: category)
        } else {
            WatchQuizView(category: category, level: 1)
        }
    }
}

struct WatchCategoryRow: View {
    let category: LearningCategory

    var body: some View {
        HStack(spacing: 10) {
            Text(category.emoji)
                .font(.title3)
                .frame(width: 30, height: 30)
                .background(
                    LinearGradient(colors: category.gradient,
                                   startPoint: .topLeading, endPoint: .bottomTrailing)
                        .opacity(0.35)
                )
                .clipShape(RoundedRectangle(cornerRadius: 8))
            Text(category.label)
                .font(.body)
                .lineLimit(1)
                .minimumScaleFactor(0.8)
        }
        .padding(.vertical, 2)
    }
}

struct WatchLevelsView: View {
    let category: LearningCategory

    var body: some View {
        List {
            ForEach(levelsFor(category)) { level in
                NavigationLink {
                    WatchQuizView(category: category, level: level.id)
                } label: {
                    Text("\(level.emoji) \(level.label)")
                        .lineLimit(1)
                        .minimumScaleFactor(0.8)
                }
            }
        }
        .navigationTitle(category.label)
    }
}
