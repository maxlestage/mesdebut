import ActivityKit
import Foundation

/// Ce que l'écran verrouillé et la Dynamic Island montrent pendant un quiz.
///
/// Partagé entre l'app (qui démarre et met à jour l'activité) et l'extension
/// widget (qui la dessine) : les deux cibles compilent ce même fichier.
@available(iOS 16.1, *)
struct QuizActivityAttributes: ActivityAttributes {

    /// La partie qui change à chaque réponse.
    public struct ContentState: Codable, Hashable {
        /// Numéro de la question en cours, à partir de 1.
        var current: Int
        /// Nombre de questions du quiz (il augmente si une question est à refaire).
        var total: Int
        /// Bonnes réponses jusqu'ici.
        var score: Int
        /// Vrai/faux pour la dernière réponse ; nil tant qu'aucune n'a été donnée.
        var lastCorrect: Bool?

        /// Progression entre 0 et 1, sans division par zéro.
        var fraction: Double {
            total <= 0 ? 0 : min(1, Double(current - 1) / Double(total))
        }
    }

    /// La partie fixe, décidée au démarrage du quiz.
    var categoryEmoji: String
    var categoryTitle: String
}
