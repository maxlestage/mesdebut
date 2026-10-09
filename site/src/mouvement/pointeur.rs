//! Ce qui suit la souris (jamais au toucher) : le curseur à anneau, les
//! boutons aimantés, les cartes qui s'inclinent et la lueur du tarif.

use std::cell::Cell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use web_sys::{Element, EventTarget, HtmlElement, PointerEvent};

use super::Etat;
use crate::outils::{self, Boucle, classe, ecouter, style};

const SURVOLABLES: &str = "a, button, [data-incline]";

pub fn installer(etat: Rc<Etat>) {
    curseur(etat);
    for el in outils::tous("[data-aimant]") {
        aimant(el);
    }
    for el in outils::tous("[data-incline], [data-lueur]") {
        incliner(el);
    }
}

fn curseur(etat: Rc<Etat>) {
    let Some(anneau) = outils::un(".curseur") else {
        return;
    };
    let Some(point) = outils::un(".curseur-point") else {
        return;
    };
    let position = Rc::new(Cell::new((-100.0f64, -100.0f64)));

    let boucle = {
        let (anneau, etat, position) = (anneau.clone(), etat.clone(), position.clone());
        Boucle::new(move |_| {
            let (cx, cy) = etat.pointeur.get();
            let (x, y) = position.get();
            let (nx, ny) = (x + (cx - x) * 0.2, y + (cy - y) * 0.2);
            position.set((nx, ny));
            style(&anneau, "translate", &format!("{nx:.1}px {ny:.1}px"));
            (cx - nx).abs() > 0.1 || (cy - ny).abs() > 0.1
        })
    };

    let doc: EventTarget = outils::document().into();
    {
        let (anneau, boucle) = (anneau.clone(), boucle.clone());
        ecouter::<PointerEvent>(&doc, "pointermove", move |e| {
            if e.pointer_type() != "mouse" {
                return;
            }
            let (x, y) = (e.client_x() as f64, e.client_y() as f64);
            if etat.pointeur.get().0 < -1e3 {
                position.set((x, y)); // premier mouvement : pas de glissade depuis le coin
            }
            etat.pointeur.set((x, y));
            style(&point, "translate", &format!("{x}px {y}px"));
            classe(&anneau, "visible", true);
            boucle.relancer();
        });
    }
    {
        let anneau = anneau.clone();
        ecouter::<PointerEvent>(&doc, "pointerover", move |e| {
            let survol = e
                .target()
                .and_then(|t| t.dyn_into::<Element>().ok())
                .and_then(|el| el.closest(SURVOLABLES).ok().flatten())
                .is_some();
            classe(&anneau, "survol", survol);
        });
    }
    let html: EventTarget = outils::document()
        .document_element()
        .expect("<html>")
        .into();
    ecouter::<PointerEvent>(&html, "pointerleave", move |_| {
        classe(&anneau, "visible", false)
    });
}

/// Le bouton suit le pointeur d'un tiers de l'écart, puis revient en place.
fn aimant(el: HtmlElement) {
    let cible: EventTarget = el.clone().into();
    {
        let el = el.clone();
        ecouter::<PointerEvent>(&cible, "pointermove", move |e| {
            let r = el.get_bounding_client_rect();
            let dx = e.client_x() as f64 - (r.left() + r.width() / 2.0);
            let dy = e.client_y() as f64 - (r.top() + r.height() / 2.0);
            style(
                &el,
                "translate",
                &format!("{:.1}px {:.1}px", dx * 0.3, dy * 0.4),
            );
        });
    }
    ecouter::<PointerEvent>(&cible, "pointerleave", move |_| {
        style(&el, "translate", "0 0")
    });
}

/// Inclinaison en 3D et reflet qui suit le pointeur, par variables CSS.
fn incliner(el: HtmlElement) {
    let cible: EventTarget = el.clone().into();
    {
        let el = el.clone();
        ecouter::<PointerEvent>(&cible, "pointermove", move |e| {
            let r = el.get_bounding_client_rect();
            let px = ((e.client_x() as f64 - r.left()) / r.width()).clamp(0.0, 1.0);
            let py = ((e.client_y() as f64 - r.top()) / r.height()).clamp(0.0, 1.0);
            style(&el, "--rx", &format!("{:.2}deg", (0.5 - py) * 14.0));
            style(&el, "--ry", &format!("{:.2}deg", (px - 0.5) * 16.0));
            style(&el, "--mx", &format!("{:.1}%", px * 100.0));
            style(&el, "--my", &format!("{:.1}%", py * 100.0));
        });
    }
    ecouter::<PointerEvent>(&cible, "pointerleave", move |_| {
        style(&el, "--rx", "0deg");
        style(&el, "--ry", "0deg");
    });
}
