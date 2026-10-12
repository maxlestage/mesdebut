//! Ce qui illustre les questions et les révisions : horloge à aiguilles, billes
//! à compter, formes, pastilles de couleur et étapes d'une activité. Tout est
//! dessiné en SVG ou en CSS : net à toutes les tailles, rien à télécharger.

use std::f64::consts::PI;

use active::prelude::*;

use crate::moteur::question::Illustration;

/// L'illustration d'une question, à la taille d'un énoncé (`grand`) ou d'une
/// ligne de révision.
pub fn illustration(i: &Illustration, grand: bool) -> Node {
    match i {
        Illustration::Billes {
            nombre,
            par_rang,
            couleur_par_rang,
        } => {
            let taille = match (grand, *nombre > 10 || *par_rang == 10) {
                (true, false) => 28,
                (true, true) => 22,
                (false, false) => 16,
                (false, true) => 13,
            };
            billes(*nombre, *par_rang, *couleur_par_rang, taille)
        }
        Illustration::Pastille(hex) => pastille(hex, if grand { 72 } else { 30 }),
        Illustration::Forme(nom) => forme(nom, if grand { 84 } else { 32 }),
        Illustration::Horloge { heures, minutes } => {
            horloge(*heures, *minutes, if grand { 176 } else { 56 })
        }
        Illustration::Etapes {
            etapes: liste,
            trou,
        } => etapes(liste, *trou),
    }
}

fn nombre(x: f64) -> String {
    // deux décimales suffisent dans un repère 100 × 100
    let s = format!("{x:.2}");
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}

/// Horloge analogique. La petite aiguille (foncée, courte) donne les heures, la
/// grande (violette, longue) les minutes.
pub fn horloge(heures: u32, minutes: u32, taille: u32) -> Node {
    let rad = |deg: f64| deg * PI / 180.0;
    let bout =
        |angle: f64, longueur: f64| (50.0 + longueur * angle.sin(), 50.0 - longueur * angle.cos());
    let trait_ = |(x1, y1): (f64, f64), (x2, y2): (f64, f64)| {
        line()
            .attr("x1", nombre(x1))
            .attr("y1", nombre(y1))
            .attr("x2", nombre(x2))
            .attr("y2", nombre(y2))
    };

    let graduations = (0..60).map(|i| {
        let a = rad(f64::from(i) * 6.0);
        let majeure = i % 5 == 0;
        trait_(bout(a, if majeure { 41.0 } else { 44.0 }), bout(a, 46.0))
            .attr("stroke", if majeure { "#8a7fc9" } else { "#d5cff0" })
            .attr("stroke-width", if majeure { "1.6" } else { "0.7" })
    });
    let chiffres = (0..12).map(|i| {
        let (x, y) = bout(rad(f64::from(i) * 30.0), 33.0);
        text_svg()
            .attr("x", nombre(x))
            .attr("y", nombre(y))
            .attr("text-anchor", "middle")
            .attr("dominant-baseline", "central")
            .text(if i == 0 {
                "12".to_string()
            } else {
                i.to_string()
            })
    });
    // l'aiguille des heures avance aussi avec les minutes (3 h 30 → entre 3 et 4)
    let angle_heures = rad(f64::from(heures % 12) * 30.0 + f64::from(minutes) * 0.5);
    let angle_minutes = rad(f64::from(minutes) * 6.0);

    svg()
        .class("horloge")
        .attr("width", taille.to_string())
        .attr("height", taille.to_string())
        .attr("viewBox", "0 0 100 100")
        .attr("role", "img")
        .attr(
            "aria-label",
            format!("Horloge indiquant {heures} heures {minutes} minutes"),
        )
        .child(
            circle()
                .class("cadran")
                .attr("cx", "50")
                .attr("cy", "50")
                .attr("r", "47"),
        )
        .children(graduations)
        .child(g().class("chiffres").children(chiffres))
        .child(
            trait_((50.0, 50.0), bout(angle_heures, 25.0))
                .class("aiguille-heures")
                .attr("stroke-width", "5")
                .attr("stroke-linecap", "round"),
        )
        .child(
            trait_((50.0, 50.0), bout(angle_minutes, 37.0))
                .class("aiguille-minutes")
                .attr("stroke-width", "3")
                .attr("stroke-linecap", "round"),
        )
        .child(
            circle()
                .class("axe")
                .attr("cx", "50")
                .attr("cy", "50")
                .attr("r", "3.2"),
        )
        .into()
}

const COULEURS_BILLES: [&str; 10] = [
    "#e53935", "#1e88e5", "#fdd835", "#43a047", "#fb8c00", "#8e24aa", "#f06292", "#26c6da",
    "#795548", "#5c6bc0",
];

/// Des billes à compter, par rangées : 5 pour les petits nombres, 10 pour voir
/// les dizaines (une couleur par rangée).
pub fn billes(nombre: u32, par_rang: u32, couleur_par_rang: bool, taille: u32) -> Node {
    if nombre == 0 {
        return span().class("billes-vide").text("(aucune bille)").into();
    }
    let par_rang = par_rang.max(1);
    let rangs = (0..nombre).step_by(par_rang as usize).enumerate().map(|(r, debut)| {
        let fin = (debut + par_rang).min(nombre);
        span().class("billes-rang").children((debut..fin).map(move |i| {
            let couleur = COULEURS_BILLES[if couleur_par_rang { r } else { i as usize } % COULEURS_BILLES.len()];
            span().class("bille").style(format!(
                "width:{taille}px;height:{taille}px;\
                 background:radial-gradient(circle at 32% 30%,rgba(255,255,255,.9),rgba(255,255,255,0) 45%),{couleur}"
            ))
        }))
    });
    span()
        .class("billes")
        .attr("role", "img")
        .attr(
            "aria-label",
            format!("{nombre} bille{}", crate::moteur::s(nombre)),
        )
        .children(rangs)
        .into()
}

/// Une forme géométrique, dans un repère 100 × 100.
pub fn forme(nom: &str, taille: u32) -> Node {
    let dessin: Element = match nom {
        "cercle" => circle().attr("cx", "50").attr("cy", "50").attr("r", "42"),
        "carré" => rect()
            .attr("x", "14")
            .attr("y", "14")
            .attr("width", "72")
            .attr("height", "72")
            .attr("rx", "4"),
        "triangle" => polygon().attr("points", "50,10 90,86 10,86"),
        "rectangle" => rect()
            .attr("x", "6")
            .attr("y", "26")
            .attr("width", "88")
            .attr("height", "48")
            .attr("rx", "4"),
        "losange" => polygon().attr("points", "50,6 90,50 50,94 10,50"),
        "ovale" => ellipse()
            .attr("cx", "50")
            .attr("cy", "50")
            .attr("rx", "44")
            .attr("ry", "28"),
        "étoile" => polygon().attr(
            "points",
            "50,6 61,36 93,36 67,56 77,88 50,69 23,88 33,56 7,36 39,36",
        ),
        "cœur" => path().attr(
            "d",
            "M50 84 C50 84 12 58 12 32 C12 18 24 12 33 12 C42 12 48 18 50 24 \
             C52 18 58 12 67 12 C76 12 88 18 88 32 C88 58 50 84 50 84 Z",
        ),
        "hexagone" => polygon().attr("points", "50,6 88,28 88,72 50,94 12,72 12,28"),
        "octogone" => polygon().attr("points", "31,8 69,8 92,31 92,69 69,92 31,92 8,69 8,31"),
        _ => g(),
    };
    svg()
        .class("forme")
        .attr("width", taille.to_string())
        .attr("height", taille.to_string())
        .attr("viewBox", "0 0 100 100")
        .attr("role", "img")
        .attr("aria-label", nom.to_string())
        .child(dessin)
        .into()
}

pub fn pastille(hex: &str, taille: u32) -> Node {
    let mut p = span()
        .class("pastille")
        .attr("role", "img")
        .attr("aria-label", "pastille de couleur")
        .style(format!(
            "width:{taille}px;height:{taille}px;background:{hex}"
        ));
    if hex.eq_ignore_ascii_case("#ffffff") {
        p = p.class("pastille-blanche");
    }
    p.into()
}

/// Les étapes d'une activité ; à la place du trou, la case à trouver.
pub fn etapes(liste: &[String], trou: Option<usize>) -> Node {
    let mut lignes: Vec<Option<&str>> = liste.iter().map(|e| Some(e.as_str())).collect();
    if let Some(t) = trou.filter(|&t| t <= lignes.len()) {
        lignes.insert(t, None);
    }
    ol().class("etapes")
        .children(lignes.into_iter().map(|l| match l {
            Some(e) => li().text(e.to_string()),
            None => li().class("trou").text("❓ à trouver"),
        }))
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horloge_a_trois_heures() {
        let html = horloge(3, 0, 100).render();
        // la petite aiguille pointe vers le 3, la grande vers le 12
        assert!(
            html.contains(r#"<line class="aiguille-heures" x1="50" y1="50" x2="75" y2="50""#),
            "{html}"
        );
        assert!(
            html.contains(r#"<line class="aiguille-minutes" x1="50" y1="50" x2="50" y2="13""#),
            "{html}"
        );
        assert_eq!(html.matches("<text").count(), 12);
    }

    #[test]
    fn billes_par_rangees() {
        let html = billes(23, 10, true, 20).render();
        assert_eq!(html.matches(r#"class="billes-rang""#).count(), 3);
        assert_eq!(html.matches(r#"class="bille""#).count(), 23);
    }

    #[test]
    fn le_trou_prend_sa_place() {
        let html = etapes(&["a".into(), "c".into()], Some(1)).render();
        assert_eq!(
            html,
            r#"<ol class="etapes"><li>a</li><li class="trou">❓ à trouver</li><li>c</li></ol>"#
        );
    }
}
