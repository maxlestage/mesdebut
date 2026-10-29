import SwiftUI

struct RootView: View {
    private enum Screen { case menu, category, learn, levels, quiz, end, progress, planner, hanoiLevels, hanoi }

    @State private var screen: Screen = .menu
    @State private var categoryKey: String = "melange"
    @State private var level: Int = 1
    @State private var resultScore = 0
    @State private var resultTotal = 0
    @State private var quizID = UUID()
    @State private var hanoiLevel = 1
    @State private var hanoiID = UUID()

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
            MenuView(onSelect: open, onProgress: { screen = .progress },
                     onHanoi: { screen = .hanoiLevels })
        case .category:
            CategoryView(category: cat,
                         onLearn: { screen = .learn },
                         onQuiz: goToQuiz,
                         onPlan: categoryKey == "planifier" ? { screen = .planner } : nil,
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
        case .planner:
            PlannerView(onBack: goMenu)
        case .hanoiLevels:
            HanoiLevelsView(onSelect: startHanoi, onBack: goMenu)
        case .hanoi:
            if let level = HanoiEngine.levels.first(where: { $0.id == hanoiLevel }) {
                HanoiView(level: level, onLevels: { screen = .hanoiLevels }, onNext: startHanoi)
                    .id(hanoiID)
            }
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

    private func startHanoi(_ id: Int) {
        hanoiLevel = id
        hanoiID = UUID()
        screen = .hanoi
    }

    private func finish(_ score: Int, _ total: Int) {
        resultScore = score
        resultTotal = total
        screen = .end
    }
}
