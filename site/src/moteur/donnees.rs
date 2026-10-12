//! Le contenu des questions : jours, mois, couleurs, formes, nombres et heures
//! en toutes lettres.

pub const JOURS: [&str; 7] = [
    "lundi", "mardi", "mercredi", "jeudi", "vendredi", "samedi", "dimanche",
];

pub const MOIS: [&str; 12] = [
    "janvier",
    "février",
    "mars",
    "avril",
    "mai",
    "juin",
    "juillet",
    "août",
    "septembre",
    "octobre",
    "novembre",
    "décembre",
];

pub const ALPHABET: [&str; 26] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z",
];

pub const SAISONS: [&str; 4] = ["printemps", "été", "automne", "hiver"];

pub fn saison_avec_article(saison: &str) -> &'static str {
    match saison {
        "printemps" => "le printemps",
        "été" => "l'été",
        "automne" => "l'automne",
        _ => "l'hiver",
    }
}

/// Les mois qui tombent entièrement dans une saison : mars, juin, septembre et
/// décembre changent de saison en cours de route, on ne les demande pas.
pub const MOIS_SAISON: [(&str, &str); 8] = [
    ("janvier", "hiver"),
    ("février", "hiver"),
    ("avril", "printemps"),
    ("mai", "printemps"),
    ("juillet", "été"),
    ("août", "été"),
    ("octobre", "automne"),
    ("novembre", "automne"),
];

pub struct Couleur {
    pub nom: &'static str,
    pub hex: &'static str,
}

pub const COULEURS: [Couleur; 11] = [
    Couleur {
        nom: "rouge",
        hex: "#e53935",
    },
    Couleur {
        nom: "bleu",
        hex: "#1e88e5",
    },
    Couleur {
        nom: "jaune",
        hex: "#fdd835",
    },
    Couleur {
        nom: "vert",
        hex: "#43a047",
    },
    Couleur {
        nom: "orange",
        hex: "#fb8c00",
    },
    Couleur {
        nom: "violet",
        hex: "#8e24aa",
    },
    Couleur {
        nom: "rose",
        hex: "#f06292",
    },
    Couleur {
        nom: "marron",
        hex: "#795548",
    },
    Couleur {
        nom: "gris",
        hex: "#9e9e9e",
    },
    Couleur {
        nom: "noir",
        hex: "#212121",
    },
    Couleur {
        nom: "blanc",
        hex: "#ffffff",
    },
];

/// (a, b, ce que donne leur mélange)
pub const MELANGES_COULEURS: [(&str, &str, &str); 5] = [
    ("bleu", "jaune", "vert"),
    ("rouge", "jaune", "orange"),
    ("rouge", "bleu", "violet"),
    ("rouge", "blanc", "rose"),
    ("noir", "blanc", "gris"),
];

pub const OBJETS_COULEUR: [(&str, &str); 13] = [
    ("un citron", "jaune"),
    ("une banane", "jaune"),
    ("le soleil", "jaune"),
    ("le ciel", "bleu"),
    ("un sapin", "vert"),
    ("une tomate", "rouge"),
    ("une fraise", "rouge"),
    ("une carotte", "orange"),
    ("un cochon", "rose"),
    ("le chocolat", "marron"),
    ("un éléphant", "gris"),
    ("un corbeau", "noir"),
    ("le lait", "blanc"),
];

pub struct Forme {
    pub nom: &'static str,
    pub detail: &'static str,
}

pub const FORMES: [Forme; 10] = [
    Forme {
        nom: "cercle",
        detail: "parfaitement rond",
    },
    Forme {
        nom: "carré",
        detail: "4 côtés égaux",
    },
    Forme {
        nom: "triangle",
        detail: "3 côtés",
    },
    Forme {
        nom: "rectangle",
        detail: "4 côtés, 2 longs et 2 courts",
    },
    Forme {
        nom: "losange",
        detail: "4 côtés égaux, penché",
    },
    Forme {
        nom: "ovale",
        detail: "comme un œuf",
    },
    Forme {
        nom: "étoile",
        detail: "5 branches",
    },
    Forme {
        nom: "cœur",
        detail: "comme dans « je t'aime »",
    },
    Forme {
        nom: "hexagone",
        detail: "6 côtés",
    },
    Forme {
        nom: "octogone",
        detail: "8 côtés",
    },
];

pub const FORME_COTES: [(&str, i32); 6] = [
    ("triangle", 3),
    ("carré", 4),
    ("rectangle", 4),
    ("losange", 4),
    ("hexagone", 6),
    ("octogone", 8),
];

/// Les nombres de côtés qui ne désignent qu'une forme.
pub const COTES_UNIQUES: [(i32, &str); 3] = [(3, "triangle"), (6, "hexagone"), (8, "octogone")];

pub const OBJETS_FORME: [(&str, &str); 7] = [
    ("d'une pièce de monnaie", "cercle"),
    ("d'un ballon", "cercle"),
    ("d'un œuf", "ovale"),
    ("d'une part de pizza", "triangle"),
    ("d'un panneau stop", "octogone"),
    ("d'une porte", "rectangle"),
    ("des alvéoles des abeilles", "hexagone"),
];

pub const UNITES: [&str; 20] = [
    "zéro", "un", "deux", "trois", "quatre", "cinq", "six", "sept", "huit", "neuf", "dix", "onze",
    "douze", "treize", "quatorze", "quinze", "seize", "dix-sept", "dix-huit", "dix-neuf",
];

fn dizaine(n: u32) -> &'static str {
    match n {
        20 => "vingt",
        30 => "trente",
        40 => "quarante",
        50 => "cinquante",
        60 => "soixante",
        80 => "quatre-vingt",
        _ => "",
    }
}

/// Un nombre de 0 à 100 en toutes lettres (orthographe traditionnelle).
pub fn nombre_en_lettres(n: u32) -> String {
    if n < 20 {
        return UNITES[n as usize].to_string();
    }
    if n == 100 {
        return "cent".to_string();
    }
    let mut d = n / 10 * 10;
    let mut u = n - d;
    // soixante-dix, quatre-vingt-dix : la dizaine d'avant, et l'unité passe à 10+
    if d == 70 || d == 90 {
        d -= 10;
        u += 10;
    }
    if u == 0 {
        return if d == 80 {
            "quatre-vingts".into()
        } else {
            dizaine(d).into()
        };
    }
    if (u == 1 || u == 11) && d != 80 {
        return format!("{} et {}", dizaine(d), UNITES[u as usize]);
    }
    format!("{}-{}", dizaine(d), UNITES[u as usize])
}

/// « une heure », « trois heures et quart », « quatre heures moins le quart »…
pub fn heure_en_lettres(h: u32, m: u32) -> String {
    let nom = |n: u32| {
        if n == 1 {
            "une heure".to_string()
        } else {
            format!("{} heures", nombre_en_lettres(n))
        }
    };
    // la minute est féminine : « onze heures une », « deux heures trente et une »
    let minute = |n: u32| {
        let mot = nombre_en_lettres(n);
        if mot == "un" {
            "une".to_string()
        } else if mot.ends_with(" et un") {
            mot + "e"
        } else {
            mot
        }
    };
    let suivante = if h == 12 { 1 } else { h + 1 };
    match m {
        0 => nom(h),
        15 => format!("{} et quart", nom(h)),
        30 => format!("{} et demie", nom(h)),
        45 => format!("{} moins le quart", nom(suivante)),
        // passé la demie, on annonce l'heure suivante : 2 h 40 → « trois heures
        // moins vingt ». Réservé aux minutes rondes : on ne dit pas « moins vingt-neuf ».
        m if m > 30 && (60 - m) % 5 == 0 => format!("{} moins {}", nom(suivante), minute(60 - m)),
        m => format!("{} {}", nom(h), minute(m)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nombres_en_lettres() {
        let attendus = [
            (0, "zéro"),
            (16, "seize"),
            (21, "vingt et un"),
            (22, "vingt-deux"),
            (61, "soixante et un"),
            (70, "soixante-dix"),
            (71, "soixante et onze"),
            (72, "soixante-douze"),
            (80, "quatre-vingts"),
            (81, "quatre-vingt-un"),
            (90, "quatre-vingt-dix"),
            (91, "quatre-vingt-onze"),
            (99, "quatre-vingt-dix-neuf"),
            (100, "cent"),
        ];
        for (n, mots) in attendus {
            assert_eq!(nombre_en_lettres(n), mots, "{n}");
        }
    }

    #[test]
    fn heures_en_lettres() {
        assert_eq!(heure_en_lettres(1, 0), "une heure");
        assert_eq!(heure_en_lettres(3, 15), "trois heures et quart");
        assert_eq!(heure_en_lettres(5, 30), "cinq heures et demie");
        assert_eq!(heure_en_lettres(12, 45), "une heure moins le quart");
        assert_eq!(heure_en_lettres(2, 40), "trois heures moins vingt");
        assert_eq!(heure_en_lettres(11, 1), "onze heures une");
        assert_eq!(heure_en_lettres(2, 31), "deux heures trente et une");
        assert_eq!(heure_en_lettres(2, 39), "deux heures trente-neuf");
        assert_eq!(heure_en_lettres(4, 5), "quatre heures cinq");
    }
}
