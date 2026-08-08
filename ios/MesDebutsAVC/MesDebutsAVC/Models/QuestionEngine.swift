import Foundation

// Port en Swift de src/questions.js (génération des questions, données, révision).
enum QuestionEngine {

    static let nbQuestions = 10

    // MARK: - Données

    static let jours = ["lundi", "mardi", "mercredi", "jeudi", "vendredi", "samedi", "dimanche"]
    static let mois = ["janvier", "février", "mars", "avril", "mai", "juin",
                       "juillet", "août", "septembre", "octobre", "novembre", "décembre"]
    static let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".map { String($0) }
    static let saisons = ["printemps", "été", "automne", "hiver"]

    static let saisonArticle = ["printemps": "le printemps", "été": "l'été",
                                "automne": "l'automne", "hiver": "l'hiver"]
    static let moisSaison: [String: String] = [
        "janvier": "hiver", "février": "hiver",
        "avril": "printemps", "mai": "printemps",
        "juillet": "été", "août": "été",
        "octobre": "automne", "novembre": "automne",
    ]

    struct Couleur { let name: String; let hex: String }
    static let couleurs: [Couleur] = [
        Couleur(name: "rouge", hex: "#e53935"), Couleur(name: "bleu", hex: "#1e88e5"),
        Couleur(name: "jaune", hex: "#fdd835"), Couleur(name: "vert", hex: "#43a047"),
        Couleur(name: "orange", hex: "#fb8c00"), Couleur(name: "violet", hex: "#8e24aa"),
        Couleur(name: "rose", hex: "#f06292"), Couleur(name: "marron", hex: "#795548"),
        Couleur(name: "gris", hex: "#9e9e9e"), Couleur(name: "noir", hex: "#212121"),
        Couleur(name: "blanc", hex: "#ffffff"),
    ]
    static func colorHex(_ name: String) -> String? { couleurs.first { $0.name == name }?.hex }

    static let melangesCouleurs: [(a: String, b: String, donne: String)] = [
        ("bleu", "jaune", "vert"), ("rouge", "jaune", "orange"), ("rouge", "bleu", "violet"),
        ("rouge", "blanc", "rose"), ("noir", "blanc", "gris"),
    ]
    static let objetsCouleur: [(objet: String, couleur: String)] = [
        ("un citron", "jaune"), ("une banane", "jaune"), ("le soleil", "jaune"),
        ("le ciel", "bleu"), ("un sapin", "vert"), ("une tomate", "rouge"),
        ("une fraise", "rouge"), ("une carotte", "orange"), ("un cochon", "rose"),
        ("le chocolat", "marron"), ("un éléphant", "gris"), ("un corbeau", "noir"),
        ("le lait", "blanc"),
    ]

    struct Forme { let name: String; let sub: String }
    static let formes: [Forme] = [
        Forme(name: "cercle", sub: "parfaitement rond"),
        Forme(name: "carré", sub: "4 côtés égaux"),
        Forme(name: "triangle", sub: "3 côtés"),
        Forme(name: "rectangle", sub: "4 côtés, 2 longs et 2 courts"),
        Forme(name: "losange", sub: "4 côtés égaux, penché"),
        Forme(name: "ovale", sub: "comme un œuf"),
        Forme(name: "étoile", sub: "5 branches"),
        Forme(name: "cœur", sub: "comme dans « je t'aime »"),
        Forme(name: "hexagone", sub: "6 côtés"),
        Forme(name: "octogone", sub: "8 côtés"),
    ]
    static let formeCotes: [String: Int] = ["triangle": 3, "carré": 4, "rectangle": 4,
                                            "losange": 4, "hexagone": 6, "octogone": 8]
    static let cotesUniques: [Int: String] = [3: "triangle", 6: "hexagone", 8: "octogone"]
    static let objetsForme: [(objet: String, forme: String)] = [
        ("d'une pièce de monnaie", "cercle"), ("d'un ballon", "cercle"), ("d'un œuf", "ovale"),
        ("d'une part de pizza", "triangle"), ("d'un panneau stop", "octogone"),
        ("d'une porte", "rectangle"), ("des alvéoles des abeilles", "hexagone"),
    ]

    static let unites = ["zéro", "un", "deux", "trois", "quatre", "cinq", "six", "sept", "huit",
                         "neuf", "dix", "onze", "douze", "treize", "quatorze", "quinze", "seize",
                         "dix-sept", "dix-huit", "dix-neuf"]
    static let dizaines: [Int: String] = [20: "vingt", 30: "trente", 40: "quarante",
                                          50: "cinquante", 60: "soixante", 80: "quatre-vingt"]

    // MARK: - Aléatoire

    private static func rand(_ lo: Int, _ hi: Int) -> Int { Int.random(in: lo...hi) }

    // MARK: - Nombres en toutes lettres

    static func numberToWords(_ n: Int) -> String {
        if n < 20 { return unites[n] }
        if n == 100 { return "cent" }
        var dizaine = (n / 10) * 10
        var unite = n - dizaine
        if dizaine == 70 || dizaine == 90 { dizaine -= 10; unite += 10 }
        if unite == 0 { return dizaine == 80 ? "quatre-vingts" : (dizaines[dizaine] ?? "") }
        if (unite == 1 || unite == 11) && dizaine != 80 {
            return "\(dizaines[dizaine] ?? "") et \(unites[unite])"
        }
        return "\(dizaines[dizaine] ?? "")-\(unites[unite])"
    }

    // MARK: - Fabrique de Question

    private static func makeQ(_ prompt: String, answer: String, choices: [String],
                              key: String? = nil, marbles: Int? = nil, perRow: Int = 5,
                              colorByRow: Bool = false, swatchHex: String? = nil,
                              shapeName: String? = nil) -> Question {
        Question(prompt: prompt, answer: answer, choices: choices,
                 options: ([answer] + choices).shuffled(),
                 dedupKey: key ?? prompt,
                 marbles: marbles, marblesPerRow: perRow, marblesColorByRow: colorByRow,
                 swatchHex: swatchHex, shapeName: shapeName)
    }

    // MARK: - Distracteurs

    private static func distinctFrom(_ list: [String], excludeIndex: Int, count: Int) -> [String] {
        var others = list
        others.remove(at: excludeIndex)
        return Array(others.shuffled().prefix(count))
    }

    private static func digitDistractors(_ answer: Int, _ lo: Int, _ hi: Int) -> [String] {
        var others: [Int] = []
        for i in lo...hi where i != answer { others.append(i) }
        return Array(others.shuffled().prefix(3)).map { String($0) }
    }

    private static func digitWordDistractors(_ n: Int) -> [String] {
        var others: [String] = []
        for i in 0...9 where i != n { others.append(unites[i].capFirst) }
        return Array(others.shuffled().prefix(3))
    }

    private static func numberDistractors(_ answer: Int, _ count: Int) -> [String] {
        var set = Set<Int>()
        var guardCount = 0
        while set.count < count && guardCount < 200 {
            guardCount += 1
            let span = max(3, Int((Double(abs(answer)) * 0.3).rounded()))
            let delta = rand(1, span)
            let candidate = Bool.random() ? answer - delta : answer + delta
            if candidate >= 0 && candidate != answer { set.insert(candidate) }
        }
        var extra = answer + count + 1
        while set.count < count { set.insert(extra); extra += 1 }
        return Array(set).map { String($0) }
    }

    private static func nearbyNumbers(_ n: Int, _ lo: Int, _ hi: Int) -> [String] {
        let raw = [n - 10, n + 10, n - 1, n + 1, n - 2, n + 2, n - 11, n + 11].shuffled()
        var set = [String]()
        for c in raw where c >= lo && c <= hi && c != n && !set.contains(String(c)) {
            set.append(String(c))
            if set.count == 3 { break }
        }
        var extra = lo
        while set.count < 3 { if extra != n && !set.contains(String(extra)) { set.append(String(extra)) }; extra += 1 }
        return set
    }

    // MARK: - Questions de séquence (jours, mois, saisons, alphabet)

    struct SeqConfig {
        var unit: String
        var unitPlural: String
        var feminine: Bool
        var cyclic: Bool
        var container: String
        var container2: String
        var display: ((String) -> String)? = nil
        var types: [String]? = nil
    }

    private static func ordinal(_ n: Int, _ feminine: Bool) -> String {
        n == 1 ? (feminine ? "1re" : "1er") : "\(n)e"
    }

    private static func makeSequence(_ list: [String], _ cfg: SeqConfig) -> Question {
        let types = cfg.types ?? ["apres", "avant", "position", "compte", "premier", "dernier"]
        let type = types.randomElement()!
        let len = list.count
        func wrap(_ i: Int) -> Int { ((i % len) + len) % len }
        let quel = cfg.feminine ? "Quelle" : "Quel"
        let show: (String) -> String = cfg.display ?? { $0 }

        switch type {
        case "apres":
            let i = cfg.cyclic ? rand(0, len - 1) : rand(0, len - 2)
            return makeQ("\(quel) \(cfg.unit) vient juste après \(show(list[i])) ?",
                         answer: list[wrap(i + 1)].capFirst,
                         choices: distinctFrom(list, excludeIndex: wrap(i + 1), count: 3).map { $0.capFirst })
        case "avant":
            let i = cfg.cyclic ? rand(0, len - 1) : rand(1, len - 1)
            return makeQ("\(quel) \(cfg.unit) vient juste avant \(show(list[i])) ?",
                         answer: list[wrap(i - 1)].capFirst,
                         choices: distinctFrom(list, excludeIndex: wrap(i - 1), count: 3).map { $0.capFirst })
        case "position":
            let i = rand(0, len - 1)
            let article = cfg.feminine ? "la" : "le"
            return makeQ("\(quel) est \(article) \(ordinal(i + 1, cfg.feminine)) \(cfg.unit) de \(cfg.container) ?",
                         answer: list[i].capFirst,
                         choices: distinctFrom(list, excludeIndex: i, count: 3).map { $0.capFirst })
        case "compte":
            let n = len
            let choices = [n - 2, n - 1, n + 1, n + 2].shuffled().prefix(3).map { String($0) }
            return makeQ("Combien y a-t-il de \(cfg.unitPlural) dans \(cfg.container2) ?",
                         answer: String(n), choices: Array(choices))
        case "premier":
            let first = cfg.feminine ? "la première" : "le premier"
            return makeQ("\(quel) est \(first) \(cfg.unit) de \(cfg.container) ?",
                         answer: list[0].capFirst,
                         choices: distinctFrom(list, excludeIndex: 0, count: 3).map { $0.capFirst })
        default: // dernier
            let last = cfg.feminine ? "la dernière" : "le dernier"
            return makeQ("\(quel) est \(last) \(cfg.unit) de \(cfg.container) ?",
                         answer: list[len - 1].capFirst,
                         choices: distinctFrom(list, excludeIndex: len - 1, count: 3).map { $0.capFirst })
        }
    }

    private static func makeMoisSaison() -> Question {
        let mois = Array(moisSaison.keys).randomElement()!
        let saison = moisSaison[mois]!
        return makeQ("En quelle saison est le mois de \(mois) ?",
                     answer: saison.capFirst,
                     choices: saisons.filter { $0 != saison }.map { $0.capFirst })
    }

    // MARK: - Couleurs

    private static func otherColorNames(_ exclude: [String], _ count: Int) -> [String] {
        Array(couleurs.filter { !exclude.contains($0.name) }.shuffled().prefix(count)).map { $0.name.capFirst }
    }

    private static func makeCouleurs() -> Question {
        let type = ["pastille", "pastille", "melange", "objet"].randomElement()!
        if type == "pastille" {
            let c = couleurs.randomElement()!
            return makeQ("Quelle est cette couleur ?", answer: c.name.capFirst,
                         choices: otherColorNames([c.name], 3),
                         key: "pastille:\(c.name)", swatchHex: c.hex)
        }
        if type == "melange" {
            let m = melangesCouleurs.randomElement()!
            return makeQ("Quelle couleur obtient-on en mélangeant du \(m.a) et du \(m.b) ?",
                         answer: m.donne.capFirst, choices: otherColorNames([m.a, m.b, m.donne], 3))
        }
        let o = objetsCouleur.randomElement()!
        return makeQ("De quelle couleur est \(o.objet) ?", answer: o.couleur.capFirst,
                     choices: otherColorNames([o.couleur], 3))
    }

    // MARK: - Formes

    private static func otherShapeNames(_ exclude: [String], _ count: Int) -> [String] {
        Array(formes.filter { !exclude.contains($0.name) }.shuffled().prefix(count)).map { $0.name.capFirst }
    }

    private static func makeFormes() -> Question {
        let type = ["visuelle", "visuelle", "cotes", "inverse", "objet"].randomElement()!
        if type == "visuelle" {
            let f = formes.randomElement()!
            return makeQ("Quelle est cette forme ?", answer: f.name.capFirst,
                         choices: otherShapeNames([f.name], 3),
                         key: "forme:\(f.name)", shapeName: f.name)
        }
        if type == "cotes" {
            if rand(0, 5) == 0 {
                return makeQ("Combien de branches a une étoile ?", answer: "5",
                             choices: Array(["3", "4", "6", "7"].shuffled().prefix(3)))
            }
            let name = Array(formeCotes.keys).randomElement()!
            let n = formeCotes[name]!
            var set = [String]()
            var d = 3
            while set.count < 3 { if d != n { set.append(String(d)) }; d += 1 }
            return makeQ("Combien de côtés a un \(name) ?", answer: String(n),
                         choices: Array(set.shuffled().prefix(3)), shapeName: name)
        }
        if type == "inverse" {
            let n = Array(cotesUniques.keys).randomElement()!
            let name = cotesUniques[n]!
            return makeQ("Quelle forme a \(n) côtés ?", answer: name.capFirst,
                         choices: otherShapeNames([name], 3))
        }
        let o = objetsForme.randomElement()!
        return makeQ("Quelle est la forme \(o.objet) ?", answer: o.forme.capFirst,
                     choices: otherShapeNames([o.forme], 3))
    }

    // MARK: - Chiffres 0 à 9

    private static func makeChiffres() -> Question {
        let type = ["compter", "compter", "ecrire", "lire", "apres", "avant"].randomElement()!
        switch type {
        case "compter":
            let n = rand(1, 9)
            return makeQ("Combien de billes comptes-tu ?", answer: String(n),
                         choices: digitDistractors(n, max(0, n - 3), min(9, n + 3)),
                         key: "billes:\(n)", marbles: n)
        case "ecrire":
            let n = rand(0, 9)
            return makeQ("Comment s'écrit le chiffre \(n) ?", answer: unites[n].capFirst,
                         choices: digitWordDistractors(n))
        case "lire":
            let n = rand(0, 9)
            return makeQ("Quel chiffre s'écrit « \(unites[n]) » ?", answer: String(n),
                         choices: digitDistractors(n, 0, 9))
        case "apres":
            let i = rand(0, 8)
            return makeQ("Quel chiffre vient juste après \(i) ?", answer: String(i + 1),
                         choices: digitDistractors(i + 1, 0, 9))
        default:
            let i = rand(1, 9)
            return makeQ("Quel chiffre vient juste avant \(i) ?", answer: String(i - 1),
                         choices: digitDistractors(i - 1, 0, 9))
        }
    }

    // MARK: - Nombres jusqu'à 50

    private static func makeCinquante() -> Question {
        let type = ["compter", "compter", "dizaines", "unites", "composer", "suite"].randomElement()!
        switch type {
        case "compter":
            let n = rand(11, 50)
            return makeQ("Combien de billes comptes-tu ?", answer: String(n),
                         choices: nearbyNumbers(n, 11, 50),
                         key: "billes50:\(n)", marbles: n, perRow: 10, colorByRow: true)
        case "dizaines":
            let n = rand(11, 50)
            let d = n / 10
            return makeQ("Dans le nombre \(n), combien y a-t-il de dizaines ?",
                         answer: String(d), choices: digitDistractors(d, 0, 5))
        case "unites":
            let n = rand(11, 49)
            let u = n % 10
            return makeQ("Dans le nombre \(n), combien y a-t-il d'unités ?",
                         answer: String(u), choices: digitDistractors(u, 0, 9))
        case "composer":
            let d = rand(1, 4)
            let u = rand(1, 9)
            let n = 10 * d + u
            let inversion = 10 * u + d
            var set = [String]()
            for c in [inversion, n - 1, n + 1, 10 * d, n + 10, n - 10]
            where c >= 1 && c <= 50 && c != n && !set.contains(String(c)) {
                set.append(String(c))
            }
            var extra = n + 2
            while set.count < 3 { if !set.contains(String(extra)) { set.append(String(extra)) }; extra += 1 }
            return makeQ("Avec \(d) dizaine\(d > 1 ? "s" : "") et \(u) unité\(u > 1 ? "s" : ""), quel nombre écris-tu ?",
                         answer: String(n), choices: Array(set.shuffled().prefix(3)))
        default: // suite
            if Bool.random() {
                let n = rand(10, 49)
                return makeQ("Quel nombre vient juste après \(n) ?", answer: String(n + 1),
                             choices: nearbyNumbers(n + 1, 0, 51))
            }
            let n = rand(11, 50)
            return makeQ("Quel nombre vient juste avant \(n) ?", answer: String(n - 1),
                         choices: nearbyNumbers(n - 1, 0, 50))
        }
    }

    // MARK: - Nombres en lettres

    private static func makeNombres(_ level: Int) -> Question {
        let ranges = [(0, 16), (17, 69), (60, 100)]
        let (lo, hi) = ranges[level - 1]
        let n = rand(lo, hi)
        var set = Set<Int>()
        var guardCount = 0
        while set.count < 3 && guardCount < 100 {
            guardCount += 1
            let delta = Bool.random() ? rand(1, 6) : -rand(1, 6)
            let d = min(max(n + delta, lo), hi)
            if d != n { set.insert(d) }
        }
        var extra = lo
        while set.count < 3 { if extra != n { set.insert(extra) }; extra += 1 }
        let nums = Array(set)
        if Bool.random() {
            return makeQ("Comment s'écrit le nombre \(n) ?", answer: numberToWords(n),
                         choices: nums.map { numberToWords($0) })
        }
        return makeQ("Quel nombre s'écrit « \(numberToWords(n)) » ?", answer: String(n),
                     choices: nums.map { String($0) })
    }

    // MARK: - Calcul

    private static func makeMath(_ op: String, _ level: Int) -> Question {
        var a = 0, b = 0, result = 0, symbol = "+"
        switch op {
        case "addition":
            let mx = [10, 20, 100][level - 1]
            a = rand(1, mx); b = rand(1, mx); result = a + b; symbol = "+"
        case "soustraction":
            let mx = [10, 20, 100][level - 1]
            a = rand(1, mx); b = rand(1, mx)
            if b > a { swap(&a, &b) }
            result = a - b; symbol = "−"
        case "multiplication":
            let mx = [5, 10, 12][level - 1]
            a = rand(1, mx); b = rand(1, 10); result = a * b; symbol = "×"
        default: // division exacte
            let mx = [5, 10, 12][level - 1]
            b = rand(2, mx); result = rand(1, mx); a = b * result; symbol = "÷"
        }
        return makeQ("\(a) \(symbol) \(b) = ?", answer: String(result),
                     choices: numberDistractors(result, 3))
    }

    // MARK: - Aiguillage

    private static func makeQuestion(_ category: String, _ level: Int) -> Question {
        switch category {
        case "jours":
            return makeSequence(jours, SeqConfig(unit: "jour", unitPlural: "jours", feminine: false,
                cyclic: true, container: "la semaine", container2: "une semaine"))
        case "mois":
            return makeSequence(mois, SeqConfig(unit: "mois", unitPlural: "mois", feminine: false,
                cyclic: true, container: "l'année", container2: "une année"))
        case "saisons":
            if Bool.random() { return makeMoisSaison() }
            return makeSequence(saisons, SeqConfig(unit: "saison", unitPlural: "saisons", feminine: true,
                cyclic: true, container: "l'année", container2: "une année",
                display: { QuestionEngine.saisonArticle[$0] ?? $0 }, types: ["apres", "avant", "compte"]))
        case "alphabet":
            return makeSequence(alphabet, SeqConfig(unit: "lettre", unitPlural: "lettres", feminine: true,
                cyclic: false, container: "l'alphabet", container2: "l'alphabet",
                display: { "la lettre \($0)" }))
        case "couleurs": return makeCouleurs()
        case "formes": return makeFormes()
        case "chiffres": return makeChiffres()
        case "cinquante": return makeCinquante()
        case "nombres": return makeNombres(level)
        default: return makeMath(category, level)
        }
    }

    // MARK: - Construction d'un quiz

    static let melangeSources = ["chiffres", "cinquante", "nombres", "addition", "soustraction",
                                 "multiplication", "division", "jours", "mois", "saisons",
                                 "alphabet", "couleurs", "formes"]

    private static func buildInterleaved() -> [Question] {
        let sources = melangeSources.shuffled()
        var qs: [Question] = []
        var seen = Set<String>()
        var i = 0
        var guardCount = 0
        while qs.count < nbQuestions && guardCount < 400 {
            guardCount += 1
            let cat = sources[i % sources.count]
            i += 1
            var q = makeQuestion(cat, rand(1, 2))
            let key = "\(cat):\(q.dedupKey)"
            if seen.contains(key) { continue }
            seen.insert(key)
            q.category = cat
            qs.append(q)
        }
        return qs
    }

    // Catégories à ensemble de questions fini : on peut y prioriser les révisions
    // dues. Le calcul (addition, etc.) a un espace infini, on le laisse aléatoire.
    static let memoryBiased: Set<String> = ["jours", "mois", "saisons", "alphabet",
        "couleurs", "formes", "chiffres", "cinquante", "nombres"]

    static func buildQuestions(category: String, level: Int) -> [Question] {
        if category == "melange" { return buildInterleaved() }
        if memoryBiased.contains(category) { return buildWithMemory(category, level) }
        return buildRandom(category, level)
    }

    private static func buildRandom(_ category: String, _ level: Int) -> [Question] {
        var qs: [Question] = []
        var seen = Set<String>()
        var guardCount = 0
        while qs.count < nbQuestions && guardCount < 300 {
            guardCount += 1
            let q = makeQuestion(category, level)
            if seen.contains(q.dedupKey) { continue }
            seen.insert(q.dedupKey)
            qs.append(q)
        }
        return qs
    }

    // Sélection guidée par la mémoire : on construit un pool de questions à clés
    // distinctes, puis on fait passer en premier les notions à réviser (date dépassée),
    // ensuite les notions jamais vues, enfin le reste. Aléatoire à l'intérieur de
    // chaque groupe pour varier.
    private static func buildWithMemory(_ category: String, _ level: Int) -> [Question] {
        let states = MemoryStore.shared.keyStates(category: category)
        let now = Date().timeIntervalSince1970

        var pool: [Question] = []
        var seen = Set<String>()
        var guardCount = 0
        while pool.count < 60 && guardCount < 800 {
            guardCount += 1
            let q = makeQuestion(category, level)
            if seen.contains(q.dedupKey) { continue }
            seen.insert(q.dedupKey)
            pool.append(q)
        }

        // 0 = à réviser (dû), 1 = jamais vu, 2 = déjà su
        func tier(_ q: Question) -> Int {
            guard let st = states["\(category):\(q.dedupKey)"] else { return 1 }
            if st.due > 0 && st.due <= now { return 0 }
            return 2
        }

        let ordered = pool.shuffled().sorted { tier($0) < tier($1) }
        return Array(ordered.prefix(nbQuestions))
    }

    static func pickPraise() -> String {
        ["✅ Bravo !", "✅ Super !", "✅ Exact !", "✅ Génial !"].randomElement()!
    }
}
