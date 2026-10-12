import Foundation

// « Ma journée » : ce qui est gardé sur l'iPhone, et ce que le noyau en calcule.
//
// Le stockage reste ici (des documents JSON dans les réglages utilisateur) ;
// les calculs — heures de chaque tâche, chronomètre, bilan des estimations,
// historique et tendance — viennent du noyau Rust, le même que sur le web.

// MARK: - Modèle

struct PlanTask: Identifiable, Codable, Equatable {
    var id = UUID()
    var label: String
    var duree: Int
    var fait: Bool = false
    /// Clé de l'activité connue d'où vient la tâche, pour pouvoir en dérouler les étapes.
    var routineKey: String? = nil
    /// Instant de démarrage tant que le chrono tourne, nil sinon.
    var demarreA: Date? = nil
    /// Minutes réellement passées, une fois le chrono arrêté.
    var reel: Int? = nil
}

/// Le prévu et le réel d'une journée révolue, pour suivre la tendance.
struct JourEstime: Codable, Identifiable, Equatable {
    var date: String
    var nb: Int
    var prevu: Int
    var reel: Int
    var id: String { date }

    /// Une ligne d'arguments pour le noyau.
    var champs: [CustomStringConvertible] { [date, nb, prevu, reel] }
}

struct DayPlan: Codable, Equatable {
    var date: String
    var debutH: Int = 9
    var debutM: Int = 0
    var taches: [PlanTask] = []
}

// MARK: - Calculs, par le noyau

/// Ce que le noyau calcule d'une journée.
struct BilanJournee: Decodable {
    /// L'heure de début de chaque tâche : chacune commence quand la précédente finit.
    let horaires: [String]
    let total: Int
    /// « 10 h 25 »
    let fin: String
    let faites: Int
    let terminee: Bool
    /// Par tâche : l'estimation était-elle juste ? `nil` tant qu'elle n'est pas chronométrée.
    let estimations: [Bool?]
    /// Le prévu et le réel des tâches chronométrées.
    var bilan: JourEstime
    /// Le bilan formulé, sans jamais reprocher un dépassement.
    let bilanEstimation: String?
}

/// Les derniers jours en barres, et ce qu'ils disent.
struct Tendance: Decodable {
    struct Barre: Decodable, Identifiable {
        let date: String
        /// « dim. 30 », ou « aujourd'hui »
        let libelle: String
        let ecartPct: Int
        let juste: Bool
        /// Longueur de la barre, de 0 à 1.
        let part: Double
        /// « +25 % », ou « ✓ »
        let valeur: String
        var id: String { date }
    }

    let barres: [Barre]
    let message: String
}

enum Journee {
    static func bilan(_ plan: DayPlan) -> BilanJournee {
        var lignes: [[CustomStringConvertible]] = [[plan.debutH, plan.debutM]]
        for t in plan.taches {
            lignes.append([t.duree, Noyau.bool(t.fait), t.reel.map { "\($0)" } ?? ""])
        }
        var b: BilanJournee = Noyau.appel("journee", Noyau.lignes(lignes))
            ?? BilanJournee(horaires: plan.taches.map { _ in "" }, total: 0, fin: "", faites: 0,
                            terminee: false, estimations: plan.taches.map { _ in nil },
                            bilan: JourEstime(date: "", nb: 0, prevu: 0, reel: 0), bilanEstimation: nil)
        b.bilan.date = plan.date
        return b
    }

    /// L'estimation d'une tâche était-elle juste (à 20 % près) ?
    static func estimationJuste(_ t: PlanTask) -> Bool? {
        let b: BilanJournee? = Noyau.appel("journee", Noyau.lignes([
            [0, 0], [t.duree, Noyau.bool(t.fait), t.reel.map { "\($0)" } ?? ""],
        ]))
        return b?.estimations.first ?? nil
    }

    /// Minutes écoulées depuis le démarrage du chrono, arrondies.
    static func ecoulees(_ t: PlanTask, maintenant: Date = Date()) -> Int {
        guard let debut = t.demarreA else { return 0 }
        return Noyau.appel("ecoulees", Noyau.lignes([[debut.timeIntervalSince1970 * 1000,
                                                      maintenant.timeIntervalSince1970 * 1000]])) ?? 0
    }

    /// Range `jour` dans l'historique (s'il a été chronométré) : un jour par
    /// date, dans l'ordre, les 60 derniers.
    static func archive(_ jour: JourEstime, dans historique: [JourEstime]) -> [JourEstime] {
        Noyau.appel("archiver", Noyau.lignes([jour.champs] + historique.map(\.champs))) ?? historique
    }

    static func tendance(_ jours: [JourEstime], aujourdhui: String) -> Tendance {
        Noyau.appel("tendance", Noyau.lignes([[aujourdhui]] + jours.map(\.champs)))
            ?? Tendance(barres: [], message: "")
    }

    /// « lundi 12 octobre »
    static func dateLisible(_ iso: String) -> String {
        Noyau.appel("date_lisible", iso) ?? iso
    }
}

// MARK: - Stockage

/// La journée planifiée, gardée d'un lancement à l'autre.
///
/// Volontairement dans les réglages utilisateur et non dans la base SQLite :
/// c'est un petit document remplacé en bloc, jamais interrogé — contrairement à
/// la mémoire de l'apprenant, qui elle est faite de lignes qu'on requête.
enum PlanStore {
    private static let cle = "reneuro-journee"
    private static let cleHist = "reneuro-estimations"

    static func aujourdhui() -> String {
        let f = DateFormatter()
        f.dateFormat = "yyyy-MM-dd"
        return f.string(from: Date())
    }

    /// On repart d'une journée vide quand la date a changé : planifier, c'est
    /// planifier aujourd'hui. La journée révolue part d'abord à l'historique,
    /// pour que la tendance des estimations survive au changement de jour.
    static func charge() -> DayPlan {
        guard let data = UserDefaults.standard.data(forKey: cle),
              let plan = try? JSONDecoder().decode(DayPlan.self, from: data)
        else { return DayPlan(date: aujourdhui()) }
        if plan.date != aujourdhui() {
            archive(plan)
            return DayPlan(date: aujourdhui())
        }
        return plan
    }

    /// Le prévu et le réel d'une journée, pour les tâches effectivement chronométrées.
    static func bilan(_ plan: DayPlan) -> JourEstime {
        Journee.bilan(plan).bilan
    }

    static func chargeHistorique() -> [JourEstime] {
        guard let data = UserDefaults.standard.data(forKey: cleHist),
              let h = try? JSONDecoder().decode([JourEstime].self, from: data)
        else { return [] }
        return h
    }

    /// Range une journée révolue dans l'historique, si elle a été chronométrée.
    static func archive(_ plan: DayPlan) {
        let h = Journee.archive(bilan(plan), dans: chargeHistorique())
        if let data = try? JSONEncoder().encode(h) {
            UserDefaults.standard.set(data, forKey: cleHist)
        }
    }

    /// L'historique des jours passés, complété par aujourd'hui tel qu'il va —
    /// la journée en cours n'est archivée qu'au changement de date.
    static func historique(avec plan: DayPlan) -> [JourEstime] {
        Journee.archive(bilan(plan), dans: chargeHistorique())
    }

    static func enregistre(_ plan: DayPlan) {
        if let data = try? JSONEncoder().encode(plan) {
            UserDefaults.standard.set(data, forKey: cle)
        }
    }
}
