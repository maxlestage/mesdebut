//! Les casse-tête : retrouver le nombre caché d'un calcul, peser des fruits
//! sur une balance, deviner un objet à partir de ce qu'il dit de lui.
//!
//! Trois niveaux, du plus concret au plus imagé : un seul calcul à défaire,
//! puis deux égalités à enchaîner, puis des mots à prendre au sens figuré.

use super::alea;
use super::question::Question;

/// Trois leurres positifs, autour de la réponse.
fn proches(reponse: i32) -> Vec<String> {
    let mut leurres = Vec::new();
    for c in alea::melange(&[
        reponse - 1,
        reponse + 1,
        reponse - 2,
        reponse + 2,
        reponse + 3,
    ]) {
        if c >= 0 && c != reponse && leurres.len() < 3 {
            leurres.push(c);
        }
    }
    leurres.iter().map(i32::to_string).collect()
}

// ---------------------------------------------------------------------------
// Niveau 1 : les nombres cachés

/// Un calcul dont un nombre est remplacé par « ? ».
fn nombre_cache() -> Question {
    // (a, symbole, b, résultat) : a symbole b = résultat
    let (a, symbole, b, c) = match alea::entre(0, 2) {
        0 => {
            let (a, b) = (alea::entre(1, 10), alea::entre(1, 10));
            (a, "+", b, a + b)
        }
        1 => {
            let (b, c) = (alea::entre(1, 10), alea::entre(1, 10));
            (b + c, "−", b, c)
        }
        _ => {
            let (a, b) = (alea::entre(2, 5), alea::entre(1, 10));
            (a, "×", b, a * b)
        }
    };
    let (enonce, reponse) = if alea::pile_ou_face() {
        (format!("? {symbole} {b} = {c}"), a)
    } else {
        (format!("{a} {symbole} ? = {c}"), b)
    };
    Question::new(
        format!("Quel nombre se cache derrière « ? » ?\n{enonce}"),
        reponse.to_string(),
        proches(reponse),
    )
}

// ---------------------------------------------------------------------------
// Niveau 2 : les balances

const FRUITS: [&str; 6] = ["🍎", "🍌", "🍇", "🍓", "🍐", "🍒"];

/// Chaque fruit vaut un nombre ; une ou deux égalités pour le retrouver.
fn balance() -> Question {
    let fruits = alea::melange(&FRUITS);
    let (x, y) = (fruits[0], fruits[1]);
    let a = alea::entre(1, 9);
    let b = alea::entre(1, 9);
    let (egalites, cherche, reponse) = match alea::entre(0, 3) {
        0 => (format!("{x} + {x} = {}", 2 * a), x, a),
        1 => (format!("{x} + {x} + {x} = {}", 3 * a), x, a),
        2 => (format!("{x} = {a}\n{x} + {y} = {}", a + b), y, b),
        _ => (format!("{x} + {x} = {}\n{y} = {x} + {b}", 2 * a), y, a + b),
    };
    Question::new(
        format!("Chaque fruit cache un nombre.\n{egalites}\nCombien vaut {cherche} ?"),
        reponse.to_string(),
        proches(reponse),
    )
}

// ---------------------------------------------------------------------------
// Niveau 3 : les devinettes

/// (devinette, réponse, leurres) : chaque leurre manque au moins un indice.
const DEVINETTES: [(&str, &str, [&str; 3]); 18] = [
    (
        "J'ai des aiguilles et un cadran, et je donne l'heure.",
        "Une horloge",
        ["Un sapin", "Un calendrier", "Un thermomètre"],
    ),
    (
        "Je suis plein de trous, mais je retiens l'eau.",
        "Une éponge",
        ["Un seau", "Une passoire", "Un verre"],
    ),
    (
        "Je monte et je descends sans jamais bouger.",
        "Un escalier",
        ["Un ascenseur", "Un ballon", "Un oiseau"],
    ),
    (
        "Plus je sèche, plus je suis mouillée.",
        "Une serviette",
        ["Un parapluie", "Une flaque", "Un nuage"],
    ),
    (
        "J'ai des dents, mais je ne mange jamais.",
        "Un peigne",
        ["Un chien", "Un bébé", "Une cuillère"],
    ),
    (
        "J'ai un cou, mais pas de tête.",
        "Une bouteille",
        ["Une girafe", "Un pull", "Un chapeau"],
    ),
    (
        "Je tombe du ciel sans jamais me faire mal.",
        "La pluie",
        ["Un vase", "Un avion", "Une assiette"],
    ),
    (
        "J'ai des roues et une poignée, et je pars en voyage avec toi.",
        "Une valise",
        ["Un vélo", "Une voiture", "Un sac à dos"],
    ),
    (
        "J'ai des pages et une couverture, mais je n'ai jamais froid.",
        "Un livre",
        ["Un lit", "Un crayon", "Une lampe"],
    ),
    (
        "Je brille dans le ciel le jour et je réchauffe la Terre.",
        "Le soleil",
        ["La lune", "Un nuage", "Une étoile filante"],
    ),
    (
        "Plus on m'enlève de terre, plus je deviens grand.",
        "Un trou",
        ["Un sac", "Un arbre", "Un ballon"],
    ),
    (
        "J'ai quatre pieds, mais je ne marche jamais.",
        "Une table",
        ["Un chat", "Un cheval", "Une chaussure"],
    ),
    (
        "Je suis blanche et froide, et je fonds au soleil.",
        "La neige",
        ["Le lait", "Le sucre", "Le sel"],
    ),
    (
        "On me casse avant de me cuire.",
        "Un œuf",
        ["Une pomme", "Une banane", "Une soupe"],
    ),
    (
        "Je vole sans ailes et je pleure sans yeux.",
        "Un nuage",
        ["Un avion", "Un oiseau", "Un papillon"],
    ),
    (
        "J'ai un lit, mais je ne dors jamais, et je cours sans jambes.",
        "Une rivière",
        ["Un chat", "Un hôpital", "Un train"],
    ),
    (
        "J'ai des touches noires et blanches, et on joue de la musique avec moi.",
        "Un piano",
        ["Une clé", "Un ballon", "Un tambour"],
    ),
    (
        "Je porte ma maison sur mon dos.",
        "L'escargot",
        ["Le chien", "Le lapin", "L'oiseau"],
    ),
];

fn devinette() -> Question {
    let (texte, reponse, leurres) = DEVINETTES[alea::indice(DEVINETTES.len())];
    Question::new(
        format!("{texte}\nQui suis-je ?"),
        reponse,
        leurres.iter().map(|l| l.to_string()).collect(),
    )
}

/// Une question du niveau demandé (1 : nombres cachés, 2 : balances,
/// 3 : devinettes).
pub fn question(niveau: u8) -> Question {
    match niveau {
        2 => balance(),
        3 => devinette(),
        _ => nombre_cache(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Remplace « ? » par la réponse et vérifie le calcul.
    #[test]
    fn le_nombre_cache_rend_le_calcul_juste() {
        alea::amorcer(5);
        for _ in 0..500 {
            let q = nombre_cache();
            let calcul = q.enonce.lines().nth(1).unwrap().replace('?', &q.reponse);
            let mots: Vec<&str> = calcul.split(' ').collect();
            let (a, b, c): (i32, i32, i32) = (
                mots[0].parse().unwrap(),
                mots[2].parse().unwrap(),
                mots[4].parse().unwrap(),
            );
            let attendu = match mots[1] {
                "+" => a + b,
                "−" => a - b,
                _ => a * b,
            };
            assert_eq!(attendu, c, "{}", q.enonce);
        }
    }

    /// Résout les égalités en remplaçant les fruits connus par leur valeur.
    #[test]
    fn la_balance_se_resout() {
        alea::amorcer(8);
        for _ in 0..500 {
            let q = balance();
            let lignes: Vec<&str> = q.enonce.lines().collect();
            let egalites = &lignes[1..lignes.len() - 1];
            let cherche = lignes.last().unwrap()["Combien vaut ".len()..]
                .trim_end_matches(" ?")
                .to_string();
            let mut valeurs: Vec<(String, i32)> = Vec::new();
            // deux passes suffisent : chaque égalité n'a qu'une inconnue
            for _ in 0..2 {
                for e in egalites {
                    let (gauche, droite) = e.split_once(" = ").unwrap();
                    let somme = |cote: &str| -> (i32, Option<(String, i32)>) {
                        let mut total = 0;
                        let mut inconnu: Option<(String, i32)> = None;
                        for t in cote.split(" + ") {
                            if let Ok(n) = t.parse::<i32>() {
                                total += n;
                            } else if let Some((_, v)) = valeurs.iter().find(|(f, _)| f == t) {
                                total += v;
                            } else {
                                let compte = inconnu.as_ref().map_or(0, |(_, k)| *k);
                                inconnu = Some((t.to_string(), compte + 1));
                            }
                        }
                        (total, inconnu)
                    };
                    let ((g, ig), (d, id)) = (somme(gauche), somme(droite));
                    match (ig, id) {
                        (Some((f, k)), None) if !valeurs.iter().any(|(v, _)| *v == f) => {
                            valeurs.push((f, (d - g) / k))
                        }
                        (None, Some((f, k))) if !valeurs.iter().any(|(v, _)| *v == f) => {
                            valeurs.push((f, (g - d) / k))
                        }
                        _ => {}
                    }
                }
            }
            let trouve = valeurs.iter().find(|(f, _)| *f == cherche).unwrap().1;
            assert_eq!(trouve.to_string(), q.reponse, "{}", q.enonce);
        }
    }

    #[test]
    fn chaque_devinette_a_quatre_reponses_distinctes() {
        for (texte, reponse, leurres) in DEVINETTES {
            let mut toutes = vec![reponse];
            toutes.extend(leurres);
            toutes.sort();
            toutes.dedup();
            assert_eq!(toutes.len(), 4, "{texte}");
        }
    }
}
