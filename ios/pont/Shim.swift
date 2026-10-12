// Sous Linux, SwiftUI n'existe pas : juste assez de `Color` pour compiler les
// fichiers de ReNeuroKit qui en parlent (les dégradés des thèmes).
struct Color {
    let rouge: Double, vert: Double, bleu: Double

    init(red: Double, green: Double, blue: Double) {
        (rouge, vert, bleu) = (red, green, blue)
    }
}
