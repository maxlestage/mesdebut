//! Les sections de la page, dans l'ordre où elles s'affichent.

use active::prelude::*;

use super::donnees::{MANIFESTE, OFFRE, PRINCIPES, SUPPORTS};
use super::texte::{mots, sans_marques, titre};
use crate::moteur::categories::CATEGORIES;

const NBSP: &str = "\u{a0}";

fn cache(el: Element) -> Element {
    el.attr("aria-hidden", "true")
}

fn etiquette(texte: &'static str) -> Element {
    p().class("etiquette").attr("data-revele", "").text(texte)
}

/// La barre du haut : elle s'efface quand on descend, revient quand on remonte.
pub fn entete() -> Node {
    header()
        .class("entete")
        .child(
            a().class("marque")
                .href("#haut")
                .attr("data-aimant", "")
                .child(
                    img()
                        .attr("src", "icon.svg")
                        .attr("alt", "")
                        .attr("width", "28")
                        .attr("height", "28"),
                )
                .child(span().text("ReNeuro")),
        )
        .child(
            nav()
                .attr("aria-label", "Sections")
                .child(a().href("#themes").text("Thèmes"))
                .child(a().href("#planifier").text("Planifier"))
                .child(a().href("#tarif").text("Tarif"))
                .child(a().class("entete-app").href("app").text("Ouvrir l'app")),
        )
        .into()
}

/// L'ouverture : une scène collée à l'écran pendant que l'on défile. La
/// poussière s'y rassemble en toque, ligne après ligne le titre monte.
pub fn hero() -> Node {
    let lignes: [(&str, &str); 3] = [
        ("Réapprendre", ""),
        ("les bases,", ""),
        ("en ", "s'amusant."),
    ];
    let titre = h1()
        .class("hero-titre")
        .attr("aria-label", "Réapprendre les bases, en s'amusant.")
        .children(lignes.iter().enumerate().map(|(i, (texte, accent))| {
            let mut dedans = span()
                .class("ligne-in")
                .style(format!("--i:{i}"))
                .text(*texte);
            if !accent.is_empty() {
                dedans = dedans.child(em().class("accent").text(*accent));
            }
            cache(span().class("ligne")).child(dedans)
        }));

    section()
        .id("haut")
        .class("hero")
        .child(
            div()
                .class("hero-scene")
                .child(cache(canvas().class("poussiere")))
                .child(
                    div()
                        .class("hero-contenu")
                        .child(
                            p().class("surtitre")
                                .child(span().class("puce"))
                                .text("iPhone, Apple Watch & navigateur"),
                        )
                        .child(titre)
                        .child(
                            div()
                                .class("hero-bas")
                                .child(p().class("sous-accroche").text(format!(
                                    "Les jours, l'heure, le calcul, les couleurs, organiser sa journée{NBSP}: \
                                     dix-huit thèmes, en questions courtes."
                                )))
                                .child(
                                    div()
                                        .class("hero-actions")
                                        // L'application s'ouvre d'ici, dans le navigateur :
                                        // un lien ordinaire, l'app a sa propre page.
                                        .child(
                                            a().class("bouton")
                                                .href("app")
                                                .attr("data-aimant", "")
                                                .child(span().text("Ouvrir l'application"))
                                                .child(cache(span().class("bouton-fleche")).text("→")),
                                        )
                                        .child(
                                            a().class("lien-discret")
                                                .href("#themes")
                                                .text("ou parcourir les thèmes ↓"),
                                        )
                                        .child(
                                            p().class("pastille-store")
                                                .text(format!("{} · ", OFFRE.disponibilite))
                                                .child(strong().text(format!("{} {}", OFFRE.prix, OFFRE.periode))),
                                        ),
                                ),
                        ),
                )
                .child(
                    cache(div().class("indice"))
                        .child(span().text("Faire défiler"))
                        .child(span().class("indice-trait")),
                ),
        )
        .into()
}

/// Un paragraphe qui s'allume mot à mot à mesure que l'on défile.
pub fn manifeste() -> Node {
    section()
        .class("manifeste")
        .attr("aria-label", "Le principe")
        .child(etiquette("(01) — Le principe"))
        .child(
            p().class("manifeste-texte")
                .attr("aria-label", sans_marques(MANIFESTE))
                .child(cache(span()).children(mots(MANIFESTE))),
        )
        .into()
}

/// La bande des thèmes, qui défile sans fin et s'incline avec la vitesse du
/// défilement. Deux copies se suivent pour que la boucle ne se voie pas.
pub fn bande() -> Node {
    let copie = |cachee: bool| {
        let piste = ul()
            .class("bande-piste")
            .children(CATEGORIES.iter().map(|c| {
                li().child(cache(span().class("bande-emoji")).text(c.emoji))
                    .text(c.libelle)
                    .child(cache(span().class("bande-sep")).text("✦"))
            }));
        if cachee { cache(piste) } else { piste }
    };
    div()
        .class("bande")
        .attr("aria-label", "Les dix-huit thèmes")
        .child(
            div()
                .class("bande-defile")
                .attr("data-defile-voulu", "")
                .child(copie(false))
                .child(copie(true)),
        )
        .into()
}

pub fn themes() -> Node {
    let avec_niveaux = CATEGORIES.iter().filter(|c| c.a_des_niveaux()).count();
    section()
        .id("themes")
        .class("section")
        .child(etiquette("(02) — Les thèmes"))
        .child(titre("Dix-huit thèmes, *dix* questions à la fois."))
        .child(
            p().class("intro")
                .attr("data-revele", "")
                .text("Chaque thème se révise d'abord, puis se travaille en quiz de dix questions."),
        )
        .child(ul().class("grille").children(CATEGORIES.iter().enumerate().map(|(i, c)| {
            let mut carte = a()
                .class("carte-theme")
                // relatif, comme toute la page : `app/heure`, `app/addition/niveaux`…
                .href(crate::appli::adresse_theme(c)[1..].to_string())
                .attr("data-incline", "")
                .attr("title", c.titre)
                .child(cache(span().class("carte-reflet")))
                .child(cache(span().class("emoji")).text(c.emoji))
                .child(span().class("nom").text(c.libelle));
            if c.a_des_niveaux() {
                carte = carte.child(span().class("niveaux").text(format!("{} niveaux", c.niveaux.len())));
            }
            li().attr("data-revele", "")
                .style(format!("--a:{};--b:{};--i:{}", c.degrade.0, c.degrade.1, i % 4))
                .child(carte)
        })))
        .child(
            p().class("note")
                .attr("data-revele", "")
                .text(format!(
                    "{avec_niveaux} thèmes ont plusieurs niveaux. L'heure en a quatre, des heures pleines \
                     à la minute près — soit "
                ))
                .child(strong().class("compte").attr("data-compte-vers", "720").text("720"))
                .text(" horaires différents à lire sur une horloge à aiguilles."),
        )
        .into()
}

struct Ligne {
    heure: &'static str,
    tache: &'static str,
    duree: &'static str,
    fait: bool,
    etat: &'static str,
}

const JOURNEE: &[Ligne] = &[
    Ligne {
        heure: "9 h 00",
        tache: "Prendre son médicament",
        duree: "2 → 2 min",
        fait: true,
        etat: "juste",
    },
    Ligne {
        heure: "9 h 02",
        tache: "Se faire un café",
        duree: "5 → 9 min",
        fait: false,
        etat: "ecart",
    },
    Ligne {
        heure: "9 h 11",
        tache: "Faire les courses",
        duree: "45 min",
        fait: false,
        etat: "",
    },
];

pub fn planification() -> Node {
    let exemple = div()
        .class("exemple")
        .attr("data-revele", "")
        .attr("data-parallaxe", "-0.06")
        .child(p().class("exemple-titre").text("Ma journée"))
        .children(JOURNEE.iter().enumerate().map(|(i, l)| {
            let mut tache = span().class("exemple-tache");
            if l.fait {
                tache = tache.class("fait");
            }
            let mut duree = span().class("exemple-duree");
            if !l.etat.is_empty() {
                duree = duree.class(l.etat);
            }
            div()
                .class("exemple-ligne")
                .style(format!("--i:{i}"))
                .child(span().class("exemple-heure").text(l.heure))
                .child(tache.text(l.tache))
                .child(duree.text(l.duree))
        }))
        .child(
            p().class("exemple-bilan")
                .text(format!("Sur 5 jours{NBSP}: "))
                .child(
                    strong()
                        .class("compte")
                        .attr("data-compte-vers", "18")
                        .attr("data-prefixe", "+")
                        .attr("data-suffixe", format!("{NBSP}%"))
                        .text(format!("+18{NBSP}%")),
                )
                .text(format!(
                    " d'écart en moyenne (contre +45{NBSP}% avant) — tu progresses{NBSP}! 🌱"
                )),
        );

    section()
        .id("planifier")
        .class("section section-planifier")
        .child(
            div()
                .class("planifier-texte")
                .child(etiquette("(03) — Planifier"))
                .child(titre("Organiser sa journée, pas *seulement* la raconter."))
                .child(p().attr("data-revele", "").text(format!(
                    "Onze activités du quotidien sont décomposées en étapes{NBSP}: se faire un café, faire une \
                     lessive, prendre le bus. On apprend à les remettre dans l'ordre, à repérer l'étape qui \
                     manque, à estimer le temps qu'elles prennent."
                )))
                .child(
                    p().attr("data-revele", "")
                        .text("Puis on passe à sa vraie journée. ")
                        .child(strong().text("Ma journée"))
                        .text(format!(
                            " laisse écrire ses tâches, les ordonner, les cocher. Chacune démarre quand la \
                             précédente finit, donc l'heure de chaque tâche s'affiche{NBSP}: on voit tout de \
                             suite quand la journée déborde."
                        )),
                )
                .child(p().attr("data-revele", "").text(
                    "Un chronomètre compare le temps prévu au temps réellement passé, et un suivi sur dix jours \
                     montre si l'écart se resserre. C'est cet écart-là qu'on travaille.",
                ))
                .child(
                    p().attr("data-revele", "")
                        .child(a().class("lien").href("app/journee").text("Essayer « Ma journée » →")),
                ),
        )
        .child(exemple)
        .into()
}

pub fn supports() -> Node {
    section()
        .class("section")
        .child(etiquette("(04) — Les écrans"))
        .child(titre("De la poche *au navigateur*."))
        .child(
            p().class("intro")
                .attr("data-revele", "")
                .text("Le même contenu, la même progression, sur trois écrans."),
        )
        .child(
            ul().class("supports")
                .children(SUPPORTS.iter().enumerate().map(|(i, s)| {
                    li().attr("data-revele", "")
                        .style(format!("--i:{i}"))
                        .child(
                            cache(span().class("support-emoji"))
                                .attr("data-parallaxe", if i % 2 == 0 { "0.08" } else { "-0.08" })
                                .text(s.emoji),
                        )
                        .child(
                            div()
                                .child(h3().text(s.nom))
                                .child(p().text(s.detail))
                                .child(p().class("support-note").text(s.note)),
                        )
                })),
        )
        .into()
}

pub fn principes() -> Node {
    section()
        .class("section section-principes")
        .child(etiquette("(05) — La méthode"))
        .child(titre("Comment c'est *construit*."))
        .child(p().class("intro").attr("data-revele", "").text(
            "Six principes issus des sciences cognitives, qui décident de la forme des exercices.",
        ))
        .child(
            ol().class("principes")
                .children(PRINCIPES.iter().enumerate().map(|(i, p_)| {
                    li().attr("data-revele", "")
                        .style(format!("--i:{}", i % 2))
                        .child(cache(span().class("principe-numero")).text(format!("{:02}", i + 1)))
                        .child(div().child(h3().text(p_.titre)).child(p().text(p_.texte)))
                })),
        )
        .into()
}

pub fn tarif() -> Node {
    section()
        .id("tarif")
        .class("section section-tarif")
        .child(etiquette("(06) — Le tarif"))
        .child(titre("Un prix, *tout* compris."))
        .child(
            div()
                .class("carte-tarif")
                .attr("data-revele", "")
                .attr("data-lueur", "")
                .child(
                    p().class("tarif-prix")
                        .child(
                            strong()
                                .class("compte")
                                .attr("data-compte-vers", OFFRE.centimes.to_string())
                                .attr("data-centimes", "")
                                .attr("data-suffixe", format!("{NBSP}€"))
                                .text(OFFRE.prix),
                        )
                        .child(span().class("tarif-periode").text(OFFRE.periode)),
                )
                .child(
                    ul().class("tarif-inclus")
                        .children(OFFRE.inclus.iter().map(|l| li().text(*l))),
                )
                .child(p().class("tarif-dispo").text(OFFRE.disponibilite)),
        )
        .into()
}

pub fn pied() -> Node {
    footer()
        .class("pied")
        .child(
            cache(p().class("pied-marque"))
                .attr("data-revele", "")
                .text("Re")
                .child(em().class("accent").text("Neuro")),
        )
        .child(
            div()
                .class("pied-bas")
                .child(
                    p().class("credits")
                        .text("Conçu et développé par Maxime Nathan Lestage")
                        .child(
                            span()
                                .class("fait-avec")
                                .text("Fait avec ")
                                .child(a().href(crate::ACTIVE).text("Active"))
                                .text(", en Rust compilé en WebAssembly."),
                        ),
                )
                .child(p().class("mention").text(
                    "ReNeuro est une application d'entraînement, pas un dispositif médical, et ne remplace \
                     aucun suivi ni aucune rééducation.",
                ))
                .child(a().class("retour").href("#haut").attr("data-aimant", "").text("Revenir en haut ↑")),
        )
        .into()
}
