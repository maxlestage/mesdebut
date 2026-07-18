import SwiftUI

// Catégories (métadonnées + couleurs) et contenu des écrans de révision.
extension QuestionEngine {

    static let categories: [LearningCategory] = [
        LearningCategory(key: "melange", emoji: "🧠", label: "Mélange", title: "Quiz mélangé",
                         gradient: [Color(hex: "#26a69a"), Color(hex: "#00695c")]),
        LearningCategory(key: "jours", emoji: "📅", label: "Les jours", title: "Les jours de la semaine",
                         hasLearn: true, gradient: [Color(hex: "#f6a821"), Color(hex: "#f68f21")]),
        LearningCategory(key: "mois", emoji: "🗓️", label: "Les mois", title: "Les mois de l'année",
                         hasLearn: true, gradient: [Color(hex: "#21b3f6"), Color(hex: "#2179f6")]),
        LearningCategory(key: "saisons", emoji: "🍂", label: "Les saisons", title: "Les saisons",
                         hasLearn: true, gradient: [Color(hex: "#8bc34a"), Color(hex: "#5a9216")]),
        LearningCategory(key: "alphabet", emoji: "🔤", label: "L'alphabet", title: "L'alphabet",
                         hasLearn: true, gradient: [Color(hex: "#26c6da"), Color(hex: "#0097a7")]),
        LearningCategory(key: "couleurs", emoji: "🎨", label: "Les couleurs", title: "Les couleurs",
                         hasLearn: true, gradient: [Color(hex: "#ff8a65"), Color(hex: "#d84315")]),
        LearningCategory(key: "formes", emoji: "📐", label: "Les formes", title: "Les formes géométriques",
                         hasLearn: true, gradient: [Color(hex: "#78909c"), Color(hex: "#455a64")]),
        LearningCategory(key: "chiffres", emoji: "🧮", label: "Les chiffres", title: "Les chiffres de 0 à 9",
                         hasLearn: true, gradient: [Color(hex: "#ffca28"), Color(hex: "#f57f17")]),
        LearningCategory(key: "cinquante", emoji: "🔟", label: "Jusqu'à 50", title: "Les nombres jusqu'à 50",
                         hasLearn: true, gradient: [Color(hex: "#ec407a"), Color(hex: "#ad1457")]),
        LearningCategory(key: "nombres", emoji: "🔢", label: "Les nombres", title: "Les nombres en lettres",
                         hasLearn: true, hasLevels: true, gradient: [Color(hex: "#5c6bc0"), Color(hex: "#3949ab")]),
        LearningCategory(key: "addition", emoji: "➕", label: "Addition", title: "Addition",
                         hasLevels: true, gradient: [Color(hex: "#4cd964"), Color(hex: "#2eb350")]),
        LearningCategory(key: "soustraction", emoji: "➖", label: "Soustraction", title: "Soustraction",
                         hasLevels: true, gradient: [Color(hex: "#ff6b6b"), Color(hex: "#e04545")]),
        LearningCategory(key: "multiplication", emoji: "✖️", label: "Multiplication", title: "Multiplication",
                         hasLevels: true, gradient: [Color(hex: "#a76bff"), Color(hex: "#7d45e0")]),
        LearningCategory(key: "division", emoji: "➗", label: "Division", title: "Division",
                         hasLevels: true, gradient: [Color(hex: "#ff6bd5"), Color(hex: "#e045a8")]),
    ]

    static func category(_ key: String) -> LearningCategory {
        categories.first { $0.key == key } ?? categories[0]
    }

    // MARK: - Contenu des révisions

    static func learnContent(_ key: String) -> LearnContent {
        switch key {
        case "jours":
            return .list(title: "📖 Les 7 jours de la semaine", intro: nil,
                         items: jours.map { LearnItem(label: $0.capFirst) })
        case "mois":
            return .list(title: "📖 Les 12 mois de l'année", intro: nil,
                         items: mois.map { LearnItem(label: $0.capFirst) })
        case "saisons":
            return .list(title: "📖 Les 4 saisons", intro: nil, items: [
                LearnItem(label: "🌸 Printemps", sub: "mars, avril, mai"),
                LearnItem(label: "☀️ Été", sub: "juin, juillet, août"),
                LearnItem(label: "🍂 Automne", sub: "septembre, octobre, novembre"),
                LearnItem(label: "❄️ Hiver", sub: "décembre, janvier, février"),
            ])
        case "alphabet":
            return .grid(title: "📖 Les 26 lettres de l'alphabet", letters: alphabet)
        case "couleurs":
            return .list(title: "📖 Les 11 couleurs", intro: nil,
                         items: couleurs.map { LearnItem(label: $0.name.capFirst, colorHex: $0.hex) })
        case "formes":
            return .list(title: "📖 Les 10 formes géométriques", intro: nil,
                         items: formes.map { LearnItem(label: $0.name.capFirst, sub: $0.sub, shapeName: $0.name) })
        case "chiffres":
            return .list(title: "📖 Les chiffres de 0 à 9", intro: nil,
                         items: (0...9).map { LearnItem(label: unites[$0].capFirst, num: $0, marbles: $0) })
        case "cinquante":
            return .list(title: "📖 Les dizaines jusqu'à 50",
                         intro: "Une rangée de 10 billes = 1 dizaine",
                         items: [10, 20, 30, 40, 50].map {
                             LearnItem(label: numberToWords($0).capFirst,
                                       sub: "\($0 / 10) dizaine\($0 > 10 ? "s" : "")",
                                       num: $0, marbles: $0, perRow: 10, colorByRow: true)
                         })
        case "nombres":
            var groups: [LearnGroup] = (0..<10).map { d in
                LearnGroup(label: "De \(d * 10) à \(d * 10 + 9)",
                           items: (d * 10 ... d * 10 + 9).map { LearnItem(label: numberToWords($0).capFirst, num: $0) })
            }
            groups.append(LearnGroup(label: "Cent", items: [LearnItem(label: numberToWords(100).capFirst, num: 100)]))
            return .accordion(title: "📖 Les nombres en lettres",
                              intro: "Appuie sur une dizaine pour dérouler tous les nombres", groups: groups)
        default:
            return .list(title: "📖", intro: nil, items: [])
        }
    }
}
