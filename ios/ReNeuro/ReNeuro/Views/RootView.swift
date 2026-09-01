import SwiftUI

struct RootView: View {
    private enum Screen { case menu, category, learn, levels, quiz, end, progress }

    @State private var screen: Screen = .menu
    @State private var categoryKey: String = "melange"
    @State private var level: Int = 1
    @State private var resultScore = 0
    @State private var resultTotal = 0
    @State private var quizID = UUID()

    private var cat: LearningCategory { QuestionEngine.category(categoryKey) }

    var body: some View {
        ZStack {
            brandGradient.ignoresSafeArea()
            ScreenCard { content }
        }
    }

    @ViewBuilder private var content: some View {
        switch screen {
        case .menu:
            MenuView(onSelect: open, onProgress: { screen = .progress })
        case .category:
            CategoryView(category: cat,
                         onLearn: { screen = .learn },
                         onQuiz: goToQuiz,
                         onBack: goMenu)
        case .learn:
            LearnView(categoryKey: categoryKey, onQuiz: goToQuiz, onBack: goMenu)
        case .levels:
            LevelsView(category: cat, onSelect: startQuiz, onBack: goMenu)
        case .quiz:
            QuizView(category: categoryKey, level: level, onFinish: finish, onQuit: goMenu)
                .id(quizID)
        case .end:
            EndView(score: resultScore, total: resultTotal,
                    onReplay: { startQuiz(level) }, onMenu: goMenu)
        case .progress:
            ProgressStatsView(onBack: goMenu)
        }
    }

    // MARK: - Navigation

    private func goMenu() { screen = .menu }

    private func open(_ key: String) {
        categoryKey = key
        let c = QuestionEngine.category(key)
        if c.hasLearn { screen = .category }
        else if c.hasLevels { screen = .levels }
        else { startQuiz(level) } // ni révision ni niveau (ex. Mélange) → quiz direct
    }

    private func goToQuiz() {
        if cat.hasLevels { screen = .levels } else { startQuiz(level) }
    }

    private func startQuiz(_ lvl: Int) {
        level = lvl
        quizID = UUID()
        screen = .quiz
    }

    private func finish(_ score: Int, _ total: Int) {
        resultScore = score
        resultTotal = total
        screen = .end
    }
}
