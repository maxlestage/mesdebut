//! Les écrans simples : le menu, un thème, le choix du niveau, la révision et
//! les progrès.

use active::prelude::*;

use super::dessins::illustration;
use super::{
    ACCUEIL, adresse_quiz_ou_niveaux, adresse_theme, gros_lien, plateforme, retour, retour_menu,
    sous_titre, stock, titre,
};
use crate::moteur::categories::{CATEGORIES, Categorie};
use crate::moteur::memoire::Memoire;
use crate::moteur::question::Illustration;
use crate::moteur::revision::{Fiche, Revision, revision as contenu_revision};
use crate::moteur::{notions_maitrisees, s};

pub fn menu() -> Node {
    let resume = stock::memoire(|m| m.resume(plateforme::maintenant_s()));
    let ligne_memoire = (resume.repondues > 0).then(|| {
        let mut texte = format!("🧠 {}", notions_maitrisees(resume.maitrisees));
        if resume.a_reviser > 0 {
            texte.push_str(&format!(" · {} à réviser", resume.a_reviser));
        }
        p().class("ligne-memoire").text(texte)
    });

    section()
        .class("menu")
        .child(Seo::new().title("Choisis ton thème"))
        .child(titre("🎓 ReNeuro"))
        .child(sous_titre("Choisis ce que tu veux apprendre !"))
        .child(ligne_memoire)
        .child(
            nav()
                .attr("aria-label", "Thèmes")
                .class("grille-themes")
                .children(CATEGORIES.iter().map(|c| {
                    a().class("bouton-theme")
                        .href(adresse_theme(c))
                        .style(format!("--a:{};--b:{}", c.degrade.0, c.degrade.1))
                        .child(
                            span()
                                .class("emoji")
                                .attr("aria-hidden", "true")
                                .text(c.emoji),
                        )
                        .child(span().class("nom").text(c.libelle))
                })),
        )
        .child(
            div()
                .class("pilules")
                .child((resume.repondues > 0).then(|| retour("/app/progres", "📊 Mes progrès")))
                .child(retour("/app/journee", "🗓️ Ma journée")),
        )
        .child(
            footer()
                .class("credits")
                .child(p().text("Conçu et développé par Maxime Nathan Lestage"))
                .child(
                    p().text("Fait avec ")
                        .child(a().href(crate::ACTIVE).text("Active"))
                        .text(", en Rust et WebAssembly"),
                ),
        )
        .into()
}

/// Un thème qui se révise : réviser d'abord, ou passer au quiz.
pub fn theme(c: &'static Categorie) -> Node {
    section()
        .child(Seo::new().title(c.titre))
        .child(titre(format!("{} {}", c.emoji, c.titre)))
        .child(
            (c.revision)
                .then(|| gros_lien(format!("/app/{}/reviser", c.cle), "📖 Réviser d'abord")),
        )
        .child(gros_lien(adresse_quiz_ou_niveaux(c), "🎯 Faire le quiz"))
        // troisième choix, propre à « Planifier » : le planificateur de journée
        .child((c.cle == "planifier").then(|| gros_lien("/app/journee", "🗓️ Ma journée")))
        .child(retour_menu())
        .into()
}

pub fn niveaux(c: &'static Categorie) -> Node {
    section()
        .child(Seo::new().title(format!("{} — niveau", c.titre)))
        .child(titre(format!("{} {}", c.emoji, c.titre)))
        .child(sous_titre("Choisis ton niveau"))
        .children(c.niveaux.iter().map(|n| {
            gros_lien(
                format!("/app/{}/quiz/{}", c.cle, n.id),
                format!("{} {}", n.emoji, n.libelle),
            )
        }))
        .child(retour_menu())
        .into()
}

/// Une ligne de révision : une image (ou un numéro), le libellé, le détail.
fn fiche(f: &Fiche, rang: usize) -> Element {
    let (tete, dessous) = match &f.image {
        Some(i @ Illustration::Billes { .. }) => (None, Some(illustration(i, false))),
        Some(i) => (Some(illustration(i, false)), None),
        None => (None, None),
    };
    let tete = tete.unwrap_or_else(|| {
        span()
            .class("numero")
            .text(f.numero.unwrap_or(rang as u32 + 1).to_string())
            .into()
    });
    li().class("fiche")
        .child(span().class("fiche-tete").child(tete))
        .child(
            div()
                .class("fiche-texte")
                .child(span().class("fiche-libelle").text(f.libelle.clone()))
                .child(
                    f.detail
                        .clone()
                        .map(|d| span().class("fiche-detail").text(d)),
                )
                .child(dessous),
        )
}

pub fn revision(c: &'static Categorie) -> Node {
    let Some(contenu) = contenu_revision(c.cle) else {
        return introuvable();
    };
    let corps: Vec<Node> = match contenu {
        Revision::Liste {
            titre: t,
            intro,
            fiches,
        } => vec![
            titre(t).into(),
            intro.map(sous_titre).into(),
            ol().class("fiches")
                .children(fiches.iter().enumerate().map(|(i, f)| fiche(f, i)))
                .into(),
        ],
        Revision::Lettres { titre: t, lettres } => vec![
            titre(t).into(),
            ul().class("lettres")
                .children(lettres.iter().map(|l| li().text(*l)))
                .into(),
        ],
        // des groupes qui se déroulent : <details>, le premier ouvert
        Revision::Accordeon {
            titre: t,
            intro,
            groupes,
        } => vec![
            titre(t).into(),
            intro.map(sous_titre).into(),
            div()
                .class("accordeon")
                .children(groupes.iter().enumerate().map(|(i, g)| {
                    let groupe = details().child(summary().text(g.libelle.clone())).child(
                        ol().class("fiches")
                            .children(g.fiches.iter().enumerate().map(|(j, f)| fiche(f, j))),
                    );
                    if i == 0 {
                        groupe.attr("open", "")
                    } else {
                        groupe
                    }
                }))
                .into(),
        ],
    };
    section()
        .child(Seo::new().title(format!("Réviser — {}", c.titre)))
        .children(corps)
        .child(gros_lien(
            adresse_quiz_ou_niveaux(c),
            "🎯 Je suis prêt·e, quiz !",
        ))
        .child(retour_menu())
        .into()
}

pub fn progres() -> Node {
    // relu après une remise à zéro
    let version = use_state(0u32);
    section()
        .child(Seo::new().title("Mes progrès"))
        .child(titre("📊 Mes progrès"))
        .children_dyn(move || {
            version.get();
            stock::memoire(|m| bilan(m, version))
        })
        .child(retour_menu())
        .into()
}

fn bilan(m: &Memoire, version: State<u32>) -> Vec<Node> {
    let r = m.resume(plateforme::maintenant_s());
    if r.repondues == 0 {
        return vec![sous_titre("Fais un quiz pour voir tes progrès apparaître ici ! 🌱").into()];
    }
    let stats = m.par_theme();
    // les thèmes réellement pratiqués, dans l'ordre du menu
    let lignes = CATEGORIES.iter().filter_map(|c| {
        let st = stats.get(c.cle).filter(|st| st.vues > 0)?;
        let pct = st.reussite();
        Some(
            li().class("stat")
                .child(
                    span()
                        .class("emoji")
                        .attr("aria-hidden", "true")
                        .text(c.emoji),
                )
                .child(
                    div()
                        .class("stat-corps")
                        .child(
                            div()
                                .class("stat-tete")
                                .child(span().class("stat-nom").text(c.libelle))
                                .child(span().class("stat-pct").text(format!("{pct} %"))),
                        )
                        .child(
                            div()
                                .class("barre")
                                .attr("role", "progressbar")
                                .attr("aria-valuenow", pct.to_string())
                                .attr("aria-valuemin", "0")
                                .attr("aria-valuemax", "100")
                                .attr("aria-label", format!("Réussite en {}", c.libelle))
                                .child(span().style(format!("width:{pct}%"))),
                        )
                        .child(span().class("stat-detail").text(format!(
                            "{} vue{} · {} maîtrisée{}",
                            st.vues,
                            s(st.vues),
                            st.maitrisees,
                            s(st.maitrisees)
                        ))),
                ),
        )
    });
    vec![
        p().class("ligne-memoire forte")
            .text(format!(
                "🧠 {} · {}/{} réussies",
                notions_maitrisees(r.maitrisees),
                r.justes,
                r.repondues
            ))
            .into(),
        ul().class("stats").children(lignes).into(),
        button()
            .class("lien-alerte")
            .attr("type", "button")
            .on_click(move |_| {
                if plateforme::confirmer("Effacer toute la progression ?") {
                    stock::modifier_memoire(|m| *m = Memoire::default());
                    version.update(|v| *v += 1);
                }
            })
            .text("♻︎ Réinitialiser mes progrès")
            .into(),
    ]
}

pub fn introuvable() -> Node {
    section()
        .child(Seo::new().title("Page introuvable").noindex(true))
        .child(titre("🤔 Page introuvable"))
        .child(sous_titre("Cette page n'existe pas, ou plus."))
        .child(gros_lien(ACCUEIL, "🏠 Menu principal"))
        .into()
}
