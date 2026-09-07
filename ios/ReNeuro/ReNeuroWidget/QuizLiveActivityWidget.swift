import ActivityKit
import SwiftUI
import WidgetKit

/// Le quiz en cours sur l'écran verrouillé et dans la Dynamic Island.
struct QuizLiveActivityWidget: Widget {

    var body: some WidgetConfiguration {
        ActivityConfiguration(for: QuizActivityAttributes.self) { context in
            LockScreenQuizView(attributes: context.attributes, state: context.state)
                .activityBackgroundTint(Color.black.opacity(0.55))
                .activitySystemActionForegroundColor(.white)
        } dynamicIsland: { context in
            DynamicIsland {
                DynamicIslandExpandedRegion(.leading) {
                    Text(context.attributes.categoryEmoji).font(.title2)
                }
                DynamicIslandExpandedRegion(.trailing) {
                    Text("⭐ \(context.state.score)")
                        .font(.title3.bold())
                        .foregroundStyle(.yellow)
                }
                DynamicIslandExpandedRegion(.center) {
                    Text(context.attributes.categoryTitle)
                        .font(.caption)
                        .foregroundStyle(.secondary)
                        .lineLimit(1)
                }
                DynamicIslandExpandedRegion(.bottom) {
                    VStack(spacing: 6) {
                        QuizProgressBar(state: context.state)
                        Text(progressLabel(context.state))
                            .font(.caption2)
                            .foregroundStyle(.secondary)
                    }
                }
            } compactLeading: {
                Text(context.attributes.categoryEmoji)
            } compactTrailing: {
                Text("\(context.state.score)/\(context.state.total)")
                    .font(.caption2.bold())
                    .foregroundStyle(.yellow)
            } minimal: {
                Text(context.attributes.categoryEmoji)
            }
        }
    }
}

/// « Question 4 / 10 » — la dernière réponse s'affiche brièvement à la place.
private func progressLabel(_ state: QuizActivityAttributes.ContentState) -> String {
    switch state.lastCorrect {
    case true?: return "✅ Bonne réponse !"
    case false?: return "❌ Ce n'est pas grave, on la refera"
    case nil: return "Question \(state.current) / \(state.total)"
    }
}

struct QuizProgressBar: View {
    let state: QuizActivityAttributes.ContentState

    var body: some View {
        GeometryReader { geo in
            ZStack(alignment: .leading) {
                Capsule().fill(.white.opacity(0.2))
                Capsule()
                    .fill(LinearGradient(colors: [Color(red: 0.40, green: 0.49, blue: 0.92),
                                                  Color(red: 0.46, green: 0.29, blue: 0.64)],
                                         startPoint: .leading, endPoint: .trailing))
                    .frame(width: max(6, geo.size.width * state.fraction))
            }
        }
        .frame(height: 8)
    }
}

/// Vue de l'écran verrouillé (et des appareils sans Dynamic Island).
struct LockScreenQuizView: View {
    let attributes: QuizActivityAttributes
    let state: QuizActivityAttributes.ContentState

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            HStack(spacing: 8) {
                Text(attributes.categoryEmoji).font(.title2)
                VStack(alignment: .leading, spacing: 1) {
                    Text(attributes.categoryTitle)
                        .font(.headline)
                        .lineLimit(1)
                    Text(progressLabel(state))
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }
                Spacer()
                Text("⭐ \(state.score)")
                    .font(.title3.bold())
                    .foregroundStyle(.yellow)
            }
            QuizProgressBar(state: state)
        }
        .padding(14)
    }
}
