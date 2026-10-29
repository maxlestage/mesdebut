import Foundation

// Les tours de Hanoï, telles que les joue le noyau Rust (`noyau/src/hanoi.rs`) :
// les mêmes règles et le même bilan que sur le web. Swift ne garde que la
// partie en cours et la dessine.

struct HanoiLevel: Decodable, Identifiable {
    let id: Int
    let emoji: String
    let libelle: String
    let disques: Int
}

/// Une partie : les trois tiges (du bas vers le haut, un disque est sa taille),
/// les coups joués, et le bilan une fois la pile arrivée.
struct HanoiGame: Decodable, Equatable {
    /// Le dernier coup demandé a-t-il été joué ?
    let accepte: Bool
    let tours: [[Int]]
    let coups: Int
    let disques: Int
    let minimum: Int
    let gagne: Bool
    let etoiles: String?
    let message: String?
}

enum HanoiEngine {
    static let levels: [HanoiLevel] = Noyau.appel("hanoi_niveaux", comme: [HanoiLevel].self) ?? []

    static func start(disks: Int) -> HanoiGame? {
        Noyau.appel("hanoi_depart", "\(disks)", comme: HanoiGame.self)
    }

    /// Le disque du haut de la tige `from` sur la tige `to`. La partie rendue a
    /// `accepte == false` si le coup est interdit ; rien n'a bougé alors.
    static func move(_ game: HanoiGame, from: Int, to: Int) -> HanoiGame? {
        let tete: [CustomStringConvertible] = [from, to, game.coups, game.disques]
        let tiges = game.tours.map { $0.map { $0 as CustomStringConvertible } }
        return Noyau.appel("hanoi_deplacer", Noyau.lignes([tete] + tiges), comme: HanoiGame.self)
    }
}
