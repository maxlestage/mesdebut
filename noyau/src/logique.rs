//! Les jeux de logique : continuer une suite, trouver l'intrus, déduire un
//! classement à partir d'indices.
//!
//! Trois niveaux, du plus visible au plus abstrait : la suite se voit, l'intrus
//! demande de chercher ce que les autres ont en commun, la déduction oblige à
//! tenir plusieurs indices en tête à la fois.

use super::alea;
use super::donnees::*;
use super::question::{Question, majuscule};

// ---------------------------------------------------------------------------
// Niveau 1 : les suites

/// Des symboles assez différents pour ne pas se confondre.
const SYMBOLES: [&str; 8] = ["🔴", "🔵", "🟡", "🟢", "⭐", "🌙", "❤️", "🔺"];

/// Un motif qui se répète (AB, ABC, AAB, ABB) : quel symbole vient ensuite ?
fn suite_de_symboles() -> Question {
    let motifs: [&[usize]; 4] = [&[0, 1], &[0, 1, 2], &[0, 0, 1], &[0, 1, 1]];
    let motif = alea::un_de(&motifs);
    let pioche = alea::melange(&SYMBOLES);
    let longueur = motif.len() * 2 + alea::entre(0, motif.len() as i32 - 1) as usize;
    let suite: Vec<&str> = (0..longueur)
        .map(|i| pioche[motif[i % motif.len()]])
        .collect();
    let reponse = pioche[motif[longueur % motif.len()]];
    // les autres symboles du motif d'abord : c'est entre eux qu'on hésite
    let leurres: Vec<String> = pioche
        .iter()
        .filter(|&&s| s != reponse)
        .take(3)
        .map(|s| s.to_string())
        .collect();
    Question::new(
        format!("Quel symbole vient ensuite ?\n{} …", suite.join(" ")),
        reponse,
        leurres,
    )
}

/// Une suite de nombres qui avance d'un pas régulier, ou qui double.
fn suite_de_nombres() -> Question {
    let (depart, suivant): (i32, Box<dyn Fn(i32) -> i32>) = match alea::entre(0, 3) {
        0 => {
            let pas = alea::entre(2, 5);
            (alea::entre(1, 10), Box::new(move |n| n + pas))
        }
        1 => {
            let pas = alea::entre(2, 5);
            (alea::entre(25, 40), Box::new(move |n| n - pas))
        }
        2 => {
            let pas = alea::un_de(&[10, 5]);
            (pas * alea::entre(0, 3), Box::new(move |n| n + pas))
        }
        _ => (alea::entre(1, 3), Box::new(|n| n * 2)),
    };
    let mut termes = vec![depart];
    for _ in 0..4 {
        termes.push(suivant(*termes.last().unwrap()));
    }
    let reponse = suivant(*termes.last().unwrap());
    let montres: Vec<String> = termes.iter().map(i32::to_string).collect();
    Question::new(
        format!("Quel nombre vient ensuite ?\n{} …", montres.join(", ")),
        reponse.to_string(),
        proches(reponse),
    )
}

/// Trois leurres positifs, autour de la réponse.
fn proches(reponse: i32) -> Vec<String> {
    let mut leurres = Vec::new();
    for c in alea::melange(&[
        reponse - 1,
        reponse + 1,
        reponse - 2,
        reponse + 2,
        reponse + 10,
    ]) {
        if c >= 0 && c != reponse && leurres.len() < 3 {
            leurres.push(c);
        }
    }
    leurres.iter().map(i32::to_string).collect()
}

/// Les jours ou les mois, un sur deux : lequel vient ensuite ?
fn suite_un_sur_deux() -> Question {
    let liste: &[&str] = if alea::pile_ou_face() { &JOURS } else { &MOIS };
    let depart = alea::indice(liste.len());
    let a = |k: usize| liste[(depart + 2 * k) % liste.len()];
    let reponse = a(3);
    // juste avant la réponse, juste après, et le dernier montré
    let leurres = vec![
        majuscule(liste[(depart + 5) % liste.len()]),
        majuscule(liste[(depart + 7) % liste.len()]),
        majuscule(a(2)),
    ];
    Question::new(
        format!(
            "Un sur deux : quel est le suivant ?\n{}, {}, {} …",
            majuscule(a(0)),
            a(1),
            a(2)
        ),
        majuscule(reponse),
        leurres,
    )
}

fn suites() -> Question {
    match alea::entre(0, 2) {
        0 => suite_de_symboles(),
        1 => suite_de_nombres(),
        _ => suite_un_sur_deux(),
    }
}

// ---------------------------------------------------------------------------
// Niveau 2 : l'intrus

const FRUITS: [&str; 8] = [
    "pomme", "banane", "poire", "fraise", "cerise", "raisin", "ananas", "kiwi",
];
const ANIMAUX: [&str; 8] = [
    "chien", "chat", "cheval", "lapin", "vache", "mouton", "canard", "cochon",
];
const VEHICULES: [&str; 7] = [
    "voiture", "vélo", "bus", "train", "avion", "bateau", "camion",
];
const VETEMENTS: [&str; 7] = [
    "pull",
    "pantalon",
    "chemise",
    "manteau",
    "jupe",
    "écharpe",
    "chaussette",
];

/// Une famille de mots : trois de la même, et l'intrus d'une autre.
struct Famille {
    nom: &'static str,
    mots: Vec<&'static str>,
}

fn familles() -> Vec<Famille> {
    vec![
        Famille {
            nom: "fruits",
            mots: FRUITS.to_vec(),
        },
        Famille {
            nom: "animaux",
            mots: ANIMAUX.to_vec(),
        },
        Famille {
            nom: "véhicules",
            mots: VEHICULES.to_vec(),
        },
        Famille {
            nom: "vêtements",
            mots: VETEMENTS.to_vec(),
        },
        Famille {
            nom: "jours",
            mots: JOURS.to_vec(),
        },
        Famille {
            nom: "mois",
            mots: MOIS.to_vec(),
        },
        Famille {
            nom: "couleurs",
            mots: COULEURS.iter().map(|c| c.nom).collect(),
        },
        Famille {
            nom: "formes",
            mots: FORMES.iter().map(|f| f.nom).collect(),
        },
    ]
}

/// Quatre mots, dont un n'est pas de la famille des trois autres.
fn intrus_de_mots() -> Question {
    let toutes = familles();
    let ordre = alea::melange(&(0..toutes.len()).collect::<Vec<_>>());
    let (groupe, autre) = (&toutes[ordre[0]], &toutes[ordre[1]]);
    let mut trois = alea::melange(&groupe.mots);
    trois.truncate(3);
    let intrus = alea::un_de(&autre.mots);
    Question::new(
        "Trouve l'intrus : lequel n'est pas de la même famille ?",
        majuscule(intrus),
        trois.iter().map(|m| majuscule(m)).collect(),
    )
    .cle(format!("intrus:{}:{}:{intrus}", groupe.nom, autre.nom))
}

/// Quatre nombres : trois pairs et un impair, ou l'inverse ; ou trois dizaines
/// rondes et un nombre qui ne l'est pas.
fn intrus_de_nombres() -> Question {
    let (regle, intrus, autres): (&str, i32, Vec<i32>) = match alea::entre(0, 2) {
        0 => {
            let pairs: Vec<i32> = alea::melange(&(1..=20).map(|n| n * 2).collect::<Vec<_>>());
            ("pairs", alea::entre(0, 19) * 2 + 1, pairs[..3].to_vec())
        }
        1 => {
            let impairs: Vec<i32> = alea::melange(&(0..20).map(|n| n * 2 + 1).collect::<Vec<_>>());
            ("impairs", alea::entre(1, 20) * 2, impairs[..3].to_vec())
        }
        _ => {
            let ronds: Vec<i32> = alea::melange(&(1..=9).map(|n| n * 10).collect::<Vec<_>>());
            let mut intrus = alea::entre(11, 99);
            if intrus % 10 == 0 {
                intrus += 5;
            }
            ("dizaines", intrus, ronds[..3].to_vec())
        }
    };
    Question::new(
        "Trouve l'intrus parmi ces nombres",
        intrus.to_string(),
        autres.iter().map(i32::to_string).collect(),
    )
    .cle(format!("intrus:{regle}:{intrus}"))
}

fn intrus() -> Question {
    if alea::entre(0, 3) == 0 {
        intrus_de_nombres()
    } else {
        intrus_de_mots()
    }
}

// ---------------------------------------------------------------------------
// Niveau 3 : les déductions

const PRENOMS: [&str; 10] = [
    "Léa", "Tom", "Sam", "Inès", "Hugo", "Zoé", "Paul", "Nina", "Jules", "Emma",
];

/// Un critère de classement. Les phrases ne s'accordent pas : elles conviennent
/// à tous les prénoms.
struct Critere {
    avant: &'static str,
    apres: &'static str,
    premier: &'static str,
    dernier: &'static str,
    deuxieme: &'static str,
}

const CRITERES: [Critere; 3] = [
    Critere {
        avant: "arrive avant",
        apres: "arrive après",
        premier: "Dans la course, qui arrive en premier ?",
        dernier: "Dans la course, qui arrive en dernier ?",
        deuxieme: "Dans la course, qui arrive en deuxième ?",
    },
    Critere {
        avant: "se lève avant",
        apres: "se lève après",
        premier: "Le matin, qui se lève en premier ?",
        dernier: "Le matin, qui se lève en dernier ?",
        deuxieme: "Le matin, qui se lève en deuxième ?",
    },
    Critere {
        avant: "a plus de billes que",
        apres: "a moins de billes que",
        premier: "Qui a le plus de billes ?",
        dernier: "Qui a le moins de billes ?",
        deuxieme: "Qui a le plus de billes, juste après le premier ?",
    },
];

/// « a plus de billes que » + « Inès » → « a plus de billes qu'Inès ».
fn relier(phrase: &str, prenom: &str) -> String {
    let voyelle = prenom.starts_with(['A', 'E', 'I', 'O', 'U', 'É']);
    match phrase.strip_suffix(" que") {
        Some(debut) if voyelle => format!("{debut} qu'{prenom}"),
        _ => format!("{phrase} {prenom}"),
    }
}

/// Quatre personnes rangées de la première à la dernière ; trois indices, chacun
/// sur deux voisines, dans le désordre. Il faut reconstruire le classement.
fn classement() -> Question {
    let mut prenoms = alea::melange(&PRENOMS);
    prenoms.truncate(4);
    let critere = &CRITERES[alea::indice(CRITERES.len())];
    let mut indices: Vec<String> = (0..3)
        .map(|i| {
            if alea::pile_ou_face() {
                format!("{} {}.", prenoms[i], relier(critere.avant, prenoms[i + 1]))
            } else {
                format!("{} {}.", prenoms[i + 1], relier(critere.apres, prenoms[i]))
            }
        })
        .collect();
    alea::melanger(&mut indices);
    let (demande, rang) = alea::un_de(&[
        (critere.premier, 0),
        (critere.dernier, 3),
        (critere.deuxieme, 1),
    ]);
    let reponse = prenoms[rang];
    Question::new(
        format!("{}\n{demande}", indices.join("\n")),
        reponse,
        prenoms
            .iter()
            .filter(|&&p| p != reponse)
            .map(|p| p.to_string())
            .collect(),
    )
}

/// Des objets dans des boîtes, avec deux indices : où est le dernier ?
fn boites() -> Question {
    let couleurs = alea::melange(&["rouge", "bleue", "verte", "jaune"]);
    let objets = alea::melange(&["la clé", "la balle", "le livre"]);
    // les objets sont dans les trois premières boîtes ; la quatrième est vide
    let mut indices = vec![
        format!(
            "{} est dans la boîte {}.",
            majuscule(objets[0]),
            couleurs[0]
        ),
        format!(
            "{} n'est ni dans la boîte {}, ni dans la boîte {}.",
            majuscule(objets[1]),
            couleurs[2],
            couleurs[3]
        ),
        format!("La boîte {} est vide.", couleurs[3]),
    ];
    alea::melanger(&mut indices);
    Question::new(
        format!(
            "Quatre boîtes : {}. Trois objets, un par boîte.\n{}\nOù est {} ?",
            couleurs.join(", "),
            indices.join("\n"),
            objets[2]
        ),
        format!("Boîte {}", couleurs[2]),
        [0, 1, 3]
            .iter()
            .map(|&i| format!("Boîte {}", couleurs[i]))
            .collect(),
    )
}

fn deductions() -> Question {
    if alea::entre(0, 2) == 0 {
        boites()
    } else {
        classement()
    }
}

/// Une question du niveau demandé (1 : suites, 2 : intrus, 3 : déductions).
pub fn question(niveau: u8) -> Question {
    match niveau {
        2 => intrus(),
        3 => deductions(),
        _ => suites(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn les_familles_ne_se_chevauchent_pas() {
        let familles = familles();
        for (i, a) in familles.iter().enumerate() {
            for b in &familles[i + 1..] {
                for mot in &a.mots {
                    assert!(!b.mots.contains(mot), "{mot} : {} et {}", a.nom, b.nom);
                }
            }
        }
    }

    #[test]
    fn que_s_elide_devant_une_voyelle() {
        assert_eq!(
            relier("a moins de billes que", "Inès"),
            "a moins de billes qu'Inès"
        );
        assert_eq!(
            relier("a moins de billes que", "Tom"),
            "a moins de billes que Tom"
        );
        assert_eq!(relier("arrive avant", "Emma"), "arrive avant Emma");
    }

    #[test]
    fn la_suite_de_nombres_est_reguliere() {
        alea::amorcer(3);
        for _ in 0..300 {
            let q = suite_de_nombres();
            let ligne = q.enonce.lines().nth(1).unwrap().trim_end_matches(" …");
            let n: Vec<i32> = ligne.split(", ").map(|t| t.parse().unwrap()).collect();
            let r: i32 = q.reponse.parse().unwrap();
            let pas = n[1] - n[0];
            if n[1] == n[0] * 2 && n[2] == n[1] * 2 {
                assert_eq!(r, n[4] * 2, "{q:?}");
            } else {
                assert!(n.windows(2).all(|w| w[1] - w[0] == pas), "{q:?}");
                assert_eq!(r, n[4] + pas, "{q:?}");
            }
            assert!(r >= 0);
        }
    }

    #[test]
    fn le_classement_se_deduit_des_indices() {
        alea::amorcer(11);
        for _ in 0..300 {
            let q = classement();
            let lignes: Vec<&str> = q.enonce.lines().collect();
            let critere = CRITERES
                .iter()
                .find(|c| [c.premier, c.dernier, c.deuxieme].contains(&lignes[3]))
                .unwrap();
            // reconstruit l'ordre : « a arrive avant b » met a devant b
            let mut devant: Vec<(&str, &str)> = Vec::new();
            let lignes_sans_elision: Vec<String> = lignes[..3]
                .iter()
                .map(|l| l.replace(" qu'", " que "))
                .collect();
            for l in &lignes_sans_elision {
                let l = l.trim_end_matches('.');
                if let Some((a, b)) = l.split_once(&format!(" {} ", critere.avant)) {
                    devant.push((a, b));
                } else {
                    let (a, b) = l.split_once(&format!(" {} ", critere.apres)).unwrap();
                    devant.push((b, a));
                }
            }
            let premier = devant
                .iter()
                .map(|&(a, _)| a)
                .find(|a| devant.iter().all(|(_, b)| b != a))
                .unwrap();
            let mut ordre = vec![premier];
            while ordre.len() < 4 {
                let dernier = *ordre.last().unwrap();
                ordre.push(devant.iter().find(|(a, _)| *a == dernier).unwrap().1);
            }
            let rang = if lignes[3] == critere.premier {
                0
            } else if lignes[3] == critere.dernier {
                3
            } else {
                1
            };
            assert_eq!(q.reponse, ordre[rang], "{}", q.enonce);
        }
    }

    #[test]
    fn la_boite_se_deduit_des_indices() {
        alea::amorcer(7);
        for _ in 0..100 {
            let q = boites();
            let couleurs: Vec<&str> = q.enonce.lines().next().unwrap()["Quatre boîtes : ".len()..]
                .split_once('.')
                .unwrap()
                .0
                .split(", ")
                .collect();
            assert_eq!(q.reponse, format!("Boîte {}", couleurs[2]), "{}", q.enonce);
        }
    }
}
