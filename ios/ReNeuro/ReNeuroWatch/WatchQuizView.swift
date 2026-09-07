import SwiftUI
import WatchKit

/// Le quiz sur la montre : même moteur de questions et même répétition
/// espacée que sur l'iPhone, dans une mise en page faite pour un petit écran.
struct WatchQuizView: View {
    let category: LearningCategory
    let level: Int

    @State private var questions: [Question]
    @State private var index = 0
    @State private var score = 0
    @State private var selected: String?
    @State private var scheduled: Set<String> = []
    @State private var finished = false

    init(category: LearningCategory, level: Int) {
        self.category = category
        self.level = level
        _questions = State(initialValue: QuestionEngine.buildQuestions(category: category.key, level: level))
    }

    private var question: Question { questions[min(index, questions.count - 1)] }

    var body: some View {
        Group {
            if finished {
                WatchEndView(score: score, total: questions.count) { replay() }
            } else {
                quiz
            }
        }
        .navigationTitle(finished ? "Résultat" : "\(index + 1)/\(questions.count)  ⭐\(score)")
    }

    private var quiz: some View {
        ScrollView {
            VStack(spacing: 10) {
                Text(question.prompt)
                    .font(.headline)
                    .multilineTextAlignment(.center)
                    .frame(maxWidth: .infinity)

                visual

                ForEach(question.options, id: \.self) { option in
                    Button { select(option) } label: {
                        Text(option)
                            .font(.body)
                            .lineLimit(2)
                            .minimumScaleFactor(0.7)
                            .frame(maxWidth: .infinity, minHeight: 34)
                    }
                    .buttonStyle(.borderedProminent)
                    .tint(tint(option))
                    .disabled(selected != nil)
                }
            }
            .padding(.horizontal, 2)
        }
    }

    @ViewBuilder private var visual: some View {
        if let hex = question.swatchHex {
            Circle()
                .fill(Color(hex: hex))
                .frame(width: 44, height: 44)
                .overlay(Circle().stroke(.gray.opacity(0.4), lineWidth: 1))
        }
        if let shape = question.shapeName {
            GeometricShapeView(name: shape, size: 54)
        }
        if let hours = question.clockHours {
            ClockView(hours: hours, minutes: question.clockMinutes, size: 92)
        }
        if let marbles = question.marbles {
            MarblesView(count: marbles, size: marbles > 10 ? 11 : 16,
                        perRow: question.marblesPerRow, colorByRow: question.marblesColorByRow)
        }
    }

    /// Neutre tant qu'on n'a pas répondu, puis vert sur la bonne réponse et
    /// rouge sur celle qu'on a choisie à tort.
    private func tint(_ option: String) -> Color {
        guard selected != nil else { return Color(hex: "#4a3f8f") }
        if option == question.answer { return Color(hex: "#2eb350") }
        if option == selected { return Color(hex: "#e04545") }
        return Color(hex: "#4a3f8f").opacity(0.4)
    }

    // MARK: - Logique

    private func select(_ option: String) {
        guard selected == nil else { return }
        selected = option
        let q = question
        let good = option == q.answer

        let itemCategory = q.category ?? category.key
        MemoryStore.shared.recordAnswer(key: "\(itemCategory):\(q.dedupKey)",
                                        category: itemCategory, correct: good)

        if good {
            score += 1
            WKInterfaceDevice.current().play(.success)
        } else {
            WKInterfaceDevice.current().play(.failure)
            scheduleRetries(q, at: index)
        }
        DispatchQueue.main.asyncAfter(deadline: .now() + (good ? 0.8 : 1.6)) { advance() }
    }

    private func advance() {
        if index + 1 < questions.count {
            index += 1
            selected = nil
        } else {
            MemoryStore.shared.recordSession(category: category.key, score: score, total: questions.count)
            finished = true
        }
    }

    /// Répétition espacée : la question ratée revient 3 questions plus loin,
    /// puis encore 5 après — une seule fois, pour que la file reste bornée.
    private func scheduleRetries(_ q: Question, at i: Int) {
        guard !scheduled.contains(q.dedupKey) else { return }
        scheduled.insert(q.dedupKey)
        var next = questions
        for target in [i + 3, i + 8] {
            var copy = q
            copy.id = UUID()
            copy.options = ([q.answer] + q.choices).shuffled()
            next.insert(copy, at: min(target, next.count))
        }
        questions = next
    }

    private func replay() {
        questions = QuestionEngine.buildQuestions(category: category.key, level: level)
        index = 0
        score = 0
        selected = nil
        scheduled = []
        finished = false
    }
}

struct WatchEndView: View {
    let score: Int
    let total: Int
    var onReplay: () -> Void

    var body: some View {
        ScrollView {
            VStack(spacing: 8) {
                Text(endSummary(score: score, total: total).stars)
                    .font(.title3)
                Text("\(score) / \(total)")
                    .font(.title2.bold())
                Text(endSummary(score: score, total: total).message)
                    .font(.footnote)
                    .multilineTextAlignment(.center)
                Button("Rejouer", action: onReplay)
                    .buttonStyle(.borderedProminent)
                    .tint(Color(hex: "#4a3f8f"))
            }
            .padding(.horizontal, 4)
        }
    }
}
