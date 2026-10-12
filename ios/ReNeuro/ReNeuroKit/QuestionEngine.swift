import Foundation

/// Les questions des quiz. Leur fabrique vit dans le noyau Rust, commun à
/// l'iPhone, à la montre et au web : ce fichier la traduit dans les types des
/// vues.
enum QuestionEngine {

    static var nbQuestions: Int { Noyau.constantes.nbQuestions }

    /// Un quiz. Dans les thèmes à ensemble fini (jours, couleurs, heure…), le
    /// noyau fait passer d'abord les notions dont la révision est due, puis
    /// celles jamais vues : il reçoit pour cela ce que la mémoire en sait.
    static func buildQuestions(category: String, level: Int) -> [Question] {
        var lignes: [[CustomStringConvertible]] = [[category, level, Date().timeIntervalSince1970]]
        for (cle, etat) in MemoryStore.shared.keyStates(category: category) {
            lignes.append([cle, etat.box, etat.due])
        }
        let questions: [NoyauQuestion] = Noyau.appel("quiz", Noyau.lignes(lignes)) ?? []
        return questions.map(Question.init(noyau:))
    }

    /// Où revient une question ratée à la position `index` : 3 questions plus
    /// loin, puis encore 5 après. Les positions s'appliquent dans l'ordre.
    static func retryPositions(after index: Int, count: Int) -> [Int] {
        Noyau.appel("retours", Noyau.lignes([[index, count]])) ?? []
    }

    static func pickPraise() -> String {
        Noyau.appel("bravo") ?? "✅ Bravo !"
    }
}

extension Question {
    init(noyau q: NoyauQuestion) {
        self.init(prompt: q.enonce, answer: q.reponse, choices: q.leurres,
                  options: q.options, dedupKey: q.cle)
        category = q.categorie
        guard let i = q.illustration else { return }
        switch i.genre {
        case "billes":
            marbles = i.nombre
            marblesPerRow = i.parRang ?? 5
            marblesColorByRow = i.couleurParRang ?? false
        case "pastille":
            swatchHex = i.hex
        case "forme":
            shapeName = i.nom
        case "horloge":
            clockHours = i.heures
            clockMinutes = i.minutes ?? 0
        case "etapes":
            steps = i.etapes
            stepHole = i.trou
        default:
            break
        }
    }
}
