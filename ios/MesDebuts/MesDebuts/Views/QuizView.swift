import SwiftUI

struct QuizView: View {
    let category: String
    let level: Int
    var onFinish: (Int, Int) -> Void
    var onQuit: () -> Void

    @State private var questions: [Question]
    @State private var index = 0
    @State private var score = 0
    @State private var selected: String? = nil
    @State private var feedbackGood: Bool? = nil
    @State private var feedbackText: String = ""
    @State private var scheduled: Set<String> = []

    init(category: String, level: Int,
         onFinish: @escaping (Int, Int) -> Void, onQuit: @escaping () -> Void) {
        self.category = category
        self.level = level
        self.onFinish = onFinish
        self.onQuit = onQuit
        _questions = State(initialValue: QuestionEngine.buildQuestions(category: category, level: level))
    }

    private var total: Int { max(questions.count, 1) }
    private var question: Question { questions[min(index, questions.count - 1)] }

    private let answerColumns = [GridItem(.flexible(), spacing: 12), GridItem(.flexible(), spacing: 12)]

    var body: some View {
        Group {
            ProgressView(value: Double(index), total: Double(total))
                .tint(Color(hex: "#667eea"))

            HStack {
                Text("Question \(index + 1) / \(questions.count)")
                Spacer()
                Text("⭐ \(score)")
            }
            .font(.footnote)
            .foregroundColor(.secondary)

            VStack(spacing: 12) {
                Text(question.prompt)
                    .font(.title3.bold())
                    .foregroundColor(Color(hex: "#333333"))
                    .multilineTextAlignment(.center)

                if let hex = question.swatchHex {
                    Circle()
                        .fill(Color(hex: hex))
                        .frame(width: 72, height: 72)
                        .overlay(Circle().stroke(Color(hex: "#cccccc"),
                                                 lineWidth: hex.lowercased() == "#ffffff" ? 2 : 0))
                        .shadow(color: .black.opacity(0.2), radius: 4, y: 3)
                }
                if let shape = question.shapeName {
                    GeometricShapeView(name: shape, size: 80)
                }
                if let m = question.marbles {
                    MarblesView(count: m, size: m > 10 ? 22 : 28,
                                perRow: question.marblesPerRow, colorByRow: question.marblesColorByRow)
                }
            }
            .padding(.vertical, 6)

            LazyVGrid(columns: answerColumns, spacing: 12) {
                ForEach(question.options, id: \.self) { opt in
                    Button { select(opt) } label: {
                        Text(opt)
                            .font(.title3.bold())
                            .frame(maxWidth: .infinity, minHeight: 60)
                            .foregroundColor(fg(opt))
                            .background(bg(opt))
                            .clipShape(RoundedRectangle(cornerRadius: 14))
                            .overlay(RoundedRectangle(cornerRadius: 14).stroke(border(opt), lineWidth: 3))
                    }
                    .buttonStyle(.plain)
                    .allowsHitTesting(selected == nil)
                }
            }

            Text(feedbackText)
                .font(.headline)
                .foregroundColor(feedbackGood == true ? Color(hex: "#2eb350") : Color(hex: "#e04545"))
                .frame(minHeight: 34)

            BackButton(title: "← Abandonner", action: onQuit)
        }
    }

    // MARK: - État visuel d'une réponse

    private func kind(_ opt: String) -> Int { // 0 neutre, 1 correcte, 2 fausse choisie
        guard selected != nil else { return 0 }
        if opt == question.answer { return 1 }
        if opt == selected { return 2 }
        return 0
    }
    private func bg(_ opt: String) -> Color {
        switch kind(opt) { case 1: return Color(hex: "#d4f8dc"); case 2: return Color(hex: "#ffe0e0"); default: return Color(hex: "#faf9ff") }
    }
    private func fg(_ opt: String) -> Color {
        switch kind(opt) { case 1: return Color(hex: "#1d7534"); case 2: return Color(hex: "#a52727"); default: return Color(hex: "#4a3f8f") }
    }
    private func border(_ opt: String) -> Color {
        switch kind(opt) { case 1: return Color(hex: "#2eb350"); case 2: return Color(hex: "#e04545"); default: return Color(hex: "#e2ddff") }
    }

    // MARK: - Logique

    private func select(_ option: String) {
        guard selected == nil else { return }
        selected = option
        let q = question
        let good = option == q.answer

        // mémoire persistée : on note la réponse pour la répétition espacée entre sessions
        let itemCategory = q.category ?? category
        MemoryStore.shared.recordAnswer(key: "\(itemCategory):\(q.dedupKey)",
                                        category: itemCategory, correct: good)

        if good {
            score += 1
            feedbackGood = true
            feedbackText = QuestionEngine.pickPraise()
        } else {
            feedbackGood = false
            feedbackText = "❌ La bonne réponse était : \(q.answer)"
            scheduleRetries(q, at: index)
        }
        DispatchQueue.main.asyncAfter(deadline: .now() + (good ? 0.9 : 1.8)) {
            advance()
        }
    }

    private func advance() {
        if index + 1 < questions.count {
            index += 1
            selected = nil
            feedbackGood = nil
            feedbackText = ""
        } else {
            MemoryStore.shared.recordSession(category: category, score: score, total: questions.count)
            onFinish(score, questions.count)
        }
    }

    // Répétition espacée : la question ratée revient 3 questions plus loin, puis encore 5 après.
    private func scheduleRetries(_ q: Question, at i: Int) {
        guard !scheduled.contains(q.dedupKey) else { return } // une seule fois par question
        scheduled.insert(q.dedupKey)
        var next = questions
        for t in [i + 3, i + 8] {
            var copy = q
            copy.id = UUID()
            copy.options = ([q.answer] + q.choices).shuffled()
            next.insert(copy, at: min(t, next.count))
        }
        questions = next
    }
}
