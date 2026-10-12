import Foundation
import ReNeuroNoyauC

/// Le noyau de ReNeuro, en Rust (`noyau/` à la racine du dépôt) : les thèmes,
/// les questions, la répétition espacée et la journée planifiée. Le même code
/// sert l'iPhone, la montre et l'application web ; Swift ne garde que
/// l'interface et le stockage.
///
/// Le noyau ne connaît que du texte : on lui passe des arguments (champs
/// séparés par des tabulations, lignes par des sauts de ligne), il répond en
/// JSON. La liste des fonctions est dans `noyau/src/ffi.rs`.
enum Noyau {
    private static let decodeur: JSONDecoder = {
        let d = JSONDecoder()
        d.keyDecodingStrategy = .convertFromSnakeCase
        return d
    }()

    /// Appelle `fonction` et lit sa réponse. `nil` si le noyau signale une
    /// erreur — ce qui ne devrait jamais arriver : les tests le vérifient.
    static func appel<Reponse: Decodable>(_ fonction: String, _ arguments: String = "",
                                          comme type: Reponse.Type = Reponse.self) -> Reponse? {
        guard let brut = reneuro_appel(fonction, arguments) else { return nil }
        defer { reneuro_liberer(brut) }
        let texte = String(cString: brut)
        do {
            return try decodeur.decode(Reponse.self, from: Data(texte.utf8))
        } catch {
            assertionFailure("noyau, « \(fonction) » : \(texte) (\(error))")
            return nil
        }
    }

    /// Met des arguments en forme : une ligne par élément, des tabulations
    /// entre les champs.
    static func lignes(_ lignes: [[CustomStringConvertible]]) -> String {
        lignes.map { $0.map { "\($0)" }.joined(separator: "\t") }.joined(separator: "\n")
    }

    /// Un booléen, tel que le noyau le lit.
    static func bool(_ b: Bool) -> String { b ? "1" : "0" }
}

// MARK: - Ce que répond le noyau

struct NoyauNiveau: Decodable {
    let id: Int
    let emoji: String
    let libelle: String
}

struct NoyauCategorie: Decodable {
    let cle: String
    let emoji: String
    let libelle: String
    let titre: String
    let revision: Bool
    let niveaux: [NoyauNiveau]
    let degrade: [String]
}

/// Ce qui illustre une question ou une ligne de révision. `genre` dit lequel
/// des champs est rempli.
struct NoyauIllustration: Decodable {
    let genre: String
    var nombre: Int?
    var parRang: Int?
    var couleurParRang: Bool?
    var hex: String?
    var nom: String?
    var heures: Int?
    var minutes: Int?
    var etapes: [String]?
    var trou: Int?
}

struct NoyauQuestion: Decodable {
    let enonce: String
    let reponse: String
    let leurres: [String]
    let options: [String]
    let cle: String
    let categorie: String?
    let illustration: NoyauIllustration?
}

struct NoyauFiche: Decodable {
    let libelle: String
    let detail: String?
    let numero: Int?
    let image: NoyauIllustration?
}

struct NoyauGroupe: Decodable {
    let libelle: String
    let fiches: [NoyauFiche]
}

/// « Réviser d'abord » : une liste, une grille de lettres ou un accordéon.
struct NoyauRevision: Decodable {
    let genre: String
    let titre: String
    var intro: String?
    var fiches: [NoyauFiche]?
    var lettres: [String]?
    var groupes: [NoyauGroupe]?
}

struct NoyauConstantes: Decodable {
    /// Questions d'un quiz, avant celles qui reviennent.
    let nbQuestions: Int
    /// Boîte de Leitner à partir de laquelle une notion est maîtrisée.
    let maitrise: Int
    /// Durées proposées dans « Ma journée », en minutes.
    let durees: [Int]
}

extension Noyau {
    static let constantes: NoyauConstantes = appel("constantes")
        ?? NoyauConstantes(nbQuestions: 10, maitrise: 4, durees: [15])
}
