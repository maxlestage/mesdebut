//! Contenu du site : les thèmes présentés, les supports, les principes, l'offre.
//!
//! Cette liste reprend les catégories de l'application iOS (`ios/`), qui reste
//! la seule implémentation. Les deux sont écrites séparément : en ajouter une ici
//! suppose de la retrouver là-bas.

pub struct Categorie {
    pub key: &'static str,
    pub emoji: &'static str,
    pub label: &'static str,
    pub title: &'static str,
    /// Dégradé repris de l'application, pour que les couleurs se correspondent.
    pub degrade: (&'static str, &'static str),
    pub niveaux: &'static [&'static str],
}

const TROIS: &[&str] = &["Facile", "Moyen", "Difficile"];

pub const CATEGORIES: &[Categorie] = &[
    Categorie {
        key: "melange",
        emoji: "🧠",
        label: "Mélange",
        title: "Quiz mélangé",
        degrade: ("#26a69a", "#00695c"),
        niveaux: &[],
    },
    Categorie {
        key: "jours",
        emoji: "📅",
        label: "Les jours",
        title: "Les jours de la semaine",
        degrade: ("#f6a821", "#f68f21"),
        niveaux: &[],
    },
    Categorie {
        key: "mois",
        emoji: "🗓️",
        label: "Les mois",
        title: "Les mois de l'année",
        degrade: ("#21b3f6", "#2179f6"),
        niveaux: &[],
    },
    Categorie {
        key: "saisons",
        emoji: "🍂",
        label: "Les saisons",
        title: "Les saisons",
        degrade: ("#8bc34a", "#5a9216"),
        niveaux: &[],
    },
    Categorie {
        key: "alphabet",
        emoji: "🔤",
        label: "L'alphabet",
        title: "L'alphabet",
        degrade: ("#26c6da", "#0097a7"),
        niveaux: &[],
    },
    Categorie {
        key: "heure",
        emoji: "🕐",
        label: "L'heure",
        title: "Lire l'heure",
        degrade: ("#ab47bc", "#6a1b9a"),
        niveaux: &[
            "Heures pleines",
            "Quarts et demies",
            "De 5 en 5",
            "Minute par minute",
        ],
    },
    Categorie {
        key: "planifier",
        emoji: "📋",
        label: "Planifier",
        title: "Apprendre à planifier",
        degrade: ("#26c6da", "#00838f"),
        niveaux: &["Les étapes", "L'ordre", "Le temps"],
    },
    Categorie {
        key: "couleurs",
        emoji: "🎨",
        label: "Les couleurs",
        title: "Les couleurs",
        degrade: ("#ff8a65", "#d84315"),
        niveaux: &[],
    },
    Categorie {
        key: "formes",
        emoji: "📐",
        label: "Les formes",
        title: "Les formes géométriques",
        degrade: ("#78909c", "#455a64"),
        niveaux: &[],
    },
    Categorie {
        key: "chiffres",
        emoji: "🧮",
        label: "Les chiffres",
        title: "Les chiffres de 0 à 9",
        degrade: ("#ffca28", "#f57f17"),
        niveaux: &[],
    },
    Categorie {
        key: "cinquante",
        emoji: "🔟",
        label: "Jusqu'à 50",
        title: "Les nombres jusqu'à 50",
        degrade: ("#ec407a", "#ad1457"),
        niveaux: &[],
    },
    Categorie {
        key: "nombres",
        emoji: "🔢",
        label: "Les nombres",
        title: "Les nombres en lettres",
        degrade: ("#5c6bc0", "#3949ab"),
        niveaux: TROIS,
    },
    Categorie {
        key: "addition",
        emoji: "➕",
        label: "Addition",
        title: "Addition",
        degrade: ("#4cd964", "#2eb350"),
        niveaux: TROIS,
    },
    Categorie {
        key: "soustraction",
        emoji: "➖",
        label: "Soustraction",
        title: "Soustraction",
        degrade: ("#ff6b6b", "#e04545"),
        niveaux: TROIS,
    },
    Categorie {
        key: "multiplication",
        emoji: "✖️",
        label: "Multiplication",
        title: "Multiplication",
        degrade: ("#a76bff", "#7d45e0"),
        niveaux: TROIS,
    },
    Categorie {
        key: "division",
        emoji: "➗",
        label: "Division",
        title: "Division",
        degrade: ("#ff6bd5", "#e045a8"),
        niveaux: TROIS,
    },
];

pub struct Support {
    pub emoji: &'static str,
    pub nom: &'static str,
    pub detail: &'static str,
    pub note: &'static str,
}

pub const SUPPORTS: &[Support] = &[
    Support {
        emoji: "📱",
        nom: "iPhone",
        detail: "Application native, écrite en Swift.",
        note: "Le quiz en cours s'affiche sur l'écran verrouillé et la Dynamic Island.",
    },
    Support {
        emoji: "⌚",
        nom: "Apple Watch",
        detail: "Les mêmes exercices, au poignet.",
        note: "Pratique pour l'heure : apprendre à lire une montre sur une montre.",
    },
];

pub struct Principe {
    pub titre: &'static str,
    pub texte: &'static str,
}

pub const PRINCIPES: &[Principe] = &[
    Principe {
        titre: "On répond, on ne relit pas",
        texte: "Se tester ancre mieux que relire. Chaque thème est une suite de questions, pas une leçon.",
    },
    Principe {
        titre: "La réponse tout de suite",
        texte: "Juste ou faux, on le sait immédiatement, et la bonne réponse s'affiche.",
    },
    Principe {
        titre: "Ce qui est raté revient",
        texte: "Une question manquée revient 3 questions plus loin, puis 5 après. D'une session à l'autre, l'app se souvient de ce qui résiste.",
    },
    Principe {
        titre: "Les thèmes sont mélangés",
        texte: "Le mode Mélange alterne les sujets plutôt que de les réviser en blocs : il faut à chaque fois reconnaître de quoi il s'agit.",
    },
    Principe {
        titre: "Une image avec chaque mot",
        texte: "Des billes à compter, des horloges à aiguilles, des formes et des couleurs dessinées.",
    },
    Principe {
        titre: "On encourage, on ne sanctionne pas",
        texte: "Des étoiles, des félicitations, et jamais de reproche sur une erreur.",
    },
];

/// Le manifeste qui s'allume mot à mot au défilement. Les mots entre `*` sont
/// les mots d'accent, en serif italique.
pub const MANIFESTE: &str = "Se tester ancre mieux que relire. Alors ici, on *répond*, on ne relit pas. \
    Juste ou faux, on le sait tout de suite. Ce qui est raté revient, \
    et jamais un reproche : des *étoiles*.";

pub struct Offre {
    /// Le prix en centimes, pour que le compteur l'atteigne exactement.
    pub centimes: u32,
    pub prix: &'static str,
    pub periode: &'static str,
    pub inclus: &'static [&'static str],
    pub disponibilite: &'static str,
}

pub const OFFRE: Offre = Offre {
    centimes: 499,
    prix: "4,99 €",
    periode: "par mois",
    inclus: &[
        "Les seize thèmes, sans exception",
        "L'iPhone et l'Apple Watch",
        "La progression retenue d'une séance à l'autre",
        "Le planificateur de journée et son suivi",
        "Les mises à jour et les thèmes à venir",
    ],
    disponibilite: "Bientôt sur l'App Store",
};
