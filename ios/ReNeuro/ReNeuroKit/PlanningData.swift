import Foundation

// Planifier — décomposer une activité en étapes, les ordonner, estimer le temps
// qu'elles prennent. Les onze activités du quotidien, leurs étapes et leur durée
// viennent du noyau Rust : les mêmes sur l'iPhone, la montre et le web.

struct Routine: Identifiable, Decodable {
    let key: String
    let emoji: String
    let label: String
    /// Durée estimée, en minutes.
    let duree: Int
    let etapes: [String]
    var id: String { key }

    private enum CodingKeys: String, CodingKey {
        case key = "cle", emoji, label = "libelle", duree, etapes
    }
}

let ROUTINES: [Routine] = Noyau.appel("routines") ?? []

func routine(_ key: String) -> Routine? { ROUTINES.first { $0.key == key } }
