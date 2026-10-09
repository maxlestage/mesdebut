//! La poussière de l'ouverture. Des grains flottent, puis se rassemblent en
//! toque — la marque de ReNeuro — à mesure que l'on défile ; une fois la toque
//! formée, un balayage de lumière la traverse. Le pointeur écarte les grains.
//!
//! Pas de séquence d'images : les cibles des grains sont tirées de la géométrie
//! même de l'icône (les tracés d'`icon.svg`), rasterisée une fois, hors écran.

use std::cell::RefCell;
use std::f64::consts::TAU;
use std::rc::Rc;

use js_sys::Math::random;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Path2d};

use super::Etat;
use crate::outils::{self, Boucle, ecouter, fenetre, lisser, maintenant};

/// Cadre de la toque dans icon.svg (viewBox 1024).
const CADRE: (f64, f64, f64, f64) = (180.0, 290.0, 664.0, 404.0);
/// Résolution de l'échantillonnage : un pixel hors écran par 1/ECHELLE unité SVG.
const ECHELLE: f64 = 0.4;

/// Les tracés d'icon.svg, dans l'ordre du dessin, avec la teinte qu'ils
/// prennent ici. Un trait épais s'écrit avec sa largeur ; un aplat avec 0.
const TRACES: &[(&str, usize, f64)] = &[
    (
        "M 372 470 L 372 592 Q 372 686 512 686 Q 652 686 652 592 L 652 470 Z",
        0,
        0.0,
    ),
    ("M 512 296 L 836 436 L 512 576 L 188 436 Z", 1, 0.0),
    (
        "M 188 436 L 512 576 L 836 436 L 836 452 L 512 592 L 188 452 Z",
        2,
        0.0,
    ),
    (
        "M 486 436 A 26 14 0 1 0 538 436 A 26 14 0 1 0 486 436 Z",
        1,
        0.0,
    ),
    ("M 512 436 Q 690 424 792 470", 3, 20.0),
    (
        "M 766 502 A 26 32 0 1 0 818 502 A 26 32 0 1 0 766 502 Z",
        3,
        0.0,
    ),
    (
        "M 770 524 L 762 626 M 785 528 L 782 636 M 800 528 L 806 634 M 814 522 L 826 620",
        3,
        17.0,
    ),
];

/// Teintes de rendu, sur fond sombre : le corps, le plateau, sa tranche, l'or.
/// Les couleurs de pochoir servent seulement à reconnaître la teinte au relevé.
const TEINTES: [&str; 4] = ["#8f86d8", "#efeaff", "#6a62b8", "#ffc93c"];
const POCHOIRS: [&str; 4] = ["#ff0000", "#00ff00", "#0000ff", "#ffff00"];

struct Grain {
    /// Position libre, en fraction de la scène.
    lx: f64,
    ly: f64,
    /// Place dans la toque, en fraction du cadre.
    tx: f64,
    ty: f64,
    retard: f64,
    phase: f64,
    vitesse: f64,
    taille: f64,
    /// Écart dû au pointeur, lissé.
    ox: f64,
    oy: f64,
}

struct Scene {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    /// Les grains rangés par teinte, pour ne changer de couleur que quatre fois.
    grains: [Vec<Grain>; 4],
    largeur: f64,
    hauteur: f64,
}

pub fn installer(etat: Rc<Etat>, calme: bool) -> Boucle {
    let canvas =
        outils::un("canvas.poussiere").and_then(|c| c.dyn_into::<HtmlCanvasElement>().ok());
    let scene = canvas.and_then(|canvas| {
        let ctx = canvas
            .get_context("2d")
            .ok()??
            .dyn_into::<CanvasRenderingContext2d>()
            .ok()?;
        let places = releve()?;
        let nombre = if outils::largeur_ecran() < 700.0 {
            900
        } else {
            1700
        };
        let mut grains: [Vec<Grain>; 4] = Default::default();
        for _ in 0..nombre {
            let (tx, ty, teinte) = places[(random() * places.len() as f64) as usize];
            grains[teinte].push(Grain {
                lx: random(),
                ly: random(),
                tx,
                ty,
                retard: random(),
                phase: random() * TAU,
                vitesse: 0.4 + random() * 0.8,
                taille: 0.8 + random() * 1.4,
                ox: 0.0,
                oy: 0.0,
            });
        }
        Some(Rc::new(RefCell::new(Scene {
            canvas,
            ctx,
            grains,
            largeur: 0.0,
            hauteur: 0.0,
        })))
    });

    let Some(scene) = scene else {
        return Boucle::new(|_| false);
    };
    dimensionner(&mut scene.borrow_mut());

    let boucle = {
        let (scene, etat) = (scene.clone(), etat.clone());
        Boucle::new(move |_| {
            let mut s = scene.borrow_mut();
            let visible = s.canvas.get_bounding_client_rect().bottom() > 0.0;
            if !visible {
                return false;
            }
            // Sous « réduire les animations » : la toque formée, immobile, une image.
            let ouverture = if calme { 1.0 } else { etat.ouverture.get() };
            dessiner(&mut s, &etat, ouverture, calme);
            !calme && !outils::document().hidden()
        })
    };

    let fen: web_sys::EventTarget = fenetre().into();
    {
        let (scene, boucle) = (scene.clone(), boucle.clone());
        ecouter::<web_sys::Event>(&fen, "resize", move |_| {
            dimensionner(&mut scene.borrow_mut());
            boucle.relancer();
        });
    }
    {
        let boucle = boucle.clone();
        let doc: web_sys::EventTarget = outils::document().into();
        ecouter::<web_sys::Event>(&doc, "visibilitychange", move |_| boucle.relancer());
    }
    if !calme {
        // Le pointeur réveille la scène, même sans défilement.
        let boucle = boucle.clone();
        let doc: web_sys::EventTarget = outils::document().into();
        ecouter::<web_sys::Event>(&doc, "pointermove", move |_| boucle.relancer());
    }
    boucle.relancer();
    boucle
}

/// Rasterise la toque hors écran et relève chaque pixel couvert, avec sa teinte.
fn releve() -> Option<Vec<(f64, f64, usize)>> {
    let (x0, y0, l, h) = CADRE;
    let (lp, hp) = ((l * ECHELLE).ceil() as u32, (h * ECHELLE).ceil() as u32);
    let hors: HtmlCanvasElement = outils::document()
        .create_element("canvas")
        .ok()?
        .dyn_into()
        .ok()?;
    hors.set_width(lp);
    hors.set_height(hp);
    let ctx: CanvasRenderingContext2d = hors.get_context("2d").ok()??.dyn_into().ok()?;
    ctx.scale(ECHELLE, ECHELLE).ok()?;
    ctx.translate(-x0, -y0).ok()?;
    ctx.set_line_cap("round");
    for (trace, teinte, epaisseur) in TRACES {
        let chemin = Path2d::new_with_path_string(trace).ok()?;
        if *epaisseur > 0.0 {
            ctx.set_stroke_style_str(POCHOIRS[*teinte]);
            ctx.set_line_width(*epaisseur);
            ctx.stroke_with_path(&chemin);
        } else {
            ctx.set_fill_style_str(POCHOIRS[*teinte]);
            ctx.fill_with_path_2d(&chemin);
        }
    }
    let donnees = ctx
        .get_image_data(0.0, 0.0, lp as f64, hp as f64)
        .ok()?
        .data();
    let mut places = Vec::new();
    for y in 0..hp {
        for x in 0..lp {
            let i = ((y * lp + x) * 4) as usize;
            let (r, g, b, a) = (donnees[i], donnees[i + 1], donnees[i + 2], donnees[i + 3]);
            if a < 160 {
                continue;
            }
            let teinte = match (r > 127, g > 127, b > 127) {
                (true, true, _) => 3,
                (false, true, _) => 1,
                (_, _, true) => 2,
                (true, false, false) => 0,
                _ => continue,
            };
            places.push((
                (x as f64 + 0.5) / lp as f64,
                (y as f64 + 0.5) / hp as f64,
                teinte,
            ));
        }
    }
    (!places.is_empty()).then_some(places)
}

fn dimensionner(s: &mut Scene) {
    let dpr = fenetre().device_pixel_ratio().clamp(1.0, 2.0);
    let r = s.canvas.get_bounding_client_rect();
    s.largeur = r.width().max(1.0);
    s.hauteur = r.height().max(1.0);
    s.canvas.set_width((s.largeur * dpr) as u32);
    s.canvas.set_height((s.hauteur * dpr) as u32);
    let _ = s.ctx.set_transform(dpr, 0.0, 0.0, dpr, 0.0, 0.0);
}

fn dessiner(s: &mut Scene, etat: &Etat, ouverture: f64, fige: bool) {
    let (w, h) = (s.largeur, s.hauteur);
    let t = if fige { 0.0 } else { maintenant() };

    // Où se forme la toque : à droite sur grand écran, en haut sur téléphone.
    let large = w >= 820.0;
    let lt = if large {
        (w * 0.4).min(580.0)
    } else {
        (w * 0.8).min(440.0)
    };
    let ht = lt * CADRE.3 / CADRE.2;
    let (cx, cy) = if large {
        (w * 0.73, h * 0.42)
    } else {
        (w * 0.5, h * 0.27)
    };

    // Déjà un nuage à l'arrivée ; toque formée aux deux tiers de l'ouverture.
    let forme = 0.12 + 0.88 * lisser(ouverture / 0.62);
    let lumiere = lisser((forme - 0.9) / 0.1);
    // Le balayage traverse la toque toutes les 3,2 s, avec un temps de repos.
    let balayage = cx - lt * 0.7 + ((t / 3200.0) % 1.0) * lt * 2.0;

    let r = s.canvas.get_bounding_client_rect();
    let (px, py) = etat.pointeur.get();
    let (px, py) = (px - r.left(), py - r.top());

    s.ctx.clear_rect(0.0, 0.0, w, h);
    for (teinte, grains) in s.grains.iter_mut().enumerate() {
        s.ctx.set_fill_style_str(TEINTES[teinte]);
        for g in grains.iter_mut() {
            let f = ((forme - g.retard * 0.3) / 0.7).clamp(0.0, 1.0);
            let e = if f < 0.5 {
                4.0 * f * f * f
            } else {
                1.0 - (-2.0 * f + 2.0).powi(3) / 2.0
            };

            // Libre, le grain dérive lentement vers le haut en ondulant.
            let monte = (g.ly - t * 0.000_012 * g.vitesse).rem_euclid(1.0);
            let lx = g.lx * w + (t * 0.0004 * g.vitesse + g.phase).sin() * 24.0;
            let ly = monte * h + (t * 0.0003 * g.vitesse + g.phase).cos() * 18.0;
            // En place, il frémit à peine.
            let tx = cx + (g.tx - 0.5) * lt + (t * 0.002 + g.phase).sin() * 0.6;
            let ty = cy + (g.ty - 0.5) * ht + (t * 0.0017 + g.phase).cos() * 0.6;

            let mut x = lx + (tx - lx) * e;
            let mut y = ly + (ty - ly) * e;

            // Le pointeur écarte les grains, qui reviennent ensuite.
            let (dx, dy) = (x - px, y - py);
            let d = (dx * dx + dy * dy).sqrt().max(1.0);
            let poussee = (1.0 - d / 110.0).max(0.0).powi(2) * 38.0;
            g.ox += (dx / d * poussee - g.ox) * 0.12;
            g.oy += (dy / d * poussee - g.oy) * 0.12;
            x += g.ox;
            y += g.oy;

            let eclat = lumiere * (-((x - balayage) / (lt * 0.07)).powi(2)).exp();
            let alpha = (0.28 + 0.6 * e + eclat * 0.6).min(1.0);
            let taille = g.taille * (1.0 + eclat * 0.9);
            s.ctx.set_global_alpha(alpha);
            s.ctx
                .fill_rect(x - taille / 2.0, y - taille / 2.0, taille, taille);
        }
    }
    s.ctx.set_global_alpha(1.0);
}
