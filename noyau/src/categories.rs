//! Les dix-sept thèmes : leur nom, leurs couleurs, leurs niveaux.
//!
//! Une seule liste pour tout le site : la page de présentation et l'application
//! la lisent toutes deux. Elle reprend celle de l'application iOS
//! (`ios/ReNeuro/ReNeuroKit/CategoryData.swift`), écrite séparément : ajouter un
//! thème ici suppose de l'ajouter là-bas aussi.

pub struct Niveau {
    pub id: u8,
    pub emoji: &'static str,
    pub libelle: &'static str,
}

pub struct Categorie {
    pub cle: &'static str,
    pub emoji: &'static str,
    pub libelle: &'static str,
    pub titre: &'static str,
    /// Le thème a un écran de révision avant le quiz.
    pub revision: bool,
    /// Vide : un seul niveau, le quiz démarre directement.
    pub niveaux: &'static [Niveau],
    /// Dégradé du bouton, repris de l'application iOS.
    pub degrade: (&'static str, &'static str),
}

impl Categorie {
    pub fn a_des_niveaux(&self) -> bool {
        !self.niveaux.is_empty()
    }
}

/// Les niveaux communs aux thèmes qui n'ont pas les leurs.
const TROIS: &[Niveau] = &[
    Niveau {
        id: 1,
        emoji: "🌱",
        libelle: "Facile",
    },
    Niveau {
        id: 2,
        emoji: "🌿",
        libelle: "Moyen",
    },
    Niveau {
        id: 3,
        emoji: "🌳",
        libelle: "Difficile",
    },
];

pub const CATEGORIES: &[Categorie] = &[
    Categorie {
        cle: "melange",
        emoji: "🧠",
        libelle: "Mélange",
        titre: "Quiz mélangé",
        revision: false,
        niveaux: &[],
        degrade: ("#26a69a", "#00695c"),
    },
    Categorie {
        cle: "jours",
        emoji: "📅",
        libelle: "Les jours",
        titre: "Les jours de la semaine",
        revision: true,
        niveaux: &[],
        degrade: ("#f6a821", "#f68f21"),
    },
    Categorie {
        cle: "mois",
        emoji: "🗓️",
        libelle: "Les mois",
        titre: "Les mois de l'année",
        revision: true,
        niveaux: &[],
        degrade: ("#21b3f6", "#2179f6"),
    },
    Categorie {
        cle: "saisons",
        emoji: "🍂",
        libelle: "Les saisons",
        titre: "Les saisons",
        revision: true,
        niveaux: &[],
        degrade: ("#8bc34a", "#5a9216"),
    },
    Categorie {
        cle: "alphabet",
        emoji: "🔤",
        libelle: "L'alphabet",
        titre: "L'alphabet",
        revision: true,
        niveaux: &[],
        degrade: ("#26c6da", "#0097a7"),
    },
    Categorie {
        cle: "heure",
        emoji: "🕐",
        libelle: "L'heure",
        titre: "Lire l'heure",
        revision: true,
        // de l'heure pile à la minute près
        niveaux: &[
            Niveau {
                id: 1,
                emoji: "🌱",
                libelle: "Heures pleines",
            },
            Niveau {
                id: 2,
                emoji: "🌿",
                libelle: "Quarts et demies",
            },
            Niveau {
                id: 3,
                emoji: "🌳",
                libelle: "De 5 en 5",
            },
            Niveau {
                id: 4,
                emoji: "🏆",
                libelle: "Minute par minute",
            },
        ],
        degrade: ("#ab47bc", "#6a1b9a"),
    },
    Categorie {
        cle: "planifier",
        emoji: "📋",
        libelle: "Planifier",
        titre: "Apprendre à planifier",
        revision: true,
        // du plus concret au plus abstrait : les étapes, leur ordre, le temps
        niveaux: &[
            Niveau {
                id: 1,
                emoji: "🌱",
                libelle: "Les étapes",
            },
            Niveau {
                id: 2,
                emoji: "🌿",
                libelle: "L'ordre",
            },
            Niveau {
                id: 3,
                emoji: "🌳",
                libelle: "Le temps",
            },
        ],
        degrade: ("#26c6da", "#00838f"),
    },
    Categorie {
        cle: "couleurs",
        emoji: "🎨",
        libelle: "Les couleurs",
        titre: "Les couleurs",
        revision: true,
        niveaux: &[],
        degrade: ("#ff8a65", "#d84315"),
    },
    Categorie {
        cle: "formes",
        emoji: "📐",
        libelle: "Les formes",
        titre: "Les formes géométriques",
        revision: true,
        niveaux: &[],
        degrade: ("#78909c", "#455a64"),
    },
    Categorie {
        cle: "chiffres",
        emoji: "🧮",
        libelle: "Les chiffres",
        titre: "Les chiffres de 0 à 9",
        revision: true,
        niveaux: &[],
        degrade: ("#ffca28", "#f57f17"),
    },
    Categorie {
        cle: "cinquante",
        emoji: "🔟",
        libelle: "Jusqu'à 50",
        titre: "Les nombres jusqu'à 50",
        revision: true,
        niveaux: &[],
        degrade: ("#ec407a", "#ad1457"),
    },
    Categorie {
        cle: "nombres",
        emoji: "🔢",
        libelle: "Les nombres",
        titre: "Les nombres en lettres",
        revision: true,
        niveaux: TROIS,
        degrade: ("#5c6bc0", "#3949ab"),
    },
    Categorie {
        cle: "addition",
        emoji: "➕",
        libelle: "Addition",
        titre: "Addition",
        revision: false,
        niveaux: TROIS,
        degrade: ("#4cd964", "#2eb350"),
    },
    Categorie {
        cle: "soustraction",
        emoji: "➖",
        libelle: "Soustraction",
        titre: "Soustraction",
        revision: false,
        niveaux: TROIS,
        degrade: ("#ff6b6b", "#e04545"),
    },
    Categorie {
        cle: "multiplication",
        emoji: "✖️",
        libelle: "Multiplication",
        titre: "Multiplication",
        revision: false,
        niveaux: TROIS,
        degrade: ("#a76bff", "#7d45e0"),
    },
    Categorie {
        cle: "division",
        emoji: "➗",
        libelle: "Division",
        titre: "Division",
        revision: false,
        niveaux: TROIS,
        degrade: ("#ff6bd5", "#e045a8"),
    },
    Categorie {
        cle: "logique",
        emoji: "🧩",
        libelle: "Logique",
        titre: "Jeux de logique",
        revision: true,
        // ce qui se voit, ce qui se compare, ce qui se déduit
        niveaux: &[
            Niveau {
                id: 1,
                emoji: "🌱",
                libelle: "Les suites",
            },
            Niveau {
                id: 2,
                emoji: "🌿",
                libelle: "L'intrus",
            },
            Niveau {
                id: 3,
                emoji: "🌳",
                libelle: "Les déductions",
            },
        ],
        degrade: ("#ffa726", "#6d4c41"),
    },
];

/// Le thème de cette clé, s'il existe.
pub fn categorie(cle: &str) -> Option<&'static Categorie> {
    CATEGORIES.iter().find(|c| c.cle == cle)
}
