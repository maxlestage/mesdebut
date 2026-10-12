//! Le quiz : dix questions, la réponse tout de suite, et ce qui est raté revient.

use active::prelude::*;

use super::dessins::illustration;
use super::{ACCUEIL, gros_bouton, plateforme, retour, stock, titre};
use crate::moteur::categories::Categorie;
use crate::moteur::question::Question;
use crate::moteur::questions::{self, bravo};
use crate::moteur::{mot_de_la_fin, notions_maitrisees};

/// Délai avant la question suivante : le temps de lire la correction.
const APRES_JUSTE_MS: i32 = 900;
const APRES_FAUX_MS: i32 = 1800;

pub fn page(c: &'static Categorie, niveau: u8) -> Node {
    // `None` pendant la partie, le score une fois terminée
    let resultat: State<Option<(u32, u32)>> = use_state(None);
    let titre_page = match c.niveaux.iter().find(|n| n.id == niveau) {
        Some(n) if c.niveaux.len() > 1 => format!("Quiz — {} ({})", c.titre, n.libelle),
        _ => format!("Quiz — {}", c.titre),
    };
    section()
        .class("quiz")
        .child(Seo::new().title(titre_page))
        .child(
            h1().class("titre-ecran petit")
                .text(format!("{} {}", c.emoji, c.titre)),
        )
        // « Rejouer » remet le résultat à `None` : une partie toute neuve est construite
        .children_dyn(move || match resultat.get() {
            Some((score, total)) => vec![fin(score, total, resultat)],
            None => vec![partie(c, niveau, resultat)],
        })
        .into()
}

/// Ce que l'écran montre de la réponse donnée.
#[derive(Clone, PartialEq)]
enum Correction {
    Juste(&'static str),
    Faux(String),
}

fn partie(c: &'static Categorie, niveau: u8, resultat: State<Option<(u32, u32)>>) -> Node {
    let etats = stock::memoire(|m| m.etats(c.cle));
    let questions = use_state(questions::quiz(
        c.cle,
        niveau,
        &etats,
        plateforme::maintenant_s(),
    ));
    let index = use_state(0usize);
    let score = use_state(0u32);
    let choisie: State<Option<String>> = use_state(None);
    let correction: State<Option<Correction>> = use_state(None);
    // une question ratée ne revient qu'une fois, même ratée de nouveau
    let reprogrammees: State<Vec<String>> = use_state(Vec::new());

    let total = move || questions.with(Vec::len).max(1);

    let avancer = move || {
        if !index.is_alive() {
            return; // abandonné entre-temps
        }
        let suivante = index.get() + 1;
        if suivante < questions.with(Vec::len) {
            choisie.set(None);
            correction.set(None);
            index.set(suivante);
        } else {
            let (s, t) = (score.get(), questions.with(Vec::len) as u32);
            stock::modifier_memoire(|m| m.termine(c.cle, s, t, plateforme::maintenant_s()));
            resultat.set(Some((s, t)));
        }
    };

    let repondre = move |option: String| {
        if choisie.with(Option::is_some) {
            return;
        }
        let i = index.get();
        let Some(q) = questions.with(|qs| qs.get(i).cloned()) else {
            return;
        };
        let juste = option == q.reponse;
        choisie.set(Some(option));
        // la mémoire note chaque réponse, pour la répétition espacée d'une séance à l'autre
        stock::modifier_memoire(|m| {
            m.note(
                &q.cle_memoire(c.cle),
                q.categorie.unwrap_or(c.cle),
                juste,
                plateforme::maintenant_s(),
            )
        });
        if juste {
            score.update(|s| *s += 1);
            correction.set(Some(Correction::Juste(bravo())));
            plateforme::vibrer(25);
        } else {
            correction.set(Some(Correction::Faux(format!(
                "❌ La bonne réponse était : {}",
                q.reponse
            ))));
            plateforme::vibrer(120);
            if !reprogrammees.with(|r| r.contains(&q.cle)) {
                reprogrammees.update(|r| r.push(q.cle.clone()));
                questions.update(|qs| reprogrammer(qs, &q, i));
            }
        }
        plateforme::plus_tard(if juste { APRES_JUSTE_MS } else { APRES_FAUX_MS }, avancer);
    };

    div()
        .class("partie")
        .child(div().class("barre").attr("aria-hidden", "true").child(
            span().attr_dyn("style", move || {
                format!("width:{}%", index.get() * 100 / total())
            }),
        ))
        .child(
            div()
                .class("quiz-tete")
                .child(
                    span().text_dyn(move || format!("Question {} / {}", index.get() + 1, total())),
                )
                .child(span().text_dyn(move || format!("⭐ {}", score.get()))),
        )
        // reconstruit à chaque question ; la liste, elle, peut s'allonger sans
        // toucher à la question en cours
        .children_dyn(move || {
            let i = index.get();
            let q = untrack(|| questions.with(|qs| qs.get(i).cloned()));
            q.map(|q| vec![question(q, choisie, repondre)])
                .unwrap_or_default()
        })
        .child(
            p().class("correction")
                .attr("aria-live", "polite")
                .class_if("juste", move || {
                    matches!(correction.get(), Some(Correction::Juste(_)))
                })
                .class_if("faux", move || {
                    matches!(correction.get(), Some(Correction::Faux(_)))
                })
                .text_dyn(move || match correction.get() {
                    Some(Correction::Juste(t)) => t.to_string(),
                    Some(Correction::Faux(t)) => t,
                    None => String::new(),
                }),
        )
        .child(retour(ACCUEIL, "← Abandonner"))
        .into()
}

/// Répétition espacée dans la partie : la question ratée revient 3 questions
/// plus loin, puis encore 5 après, ses réponses dans un autre ordre.
fn reprogrammer(questions: &mut Vec<Question>, q: &Question, i: usize) {
    for cible in [i + 3, i + 8] {
        let mut copie = q.clone();
        copie.rebattre();
        let position = cible.min(questions.len());
        questions.insert(position, copie);
    }
}

fn question(
    q: Question,
    choisie: State<Option<String>>,
    repondre: impl Fn(String) + Copy + 'static,
) -> Node {
    let reponse = q.reponse.clone();
    let options = q.options.iter().map(|option| {
        let (a_choisir, juste, cette) = (option.clone(), reponse.clone(), option.clone());
        let est_la_bonne = *option == reponse;
        button()
            .class("option")
            .attr("type", "button")
            .class_if("juste", move || {
                est_la_bonne && choisie.with(Option::is_some)
            })
            .class_if("faux", move || {
                choisie.with(|c| c.as_deref() == Some(cette.as_str())) && cette != juste
            })
            .bool_attr("disabled", move || choisie.with(Option::is_some))
            .on_click(move |_| repondre(a_choisir.clone()))
            .text(option.clone())
    });
    div()
        .class("question")
        .child(h2().class("enonce").text(q.enonce.clone()))
        .child(
            q.illustration
                .as_ref()
                .map(|i| div().class("illustration").child(illustration(i, true))),
        )
        .child(div().class("options").children(options))
        .into()
}

fn fin(score: u32, total: u32, resultat: State<Option<(u32, u32)>>) -> Node {
    let (etoiles, message) = mot_de_la_fin(score, total);
    let maitrisees = stock::memoire(|m| m.resume(plateforme::maintenant_s()).maitrisees);
    div()
        .class("fin")
        .child(titre("Quiz terminé !"))
        .child(
            p().class("etoiles")
                .attr(
                    "aria-label",
                    format!("{} étoiles sur 5", etoiles.chars().count()),
                )
                .text(etoiles),
        )
        .child(
            p().class("score")
                .text(format!("{score} / {total} bonnes réponses")),
        )
        .child(p().class("sous-titre").text(message))
        .child((maitrisees > 0).then(|| {
            p().class("ligne-memoire forte")
                .text(format!("🧠 {} au total", notions_maitrisees(maitrisees)))
        }))
        .child(gros_bouton("🔁 Rejouer", move |_| resultat.set(None)))
        .child(
            a().class("gros-bouton secondaire")
                .href(ACCUEIL)
                .text("🏠 Menu principal"),
        )
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::moteur::alea;

    #[test]
    fn la_question_ratee_revient_deux_fois() {
        alea::amorcer(4);
        let mut qs = questions::quiz("addition", 1, &Default::default(), 0.0);
        let ratee = qs[2].clone();
        reprogrammer(&mut qs, &ratee, 2);
        assert_eq!(qs.len(), 12);
        assert_eq!(qs[5].cle, ratee.cle);
        assert_eq!(qs[10].cle, ratee.cle);
        // en fin de quiz, elle s'ajoute à la suite
        let mut qs2 = qs.clone();
        let derniere = qs2[11].clone();
        reprogrammer(&mut qs2, &derniere, 11);
        assert_eq!(qs2.len(), 14);
        assert_eq!(qs2[12].cle, derniere.cle);
        assert_eq!(qs2[13].cle, derniere.cle);
    }

    #[test]
    fn la_page_du_quiz_montre_la_premiere_question() {
        alea::amorcer(8);
        let c = crate::moteur::categories::categorie("heure").unwrap();
        let html = page(c, 2).render();
        assert!(html.contains("Question 1 / 10"), "{html}");
        assert_eq!(html.matches(r#"class="option""#).count(), 4);
    }
}
