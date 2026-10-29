import Foundation

// Les vérifications du pont : chaque fonction de ReNeuroKit et de PlanStore qui
// passe par le noyau, appelée comme les vues l'appellent.

var echecs = 0

func verifie(_ condition: Bool, _ quoi: @autoclosure () -> String,
             ligne: Int = #line) {
    if condition {
        print("  ✅ \(quoi())")
    } else {
        print("  ❌ \(quoi()) (main.swift:\(ligne))")
        echecs += 1
    }
}

print("— thèmes —")
let themes = QuestionEngine.categories
verifie(themes.count == 18, "dix-huit thèmes (\(themes.count))")
verifie(themes.first?.key == "melange" && themes.first?.hasLevels == false, "le Mélange d'abord, sans niveau")
let heure = QuestionEngine.category("heure")
verifie(heure.title == "Lire l'heure" && heure.hasLearn, "« Lire l'heure » se révise")
verifie(levelsFor(heure).map(\.label) == ["Heures pleines", "Quarts et demies", "De 5 en 5", "Minute par minute"],
        "l'heure a ses quatre niveaux")
verifie(levelsFor(QuestionEngine.category("addition")).map(\.id) == [1, 2, 3], "l'addition a trois niveaux")
verifie(themes.allSatisfy { $0.gradient.count == 2 }, "chaque thème a son dégradé")

print("— révisions —")
func elements(_ c: LearnContent) -> Int {
    switch c {
    case let .list(_, _, items): return items.count
    case let .grid(_, letters): return letters.count
    case let .accordion(_, _, groups): return groups.count
    }
}
for c in themes where c.hasLearn {
    verifie(elements(QuestionEngine.learnContent(c.key)) > 0, "\(c.key) : une révision")
}
verifie(elements(QuestionEngine.learnContent("alphabet")) == 26, "l'alphabet en grille de 26 lettres")
verifie(elements(QuestionEngine.learnContent("nombres")) == 11, "les nombres en 11 groupes")
verifie(elements(QuestionEngine.learnContent("addition")) == 0, "l'addition n'a pas de révision")
if case let .list(_, intro, items) = QuestionEngine.learnContent("heure") {
    verifie(intro != nil && items.allSatisfy { $0.clockHours != nil }, "les horloges de la révision de l'heure")
    verifie(items.first?.label == "Une heure" && items.first?.sub == "la grande aiguille est sur le 12",
            "« Une heure », grande aiguille sur le 12")
}
if case let .list(_, _, items) = QuestionEngine.learnContent("cinquante") {
    verifie(items.map(\.num) == [10, 20, 30, 40, 50] && items.allSatisfy { $0.perRow == 10 && $0.colorByRow },
            "les dizaines en billes, une couleur par rangée")
}
if case let .list(_, _, items) = QuestionEngine.learnContent("couleurs") {
    verifie(items.first?.colorHex == "#e53935", "les pastilles de couleur")
}

print("— quiz —")
MemoryStore.shared.reset()
for c in themes {
    for niveau in c.hasLevels ? levelsFor(c).map(\.id) : [1] {
        let qs = QuestionEngine.buildQuestions(category: c.key, level: niveau)
        let justes = qs.allSatisfy { $0.options.count == 4 && $0.options.contains($0.answer) && !$0.choices.contains($0.answer) }
        let cles = Set(qs.map { "\($0.category ?? c.key):\($0.dedupKey)" })
        verifie(qs.count == QuestionEngine.nbQuestions && justes && cles.count == qs.count,
                "\(c.key) niveau \(niveau) : \(qs.count) questions, quatre réponses, aucune en double")
    }
}
let horloges = QuestionEngine.buildQuestions(category: "heure", level: 4).filter { $0.prompt == "Quelle heure est-il ?" }
verifie(!horloges.isEmpty && horloges.allSatisfy { $0.clockHours != nil }, "« Quelle heure est-il ? » montre une horloge")
let melange = QuestionEngine.buildQuestions(category: "melange", level: 1)
verifie(melange.allSatisfy { $0.category != nil } && Set(melange.compactMap(\.category)).count >= 5,
        "le Mélange alterne les thèmes")
let manquantes = (0..<20).flatMap { _ in QuestionEngine.buildQuestions(category: "planifier", level: 2) }
    .filter { $0.prompt.hasSuffix("quelle étape manque ?") }
verifie(!manquantes.isEmpty && manquantes.allSatisfy { $0.steps != nil && $0.stepHole != nil },
        "« quelle étape manque ? » montre les étapes et le trou")
verifie(QuestionEngine.retryPositions(after: 2, count: 10) == [5, 10], "une question ratée revient 3 puis 5 plus loin")
verifie(QuestionEngine.retryPositions(after: 9, count: 10) == [10, 11], "en fin de partie, elle s'ajoute à la suite")
verifie(QuestionEngine.pickPraise().hasPrefix("✅"), "des félicitations")
verifie(endSummary(score: 10, total: 10).stars == "⭐⭐⭐⭐⭐", "cinq étoiles pour un sans-faute")
verifie(endSummary(score: 0, total: 10).message.hasPrefix("Courage"), "on encourage, on ne sanctionne pas")

print("— mémoire —")
MemoryStore.shared.reset()
for _ in 0..<3 { MemoryStore.shared.recordAnswer(key: "jours:x", category: "jours", correct: true) }
MemoryStore.shared.recordAnswer(key: "jours:y", category: "jours", correct: false)
let resume = MemoryStore.shared.summary()
verifie(resume.answered == 4 && resume.correct == 3 && resume.mastered == 1,
        "3 bonnes réponses mènent à la boîte 4 : maîtrisée (\(resume))")
let etats = MemoryStore.shared.keyStates(category: "jours")
verifie(etats["jours:x"]?.box == 4 && etats["jours:y"]?.box == 1, "les boîtes de Leitner")
let dans = (etats["jours:x"]!.due - Date().timeIntervalSince1970) / 86_400
verifie(abs(dans - 7) < 0.01, "boîte 4 : prochaine révision dans 7 jours")
verifie(MemoryStore.shared.categoryStats()["jours"]?.seen == 4, "les statistiques par thème")
MemoryStore.shared.reset()

print("— ma journée —")
verifie(Noyau.constantes.durees == [5, 10, 15, 20, 30, 45, 60], "les durées proposées")
verifie(ROUTINES.count == 11 && routine("cafe")?.etapes.count == 4, "les onze activités du quotidien")
var plan = DayPlan(date: "2026-10-12", debutH: 23, debutM: 50)
plan.taches = [
    PlanTask(label: "Café", duree: 5, fait: true, routineKey: "cafe", reel: 9),
    PlanTask(label: "Lire", duree: 30),
]
let calcul = Journee.bilan(plan)
verifie(calcul.horaires == ["23 h 50", "23 h 55"] && calcul.fin == "0 h 25" && calcul.total == 35,
        "chaque tâche commence quand la précédente finit, et la journée passe minuit")
verifie(calcul.faites == 1 && !calcul.terminee, "une tâche faite sur deux")
verifie(calcul.estimations == [false, nil] && Journee.estimationJuste(plan.taches[0]) == false,
        "5 → 9 min : l'estimation n'était pas juste")
verifie(calcul.bilan == JourEstime(date: "2026-10-12", nb: 1, prevu: 5, reel: 9), "le bilan du jour")
verifie(calcul.bilanEstimation?.contains("sous-estimer") == true, "« tu as tendance à sous-estimer »")
var enCours = PlanTask(label: "Courses", duree: 45)
enCours.demarreA = Date(timeIntervalSinceNow: -150)
verifie(Journee.ecoulees(enCours) == 3, "le chrono : 2 min 30 s font 3 min")
let historique = Journee.archive(calcul.bilan, dans: [
    JourEstime(date: "2026-10-11", nb: 1, prevu: 100, reel: 150),
    JourEstime(date: "2026-10-12", nb: 9, prevu: 9, reel: 9),
])
verifie(historique.map(\.date) == ["2026-10-11", "2026-10-12"] && historique.last?.prevu == 5,
        "l'historique garde un jour par date")
let tendance = Journee.tendance(historique, aujourdhui: "2026-10-12")
verifie(tendance.barres.map(\.libelle) == ["dim. 11", "aujourd'hui"] && tendance.barres[0].valeur == "+50 %",
        "les barres de la tendance")
verifie(tendance.message.hasPrefix("Sur 2 jours"), "la tendance : « \(tendance.message) »")
verifie(Journee.dateLisible("2026-10-12") == "lundi 12 octobre", "« lundi 12 octobre »")
PlanStore.enregistre(plan)
verifie(PlanStore.historique(avec: plan).last?.date == "2026-10-12", "l'historique compte aujourd'hui")

print(echecs == 0 ? "\n✅ le pont Swift ↔ noyau tient" : "\n❌ \(echecs) vérification(s) en échec")
exit(echecs == 0 ? 0 : 1)
