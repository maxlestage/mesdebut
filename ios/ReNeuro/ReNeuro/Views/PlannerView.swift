import Combine
import SwiftUI

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

    /// Écart en pourcentage entre le temps réel et le temps prévu.
    var ecartPct: Int { prevu == 0 ? 0 : Int((Double(reel - prevu) / Double(prevu) * 100).rounded()) }
    var juste: Bool { abs(ecartPct) <= 10 }
}

struct DayPlan: Codable, Equatable {
    var date: String
    var debutH: Int = 9
    var debutM: Int = 0
    var taches: [PlanTask] = []
}

/// La journée planifiée, gardée d'un lancement à l'autre.
///
/// Volontairement dans les réglages utilisateur et non dans la base SQLite :
/// c'est un petit document remplacé en bloc, jamais interrogé — contrairement à
/// la mémoire de l'apprenant, qui elle est faite de lignes qu'on requête.
enum PlanStore {
    private static let cle = "reneuro-journee"

    static func aujourdhui() -> String {
        let f = DateFormatter()
        f.dateFormat = "yyyy-MM-dd"
        return f.string(from: Date())
    }

    private static let cleHist = "reneuro-estimations"
    private static let histMax = 60 // jours conservés

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
        let mesurees = plan.taches.filter { $0.reel != nil }
        return JourEstime(date: plan.date, nb: mesurees.count,
                          prevu: mesurees.reduce(0) { $0 + $1.duree },
                          reel: mesurees.reduce(0) { $0 + ($1.reel ?? 0) })
    }

    static func chargeHistorique() -> [JourEstime] {
        guard let data = UserDefaults.standard.data(forKey: cleHist),
              let h = try? JSONDecoder().decode([JourEstime].self, from: data)
        else { return [] }
        return h
    }

    /// Range une journée révolue dans l'historique, si elle a été chronométrée.
    /// On ne garde que `histMax` jours : c'est une tendance qu'on regarde, pas
    /// une archive.
    static func archive(_ plan: DayPlan) {
        let jour = bilan(plan)
        guard jour.nb > 0 else { return }
        var h = chargeHistorique().filter { $0.date != plan.date }
        h.append(jour)
        h.sort { $0.date < $1.date }
        if h.count > histMax { h = Array(h.suffix(histMax)) }
        if let data = try? JSONEncoder().encode(h) {
            UserDefaults.standard.set(data, forKey: cleHist)
        }
    }

    static func enregistre(_ plan: DayPlan) {
        if let data = try? JSONEncoder().encode(plan) {
            UserDefaults.standard.set(data, forKey: cle)
        }
    }
}

// MARK: - Écran

struct PlannerView: View {
    var onBack: () -> Void

    @State private var plan = PlanStore.charge()
    @State private var texte = ""
    @State private var duree = 15
    @State private var ouverte: UUID?
    @State private var maintenant = Date()

    private let horloge = Timer.publish(every: 1, on: .main, in: .common).autoconnect()

    private let durees = [5, 10, 15, 20, 30, 45, 60]

    /// Chaque tâche démarre quand la précédente se termine : c'est ce calcul qui
    /// rend le plan concret, et qui montre tout de suite si la journée déborde.
    private var horaires: [String] {
        var h = plan.debutH, m = plan.debutM
        return plan.taches.map { t in
            let debut = formatHeure(h, m)
            (h, m) = ajouteMinutes(h, m, t.duree)
            return debut
        }
    }

    private var total: Int { plan.taches.reduce(0) { $0 + $1.duree } }

    /// Minutes écoulées depuis le démarrage, arrondies à la minute la plus proche.
    /// On lit l'horloge à l'instant même : `maintenant` ne sert qu'à provoquer
    /// le réaffichage, sa valeur pourrait avoir jusqu'à une seconde de retard.
    private func ecoulees(_ t: PlanTask) -> Int {
        guard let debut = t.demarreA else { return 0 }
        return max(0, Int((Date().timeIntervalSince(debut) / 60).rounded()))
    }

    /// Vert quand l'estimation était juste (à 20 % près), orange sinon. On ne
    /// reproche jamais un dépassement : mal estimer est précisément ce qu'on
    /// travaille, et le voir suffit à progresser.
    private func couleurEcart(_ t: PlanTask) -> Color {
        guard let reel = t.reel else { return .secondary }
        let marge = max(2.0, Double(t.duree) * 0.2)
        return abs(Double(reel - t.duree)) <= marge ? Color(hex: "#2eb350") : Color(hex: "#e08a00")
    }

    private var mesurees: [PlanTask] { plan.taches.filter { $0.reel != nil } }

    /// L'historique des jours passés, complété par aujourd'hui tel qu'il va —
    /// la journée en cours n'est archivée qu'au changement de date.
    private var historique: [JourEstime] {
        let passe = PlanStore.chargeHistorique().filter { $0.date != plan.date }
        let ceJour = PlanStore.bilan(plan)
        return ceJour.nb > 0 ? passe + [ceJour] : passe
    }

    private var bilanEstimation: String? {
        guard !mesurees.isEmpty else { return nil }
        let prevu = mesurees.reduce(0) { $0 + $1.duree }
        let reel = mesurees.reduce(0) { $0 + ($1.reel ?? 0) }
        let pluriel = mesurees.count > 1 ? "s" : ""
        let debut = "Sur \(mesurees.count) tâche\(pluriel) chronométrée\(pluriel) : "
            + "\(prevu) min prévues, \(reel) min réelles"
        let ecart = reel - prevu
        if abs(Double(ecart)) <= max(3.0, Double(prevu) * 0.15) {
            return "\(debut) — tes estimations sont justes ! 🎯"
        }
        return ecart > 0
            ? "\(debut) — tu as tendance à sous-estimer, c'est très courant. 🌱"
            : "\(debut) — tu prends moins de temps que prévu."
    }
    private var faites: Int { plan.taches.filter(\.fait).count }

    private var dateLisible: String {
        let f = DateFormatter()
        f.locale = Locale(identifier: "fr_FR")
        f.dateFormat = "EEEE d MMMM"
        return f.string(from: Date())
    }

    var body: some View {
        Group {
            ScreenTitle(text: "🗓️ Ma journée")
            ScreenSubtitle(text: dateLisible)

            debutPicker

            if plan.taches.isEmpty {
                Text("Aucune tâche pour l'instant.\nAjoute ce que tu veux faire aujourd'hui, dans l'ordre.")
                    .font(.subheadline)
                    .foregroundColor(.secondary)
                    .multilineTextAlignment(.center)
                    .frame(maxWidth: .infinity)
                    .padding(.vertical, 12)
            } else {
                VStack(spacing: 8) {
                    ForEach(Array(plan.taches.enumerated()), id: \.element.id) { i, tache in
                        ligne(i, tache)
                    }
                }
                bilan
            }

            ajout
            activitesConnues

            if historique.count > 1 {
                TendanceView(jours: historique, dateDuJour: plan.date)
            }

            if !plan.taches.isEmpty {
                BigButton(title: "🗑️ Vider la journée", secondary: true) {
                    plan = DayPlan(date: PlanStore.aujourdhui())
                }
            }
            BackButton(title: "← Retour au menu", action: onBack)
        }
        .onChange(of: plan) { _ in PlanStore.enregistre(plan) }
        .onReceive(horloge) { instant in
            // on ne réveille l'affichage que s'il y a un chrono à faire avancer
            if plan.taches.contains(where: { $0.demarreA != nil }) { maintenant = instant }
        }
    }

    // MARK: - Morceaux

    private var debutPicker: some View {
        HStack(spacing: 10) {
            Text("Je commence à").foregroundColor(.secondary).fontWeight(.semibold)
            DatePicker("", selection: Binding(
                get: {
                    Calendar.current.date(bySettingHour: plan.debutH, minute: plan.debutM,
                                          second: 0, of: Date()) ?? Date()
                },
                set: { nouvelle in
                    let c = Calendar.current.dateComponents([.hour, .minute], from: nouvelle)
                    plan.debutH = c.hour ?? 9
                    plan.debutM = c.minute ?? 0
                }
            ), displayedComponents: .hourAndMinute)
            .labelsHidden()
        }
        .frame(maxWidth: .infinity)
    }

    private func ligne(_ i: Int, _ tache: PlanTask) -> some View {
        VStack(alignment: .leading, spacing: 6) {
            HStack(spacing: 8) {
                Text(horaires[i])
                    .font(.caption.bold().monospacedDigit())
                    .foregroundColor(Color(hex: "#6a5fbb"))
                    .frame(minWidth: 52, alignment: .leading)

                Button { bascule(tache) } label: {
                    Image(systemName: tache.fait ? "checkmark.square.fill" : "square")
                        .font(.title3)
                        .foregroundColor(tache.fait ? Color(hex: "#2eb350") : Color(hex: "#4a3f8f"))
                }
                .buttonStyle(.plain)

                Text(tache.label)
                    .fontWeight(.semibold)
                    .strikethrough(tache.fait)
                    .foregroundColor(tache.fait ? Color(hex: "#7a9a83") : Color(hex: "#333333"))
                    .frame(maxWidth: .infinity, alignment: .leading)

                if tache.demarreA != nil {
                    Text("⏱ \(ecoulees(tache)) min")
                        .font(.caption.bold())
                        .foregroundColor(Color(hex: "#e08a00"))
                } else if let reel = tache.reel {
                    Text("\(tache.duree) → \(reel) min")
                        .font(.caption.bold())
                        .foregroundColor(couleurEcart(tache))
                } else {
                    Text("\(tache.duree) min")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
            }

            HStack(spacing: 6) {
                Spacer()
                if !tache.fait {
                    if tache.demarreA != nil {
                        mini("■ arrêter", enCours: true) { arrete(tache) }
                    } else {
                        mini("▶ démarrer") { demarre(tache) }
                    }
                }
                if let key = tache.routineKey, routine(key) != nil {
                    mini(ouverte == tache.id ? "▾ étapes" : "▸ étapes") {
                        ouverte = ouverte == tache.id ? nil : tache.id
                    }
                }
                mini("↑", actif: i > 0) { deplace(i, -1) }
                mini("↓", actif: i < plan.taches.count - 1) { deplace(i, 1) }
                mini("✕", danger: true) { plan.taches.removeAll { $0.id == tache.id } }
            }

            if ouverte == tache.id, let key = tache.routineKey, let r = routine(key) {
                StepsView(steps: r.etapes)
                    .padding(.leading, 4)
            }
        }
        .padding(10)
        .background(tache.fait ? Color(hex: "#f2fbf4") : Color(hex: "#faf9ff"))
        .clipShape(RoundedRectangle(cornerRadius: 14))
        .overlay(
            RoundedRectangle(cornerRadius: 14)
                .stroke(tache.fait ? Color(hex: "#cdeed6") : Color(hex: "#e2ddff"), lineWidth: 2)
        )
    }

    private func mini(_ titre: String, actif: Bool = true, danger: Bool = false,
                      enCours: Bool = false,
                      _ action: @escaping () -> Void) -> some View {
        Button(action: action) {
            Text(titre)
                .font(.caption.weight(.semibold))
                .padding(.horizontal, 10)
                .frame(minHeight: 30)
                .foregroundColor(danger ? Color(hex: "#e04545")
                                 : enCours ? Color(hex: "#a97a10") : Color(hex: "#4a3f8f"))
                .background(enCours ? Color(hex: "#fff7e0") : Color.white)
                .clipShape(RoundedRectangle(cornerRadius: 9))
                .overlay(
                    RoundedRectangle(cornerRadius: 9)
                        .stroke(danger ? Color(hex: "#ffd6d6")
                                : enCours ? Color(hex: "#ffd98a") : Color(hex: "#ddd7f5"),
                                lineWidth: 1.5)
                )
        }
        .buttonStyle(.plain)
        .disabled(!actif)
        .opacity(actif ? 1 : 0.35)
    }

    private var bilan: some View {
        let (fh, fm) = ajouteMinutes(plan.debutH, plan.debutM, total)
        let tout = faites == plan.taches.count
        return VStack(spacing: 2) {
            (Text("Fin prévue à ")
             + Text(formatHeure(fh, fm)).bold().foregroundColor(Color(hex: "#4a3f8f"))
             + Text(" · \(total) min au total"))
            Text("\(faites) faite\(faites > 1 ? "s" : "") sur \(plan.taches.count)"
                 + (tout ? " — journée terminée ! 🎉" : ""))
                .foregroundColor(tout ? Color(hex: "#2eb350") : .secondary)
                .fontWeight(tout ? .bold : .regular)
            if let bilanEstimation {
                Text(bilanEstimation)
                    .foregroundColor(Color(hex: "#6a5fbb"))
                    .multilineTextAlignment(.center)
                    .padding(.top, 4)
            }
        }
        .font(.subheadline)
        .foregroundColor(.secondary)
        .frame(maxWidth: .infinity)
        .padding(.vertical, 4)
    }

    private var ajout: some View {
        VStack(spacing: 10) {
            TextField("Qu'est-ce que tu veux faire ?", text: $texte)
                .textFieldStyle(.plain)
                .padding(12)
                .background(Color(hex: "#faf9ff"))
                .clipShape(RoundedRectangle(cornerRadius: 14))
                .overlay(
                    RoundedRectangle(cornerRadius: 14)
                        .stroke(Color(hex: "#e2ddff"), lineWidth: 2)
                )

            FlowChips(items: durees.map { "\($0) min" },
                      selection: "\(duree) min") { choix in
                duree = Int(choix.replacingOccurrences(of: " min", with: "")) ?? 15
            }

            BigButton(title: "➕ Ajouter à ma journée") { ajoute(texte, duree) }
                .opacity(texte.trimmingCharacters(in: .whitespaces).isEmpty ? 0.5 : 1)
                .disabled(texte.trimmingCharacters(in: .whitespaces).isEmpty)
        }
    }

    private var activitesConnues: some View {
        VStack(spacing: 8) {
            Text("ou une activité que tu connais déjà :")
                .font(.caption)
                .foregroundColor(.secondary)
            FlowChips(items: ROUTINES.map { "\($0.emoji) \($0.label) · \($0.duree) min" }) { choix in
                if let r = ROUTINES.first(where: { choix.hasPrefix("\($0.emoji) \($0.label)") }) {
                    ajoute(r.label, r.duree, r.key)
                }
            }
        }
        .padding(.top, 6)
    }

    // MARK: - Actions

    private func ajoute(_ label: String, _ duree: Int, _ routineKey: String? = nil) {
        let propre = label.trimmingCharacters(in: .whitespaces)
        guard !propre.isEmpty else { return }
        plan.taches.append(PlanTask(label: propre, duree: duree, routineKey: routineKey))
        if routineKey == nil { texte = "" }
    }

    /// Démarre une tâche — et arrête celle qui tournait : on fait une chose à la
    /// fois, c'est justement ce qu'on apprend ici.
    private func demarre(_ tache: PlanTask) {
        for i in plan.taches.indices where plan.taches[i].demarreA != nil {
            plan.taches[i].reel = ecoulees(plan.taches[i])
            plan.taches[i].demarreA = nil
        }
        guard let i = plan.taches.firstIndex(where: { $0.id == tache.id }) else { return }
        plan.taches[i].demarreA = Date()
    }

    private func arrete(_ tache: PlanTask) {
        guard let i = plan.taches.firstIndex(where: { $0.id == tache.id }),
              plan.taches[i].demarreA != nil else { return }
        plan.taches[i].reel = ecoulees(plan.taches[i])
        plan.taches[i].demarreA = nil
    }

    /// Cocher une tâche en cours arrête aussi son chrono et enregistre le temps mis.
    private func bascule(_ tache: PlanTask) {
        guard let i = plan.taches.firstIndex(where: { $0.id == tache.id }) else { return }
        if !plan.taches[i].fait, plan.taches[i].demarreA != nil {
            plan.taches[i].reel = ecoulees(plan.taches[i])
            plan.taches[i].demarreA = nil
        }
        plan.taches[i].fait.toggle()
        Haptics.success()
    }

    private func deplace(_ i: Int, _ sens: Int) {
        let j = i + sens
        guard j >= 0, j < plan.taches.count else { return }
        plan.taches.swapAt(i, j)
    }
}

/// Petites pastilles qui passent à la ligne toutes seules.
struct FlowChips: View {
    let items: [String]
    var selection: String? = nil
    let onTap: (String) -> Void

    private let colonnes = [GridItem(.adaptive(minimum: 92), spacing: 8)]

    var body: some View {
        LazyVGrid(columns: colonnes, spacing: 8) {
            ForEach(items, id: \.self) { item in
                Button { onTap(item) } label: {
                    Text(item)
                        .font(.caption.weight(.semibold))
                        .lineLimit(1)
                        .minimumScaleFactor(0.75)
                        .padding(.horizontal, 12)
                        .frame(maxWidth: .infinity, minHeight: 36)
                        .foregroundColor(item == selection ? .white : Color(hex: "#4a3f8f"))
                        .background(item == selection ? Color(hex: "#4a3f8f") : Color.white)
                        .clipShape(Capsule())
                        .overlay(Capsule().stroke(Color(hex: "#e2ddff"), lineWidth: 2))
                }
                .buttonStyle(.plain)
            }
        }
    }
}

/// La tendance des estimations sur les derniers jours. C'est le vrai signal :
/// savoir si l'écart entre ce qu'on prévoit et ce qu'on met se resserre.
struct TendanceView: View {
    let jours: [JourEstime]
    let dateDuJour: String

    private var derniers: [JourEstime] { Array(jours.suffix(10)) }

    /// Le plus grand écart affiché, pour mettre les barres à la même échelle.
    private var pire: Int { max(60, derniers.map { abs($0.ecartPct) }.max() ?? 0) }

    private func moyenne(_ liste: [JourEstime]) -> Int? {
        guard !liste.isEmpty else { return nil }
        let prevu = liste.reduce(0) { $0 + $1.prevu }
        let reel = liste.reduce(0) { $0 + $1.reel }
        guard prevu > 0 else { return nil }
        return Int((Double(reel - prevu) / Double(prevu) * 100).rounded())
    }

    private func signe(_ n: Int) -> String { n > 0 ? "+\(n)" : "\(n)" }

    private var message: String {
        let recents = Array(jours.suffix(5))
        let avant = Array(jours.dropLast(5).suffix(5))
        guard let mRecents = moyenne(recents) else { return "" }
        var texte = "Sur \(recents.count) jour\(recents.count > 1 ? "s" : "") : "
            + "\(signe(mRecents)) % d'écart en moyenne"
        if abs(mRecents) <= 10 {
            texte += " — tes estimations sont fiables 🎯"
        } else if let mAvant = moyenne(avant), abs(mRecents) < abs(mAvant) {
            texte += " (contre \(signe(mAvant)) % avant) — tu progresses ! 🌱"
        } else if mRecents > 0 {
            texte += " — tu sous-estimes encore un peu"
        }
        return texte
    }

    /// « dim. 30 », ou « aujourd'hui » pour la journée en cours.
    private func libelle(_ jour: JourEstime) -> String {
        if jour.date == dateDuJour { return "aujourd'hui" }
        let entree = DateFormatter()
        entree.dateFormat = "yyyy-MM-dd"
        guard let d = entree.date(from: jour.date) else { return jour.date }
        let sortie = DateFormatter()
        sortie.locale = Locale(identifier: "fr_FR")
        sortie.dateFormat = "EEE d"
        return sortie.string(from: d)
    }

    var body: some View {
        VStack(alignment: .leading, spacing: 6) {
            Text("📈 Mes estimations")
                .font(.subheadline.bold())
                .foregroundColor(Color(hex: "#4a3f8f"))
                .frame(maxWidth: .infinity)

            ForEach(derniers) { jour in
                HStack(spacing: 8) {
                    Text(libelle(jour))
                        .font(.caption2)
                        .foregroundColor(.secondary)
                        .frame(width: 74, alignment: .leading)

                    GeometryReader { geo in
                        ZStack(alignment: .leading) {
                            Capsule().fill(Color(hex: "#f0edff"))
                            Capsule()
                                .fill(jour.juste ? Color(hex: "#2eb350") : Color(hex: "#e08a00"))
                                .frame(width: geo.size.width
                                       * min(1, Double(abs(jour.ecartPct)) / Double(pire)))
                        }
                    }
                    .frame(height: 10)

                    Text(jour.ecartPct == 0 ? "✓" : "\(signe(jour.ecartPct)) %")
                        .font(.caption2.bold().monospacedDigit())
                        .foregroundColor(jour.juste ? Color(hex: "#2eb350") : Color(hex: "#e08a00"))
                        .frame(width: 48, alignment: .trailing)
                }
            }

            Text(message)
                .font(.caption)
                .foregroundColor(Color(hex: "#6a5fbb"))
                .multilineTextAlignment(.center)
                .frame(maxWidth: .infinity)
                .padding(.top, 4)
        }
        .padding(.top, 12)
    }
}
