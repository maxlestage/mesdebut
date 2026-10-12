import Combine
import SwiftUI

// Le modèle, le stockage et les calculs de la journée sont dans
// Services/PlanStore.swift ; les calculs eux-mêmes viennent du noyau Rust.

// MARK: - Écran

struct PlannerView: View {
    var onBack: () -> Void

    @State private var plan = PlanStore.charge()
    @State private var texte = ""
    @State private var duree = 15
    @State private var ouverte: UUID?
    @State private var maintenant = Date()

    private let horloge = Timer.publish(every: 1, on: .main, in: .common).autoconnect()

    private let durees = Noyau.constantes.durees

    /// Ce que le noyau calcule de la journée : heures, fin, bilan.
    private var calcul: BilanJournee { Journee.bilan(plan) }

    /// Chaque tâche démarre quand la précédente se termine : c'est ce calcul qui
    /// rend le plan concret, et qui montre tout de suite si la journée déborde.
    private var horaires: [String] { calcul.horaires }

    /// Minutes écoulées depuis le démarrage, arrondies à la minute la plus proche.
    /// On lit l'horloge à l'instant même : `maintenant` ne sert qu'à provoquer
    /// le réaffichage, sa valeur pourrait avoir jusqu'à une seconde de retard.
    private func ecoulees(_ t: PlanTask) -> Int {
        Journee.ecoulees(t)
    }

    /// Vert quand l'estimation était juste (à 20 % près), orange sinon. On ne
    /// reproche jamais un dépassement : mal estimer est précisément ce qu'on
    /// travaille, et le voir suffit à progresser.
    private func couleurEcart(_ t: PlanTask) -> Color {
        guard let juste = Journee.estimationJuste(t) else { return .secondary }
        return juste ? Color(hex: "#2eb350") : Color(hex: "#e08a00")
    }

    /// L'historique des jours passés, complété par aujourd'hui tel qu'il va —
    /// la journée en cours n'est archivée qu'au changement de date.
    private var historique: [JourEstime] { PlanStore.historique(avec: plan) }

    /// Le bilan des estimations, formulé sans jamais reprocher un dépassement.
    private var bilanEstimation: String? { calcul.bilanEstimation }

    private var dateLisible: String { Journee.dateLisible(plan.date) }

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
                    .foregroundColor(.accentDoux)
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
                        .foregroundColor(.attention)
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
        let c = calcul
        let faites = c.faites
        let tout = c.terminee
        return VStack(spacing: 2) {
            (Text("Fin prévue à ")
             + Text(c.fin).bold().foregroundColor(.titre)
             + Text(" · \(c.total) min au total"))
            Text("\(faites) faite\(faites > 1 ? "s" : "") sur \(plan.taches.count)"
                 + (tout ? " — journée terminée ! 🎉" : ""))
                .foregroundColor(tout ? Color(hex: "#2eb350") : .secondary)
                .fontWeight(tout ? .bold : .regular)
            if let bilanEstimation {
                Text(bilanEstimation)
                    .foregroundColor(.accentDoux)
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

    /// Les barres des dix derniers jours, à la même échelle, et ce qu'elles
    /// disent : calculées par le noyau, comme sur le web.
    private var tendance: Tendance { Journee.tendance(jours, aujourdhui: dateDuJour) }

    var body: some View {
        let tendance = self.tendance
        return VStack(alignment: .leading, spacing: 6) {
            Text("📈 Mes estimations")
                .font(.subheadline.bold())
                .foregroundColor(.titre)
                .frame(maxWidth: .infinity)

            ForEach(tendance.barres) { barre in
                HStack(spacing: 8) {
                    Text(barre.libelle)
                        .font(.caption2)
                        .foregroundColor(.secondary)
                        .frame(width: 74, alignment: .leading)

                    GeometryReader { geo in
                        ZStack(alignment: .leading) {
                            Capsule().fill(Color(hex: "#f0edff"))
                            Capsule()
                                .fill(barre.juste ? Color(hex: "#2eb350") : Color(hex: "#e08a00"))
                                .frame(width: geo.size.width * barre.part)
                        }
                    }
                    .frame(height: 10)

                    Text(barre.valeur)
                        .font(.caption2.bold().monospacedDigit())
                        .foregroundColor(barre.juste ? Color(hex: "#2eb350") : Color(hex: "#e08a00"))
                        .frame(width: 48, alignment: .trailing)
                }
            }

            Text(tendance.message)
                .font(.caption)
                .foregroundColor(.accentDoux)
                .multilineTextAlignment(.center)
                .frame(maxWidth: .infinity)
                .padding(.top, 4)
        }
        .padding(.top, 12)
    }
}
