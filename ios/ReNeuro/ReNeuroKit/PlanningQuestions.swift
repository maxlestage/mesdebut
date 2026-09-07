import Foundation

// Génération des questions de la catégorie « Planifier ».
// Port de la partie correspondante de src/questions.js.
extension QuestionEngine {

    /// Autres étapes de la même activité, comme leurres plausibles.
    private static func autresEtapes(_ r: Routine, sauf: [String], _ nb: Int) -> [String] {
        Array(r.etapes.filter { !sauf.contains($0) }.shuffled().prefix(nb))
    }

    /// Étapes puisées dans d'autres activités (pour l'intrus et les compléments).
    private static func etapesAilleurs(_ r: Routine, _ nb: Int) -> [String] {
        Array(ROUTINES.filter { $0.key != r.key }.flatMap(\.etapes).shuffled().prefix(nb))
    }

    /// Complète une liste de leurres jusqu'à `nb`, sans doublon avec la réponse.
    private static func completeChoix(_ choix: [String], _ reponse: String,
                                      _ r: Routine, _ nb: Int) -> [String] {
        var out = choix
        var vus = Set([reponse] + choix)
        for e in etapesAilleurs(r, 12) where out.count < nb {
            if !vus.contains(e) { out.append(e); vus.insert(e) }
        }
        return Array(out.prefix(nb))
    }

    // 🌱 Les étapes : repérer le début, la fin, et ce qui suit
    private static func planEtapes() -> Question {
        let r = ROUTINES.randomElement()!
        switch ["premiere", "derniere", "apres"].randomElement()! {
        case "premiere":
            let reponse = r.etapes[0]
            return makeQ("« \(r.label) » : par quoi commence-t-on ?",
                         answer: reponse,
                         choices: completeChoix(autresEtapes(r, sauf: [reponse], 3), reponse, r, 3),
                         key: "plan:premiere:\(r.key)")
        case "derniere":
            let reponse = r.etapes[r.etapes.count - 1]
            return makeQ("« \(r.label) » : quelle est la dernière étape ?",
                         answer: reponse,
                         choices: completeChoix(autresEtapes(r, sauf: [reponse], 3), reponse, r, 3),
                         key: "plan:derniere:\(r.key)")
        default:
            let i = Int.random(in: 0 ..< (r.etapes.count - 1))
            let reponse = r.etapes[i + 1]
            return makeQ("« \(r.label) » : que fait-on juste après « \(r.etapes[i]) » ?",
                         answer: reponse,
                         choices: completeChoix(autresEtapes(r, sauf: [reponse, r.etapes[i]], 3),
                                                reponse, r, 3),
                         key: "plan:apres:\(r.key):\(i)")
        }
    }

    // 🌿 L'ordre : l'étape qui manque, l'intrus, ce qui vient avant
    private static func planOrdre() -> Question {
        let r = ROUTINES.randomElement()!
        switch ["manquante", "intrus", "avant"].randomElement()! {
        case "manquante":
            let i = Int.random(in: 0 ..< r.etapes.count)
            let reponse = r.etapes[i]
            let restantes = r.etapes.enumerated().filter { $0.offset != i }.map(\.element)
            return makeQ("« \(r.label) » : quelle étape manque ?",
                         answer: reponse,
                         choices: completeChoix([], reponse, r, 3),
                         key: "plan:manquante:\(r.key):\(i)",
                         steps: restantes, stepHole: i)
        case "intrus":
            let intrus = etapesAilleurs(r, 1)[0]
            let gardees = Array(r.etapes.shuffled().prefix(3))
            return makeQ("« \(r.label) » : quelle étape n'en fait pas partie ?",
                         answer: intrus,
                         choices: gardees,
                         key: "plan:intrus:\(r.key):\(intrus)",
                         steps: (gardees + [intrus]).shuffled())
        default:
            let i = Int.random(in: 1 ..< r.etapes.count)
            let reponse = r.etapes[i - 1]
            return makeQ("« \(r.label) » : que fait-on juste avant « \(r.etapes[i]) » ?",
                         answer: reponse,
                         choices: completeChoix(autresEtapes(r, sauf: [reponse, r.etapes[i]], 3),
                                                reponse, r, 3),
                         key: "plan:avant:\(r.key):\(i)")
        }
    }

    // 🌳 Le temps : estimer une durée, calculer une fin, tenir dans un créneau
    private static func planTemps() -> Question {
        let r = ROUTINES.randomElement()!
        switch ["duree", "fin", "tient", "pluslongue"].randomElement()! {
        case "duree":
            let autres = Array(Set(ROUTINES.map(\.duree)).filter { $0 != r.duree }).shuffled()
            return makeQ("Combien de temps prend « \(r.label) » à peu près ?",
                         answer: "\(r.duree) min",
                         choices: autres.prefix(3).map { "\($0) min" },
                         key: "plan:duree:\(r.key)")
        case "fin":
            let h = Int.random(in: 8 ... 17)
            let m = [0, 15, 30, 45].randomElement()!
            let (fh, fm) = ajouteMinutes(h, m, r.duree)
            var leurres: [String] = []
            for ecart in [-30, -15, -10, 10, 15, 30, 60].shuffled() where leurres.count < 3 {
                let (lh, lm) = ajouteMinutes(fh, fm, ecart)
                let texte = formatHeure(lh, lm)
                if texte != formatHeure(fh, fm) && !leurres.contains(texte) { leurres.append(texte) }
            }
            return makeQ("Tu commences « \(r.label) » à \(formatHeure(h, m)). "
                         + "Ça prend \(r.duree) min. À quelle heure as-tu fini ?",
                         answer: formatHeure(fh, fm), choices: leurres,
                         key: "plan:fin:\(r.key):\(h):\(m)")
        case "tient":
            // une seule activité tient dans le créneau, les trois autres sont trop longues
            let creneau = [5, 10, 15, 20].randomElement()!
            let tiennent = ROUTINES.filter { $0.duree <= creneau }
            let trop = ROUTINES.filter { $0.duree > creneau }
            if let bonne = tiennent.randomElement(), trop.count >= 3 {
                return makeQ("Tu as \(creneau) minutes devant toi. Qu'est-ce qui rentre ?",
                             answer: bonne.label,
                             choices: trop.shuffled().prefix(3).map(\.label),
                             key: "plan:tient:\(creneau):\(bonne.key)")
            }
            return planTemps()
        default:
            let quatre = Array(ROUTINES.shuffled().prefix(4))
            let plusLongue = quatre.max(by: { $0.duree < $1.duree })!
            // égalité en tête : la question n'aurait pas de réponse unique
            if quatre.filter({ $0.duree == plusLongue.duree }).count > 1 { return planTemps() }
            return makeQ("Laquelle de ces activités prend le plus de temps ?",
                         answer: plusLongue.label,
                         choices: quatre.filter { $0.key != plusLongue.key }.map(\.label),
                         key: "plan:pluslongue:\(quatre.map(\.key).sorted().joined(separator: "-"))")
        }
    }

    static func makePlanifier(_ level: Int) -> Question {
        switch level {
        case 1: return planEtapes()
        case 2: return planOrdre()
        default: return planTemps()
        }
    }
}
