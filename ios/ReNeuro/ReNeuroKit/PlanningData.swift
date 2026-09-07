import Foundation

// Planifier — décomposer une activité en étapes, les ordonner, estimer le temps
// qu'elles prennent. C'est une fonction exécutive, et elle se rééduque.
//
// Port de la partie « planifier » de src/questions.js. Les activités sont celles
// de la vie quotidienne, et leurs étapes s'enchaînent vraiment : chaque étape
// suppose la précédente, pour qu'il n'y ait qu'un seul ordre correct.
// `duree` est une estimation en minutes, volontairement ronde.

struct Routine: Identifiable {
    let key: String
    let emoji: String
    let label: String
    /// Durée estimée, en minutes.
    let duree: Int
    let etapes: [String]
    var id: String { key }
}

let ROUTINES: [Routine] = [
    Routine(key: "medicament", emoji: "💊", label: "Prendre son médicament", duree: 2, etapes: [
        "Regarder l'ordonnance",
        "Sortir la bonne boîte",
        "Prendre le comprimé avec un verre d'eau",
        "Cocher sur le carnet",
    ]),
    Routine(key: "dents", emoji: "🦷", label: "Se brosser les dents", duree: 3, etapes: [
        "Prendre la brosse à dents",
        "Mettre le dentifrice dessus",
        "Se brosser les dents",
        "Se rincer la bouche",
    ]),
    Routine(key: "cafe", emoji: "☕", label: "Se faire un café", duree: 5, etapes: [
        "Faire chauffer l'eau",
        "Mettre le café dans la tasse",
        "Verser l'eau chaude dans la tasse",
        "Remuer avec la cuillère",
    ]),
    Routine(key: "sandwich", emoji: "🥪", label: "Préparer un sandwich", duree: 5, etapes: [
        "Sortir le pain et le jambon",
        "Couper le pain en deux",
        "Étaler le beurre",
        "Poser le jambon dessus",
        "Refermer le sandwich",
    ]),
    Routine(key: "lessive", emoji: "🧺", label: "Faire une lessive", duree: 10, etapes: [
        "Trier le linge sale",
        "Mettre le linge dans la machine",
        "Ajouter la lessive",
        "Lancer la machine",
        "Étendre le linge",
    ]),
    Routine(key: "habiller", emoji: "👕", label: "S'habiller", duree: 10, etapes: [
        "Choisir ses vêtements",
        "Enlever son pyjama",
        "Mettre son pantalon et son haut",
        "Mettre ses chaussettes",
        "Mettre ses chaussures",
    ]),
    Routine(key: "pates", emoji: "🍝", label: "Préparer des pâtes", duree: 15, etapes: [
        "Remplir la casserole d'eau",
        "Faire bouillir l'eau",
        "Verser les pâtes dans l'eau",
        "Attendre la cuisson",
        "Égoutter les pâtes",
    ]),
    Routine(key: "douche", emoji: "🚿", label: "Prendre une douche", duree: 15, etapes: [
        "Préparer sa serviette",
        "Régler la température de l'eau",
        "Se laver",
        "Se sécher avec la serviette",
    ]),
    Routine(key: "lettre", emoji: "✉️", label: "Envoyer une lettre", duree: 15, etapes: [
        "Écrire la lettre",
        "Mettre la lettre dans l'enveloppe",
        "Coller le timbre",
        "Poster la lettre dans la boîte",
    ]),
    Routine(key: "bus", emoji: "🚌", label: "Prendre le bus", duree: 20, etapes: [
        "Regarder l'horaire du bus",
        "Aller à l'arrêt",
        "Monter dans le bus",
        "Valider son ticket",
        "Descendre au bon arrêt",
    ]),
    Routine(key: "courses", emoji: "🛒", label: "Faire les courses", duree: 45, etapes: [
        "Écrire la liste des courses",
        "Aller au magasin",
        "Remplir le panier",
        "Passer à la caisse",
        "Ranger les courses",
    ]),
]

func routine(_ key: String) -> Routine? { ROUTINES.first { $0.key == key } }

/// « 9 h 00 », « 10 h 05 » — la notation d'un agenda, plus lisible ici que les
/// lettres, et cohérente avec la catégorie « Lire l'heure ».
func formatHeure(_ h: Int, _ m: Int) -> String {
    String(format: "%d h %02d", h, m)
}

/// Ajoute des minutes à une heure, sur 24 h.
func ajouteMinutes(_ h: Int, _ m: Int, _ minutes: Int) -> (Int, Int) {
    let total = ((h * 60 + m + minutes) % 1440 + 1440) % 1440
    return (total / 60, total % 60)
}
