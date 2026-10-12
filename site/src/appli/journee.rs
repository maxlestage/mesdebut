//! « 🗓️ Ma journée » : le planificateur, son chronomètre et la tendance des
//! estimations.

use active::prelude::*;

use super::dessins::etapes;
use super::{gros_bouton, plateforme, retour_menu, sous_titre, stock, titre};
use crate::moteur::journee::{
    DUREES, JourEstime, Journee, Tache, avec_aujourdhui, barres, date_lisible, tendance,
};
use crate::moteur::planifier::{ROUTINES, format_heure, routine};
use crate::moteur::s;

pub fn journee() -> Node {
    let (charge, historique) = stock::charger_journee();
    let date = charge.date.clone();
    let journee = use_state(charge);
    let texte = use_state(String::new());
    let duree = use_state(15u32);
    // la tâche dont on a déroulé les étapes
    let ouverte: State<Option<u32>> = use_state(None);
    // l'heure, rafraîchie chaque seconde tant qu'un chrono tourne
    let tic = use_state(plateforme::maintenant_ms());

    effect(move || journee.with(stock::enregistrer_journee));
    plateforme::chaque_seconde(move || {
        if journee.with(Journee::chrono_en_cours) {
            tic.set(plateforme::maintenant_ms());
        }
    });

    let ajouter = move || {
        let libelle = texte.get();
        let mut ajoutee = false;
        journee.update(|j| ajoutee = j.ajoute(&libelle, duree.get(), None));
        if ajoutee {
            texte.set(String::new());
        }
    };

    section()
        .class("journee")
        .child(Seo::new().title("Ma journée"))
        .child(titre("🗓️ Ma journée"))
        .child(sous_titre(date_lisible(&date)))
        .child(
            label()
                .class("debut")
                .child(span().text("Je commence à"))
                .child(
                    input()
                        .attr("type", "time")
                        .attr_dyn("value", move || {
                            journee.with(|j| format!("{:02}:{:02}", j.debut_h, j.debut_m))
                        })
                        .on("change", move |e| {
                            if let Some((h, m)) = lire_heure(&e.value()) {
                                journee.update(|j| (j.debut_h, j.debut_m) = (h, m));
                            }
                        }),
                ),
        )
        .children_dyn(move || {
            let j = journee.get();
            if j.taches.is_empty() {
                return vec![
                    p().class("vide")
                        .text("Aucune tâche pour l'instant.")
                        .child(br())
                        .text("Ajoute ce que tu veux faire aujourd'hui, dans l'ordre.")
                        .into(),
                ];
            }
            let horaires = j.horaires();
            let n = j.taches.len();
            // dérouler les étapes d'une tâche reconstruit aussi la liste
            let deroulee = ouverte.get();
            vec![
                ol().class("taches")
                    .children(j.taches.iter().enumerate().map(|(i, t)| {
                        let place = Place {
                            horaire: &horaires[i],
                            rang: i,
                            nombre: n,
                        };
                        ligne(
                            t.clone(),
                            place,
                            deroulee == Some(t.id),
                            journee,
                            tic,
                            ouverte,
                        )
                    }))
                    .into(),
                bilan(&j),
            ]
        })
        .child(
            form()
                .class("ajout")
                .on("submit", move |e| {
                    e.prevent_default();
                    ajouter();
                })
                .child(
                    input()
                        .attr("type", "text")
                        .attr("aria-label", "Nouvelle tâche")
                        .attr("placeholder", "Qu'est-ce que tu veux faire ?")
                        .attr("maxlength", "80")
                        .attr_dyn("value", move || texte.get())
                        .on_input(move |e| texte.set(e.value())),
                )
                .child(
                    div()
                        .class("puces")
                        .attr("role", "group")
                        .attr("aria-label", "Durée prévue")
                        .children(DUREES.iter().map(|&d| {
                            button()
                                .class("puce")
                                .attr("type", "button")
                                .class_if("choisie", move || duree.get() == d)
                                .attr_dyn("aria-pressed", move || (duree.get() == d).to_string())
                                .on_click(move |_| duree.set(d))
                                .text(format!("{d} min"))
                        })),
                )
                .child(
                    button()
                        .class("gros-bouton")
                        .attr("type", "submit")
                        .bool_attr("disabled", move || texte.with(|t| t.trim().is_empty()))
                        .text("➕ Ajouter à ma journée"),
                ),
        )
        .child(
            div()
                .class("connues")
                .child(
                    p().class("petit")
                        .text("ou une activité que tu connais déjà :"),
                )
                .child(div().class("puces").children(ROUTINES.iter().map(|r| {
                    button()
                        .class("puce")
                        .attr("type", "button")
                        .on_click(move |_| {
                            journee.update(|j| {
                                j.ajoute(r.libelle, r.duree, Some(r.cle));
                            })
                        })
                        .text(format!("{} {} · {} min", r.emoji, r.libelle, r.duree))
                }))),
        )
        .children_dyn(move || {
            let jours = journee.with(|j| avec_aujourdhui(&historique, j));
            if jours.len() > 1 {
                vec![tendance_vue(&jours, &date)]
            } else {
                Vec::new()
            }
        })
        .children_dyn(move || {
            if journee.with(|j| j.taches.is_empty()) {
                return Vec::new();
            }
            vec![
                gros_bouton("🗑️ Vider la journée", move |_| {
                    if plateforme::confirmer("Vider toute la journée ?") {
                        let date = journee.with(|j| j.date.clone());
                        ouverte.set(None);
                        journee.set(Journee::new(&date));
                    }
                })
                .class("secondaire")
                .into(),
            ]
        })
        .child(retour_menu())
        .into()
}

/// « 09:30 » → (9, 30)
fn lire_heure(valeur: &str) -> Option<(u32, u32)> {
    let (h, m) = valeur.split_once(':')?;
    let (h, m): (u32, u32) = (h.parse().ok()?, m.get(..2)?.parse().ok()?);
    (h < 24 && m < 60).then_some((h, m))
}

/// Où se trouve une tâche dans la journée.
struct Place<'a> {
    horaire: &'a str,
    rang: usize,
    nombre: usize,
}

fn ligne(
    t: Tache,
    place: Place<'_>,
    deroulee: bool,
    journee: State<Journee>,
    tic: State<f64>,
    ouverte: State<Option<u32>>,
) -> Element {
    let id = t.id;
    let (i, n) = (place.rang, place.nombre);
    let duree = if t.demarre_a.is_some() {
        let tache = t.clone();
        span()
            .class("duree en-cours")
            .text_dyn(move || format!("⏱ {} min", tache.ecoulees(tic.get())))
    } else if let Some(reel) = t.reel {
        let juste = t.estimation_juste() == Some(true);
        span()
            .class(if juste { "duree juste" } else { "duree ecart" })
            .text(format!("{} → {reel} min", t.duree))
    } else {
        span().class("duree").text(format!("{} min", t.duree))
    };

    let mini = |texte: &'static str, etiquette: String| {
        button()
            .class("mini")
            .attr("type", "button")
            .attr("aria-label", etiquette)
            .text(texte)
    };
    let mut actions = div().class("actions");
    if !t.fait {
        actions = actions.child(if t.demarre_a.is_some() {
            mini(
                "■ arrêter",
                format!("Arrêter le chrono de « {} »", t.libelle),
            )
            .class("en-cours")
            .on_click(move |_| journee.update(|j| j.arrete(id, plateforme::maintenant_ms())))
        } else {
            mini(
                "▶ démarrer",
                format!("Démarrer le chrono de « {} »", t.libelle),
            )
            .on_click(move |_| journee.update(|j| j.demarre(id, plateforme::maintenant_ms())))
        });
    }
    let r = t.routine.as_deref().and_then(routine);
    if r.is_some() {
        actions = actions.child(
            mini(
                if deroulee {
                    "▾ étapes"
                } else {
                    "▸ étapes"
                },
                format!("Étapes de « {} »", t.libelle),
            )
            .attr("aria-expanded", deroulee.to_string())
            .on_click(move |_| ouverte.set(if deroulee { None } else { Some(id) })),
        );
    }
    actions = actions
        .child(
            mini("↑", format!("Monter « {} »", t.libelle))
                .bool_attr("disabled", move || i == 0)
                .on_click(move |_| journee.update(|j| j.deplace(id, -1))),
        )
        .child(
            mini("↓", format!("Descendre « {} »", t.libelle))
                .bool_attr("disabled", move || i + 1 >= n)
                .on_click(move |_| journee.update(|j| j.deplace(id, 1))),
        )
        .child(
            mini("✕", format!("Retirer « {} »", t.libelle))
                .class("danger")
                .on_click(move |_| journee.update(|j| j.retire(id))),
        );

    let mut case = input()
        .attr("type", "checkbox")
        .attr("aria-label", format!("« {} » faite", t.libelle))
        .on("change", move |_| {
            plateforme::vibrer(25);
            journee.update(|j| j.bascule(id, plateforme::maintenant_ms()))
        });
    if t.fait {
        case = case.attr("checked", "");
    }

    let mut ligne = li().class("tache");
    if t.fait {
        ligne = ligne.class("faite");
    }
    ligne
        .child(
            div()
                .class("tache-tete")
                .child(span().class("horaire").text(place.horaire.to_string()))
                .child(case)
                .child(span().class("libelle").text(t.libelle.clone()))
                .child(duree),
        )
        .child(actions)
        .child(r.filter(|_| deroulee).map(|r| {
            let liste: Vec<String> = r.etapes.iter().map(|e| e.to_string()).collect();
            etapes(&liste, None)
        }))
}

fn bilan(j: &Journee) -> Node {
    let (fh, fm) = j.fin();
    let faites = j.faites() as u32;
    let n = j.taches.len() as u32;
    let tout = faites == n;
    div()
        .class("bilan")
        .child(
            p().text("Fin prévue à ")
                .child(strong().text(format_heure(fh, fm)))
                .text(format!(" · {} min au total", j.total())),
        )
        .child({
            let mut ligne = p().text(format!(
                "{faites} faite{} sur {n}{}",
                s(faites),
                if tout {
                    " — journée terminée ! 🎉"
                } else {
                    ""
                }
            ));
            if tout {
                ligne = ligne.class("bravo");
            }
            ligne
        })
        .child(
            j.bilan_estimation()
                .map(|b| p().class("estimation").text(b)),
        )
        .into()
}

/// Les dix derniers jours : l'écart entre le prévu et le réel se resserre-t-il ?
fn tendance_vue(jours: &[JourEstime], aujourdhui: &str) -> Node {
    div()
        .class("tendance")
        .child(h2().text("📈 Mes estimations"))
        .child(
            ul().children(barres(jours, aujourdhui).into_iter().map(|b| {
                li().class(if b.juste { "juste" } else { "ecart" })
                    .child(span().class("jour").text(b.libelle))
                    .child(
                        span()
                            .class("barre")
                            .child(span().style(format!("width:{:.0}%", b.part * 100.0))),
                    )
                    .child(span().class("valeur").text(b.valeur))
            })),
        )
        .child(p().class("estimation").text(tendance(jours)))
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lire_une_heure_de_formulaire() {
        assert_eq!(lire_heure("09:30"), Some((9, 30)));
        assert_eq!(lire_heure("23:59:00"), Some((23, 59)));
        assert_eq!(lire_heure("24:00"), None);
        assert_eq!(lire_heure(""), None);
    }

    #[test]
    fn l_ecran_se_rend_vide_puis_rempli() {
        let html = journee().render();
        assert!(html.contains("Aucune tâche"));
        let mut j = Journee::new(&plateforme::aujourdhui());
        j.ajoute("Se faire un café", 5, Some("cafe"));
        stock::enregistrer_journee(&j);
        let html = journee().render();
        assert!(html.contains("Se faire un café"));
        assert!(html.contains("▸ étapes"));
        assert!(html.contains("Fin prévue à "));
    }
}
