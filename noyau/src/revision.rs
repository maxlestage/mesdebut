//! Ce que montrent les écrans « Réviser d'abord ».

use super::donnees::*;
use super::planifier::ROUTINES;
use super::question::{Illustration, majuscule};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Fiche {
    pub libelle: String,
    pub detail: Option<String>,
    /// Le numéro montré en tête de ligne, à défaut d'une image.
    pub numero: Option<u32>,
    pub image: Option<Illustration>,
}

impl Fiche {
    fn new(libelle: impl Into<String>) -> Self {
        Fiche {
            libelle: libelle.into(),
            ..Fiche::default()
        }
    }
    fn detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }
    fn numero(mut self, n: u32) -> Self {
        self.numero = Some(n);
        self
    }
    fn image(mut self, image: Illustration) -> Self {
        self.image = Some(image);
        self
    }
}

pub struct Groupe {
    pub libelle: String,
    pub fiches: Vec<Fiche>,
}

pub enum Revision {
    Liste {
        titre: &'static str,
        intro: Option<&'static str>,
        fiches: Vec<Fiche>,
    },
    Lettres {
        titre: &'static str,
        lettres: &'static [&'static str],
    },
    /// Des groupes qui se déroulent (les nombres, les activités).
    Accordeon {
        titre: &'static str,
        intro: Option<&'static str>,
        groupes: Vec<Groupe>,
    },
}

/// Une horloge à réviser : le libellé et le repère de la grande aiguille sont
/// déduits de l'horaire, donc toujours cohérents entre eux.
fn horloge(h: u32, m: u32) -> Fiche {
    Fiche::new(majuscule(&heure_en_lettres(h, m)))
        .detail(format!(
            "la grande aiguille est sur le {}",
            if m == 0 { 12 } else { m / 5 }
        ))
        .image(Illustration::Horloge {
            heures: h,
            minutes: m,
        })
}

fn billes(nombre: u32, par_rang: u32, couleur_par_rang: bool) -> Illustration {
    Illustration::Billes {
        nombre,
        par_rang,
        couleur_par_rang,
    }
}

pub fn revision(categorie: &str) -> Option<Revision> {
    let liste = |titre, intro, fiches| {
        Some(Revision::Liste {
            titre,
            intro,
            fiches,
        })
    };
    match categorie {
        "jours" => liste(
            "📖 Les 7 jours de la semaine",
            None,
            JOURS.iter().map(|j| Fiche::new(majuscule(j))).collect(),
        ),
        "mois" => liste(
            "📖 Les 12 mois de l'année",
            None,
            MOIS.iter().map(|m| Fiche::new(majuscule(m))).collect(),
        ),
        "saisons" => liste(
            "📖 Les 4 saisons",
            None,
            vec![
                Fiche::new("🌸 Printemps").detail("mars, avril, mai"),
                Fiche::new("☀️ Été").detail("juin, juillet, août"),
                Fiche::new("🍂 Automne").detail("septembre, octobre, novembre"),
                Fiche::new("❄️ Hiver").detail("décembre, janvier, février"),
            ],
        ),
        "alphabet" => Some(Revision::Lettres {
            titre: "📖 Les 26 lettres de l'alphabet",
            lettres: &ALPHABET,
        }),
        "heure" => liste(
            "📖 Lire l'heure",
            Some(
                "La petite aiguille donne les heures, la grande donne les minutes. \
                 Entre deux chiffres du cadran, il y a 5 minutes.",
            ),
            // des heures variées, du plus simple au plus fin
            [
                (1, 0),
                (6, 0),
                (10, 0),
                (2, 15),
                (5, 30),
                (8, 45),
                (4, 5),
                (9, 20),
                (11, 40),
                (12, 50),
            ]
            .iter()
            .map(|&(h, m)| horloge(h, m))
            .collect(),
        ),
        "couleurs" => liste(
            "📖 Les 11 couleurs",
            None,
            COULEURS
                .iter()
                .map(|c| Fiche::new(majuscule(c.nom)).image(Illustration::Pastille(c.hex)))
                .collect(),
        ),
        "formes" => liste(
            "📖 Les 10 formes géométriques",
            None,
            FORMES
                .iter()
                .map(|f| {
                    Fiche::new(majuscule(f.nom))
                        .detail(f.detail)
                        .image(Illustration::Forme(f.nom))
                })
                .collect(),
        ),
        "chiffres" => liste(
            "📖 Les chiffres de 0 à 9",
            None,
            (0..=9u32)
                .map(|n| {
                    Fiche::new(majuscule(UNITES[n as usize]))
                        .numero(n)
                        .image(billes(n, 5, false))
                })
                .collect(),
        ),
        "cinquante" => liste(
            "📖 Les dizaines jusqu'à 50",
            Some("Une rangée de 10 billes = 1 dizaine"),
            [10, 20, 30, 40, 50]
                .iter()
                .map(|&n| {
                    Fiche::new(majuscule(&nombre_en_lettres(n)))
                        .detail(format!(
                            "{} dizaine{}",
                            n / 10,
                            if n > 10 { "s" } else { "" }
                        ))
                        .numero(n)
                        .image(billes(n, 10, true))
                })
                .collect(),
        ),
        "planifier" => Some(Revision::Accordeon {
            titre: "📖 Les étapes des activités",
            intro: Some("Appuie sur une activité pour dérouler ses étapes, dans l'ordre"),
            groupes: ROUTINES
                .iter()
                .map(|r| Groupe {
                    libelle: format!("{} {} — {} min", r.emoji, r.libelle, r.duree),
                    fiches: r
                        .etapes
                        .iter()
                        .zip(1..)
                        .map(|(e, i)| Fiche::new(*e).numero(i))
                        .collect(),
                })
                .collect(),
        }),
        "nombres" => {
            let mut groupes: Vec<Groupe> = (0..10u32)
                .map(|d| Groupe {
                    libelle: format!("De {} à {}", d * 10, d * 10 + 9),
                    fiches: (d * 10..=d * 10 + 9)
                        .map(|n| Fiche::new(majuscule(&nombre_en_lettres(n))).numero(n))
                        .collect(),
                })
                .collect();
            groupes.push(Groupe {
                libelle: "Cent".into(),
                fiches: vec![Fiche::new("Cent").numero(100)],
            });
            Some(Revision::Accordeon {
                titre: "📖 Les nombres en lettres",
                intro: Some("Appuie sur une dizaine pour dérouler tous les nombres"),
                groupes,
            })
        }
        "logique" => liste(
            "📖 Les trois jeux de logique",
            Some(
                "Prends ton temps : chaque réponse se trouve en réfléchissant, \
                 sans rien avoir appris par cœur.",
            ),
            vec![
                Fiche::new("🌱 Les suites")
                    .detail("Trouve la règle, puis continue : 2, 4, 6, 8… → 10"),
                Fiche::new("🔴 🔵 🔴 🔵 …").detail("Les symboles se répètent : après 🔵 vient 🔴"),
                Fiche::new("🌿 L'intrus")
                    .detail("Trois mots vont ensemble, un seul n'est pas de la famille"),
                Fiche::new("Pomme, poire, cerise, vélo")
                    .detail("Trois fruits et un véhicule : l'intrus est vélo"),
                Fiche::new("🌳 Les déductions")
                    .detail("Assemble les indices un par un pour tout remettre en ordre"),
                Fiche::new("Léa arrive avant Tom. Sam arrive après Tom.")
                    .detail("L'ordre est Léa, Tom, Sam : Léa arrive en premier"),
            ],
        ),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::categories::CATEGORIES;

    #[test]
    fn chaque_theme_a_reviser_a_sa_revision() {
        for c in CATEGORIES {
            assert_eq!(revision(c.cle).is_some(), c.revision, "{}", c.cle);
        }
    }

    #[test]
    fn le_repere_de_la_grande_aiguille() {
        let f = horloge(8, 45);
        assert_eq!(f.libelle, "Neuf heures moins le quart");
        assert_eq!(f.detail.as_deref(), Some("la grande aiguille est sur le 9"));
    }
}
