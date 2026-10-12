//! Planifier — décomposer une activité en étapes, les ordonner, estimer le
//! temps qu'elles prennent. C'est une fonction exécutive, et elle se rééduque.
//!
//! Les activités sont celles de la vie quotidienne, et leurs étapes
//! s'enchaînent vraiment : chaque étape suppose la précédente, pour qu'il n'y
//! ait qu'un seul ordre correct. `duree` est une estimation en minutes,
//! volontairement ronde.

use super::alea;
use super::question::{Illustration, Question};

pub struct Routine {
    pub cle: &'static str,
    pub emoji: &'static str,
    pub libelle: &'static str,
    /// Durée estimée, en minutes.
    pub duree: u32,
    pub etapes: &'static [&'static str],
}

pub const ROUTINES: &[Routine] = &[
    Routine {
        cle: "medicament",
        emoji: "💊",
        libelle: "Prendre son médicament",
        duree: 2,
        etapes: &[
            "Regarder l'ordonnance",
            "Sortir la bonne boîte",
            "Prendre le comprimé avec un verre d'eau",
            "Cocher sur le carnet",
        ],
    },
    Routine {
        cle: "dents",
        emoji: "🦷",
        libelle: "Se brosser les dents",
        duree: 3,
        etapes: &[
            "Prendre la brosse à dents",
            "Mettre le dentifrice dessus",
            "Se brosser les dents",
            "Se rincer la bouche",
        ],
    },
    Routine {
        cle: "cafe",
        emoji: "☕",
        libelle: "Se faire un café",
        duree: 5,
        etapes: &[
            "Faire chauffer l'eau",
            "Mettre le café dans la tasse",
            "Verser l'eau chaude dans la tasse",
            "Remuer avec la cuillère",
        ],
    },
    Routine {
        cle: "sandwich",
        emoji: "🥪",
        libelle: "Préparer un sandwich",
        duree: 5,
        etapes: &[
            "Sortir le pain et le jambon",
            "Couper le pain en deux",
            "Étaler le beurre",
            "Poser le jambon dessus",
            "Refermer le sandwich",
        ],
    },
    Routine {
        cle: "lessive",
        emoji: "🧺",
        libelle: "Faire une lessive",
        duree: 10,
        etapes: &[
            "Trier le linge sale",
            "Mettre le linge dans la machine",
            "Ajouter la lessive",
            "Lancer la machine",
            "Étendre le linge",
        ],
    },
    Routine {
        cle: "habiller",
        emoji: "👕",
        libelle: "S'habiller",
        duree: 10,
        etapes: &[
            "Choisir ses vêtements",
            "Enlever son pyjama",
            "Mettre son pantalon et son haut",
            "Mettre ses chaussettes",
            "Mettre ses chaussures",
        ],
    },
    Routine {
        cle: "pates",
        emoji: "🍝",
        libelle: "Préparer des pâtes",
        duree: 15,
        etapes: &[
            "Remplir la casserole d'eau",
            "Faire bouillir l'eau",
            "Verser les pâtes dans l'eau",
            "Attendre la cuisson",
            "Égoutter les pâtes",
        ],
    },
    Routine {
        cle: "douche",
        emoji: "🚿",
        libelle: "Prendre une douche",
        duree: 15,
        etapes: &[
            "Préparer sa serviette",
            "Régler la température de l'eau",
            "Se laver",
            "Se sécher avec la serviette",
        ],
    },
    Routine {
        cle: "lettre",
        emoji: "✉️",
        libelle: "Envoyer une lettre",
        duree: 15,
        etapes: &[
            "Écrire la lettre",
            "Mettre la lettre dans l'enveloppe",
            "Coller le timbre",
            "Poster la lettre dans la boîte",
        ],
    },
    Routine {
        cle: "bus",
        emoji: "🚌",
        libelle: "Prendre le bus",
        duree: 20,
        etapes: &[
            "Regarder l'horaire du bus",
            "Aller à l'arrêt",
            "Monter dans le bus",
            "Valider son ticket",
            "Descendre au bon arrêt",
        ],
    },
    Routine {
        cle: "courses",
        emoji: "🛒",
        libelle: "Faire les courses",
        duree: 45,
        etapes: &[
            "Écrire la liste des courses",
            "Aller au magasin",
            "Remplir le panier",
            "Passer à la caisse",
            "Ranger les courses",
        ],
    },
];

pub fn routine(cle: &str) -> Option<&'static Routine> {
    ROUTINES.iter().find(|r| r.cle == cle)
}

/// « 9 h 00 », « 10 h 05 » : la notation d'un agenda, plus lisible ici que les
/// lettres.
pub fn format_heure(h: u32, m: u32) -> String {
    format!("{h} h {m:02}")
}

/// Ajoute des minutes (éventuellement négatives) à une heure, sur 24 h.
pub fn ajoute_minutes(h: u32, m: u32, minutes: i32) -> (u32, u32) {
    let total = ((h * 60 + m) as i32 + minutes).rem_euclid(1440) as u32;
    (total / 60, total % 60)
}

fn textes(liste: &[&str]) -> Vec<String> {
    liste.iter().map(|s| s.to_string()).collect()
}

/// Autres étapes de la même activité, comme leurres plausibles.
fn autres_etapes(r: &Routine, sauf: &[&str], nb: usize) -> Vec<String> {
    let restantes: Vec<&str> = r
        .etapes
        .iter()
        .copied()
        .filter(|e| !sauf.contains(e))
        .collect();
    textes(&alea::melange(&restantes)[..nb.min(restantes.len())])
}

/// Étapes puisées dans d'autres activités (pour l'intrus et les compléments).
fn etapes_ailleurs(r: &Routine, nb: usize) -> Vec<String> {
    let toutes: Vec<&str> = ROUTINES
        .iter()
        .filter(|o| o.cle != r.cle)
        .flat_map(|o| o.etapes.iter().copied())
        .collect();
    textes(&alea::melange(&toutes)[..nb.min(toutes.len())])
}

/// Complète une liste de leurres jusqu'à `nb`, sans doublon avec la réponse.
fn complete(mut choix: Vec<String>, reponse: &str, r: &Routine, nb: usize) -> Vec<String> {
    for e in etapes_ailleurs(r, 12) {
        if choix.len() >= nb {
            break;
        }
        if e != reponse && !choix.contains(&e) {
            choix.push(e);
        }
    }
    choix.truncate(nb);
    choix
}

/// 🌱 Les étapes : repérer le début, la fin, et ce qui suit.
fn etapes(r: &Routine) -> Question {
    let n = r.etapes.len();
    match alea::un_de(&["premiere", "derniere", "apres"]) {
        "premiere" => {
            let reponse = r.etapes[0];
            Question::new(
                format!("« {} » : par quoi commence-t-on ?", r.libelle),
                reponse,
                complete(autres_etapes(r, &[reponse], 3), reponse, r, 3),
            )
            .cle(format!("plan:premiere:{}", r.cle))
        }
        "derniere" => {
            let reponse = r.etapes[n - 1];
            Question::new(
                format!("« {} » : quelle est la dernière étape ?", r.libelle),
                reponse,
                complete(autres_etapes(r, &[reponse], 3), reponse, r, 3),
            )
            .cle(format!("plan:derniere:{}", r.cle))
        }
        _ => {
            let i = alea::indice(n - 1);
            let reponse = r.etapes[i + 1];
            Question::new(
                format!(
                    "« {} » : que fait-on juste après « {} » ?",
                    r.libelle, r.etapes[i]
                ),
                reponse,
                complete(autres_etapes(r, &[reponse, r.etapes[i]], 3), reponse, r, 3),
            )
            .cle(format!("plan:apres:{}:{i}", r.cle))
        }
    }
}

/// 🌿 L'ordre : l'étape qui manque, l'intrus, ce qui vient avant.
fn ordre(r: &Routine) -> Question {
    let n = r.etapes.len();
    match alea::un_de(&["manquante", "intrus", "avant"]) {
        "manquante" => {
            let i = alea::indice(n);
            let reponse = r.etapes[i];
            let restantes = r
                .etapes
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, e)| e.to_string())
                .collect();
            Question::new(
                format!("« {} » : quelle étape manque ?", r.libelle),
                reponse,
                complete(Vec::new(), reponse, r, 3),
            )
            .cle(format!("plan:manquante:{}:{i}", r.cle))
            // sans les étapes sous les yeux, la question serait insoluble
            .avec(Illustration::Etapes {
                etapes: restantes,
                trou: Some(i),
            })
        }
        "intrus" => {
            let intrus = etapes_ailleurs(r, 1).remove(0);
            let gardees: Vec<String> = textes(&alea::melange(r.etapes)[..3]);
            let mut montrees = gardees.clone();
            montrees.push(intrus.clone());
            alea::melanger(&mut montrees);
            Question::new(
                format!("« {} » : quelle étape n'en fait pas partie ?", r.libelle),
                intrus.clone(),
                gardees,
            )
            .cle(format!("plan:intrus:{}:{intrus}", r.cle))
            .avec(Illustration::Etapes {
                etapes: montrees,
                trou: None,
            })
        }
        _ => {
            let i = 1 + alea::indice(n - 1);
            let reponse = r.etapes[i - 1];
            Question::new(
                format!(
                    "« {} » : que fait-on juste avant « {} » ?",
                    r.libelle, r.etapes[i]
                ),
                reponse,
                complete(autres_etapes(r, &[reponse, r.etapes[i]], 3), reponse, r, 3),
            )
            .cle(format!("plan:avant:{}:{i}", r.cle))
        }
    }
}

/// 🌳 Le temps : estimer une durée, calculer une fin, tenir dans un créneau.
fn temps() -> Question {
    loop {
        let r = &ROUTINES[alea::indice(ROUTINES.len())];
        match alea::un_de(&["duree", "fin", "tient", "pluslongue"]) {
            "duree" => {
                let mut autres: Vec<u32> = ROUTINES
                    .iter()
                    .map(|o| o.duree)
                    .filter(|&d| d != r.duree)
                    .collect();
                autres.sort_unstable();
                autres.dedup();
                alea::melanger(&mut autres);
                return Question::new(
                    format!("Combien de temps prend « {} » à peu près ?", r.libelle),
                    format!("{} min", r.duree),
                    autres.iter().take(3).map(|d| format!("{d} min")).collect(),
                )
                .cle(format!("plan:duree:{}", r.cle));
            }
            "fin" => {
                let h = alea::entre(8, 17) as u32;
                let m = alea::un_de(&[0, 15, 30, 45]);
                let (fh, fm) = ajoute_minutes(h, m, r.duree as i32);
                let juste = format_heure(fh, fm);
                let mut leurres = Vec::new();
                for ecart in alea::melange(&[-30, -15, -10, 10, 15, 30, 60]) {
                    if leurres.len() == 3 {
                        break;
                    }
                    let (lh, lm) = ajoute_minutes(fh, fm, ecart);
                    let texte = format_heure(lh, lm);
                    if texte != juste && !leurres.contains(&texte) {
                        leurres.push(texte);
                    }
                }
                return Question::new(
                    format!(
                        "Tu commences « {} » à {}. Ça prend {} min. À quelle heure as-tu fini ?",
                        r.libelle,
                        format_heure(h, m),
                        r.duree
                    ),
                    juste,
                    leurres,
                )
                .cle(format!("plan:fin:{}:{h}:{m}", r.cle));
            }
            "tient" => {
                // une seule activité tient dans le créneau, les trois autres sont trop longues
                let creneau = alea::un_de(&[5, 10, 15, 20]);
                let tiennent: Vec<&Routine> =
                    ROUTINES.iter().filter(|o| o.duree <= creneau).collect();
                let trop: Vec<&Routine> = ROUTINES.iter().filter(|o| o.duree > creneau).collect();
                if tiennent.is_empty() || trop.len() < 3 {
                    continue;
                }
                let bonne = tiennent[alea::indice(tiennent.len())];
                return Question::new(
                    format!("Tu as {creneau} minutes devant toi. Qu'est-ce qui rentre ?"),
                    bonne.libelle,
                    alea::melange(&trop)
                        .iter()
                        .take(3)
                        .map(|o| o.libelle.to_string())
                        .collect(),
                )
                .cle(format!("plan:tient:{creneau}:{}", bonne.cle));
            }
            _ => {
                let quatre: Vec<&Routine> =
                    alea::melange(&ROUTINES.iter().collect::<Vec<_>>())[..4].to_vec();
                let max = quatre.iter().map(|o| o.duree).max().unwrap_or(0);
                // égalité en tête : la question n'aurait pas de réponse unique
                if quatre.iter().filter(|o| o.duree == max).count() > 1 {
                    continue;
                }
                let plus_longue = quatre.iter().find(|o| o.duree == max).expect("un maximum");
                let mut cles: Vec<&str> = quatre.iter().map(|o| o.cle).collect();
                cles.sort_unstable();
                return Question::new(
                    "Laquelle de ces activités prend le plus de temps ?",
                    plus_longue.libelle,
                    quatre
                        .iter()
                        .filter(|o| o.cle != plus_longue.cle)
                        .map(|o| o.libelle.to_string())
                        .collect(),
                )
                .cle(format!("plan:pluslongue:{}", cles.join("-")));
            }
        }
    }
}

pub fn question(niveau: u8) -> Question {
    let r = &ROUTINES[alea::indice(ROUTINES.len())];
    match niveau {
        1 => etapes(r),
        2 => ordre(r),
        _ => temps(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heures_d_agenda() {
        assert_eq!(format_heure(9, 5), "9 h 05");
        assert_eq!(ajoute_minutes(23, 50, 20), (0, 10));
        assert_eq!(ajoute_minutes(0, 10, -30), (23, 40));
    }

    #[test]
    fn chaque_question_a_quatre_reponses_distinctes() {
        alea::amorcer(3);
        for niveau in 1..=3 {
            for _ in 0..300 {
                let q = question(niveau);
                let mut options = q.options.clone();
                options.sort();
                options.dedup();
                assert_eq!(options.len(), 4, "{q:?}");
                assert!(q.options.contains(&q.reponse));
            }
        }
    }

    #[test]
    fn l_etape_manquante_est_montree_avec_son_trou() {
        alea::amorcer(11);
        let q = (0..500)
            .map(|_| question(2))
            .find(|q| q.cle.starts_with("plan:manquante"))
            .expect("une question « manquante »");
        let Some(Illustration::Etapes {
            etapes,
            trou: Some(i),
        }) = &q.illustration
        else {
            panic!("pas d'étapes")
        };
        assert!(!etapes.contains(&q.reponse));
        assert!(*i <= etapes.len());
    }
}
