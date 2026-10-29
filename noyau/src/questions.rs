//! La fabrique de questions, thème par thème, et la composition d'un quiz.
//!
//! Port du moteur de l'application iOS (`QuestionEngine.swift`) : mêmes
//! questions, mêmes leurres, mêmes règles de sélection.

use std::collections::{HashMap, HashSet};

use super::alea;
use super::donnees::*;
use super::logique;
use super::planifier;
use super::question::{Illustration, Question, majuscule};

/// Nombre de questions d'un quiz (avant les questions ratées qui reviennent).
pub const NB_QUESTIONS: usize = 10;

fn caps(liste: &[&str]) -> Vec<String> {
    liste.iter().map(|s| majuscule(s)).collect()
}

/// Jusqu'à `nb` éléments de `liste`, sauf celui d'indice `sauf`, au hasard.
fn sauf_un(liste: &[&str], sauf: usize, nb: usize) -> Vec<String> {
    let mut autres: Vec<&str> = liste.to_vec();
    autres.remove(sauf);
    alea::melanger(&mut autres);
    autres.truncate(nb);
    caps(&autres)
}

fn chiffres_autour(reponse: i32, bas: i32, haut: i32) -> Vec<String> {
    let mut autres: Vec<i32> = (bas..=haut).filter(|&i| i != reponse).collect();
    alea::melanger(&mut autres);
    autres.iter().take(3).map(i32::to_string).collect()
}

fn chiffres_en_lettres_sauf(n: usize) -> Vec<String> {
    let mut autres: Vec<String> = (0..=9)
        .filter(|&i| i != n)
        .map(|i| majuscule(UNITES[i]))
        .collect();
    alea::melanger(&mut autres);
    autres.truncate(3);
    autres
}

/// Leurres d'un résultat de calcul : proches de la réponse, positifs.
fn resultats_proches(reponse: i32, nb: usize) -> Vec<String> {
    let mut vus: Vec<i32> = Vec::new();
    let mut essais = 0;
    while vus.len() < nb && essais < 200 {
        essais += 1;
        let ecart = ((reponse.abs() as f64 * 0.3).round() as i32).max(3);
        let delta = alea::entre(1, ecart);
        let candidat = if alea::pile_ou_face() {
            reponse - delta
        } else {
            reponse + delta
        };
        if candidat >= 0 && candidat != reponse && !vus.contains(&candidat) {
            vus.push(candidat);
        }
    }
    let mut extra = reponse + nb as i32 + 1;
    while vus.len() < nb {
        if !vus.contains(&extra) {
            vus.push(extra);
        }
        extra += 1;
    }
    vus.iter().map(i32::to_string).collect()
}

/// Voisins d'un nombre, ceux qu'on confond : ±1, ±2, ±10, ±11.
fn nombres_voisins(n: i32, bas: i32, haut: i32) -> Vec<String> {
    let mut vus: Vec<i32> = Vec::new();
    for c in alea::melange(&[n - 10, n + 10, n - 1, n + 1, n - 2, n + 2, n - 11, n + 11]) {
        if (bas..=haut).contains(&c) && c != n && !vus.contains(&c) {
            vus.push(c);
            if vus.len() == 3 {
                break;
            }
        }
    }
    let mut extra = bas;
    while vus.len() < 3 {
        if extra != n && !vus.contains(&extra) {
            vus.push(extra);
        }
        extra += 1;
    }
    vus.iter().map(i32::to_string).collect()
}

// ---------------------------------------------------------------------------
// Séquences : jours, mois, saisons, alphabet

struct Sequence {
    unite: &'static str,
    pluriel: &'static str,
    feminin: bool,
    /// Après dimanche vient lundi ; après Z, rien.
    cyclique: bool,
    /// « de la semaine », « dans une semaine »
    contenant: &'static str,
    contenant2: &'static str,
    affiche: fn(&str) -> String,
    types: &'static [&'static str],
}

const TOUS_LES_TYPES: &[&str] = &["apres", "avant", "position", "compte", "premier", "dernier"];

fn tel_quel(s: &str) -> String {
    s.to_string()
}

fn ordinal(n: usize, feminin: bool) -> String {
    match (n, feminin) {
        (1, true) => "1re".into(),
        (1, false) => "1er".into(),
        _ => format!("{n}e"),
    }
}

fn sequence(liste: &[&str], cfg: &Sequence) -> Question {
    let len = liste.len();
    let enroule = |i: isize| i.rem_euclid(len as isize) as usize;
    let quel = if cfg.feminin { "Quelle" } else { "Quel" };
    let montre = cfg.affiche;
    match alea::un_de(cfg.types) {
        "apres" => {
            let i = if cfg.cyclique {
                alea::indice(len)
            } else {
                alea::indice(len - 1)
            };
            let j = enroule(i as isize + 1);
            Question::new(
                format!(
                    "{quel} {} vient juste après {} ?",
                    cfg.unite,
                    montre(liste[i])
                ),
                majuscule(liste[j]),
                sauf_un(liste, j, 3),
            )
        }
        "avant" => {
            let i = if cfg.cyclique {
                alea::indice(len)
            } else {
                1 + alea::indice(len - 1)
            };
            let j = enroule(i as isize - 1);
            Question::new(
                format!(
                    "{quel} {} vient juste avant {} ?",
                    cfg.unite,
                    montre(liste[i])
                ),
                majuscule(liste[j]),
                sauf_un(liste, j, 3),
            )
        }
        "position" => {
            let i = alea::indice(len);
            let article = if cfg.feminin { "la" } else { "le" };
            Question::new(
                format!(
                    "{quel} est {article} {} {} de {} ?",
                    ordinal(i + 1, cfg.feminin),
                    cfg.unite,
                    cfg.contenant
                ),
                majuscule(liste[i]),
                sauf_un(liste, i, 3),
            )
        }
        "compte" => {
            let n = len as i32;
            let mut choix = alea::melange(&[n - 2, n - 1, n + 1, n + 2]);
            choix.truncate(3);
            Question::new(
                format!(
                    "Combien y a-t-il de {} dans {} ?",
                    cfg.pluriel, cfg.contenant2
                ),
                n.to_string(),
                choix.iter().map(i32::to_string).collect(),
            )
        }
        "premier" => {
            let premier = if cfg.feminin {
                "la première"
            } else {
                "le premier"
            };
            Question::new(
                format!("{quel} est {premier} {} de {} ?", cfg.unite, cfg.contenant),
                majuscule(liste[0]),
                sauf_un(liste, 0, 3),
            )
        }
        _ => {
            let dernier = if cfg.feminin {
                "la dernière"
            } else {
                "le dernier"
            };
            Question::new(
                format!("{quel} est {dernier} {} de {} ?", cfg.unite, cfg.contenant),
                majuscule(liste[len - 1]),
                sauf_un(liste, len - 1, 3),
            )
        }
    }
}

fn jours() -> Question {
    sequence(
        &JOURS,
        &Sequence {
            unite: "jour",
            pluriel: "jours",
            feminin: false,
            cyclique: true,
            contenant: "la semaine",
            contenant2: "une semaine",
            affiche: tel_quel,
            types: TOUS_LES_TYPES,
        },
    )
}

fn mois() -> Question {
    sequence(
        &MOIS,
        &Sequence {
            unite: "mois",
            pluriel: "mois",
            feminin: false,
            cyclique: true,
            contenant: "l'année",
            contenant2: "une année",
            affiche: tel_quel,
            types: TOUS_LES_TYPES,
        },
    )
}

fn saisons() -> Question {
    if alea::pile_ou_face() {
        let (mois, saison) = alea::un_de(&MOIS_SAISON);
        let autres: Vec<&str> = SAISONS.iter().copied().filter(|s| *s != saison).collect();
        return Question::new(
            format!("En quelle saison est le mois de {mois} ?"),
            majuscule(saison),
            caps(&autres),
        );
    }
    sequence(
        &SAISONS,
        &Sequence {
            unite: "saison",
            pluriel: "saisons",
            feminin: true,
            cyclique: true,
            contenant: "l'année",
            contenant2: "une année",
            affiche: |s| saison_avec_article(s).to_string(),
            types: &["apres", "avant", "compte"],
        },
    )
}

fn alphabet() -> Question {
    sequence(
        &ALPHABET,
        &Sequence {
            unite: "lettre",
            pluriel: "lettres",
            feminin: true,
            cyclique: false,
            contenant: "l'alphabet",
            contenant2: "l'alphabet",
            affiche: |l| format!("la lettre {l}"),
            types: TOUS_LES_TYPES,
        },
    )
}

// ---------------------------------------------------------------------------
// Couleurs et formes

fn autres_couleurs(sauf: &[&str]) -> Vec<String> {
    let mut noms: Vec<&str> = COULEURS
        .iter()
        .map(|c| c.nom)
        .filter(|n| !sauf.contains(n))
        .collect();
    alea::melanger(&mut noms);
    noms.truncate(3);
    caps(&noms)
}

fn couleurs() -> Question {
    match alea::un_de(&["pastille", "pastille", "melange", "objet"]) {
        "pastille" => {
            let c = &COULEURS[alea::indice(COULEURS.len())];
            Question::new(
                "Quelle est cette couleur ?",
                majuscule(c.nom),
                autres_couleurs(&[c.nom]),
            )
            .cle(format!("pastille:{}", c.nom))
            .avec(Illustration::Pastille(c.hex))
        }
        "melange" => {
            let (a, b, donne) = alea::un_de(&MELANGES_COULEURS);
            Question::new(
                format!("Quelle couleur obtient-on en mélangeant du {a} et du {b} ?"),
                majuscule(donne),
                autres_couleurs(&[a, b, donne]),
            )
        }
        _ => {
            let (objet, couleur) = alea::un_de(&OBJETS_COULEUR);
            Question::new(
                format!("De quelle couleur est {objet} ?"),
                majuscule(couleur),
                autres_couleurs(&[couleur]),
            )
        }
    }
}

fn autres_formes(sauf: &[&str]) -> Vec<String> {
    let mut noms: Vec<&str> = FORMES
        .iter()
        .map(|f| f.nom)
        .filter(|n| !sauf.contains(n))
        .collect();
    alea::melanger(&mut noms);
    noms.truncate(3);
    caps(&noms)
}

fn formes() -> Question {
    match alea::un_de(&["visuelle", "visuelle", "cotes", "inverse", "objet"]) {
        "visuelle" => {
            let f = &FORMES[alea::indice(FORMES.len())];
            Question::new(
                "Quelle est cette forme ?",
                majuscule(f.nom),
                autres_formes(&[f.nom]),
            )
            .cle(format!("forme:{}", f.nom))
            .avec(Illustration::Forme(f.nom))
        }
        "cotes" => {
            if alea::entre(0, 5) == 0 {
                let mut choix = alea::melange(&["3", "4", "6", "7"].map(String::from));
                choix.truncate(3);
                return Question::new("Combien de branches a une étoile ?", "5", choix);
            }
            let (nom, n) = alea::un_de(&FORME_COTES);
            let mut leurres: Vec<String> = (3..)
                .filter(|&d| d != n)
                .take(3)
                .map(|d: i32| d.to_string())
                .collect();
            alea::melanger(&mut leurres);
            Question::new(
                format!("Combien de côtés a un {nom} ?"),
                n.to_string(),
                leurres,
            )
            .avec(Illustration::Forme(nom))
        }
        "inverse" => {
            let (n, nom) = alea::un_de(&COTES_UNIQUES);
            Question::new(
                format!("Quelle forme a {n} côtés ?"),
                majuscule(nom),
                autres_formes(&[nom]),
            )
        }
        _ => {
            let (objet, forme) = alea::un_de(&OBJETS_FORME);
            Question::new(
                format!("Quelle est la forme {objet} ?"),
                majuscule(forme),
                autres_formes(&[forme]),
            )
        }
    }
}

// ---------------------------------------------------------------------------
// Nombres

fn chiffres() -> Question {
    match alea::un_de(&["compter", "compter", "ecrire", "lire", "apres", "avant"]) {
        "compter" => {
            let n = alea::entre(1, 9);
            Question::new(
                "Combien de billes comptes-tu ?",
                n.to_string(),
                chiffres_autour(n, (n - 3).max(0), (n + 3).min(9)),
            )
            .cle(format!("billes:{n}"))
            .avec(Illustration::Billes {
                nombre: n as u32,
                par_rang: 5,
                couleur_par_rang: false,
            })
        }
        "ecrire" => {
            let n = alea::indice(10);
            Question::new(
                format!("Comment s'écrit le chiffre {n} ?"),
                majuscule(UNITES[n]),
                chiffres_en_lettres_sauf(n),
            )
        }
        "lire" => {
            let n = alea::indice(10);
            Question::new(
                format!("Quel chiffre s'écrit « {} » ?", UNITES[n]),
                n.to_string(),
                chiffres_autour(n as i32, 0, 9),
            )
        }
        "apres" => {
            let i = alea::entre(0, 8);
            Question::new(
                format!("Quel chiffre vient juste après {i} ?"),
                (i + 1).to_string(),
                chiffres_autour(i + 1, 0, 9),
            )
        }
        _ => {
            let i = alea::entre(1, 9);
            Question::new(
                format!("Quel chiffre vient juste avant {i} ?"),
                (i - 1).to_string(),
                chiffres_autour(i - 1, 0, 9),
            )
        }
    }
}

fn cinquante() -> Question {
    match alea::un_de(&[
        "compter", "compter", "dizaines", "unites", "composer", "suite",
    ]) {
        "compter" => {
            let n = alea::entre(11, 50);
            Question::new(
                "Combien de billes comptes-tu ?",
                n.to_string(),
                nombres_voisins(n, 11, 50),
            )
            .cle(format!("billes50:{n}"))
            .avec(Illustration::Billes {
                nombre: n as u32,
                par_rang: 10,
                couleur_par_rang: true,
            })
        }
        "dizaines" => {
            let n = alea::entre(11, 50);
            let d = n / 10;
            Question::new(
                format!("Dans le nombre {n}, combien y a-t-il de dizaines ?"),
                d.to_string(),
                chiffres_autour(d, 0, 5),
            )
        }
        "unites" => {
            let n = alea::entre(11, 49);
            let u = n % 10;
            Question::new(
                format!("Dans le nombre {n}, combien y a-t-il d'unités ?"),
                u.to_string(),
                chiffres_autour(u, 0, 9),
            )
        }
        "composer" => {
            let d = alea::entre(1, 4);
            let u = alea::entre(1, 9);
            let n = 10 * d + u;
            let mut vus: Vec<i32> = Vec::new();
            // l'inversion des chiffres est le piège classique
            for c in [10 * u + d, n - 1, n + 1, 10 * d, n + 10, n - 10] {
                if (1..=50).contains(&c) && c != n && !vus.contains(&c) {
                    vus.push(c);
                }
            }
            let mut extra = n + 2;
            while vus.len() < 3 {
                if !vus.contains(&extra) {
                    vus.push(extra);
                }
                extra += 1;
            }
            alea::melanger(&mut vus);
            let s = |k: i32| if k > 1 { "s" } else { "" };
            Question::new(
                format!(
                    "Avec {d} dizaine{} et {u} unité{}, quel nombre écris-tu ?",
                    s(d),
                    s(u)
                ),
                n.to_string(),
                vus.iter().take(3).map(i32::to_string).collect(),
            )
        }
        _ => {
            if alea::pile_ou_face() {
                let n = alea::entre(10, 49);
                Question::new(
                    format!("Quel nombre vient juste après {n} ?"),
                    (n + 1).to_string(),
                    nombres_voisins(n + 1, 0, 51),
                )
            } else {
                let n = alea::entre(11, 50);
                Question::new(
                    format!("Quel nombre vient juste avant {n} ?"),
                    (n - 1).to_string(),
                    nombres_voisins(n - 1, 0, 50),
                )
            }
        }
    }
}

fn nombres(niveau: u8) -> Question {
    let (bas, haut) = [(0, 16), (17, 69), (60, 100)][(niveau.clamp(1, 3) - 1) as usize];
    let n = alea::entre(bas, haut);
    let mut vus: Vec<i32> = Vec::new();
    let mut essais = 0;
    while vus.len() < 3 && essais < 100 {
        essais += 1;
        let delta = if alea::pile_ou_face() {
            alea::entre(1, 6)
        } else {
            -alea::entre(1, 6)
        };
        let d = (n + delta).clamp(bas, haut);
        if d != n && !vus.contains(&d) {
            vus.push(d);
        }
    }
    let mut extra = bas;
    while vus.len() < 3 {
        if extra != n && !vus.contains(&extra) {
            vus.push(extra);
        }
        extra += 1;
    }
    let lettres = |k: i32| nombre_en_lettres(k as u32);
    if alea::pile_ou_face() {
        Question::new(
            format!("Comment s'écrit le nombre {n} ?"),
            lettres(n),
            vus.iter().map(|&k| lettres(k)).collect(),
        )
    } else {
        Question::new(
            format!("Quel nombre s'écrit « {} » ?", lettres(n)),
            n.to_string(),
            vus.iter().map(i32::to_string).collect(),
        )
    }
}

// ---------------------------------------------------------------------------
// Lire l'heure

/// Les minutes d'un niveau : heures pleines, puis quarts, puis de 5 en 5, puis
/// toutes. Un niveau inattendu retombe sur le plus proche.
pub fn minutes_du_niveau(niveau: u8) -> Vec<u32> {
    match niveau {
        0 | 1 => vec![0],
        2 => vec![0, 15, 30, 45],
        3 => (0..60).step_by(5).collect(),
        _ => (0..60).collect(),
    }
}

/// 3 autres horaires du même niveau, en toutes lettres : de préférence la même
/// heure avec d'autres minutes, le piège le plus courant.
fn autres_horaires(h: u32, m: u32, niveau: u8) -> Vec<String> {
    let minutes = minutes_du_niveau(niveau);
    let juste = heure_en_lettres(h, m);
    let mut vus: Vec<String> = Vec::new();
    let mut essais = 0;
    while vus.len() < 3 && essais < 200 {
        essais += 1;
        let hh = if alea::entre(0, 2) == 0 {
            h
        } else {
            alea::entre(1, 12) as u32
        };
        let mm = minutes[alea::indice(minutes.len())];
        let t = heure_en_lettres(hh, mm);
        if t != juste && !vus.contains(&t) {
            vus.push(t);
        }
    }
    let mut extra = 1;
    while vus.len() < 3 {
        let t = heure_en_lettres(extra, minutes[0]);
        if t != juste && !vus.contains(&t) {
            vus.push(t);
        }
        extra += 1;
    }
    vus
}

/// Le rôle des aiguilles, indépendant du niveau.
const NOTIONS_HEURE: [(&str, &str, [&str; 3]); 11] = [
    (
        "Quelle aiguille indique les heures ?",
        "La petite",
        ["La grande", "Les deux", "Aucune"],
    ),
    (
        "Quelle aiguille indique les minutes ?",
        "La grande",
        ["La petite", "Les deux", "Aucune"],
    ),
    (
        "Combien y a-t-il de minutes dans une heure ?",
        "60",
        ["12", "30", "100"],
    ),
    (
        "Combien y a-t-il d'heures sur le cadran d'une horloge ?",
        "12",
        ["10", "24", "60"],
    ),
    (
        "Quand la grande aiguille est sur le 6, il est…",
        "et demie",
        ["et quart", "moins le quart", "pile"],
    ),
    (
        "Quand la grande aiguille est sur le 3, il est…",
        "et quart",
        ["et demie", "moins le quart", "pile"],
    ),
    (
        "Quand la grande aiguille est sur le 12, il est…",
        "pile",
        ["et quart", "et demie", "moins le quart"],
    ),
    (
        "Combien de minutes séparent deux chiffres du cadran ?",
        "5",
        ["1", "10", "12"],
    ),
    (
        "Sur quel chiffre est la grande aiguille à « et quart » ?",
        "3",
        ["6", "9", "12"],
    ),
    (
        "Sur quel chiffre est la grande aiguille à « et demie » ?",
        "6",
        ["3", "9", "12"],
    ),
    (
        "Sur quel chiffre est la grande aiguille à « moins le quart » ?",
        "9",
        ["3", "6", "12"],
    ),
];

fn heure(niveau: u8) -> Question {
    // lire l'horloge revient le plus souvent ; on ajoute les notions et une projection
    let genre = alea::un_de(&["lire", "lire", "lire", "notion", "plus_tard"]);
    if genre == "notion" {
        let (enonce, reponse, leurres) = alea::un_de(&NOTIONS_HEURE);
        return Question::new(enonce, reponse, leurres.map(String::from).to_vec());
    }
    let minutes = minutes_du_niveau(niveau);
    let h = alea::entre(1, 12) as u32;
    let m = minutes[alea::indice(minutes.len())];
    let horloge = Illustration::Horloge {
        heures: h,
        minutes: m,
    };
    if genre == "plus_tard" {
        let suivante = if h == 12 { 1 } else { h + 1 };
        return Question::new(
            "Dans une heure, quelle heure sera-t-il ?",
            heure_en_lettres(suivante, m),
            autres_horaires(suivante, m, niveau),
        )
        .cle(format!("heure+1:{h}:{m}"))
        .avec(horloge);
    }
    // l'énoncé est toujours le même : on déduplique sur l'horaire
    Question::new(
        "Quelle heure est-il ?",
        heure_en_lettres(h, m),
        autres_horaires(h, m, niveau),
    )
    .cle(format!("heure:{h}:{m}"))
    .avec(horloge)
}

// ---------------------------------------------------------------------------
// Calcul

fn calcul(operation: &str, niveau: u8) -> Question {
    let n = (niveau.clamp(1, 3) - 1) as usize;
    let (a, b, resultat, symbole) = match operation {
        "addition" => {
            let max = [10, 20, 100][n];
            let (a, b) = (alea::entre(1, max), alea::entre(1, max));
            (a, b, a + b, "+")
        }
        "soustraction" => {
            let max = [10, 20, 100][n];
            let (x, y) = (alea::entre(1, max), alea::entre(1, max));
            let (a, b) = (x.max(y), x.min(y));
            (a, b, a - b, "−")
        }
        "multiplication" => {
            let max = [5, 10, 12][n];
            let (a, b) = (alea::entre(1, max), alea::entre(1, 10));
            (a, b, a * b, "×")
        }
        _ => {
            // division exacte : on part du résultat
            let max = [5, 10, 12][n];
            let b = alea::entre(2, max);
            let r = alea::entre(1, max);
            (b * r, b, r, "÷")
        }
    };
    Question::new(
        format!("{a} {symbole} {b} = ?"),
        resultat.to_string(),
        resultats_proches(resultat, 3),
    )
}

// ---------------------------------------------------------------------------
// Aiguillage et composition du quiz

pub fn question(categorie: &str, niveau: u8) -> Question {
    match categorie {
        "jours" => jours(),
        "mois" => mois(),
        "saisons" => saisons(),
        "alphabet" => alphabet(),
        "couleurs" => couleurs(),
        "formes" => formes(),
        "chiffres" => chiffres(),
        "cinquante" => cinquante(),
        "heure" => heure(niveau),
        "nombres" => nombres(niveau),
        "planifier" => planifier::question(niveau),
        "logique" => logique::question(niveau),
        operation => calcul(operation, niveau),
    }
}

/// Les thèmes que le mode Mélange entrelace.
pub const SOURCES_MELANGE: [&str; 16] = [
    "chiffres",
    "cinquante",
    "nombres",
    "addition",
    "soustraction",
    "multiplication",
    "division",
    "jours",
    "mois",
    "saisons",
    "alphabet",
    "couleurs",
    "formes",
    "heure",
    "planifier",
    "logique",
];

/// Thèmes à ensemble de questions fini : on peut y faire revenir en priorité
/// ce qui est à réviser. Le calcul, infini, reste au hasard.
pub const GUIDES_PAR_LA_MEMOIRE: [&str; 10] = [
    "jours",
    "mois",
    "saisons",
    "alphabet",
    "couleurs",
    "formes",
    "chiffres",
    "cinquante",
    "nombres",
    "heure",
];

/// Ce que la mémoire sait d'une notion, pour choisir les questions.
#[derive(Clone, Copy, Debug)]
pub struct EtatNotion {
    pub boite: u8,
    /// Prochaine révision, en secondes depuis 1970 ; 0 si jamais vue.
    pub due: f64,
}

/// Compose un quiz. `etats` donne, par clé complète (« catégorie:clé »), ce que la
/// mémoire sait de chaque notion du thème ; `maintenant` est en secondes.
pub fn quiz(
    categorie: &str,
    niveau: u8,
    etats: &HashMap<String, EtatNotion>,
    maintenant: f64,
) -> Vec<Question> {
    if categorie == "melange" {
        return entrelace();
    }
    if GUIDES_PAR_LA_MEMOIRE.contains(&categorie) {
        return guide_par_la_memoire(categorie, niveau, etats, maintenant);
    }
    au_hasard(categorie, niveau)
}

/// Mélange : un thème après l'autre, chacun à un niveau facile ou moyen. Il
/// faut à chaque question reconnaître de quoi il s'agit.
fn entrelace() -> Vec<Question> {
    let sources = alea::melange(&SOURCES_MELANGE);
    let mut questions = Vec::new();
    let mut vues = HashSet::new();
    let mut i = 0;
    let mut essais = 0;
    while questions.len() < NB_QUESTIONS && essais < 400 {
        essais += 1;
        let cat = sources[i % sources.len()];
        i += 1;
        let mut q = question(cat, alea::entre(1, 2) as u8);
        if !vues.insert(format!("{cat}:{}", q.cle)) {
            continue;
        }
        q.categorie = Some(cat);
        questions.push(q);
    }
    questions
}

fn au_hasard(categorie: &str, niveau: u8) -> Vec<Question> {
    let mut questions = Vec::new();
    let mut vues = HashSet::new();
    let mut essais = 0;
    while questions.len() < NB_QUESTIONS && essais < 300 {
        essais += 1;
        let q = question(categorie, niveau);
        if vues.insert(q.cle.clone()) {
            questions.push(q);
        }
    }
    questions
}

/// Sélection guidée par la mémoire : un réservoir de questions toutes
/// différentes, dont passent d'abord les notions à réviser (date dépassée),
/// puis celles jamais vues, enfin le reste — au hasard dans chaque groupe.
fn guide_par_la_memoire(
    categorie: &str,
    niveau: u8,
    etats: &HashMap<String, EtatNotion>,
    maintenant: f64,
) -> Vec<Question> {
    let mut reservoir = Vec::new();
    let mut vues = HashSet::new();
    let mut essais = 0;
    while reservoir.len() < 60 && essais < 800 {
        essais += 1;
        let q = question(categorie, niveau);
        if vues.insert(q.cle.clone()) {
            reservoir.push(q);
        }
    }
    // 0 = à réviser, 1 = jamais vue, 2 = déjà sue
    let groupe = |q: &Question| match etats.get(&format!("{categorie}:{}", q.cle)) {
        None => 1,
        Some(e) if e.due > 0.0 && e.due <= maintenant => 0,
        Some(_) => 2,
    };
    alea::melanger(&mut reservoir);
    reservoir.sort_by_key(groupe); // tri stable : le hasard reste dans chaque groupe
    reservoir.truncate(NB_QUESTIONS);
    reservoir
}

/// Où revient une question ratée à la position `i`, dans une partie de
/// `nombre` questions : 3 questions plus loin, puis encore 5 après. Les deux
/// positions s'appliquent l'une après l'autre (la seconde compte la première
/// copie déjà insérée) ; en fin de partie, la question s'ajoute à la suite.
pub fn retours(i: usize, nombre: usize) -> [usize; 2] {
    let premiere = (i + 3).min(nombre);
    [premiere, (i + 8).min(nombre + 1)]
}

/// Répétition espacée dans la partie : la question ratée revient deux fois,
/// ses réponses dans un autre ordre.
pub fn reprogrammer(questions: &mut Vec<Question>, q: &Question, i: usize) {
    for position in retours(i, questions.len()) {
        let mut copie = q.clone();
        copie.rebattre();
        questions.insert(position, copie);
    }
}

/// Les félicitations, variées pour ne pas lasser.
pub fn bravo() -> &'static str {
    alea::un_de(&["✅ Bravo !", "✅ Super !", "✅ Exact !", "✅ Génial !"])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::categories::CATEGORIES;

    #[test]
    fn chaque_theme_donne_un_quiz_complet_et_sans_doublon() {
        alea::amorcer(2024);
        let vide = HashMap::new();
        for c in CATEGORIES {
            let niveaux: Vec<u8> = if c.niveaux.is_empty() {
                vec![1]
            } else {
                c.niveaux.iter().map(|n| n.id).collect()
            };
            for niveau in niveaux {
                for _ in 0..20 {
                    let qs = quiz(c.cle, niveau, &vide, 0.0);
                    assert_eq!(qs.len(), NB_QUESTIONS, "{} niveau {niveau}", c.cle);
                    let mut cles: Vec<String> = qs.iter().map(|q| q.cle_memoire(c.cle)).collect();
                    cles.sort();
                    cles.dedup();
                    assert_eq!(cles.len(), NB_QUESTIONS, "doublon dans {}", c.cle);
                    for q in &qs {
                        let mut options = q.options.clone();
                        options.sort();
                        options.dedup();
                        assert_eq!(options.len(), 4, "{} : {q:?}", c.cle);
                        assert!(q.options.contains(&q.reponse), "{q:?}");
                        assert!(!q.leurres.contains(&q.reponse), "{q:?}");
                    }
                }
            }
        }
    }

    #[test]
    fn le_melange_note_le_theme_de_chaque_question() {
        alea::amorcer(5);
        let qs = quiz("melange", 1, &HashMap::new(), 0.0);
        assert!(qs.iter().all(|q| q.categorie.is_some()));
        let themes: HashSet<_> = qs.iter().filter_map(|q| q.categorie).collect();
        assert!(
            themes.len() >= 5,
            "le mélange alterne les thèmes : {themes:?}"
        );
    }

    #[test]
    fn les_notions_a_reviser_passent_en_premier() {
        alea::amorcer(9);
        // toutes les questions de pastille sont « à réviser »
        let mut etats = HashMap::new();
        for c in &COULEURS {
            etats.insert(
                format!("couleurs:pastille:{}", c.nom),
                EtatNotion {
                    boite: 1,
                    due: 10.0,
                },
            );
        }
        let qs = quiz("couleurs", 1, &etats, 100.0);
        let pastilles = qs
            .iter()
            .take_while(|q| q.cle.starts_with("pastille:"))
            .count();
        let total = qs.iter().filter(|q| q.cle.starts_with("pastille:")).count();
        assert!(total > 0);
        assert_eq!(pastilles, total, "les révisions dues d'abord");
    }

    #[test]
    fn la_question_ratee_revient_deux_fois() {
        alea::amorcer(4);
        let mut qs = quiz("addition", 1, &Default::default(), 0.0);
        let ratee = qs[2].clone();
        reprogrammer(&mut qs, &ratee, 2);
        assert_eq!(qs.len(), 12);
        assert_eq!(qs[5].cle, ratee.cle);
        assert_eq!(qs[10].cle, ratee.cle);
        // en fin de partie, elle s'ajoute à la suite
        let derniere = qs[11].clone();
        reprogrammer(&mut qs, &derniere, 11);
        assert_eq!(qs.len(), 14);
        assert_eq!(
            (qs[12].cle.as_str(), qs[13].cle.as_str()),
            (derniere.cle.as_str(), derniere.cle.as_str())
        );
        assert_eq!(retours(0, 10), [3, 8]);
        assert_eq!(retours(9, 10), [10, 11]);
    }

    #[test]
    fn la_division_tombe_juste() {
        alea::amorcer(1);
        for niveau in 1..=3 {
            for _ in 0..200 {
                let q = question("division", niveau);
                let (a, reste) = q.enonce.split_once(" ÷ ").unwrap();
                let b = reste.split_once(' ').unwrap().0;
                let (a, b): (i32, i32) = (a.parse().unwrap(), b.parse().unwrap());
                assert_eq!(a % b, 0);
                assert_eq!((a / b).to_string(), q.reponse);
            }
        }
    }
}
