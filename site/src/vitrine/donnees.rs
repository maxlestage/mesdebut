//! Contenu de la page de présentation : les supports, les principes, l'offre.
//!
//! Les thèmes, eux, viennent du moteur (`moteur::categories`) : la page présente
//! exactement ceux que l'application propose.

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
    Support {
        emoji: "💻",
        nom: "Navigateur",
        detail: "L'application complète, sur le web, sans rien installer.",
        note: "Les dix-huit thèmes, la mémoire de ce qui résiste et « Ma journée », dans n'importe quel navigateur.",
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
        "Les dix-huit thèmes, sans exception",
        "L'iPhone, l'Apple Watch et le navigateur",
        "La progression retenue d'une séance à l'autre",
        "Le planificateur de journée et son suivi",
        "Les mises à jour et les thèmes à venir",
    ],
    disponibilite: "Bientôt sur l'App Store",
};
