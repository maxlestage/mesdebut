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
    var id: String { key }
}

struct Level: Identifiable {
    let id: Int
    let emoji: String
    let label: String
}

let LEVELS: [Level] = [
    Level(id: 1, emoji: "🌱", label: "Facile"),
    Level(id: 2, emoji: "🌿", label: "Moyen"),
    Level(id: 3, emoji: "🌳", label: "Difficile"),
]

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

struct EndSummary {
    let stars: String
    let message: String
}

func endSummary(score: Int, total: Int) -> EndSummary {
    let ratio = total == 0 ? 0 : Double(score) / Double(total)
    if ratio >= 1 { return EndSummary(stars: "⭐⭐⭐⭐⭐", message: "Score parfait, tu es un champion ! 🏆") }
    if ratio >= 0.8 { return EndSummary(stars: "⭐⭐⭐⭐", message: "Excellent travail, continue comme ça ! 🎉") }
    if ratio >= 0.6 { return EndSummary(stars: "⭐⭐⭐", message: "Bien joué ! Encore un petit effort ! 💪") }
    if ratio >= 0.4 { return EndSummary(stars: "⭐⭐", message: "Pas mal ! Entraîne-toi encore un peu ! 🙂") }
    return EndSummary(stars: "⭐", message: "Courage, réessaie : c'est en s'entraînant qu'on apprend ! 🌱")
}
