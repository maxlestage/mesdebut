import SwiftUI

// Les thèmes et le contenu des écrans de révision, tels que les donne le noyau
// Rust — la même liste que sur le web.
extension QuestionEngine {

    static let categories: [LearningCategory] =
        (Noyau.appel("categories", comme: [NoyauCategorie].self) ?? []).map { c in
            LearningCategory(key: c.cle, emoji: c.emoji, label: c.libelle, title: c.titre,
                             hasLearn: c.revision, hasLevels: !c.niveaux.isEmpty,
                             gradient: c.degrade.map { Color(hex: $0) },
                             levels: c.niveaux.map { Level(id: $0.id, emoji: $0.emoji, label: $0.libelle) })
        }

    static func category(_ key: String) -> LearningCategory {
        categories.first { $0.key == key } ?? categories[0]
    }

    static func learnContent(_ key: String) -> LearnContent {
        // `null` pour un thème sans révision : un Optional à décoder, pas une erreur
        guard let r = Noyau.appel("revision", key, comme: NoyauRevision?.self) ?? nil else {
            return .list(title: "📖", intro: nil, items: [])
        }
        switch r.genre {
        case "lettres":
            return .grid(title: r.titre, letters: r.lettres ?? [])
        case "accordeon":
            return .accordion(title: r.titre, intro: r.intro, groups: (r.groupes ?? []).map {
                LearnGroup(label: $0.libelle, items: $0.fiches.map(LearnItem.init(noyau:)))
            })
        default:
            return .list(title: r.titre, intro: r.intro,
                         items: (r.fiches ?? []).map(LearnItem.init(noyau:)))
        }
    }
}

extension LearnItem {
    init(noyau f: NoyauFiche) {
        self.init(label: f.libelle, sub: f.detail, num: f.numero)
        guard let i = f.image else { return }
        switch i.genre {
        case "billes":
            marbles = i.nombre
            perRow = i.parRang ?? 5
            colorByRow = i.couleurParRang ?? false
        case "pastille":
            colorHex = i.hex
        case "forme":
            shapeName = i.nom
        case "horloge":
            clockHours = i.heures
            clockMinutes = i.minutes ?? 0
        default:
            break
        }
    }
}
