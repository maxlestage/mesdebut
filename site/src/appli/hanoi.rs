//! « 🗼 Tours de Hanoï » : le mini-jeu de réflexion. Les règles viennent du
//! noyau (`moteur::hanoi`) ; cet écran dessine les tiges et transmet les coups.
//!
//! On touche une tige pour prendre son disque du haut, puis une autre pour l'y
//! poser. Pas de glisser-déposer : deux touches suffisent, au doigt comme au
//! clavier.

use active::prelude::*;

use super::{gros_bouton, gros_lien, plateforme, retour, retour_menu, sous_titre, titre};
use crate::moteur::hanoi::{Hanoi, NIVEAUX, niveau};

pub const ADRESSE: &str = "/app/hanoi";

const AIDE: &str =
    "Touche une tige pour prendre son disque du haut, puis une autre pour l'y poser.";
const INTERDIT: &str = "Un grand disque ne va jamais sur un plus petit !";

/// Les couleurs des disques, du plus petit au plus grand : sur chacune, le
/// numéro blanc reste lisible.
const COULEURS: [&str; 5] = ["#c2185b", "#6a1b9a", "#1565c0", "#00796b", "#e65100"];

pub fn niveaux() -> Node {
    section()
        .child(Seo::new().title("Tours de Hanoï"))
        .child(titre("🗼 Tours de Hanoï"))
        .child(sous_titre(
            "Déplace toute la pile sur la dernière tige, un disque à la fois, \
             sans jamais poser un grand disque sur un plus petit.",
        ))
        .children(NIVEAUX.iter().map(|n| {
            gros_lien(
                format!("{ADRESSE}/{}", n.id),
                format!("{} {}", n.emoji, n.libelle),
            )
        }))
        .child(retour_menu())
        .into()
}

pub fn page(id: u8) -> Node {
    let disques = niveau(id).map_or(3, |n| n.disques);
    let jeu = use_state(Hanoi::new(disques));
    // la tige dont on a pris le disque du haut
    let prise: State<Option<usize>> = use_state(None);
    let message: State<&'static str> = use_state(AIDE);

    let toucher = move |tige: usize| {
        if jeu.with(Hanoi::gagne) {
            return;
        }
        match prise.get() {
            None if jeu.with(|j| !j.tours[tige].is_empty()) => {
                prise.set(Some(tige));
                message.set(AIDE);
            }
            None => {}
            Some(de) if de == tige => prise.set(None),
            Some(de) => {
                let mut joue = false;
                jeu.update(|j| joue = j.deplacer(de, tige));
                prise.set(None);
                if joue {
                    message.set(AIDE);
                    plateforme::vibrer(if jeu.with(Hanoi::gagne) { 60 } else { 15 });
                } else {
                    message.set(INTERDIT);
                    plateforme::vibrer(120);
                }
            }
        }
    };

    let recommencer = move |_| {
        jeu.set(Hanoi::new(disques));
        prise.set(None);
        message.set(AIDE);
    };

    section()
        .class("hanoi")
        .child(Seo::new().title(format!("Tours de Hanoï — {disques} disques")))
        .child(
            h1().class("titre-ecran petit")
                .text(format!("🗼 Tours de Hanoï · {disques} disques")),
        )
        .child(
            div()
                .class("quiz-tete")
                .child(span().text_dyn(move || format!("Coups : {}", jeu.with(|j| j.coups))))
                .child(span().text_dyn(move || format!("Minimum : {}", jeu.with(Hanoi::minimum)))),
        )
        .children_dyn(move || {
            let (j, p) = (jeu.get(), prise.get());
            vec![tiges(&j, p, toucher)]
        })
        .children_dyn(move || {
            if jeu.with(Hanoi::gagne) {
                vec![fin(&jeu.get(), id, recommencer)]
            } else {
                vec![
                    p().class("correction")
                        .attr("aria-live", "polite")
                        .class_if("faux", move || message.get() == INTERDIT)
                        .text_dyn(move || message.get().to_string())
                        .into(),
                    gros_bouton("🔁 Recommencer", recommencer)
                        .class("secondaire")
                        .into(),
                ]
            }
        })
        .child(retour(ADRESSE, "← Changer de niveau"))
        .into()
}

fn tiges(jeu: &Hanoi, prise: Option<usize>, toucher: impl Fn(usize) + Copy + 'static) -> Node {
    let n = jeu.disques.max(1) as u32;
    div()
        .class("tiges")
        .children(jeu.tours.iter().enumerate().map(|(i, tour)| {
            let haut = tour.last().copied();
            let etiquette = match (haut, prise == Some(i)) {
                (_, true) => format!("Tige {} : disque {} pris", i + 1, haut.unwrap_or(0)),
                (None, _) => format!("Tige {} : vide", i + 1),
                (Some(d), _) => {
                    format!("Tige {} : {} disque(s), en haut le {d}", i + 1, tour.len())
                }
            };
            button()
                .class("tige")
                .attr("type", "button")
                .attr("aria-label", etiquette)
                .class_if("prise", move || prise == Some(i))
                .class_if("arrivee", move || i == 2)
                .on_click(move |_| toucher(i))
                .children(tour.iter().enumerate().map(|(rang, &d)| {
                    let leve = prise == Some(i) && rang + 1 == tour.len();
                    span()
                        .class("disque")
                        .class_if("leve", move || leve)
                        .attr("aria-hidden", "true")
                        .style(format!(
                            "width:{}%;background:{}",
                            30 + 70 * d as u32 / n,
                            COULEURS[(d as usize - 1) % COULEURS.len()]
                        ))
                        .text(d.to_string())
                }))
        }))
        .into()
}

fn fin(jeu: &Hanoi, id: u8, recommencer: impl Fn(Event) + 'static) -> Node {
    let (etoiles, message) = jeu.bilan();
    let suivant = niveau(id + 1);
    div()
        .class("fin")
        .child(titre("Bravo, la pile est arrivée !"))
        .child(
            p().class("etoiles")
                .attr("aria-label", format!("{} étoiles", etoiles.chars().count()))
                .text(etoiles),
        )
        .child(p().class("score").text(message))
        .child(gros_bouton("🔁 Rejouer", recommencer))
        .child(suivant.map(|n| {
            gros_lien(
                format!("{ADRESSE}/{}", n.id),
                format!("{} Essayer avec {}", n.emoji, n.libelle),
            )
        }))
        .into()
}
