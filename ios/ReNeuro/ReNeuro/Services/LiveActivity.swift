import ActivityKit
import Foundation

/// Façade sans contrainte de version : l'app appelle ces trois fonctions sans
/// se soucier de la version d'iOS ni de savoir si les activités sont autorisées.
/// Sur un appareil trop ancien ou si l'utilisateur les a désactivées, tout est
/// silencieusement sans effet — le quiz fonctionne exactement pareil.
enum QuizLiveActivity {

    static func start(emoji: String, title: String, total: Int) {
        if #available(iOS 16.2, *) { LiveActivityController.shared.start(emoji: emoji, title: title, total: total) }
    }

    static func update(current: Int, total: Int, score: Int, lastCorrect: Bool?) {
        if #available(iOS 16.2, *) {
            LiveActivityController.shared.update(current: current, total: total,
                                                 score: score, lastCorrect: lastCorrect)
        }
    }

    static func end(score: Int, total: Int) {
        if #available(iOS 16.2, *) { LiveActivityController.shared.end(score: score, total: total) }
    }

    /// Quiz abandonné : on retire l'activité tout de suite.
    static func cancel() {
        if #available(iOS 16.2, *) { LiveActivityController.shared.cancel() }
    }
}

/// Pilote l'activité en cours (une seule à la fois : le quiz courant).
@available(iOS 16.2, *)
final class LiveActivityController {

    static let shared = LiveActivityController()
    private init() {}

    private var activity: Activity<QuizActivityAttributes>?

    func start(emoji: String, title: String, total: Int) {
        // Réglages > Face à Face / Activités en direct : l'utilisateur peut refuser.
        guard ActivityAuthorizationInfo().areActivitiesEnabled else { return }
        cancel() // jamais deux activités en même temps

        let attributes = QuizActivityAttributes(categoryEmoji: emoji, categoryTitle: title)
        let state = QuizActivityAttributes.ContentState(current: 1, total: total, score: 0, lastCorrect: nil)
        activity = try? Activity.request(attributes: attributes,
                                         content: ActivityContent(state: state, staleDate: nil))
    }

    func update(current: Int, total: Int, score: Int, lastCorrect: Bool?) {
        guard let activity else { return }
        let state = QuizActivityAttributes.ContentState(current: current, total: total,
                                                       score: score, lastCorrect: lastCorrect)
        Task { await activity.update(ActivityContent(state: state, staleDate: nil)) }
    }

    /// Fin du quiz : on laisse le score affiché un court instant avant que
    /// l'écran verrouillé ne l'efface.
    func end(score: Int, total: Int) {
        guard let activity else { return }
        let state = QuizActivityAttributes.ContentState(current: total, total: total,
                                                       score: score, lastCorrect: nil)
        self.activity = nil
        Task {
            await activity.end(ActivityContent(state: state, staleDate: nil),
                               dismissalPolicy: .after(.now + 8))
        }
    }

    /// Abandon : on retire l'activité tout de suite.
    func cancel() {
        guard let activity else { return }
        self.activity = nil
        Task { await activity.end(nil, dismissalPolicy: .immediate) }
    }
}
