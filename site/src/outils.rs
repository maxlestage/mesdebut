//! Petits accès au navigateur, partagés par les sections et les animations.

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{AddEventListenerOptions, Document, Element, EventTarget, HtmlElement, Window};

pub fn fenetre() -> Window {
    web_sys::window().expect("pas de fenêtre")
}

pub fn document() -> Document {
    fenetre().document().expect("pas de document")
}

pub fn correspond(requete: &str) -> bool {
    fenetre()
        .match_media(requete)
        .ok()
        .flatten()
        .is_some_and(|m| m.matches())
}

pub fn mouvement_reduit() -> bool {
    correspond("(prefers-reduced-motion: reduce)")
}

/// Une souris ou un pavé : le curseur, l'aimant et l'inclinaison n'ont de sens
/// qu'avec un pointeur qui survole.
pub fn pointeur_fin() -> bool {
    correspond("(hover: hover) and (pointer: fine)")
}

pub fn maintenant() -> f64 {
    fenetre().performance().map_or(0.0, |p| p.now())
}

pub fn tous(selecteur: &str) -> Vec<HtmlElement> {
    let Ok(liste) = document().query_selector_all(selecteur) else {
        return Vec::new();
    };
    (0..liste.length())
        .filter_map(|i| liste.item(i))
        .filter_map(|n| n.dyn_into::<HtmlElement>().ok())
        .collect()
}

pub fn un(selecteur: &str) -> Option<HtmlElement> {
    document()
        .query_selector(selecteur)
        .ok()
        .flatten()
        .and_then(|e| e.dyn_into::<HtmlElement>().ok())
}

pub fn style(el: &HtmlElement, propriete: &str, valeur: &str) {
    let _ = el.style().set_property(propriete, valeur);
}

pub fn classe(el: &Element, nom: &str, active: bool) {
    let _ = el.class_list().toggle_with_force(nom, active);
}

/// Écoute un évènement pour toute la vie de la page. Le site n'a qu'une page et
/// ne démonte rien : la fermeture peut être abandonnée au ramasse-miettes JS.
pub fn ecouter<E: JsCast + 'static>(
    cible: &EventTarget,
    evenement: &str,
    f: impl FnMut(E) + 'static,
) {
    let mut f = f;
    let rappel = Closure::<dyn FnMut(web_sys::Event)>::new(move |e: web_sys::Event| {
        if let Ok(e) = e.dyn_into::<E>() {
            f(e)
        }
    });
    let options = AddEventListenerOptions::new();
    options.set_passive(true);
    let _ = cible.add_event_listener_with_callback_and_add_event_listener_options(
        evenement,
        rappel.as_ref().unchecked_ref(),
        &options,
    );
    rappel.forget();
}

pub fn plus_tard(ms: i32, f: impl FnOnce() + 'static) {
    let rappel = Closure::once_into_js(f);
    let _ =
        fenetre().set_timeout_with_callback_and_timeout_and_arguments_0(rappel.unchecked_ref(), ms);
}

/// Une boucle d'animation qui s'arrête d'elle-même : le rappel reçoit l'heure
/// et renvoie `true` tant qu'il veut une image de plus. `relancer` la redémarre
/// si elle s'était arrêtée, sans jamais en lancer deux.
#[derive(Clone)]
pub struct Boucle {
    interne: Rc<BoucleInterne>,
}

type RappelImage = Closure<dyn FnMut(f64)>;

struct BoucleInterne {
    rappel: RefCell<Option<RappelImage>>,
    en_cours: std::cell::Cell<bool>,
}

impl Boucle {
    pub fn new(mut image: impl FnMut(f64) -> bool + 'static) -> Self {
        let interne = Rc::new(BoucleInterne {
            rappel: RefCell::new(None),
            en_cours: std::cell::Cell::new(false),
        });
        let faible = Rc::downgrade(&interne);
        *interne.rappel.borrow_mut() = Some(Closure::new(move |t: f64| {
            let Some(interne) = faible.upgrade() else {
                return;
            };
            if image(t) {
                demander(&interne);
            } else {
                interne.en_cours.set(false);
            }
        }));
        // La boucle vit autant que la page.
        std::mem::forget(interne.clone());
        Boucle { interne }
    }

    pub fn relancer(&self) {
        if !self.interne.en_cours.replace(true) {
            demander(&self.interne);
        }
    }
}

fn demander(interne: &BoucleInterne) {
    if let Some(rappel) = interne.rappel.borrow().as_ref() {
        let _ = fenetre().request_animation_frame(rappel.as_ref().unchecked_ref());
    }
}

pub fn lisser(t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

pub fn hauteur_ecran() -> f64 {
    fenetre()
        .inner_height()
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(800.0)
}

pub fn largeur_ecran() -> f64 {
    fenetre()
        .inner_width()
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(400.0)
}
