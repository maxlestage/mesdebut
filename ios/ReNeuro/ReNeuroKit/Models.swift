import SwiftUI

// MARK: - Utilitaires

extension String {
    /// Première lettre en majuscule (gère les accents : "été" -> "Été").
    var capFirst: String {
        guard let first = first else { return self }
        return String(first).uppercased() + String(dropFirst())
    }
}

extension Color {
    /// Couleur à partir d'un code hexadécimal "#rrggbb".
    init(hex: String) {
        var s = hex
        if s.hasPrefix("#") { s.removeFirst() }
        var value: UInt64 = 0
        Scanner(string: s).scanHexInt64(&value)
        let r = Double((value >> 16) & 0xFF) / 255.0
        let g = Double((value >> 8) & 0xFF) / 255.0
        let b = Double(value & 0xFF) / 255.0
        self.init(red: r, green: g, blue: b)
    }
}

// MARK: - Modèles

struct Question: Identifiable {
    var id = UUID()
    var prompt: String
    var answer: String
    var choices: [String]
    var options: [String]
    var dedupKey: String
    var marbles: Int? = nil
    var marblesPerRow: Int = 5
    var marblesColorByRow: Bool = false
    var swatchHex: String? = nil
    var shapeName: String? = nil
    var clockHours: Int? = nil
    var clockMinutes: Int = 0
    /// Étapes montrées sous l'énoncé (planification) ; sans quoi « quelle étape
    /// manque ? » et « quel est l'intrus ? » seraient insolubles.
    var steps: [String]? = nil
    /// Position du trou dans `steps` pour « quelle étape manque ? ».
    var stepHole: Int? = nil
    var category: String? = nil
}

struct LearningCategory: Identifiable {
    let key: String
    let emoji: String
    let label: String
    let title: String
    var hasLearn: Bool = false
    var hasLevels: Bool = false
    let gradient: [Color]
    /// Niveaux de la catégorie ; vide quand elle n'en a qu'un.
    var levels: [Level] = []
    var id: String { key }
}

struct Level: Identifiable {
    let id: Int
    let emoji: String
    let label: String
}

/// Niveaux d'une catégorie, tels que les donne le noyau.
func levelsFor(_ category: LearningCategory) -> [Level] {
    category.levels
}

// MARK: - Contenu des écrans de révision

struct LearnItem: Identifiable {
    let id = UUID()
    var label: String
    var sub: String? = nil
    var num: Int? = nil
    var colorHex: String? = nil
    var shapeName: String? = nil
    var marbles: Int? = nil
    var perRow: Int = 5
    var colorByRow: Bool = false
    var clockHours: Int? = nil
    var clockMinutes: Int = 0
}

struct LearnGroup: Identifiable {
    let id = UUID()
    var label: String
    var items: [LearnItem]
}

enum LearnContent {
    case list(title: String, intro: String?, items: [LearnItem])
    case grid(title: String, letters: [String])
    case accordion(title: String, intro: String?, groups: [LearnGroup])
}

// MARK: - Fin de quiz

struct EndSummary: Decodable {
    let stars: String
    let message: String

    private enum CodingKeys: String, CodingKey {
        case stars = "etoiles"
        case message
    }
}

/// Les étoiles et le mot de la fin, donnés par le noyau : on encourage, on ne
/// sanctionne pas.
func endSummary(score: Int, total: Int) -> EndSummary {
    Noyau.appel("fin", Noyau.lignes([[score, total]]))
        ?? EndSummary(stars: "⭐", message: "")
}
