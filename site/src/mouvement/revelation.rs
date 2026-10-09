//! Ce qui entre à l'écran se révèle : `[data-revele]` reçoit la classe `vu`
//! (le CSS fait le reste), et les nombres `.compte` défilent jusqu'à leur valeur.

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{
    HtmlElement, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit,
};

use crate::outils::{self, Boucle, maintenant};

pub fn installer() {
    let rappel = Closure::<dyn FnMut(js_sys::Array, IntersectionObserver)>::new(
        |entrees: js_sys::Array, obs: IntersectionObserver| {
            for entree in entrees.iter() {
                let entree: IntersectionObserverEntry = entree.unchecked_into();
                if !entree.is_intersecting() {
                    continue;
                }
                let cible = entree.target();
                obs.unobserve(&cible);
                if cible.class_list().contains("compte") {
                    if let Ok(el) = cible.dyn_into::<HtmlElement>() {
                        compter(el);
                    }
                } else {
                    let _ = cible.class_list().add_1("vu");
                }
            }
        },
    );
    let options = IntersectionObserverInit::new();
    options.set_root_margin("0px 0px -8% 0px");
    options.set_threshold(&JsValue::from_f64(0.12));
    let Ok(obs) = IntersectionObserver::new_with_options(rappel.as_ref().unchecked_ref(), &options)
    else {
        // Pas d'observateur : on montre tout plutôt que de laisser caché.
        for el in outils::tous("[data-revele]") {
            let _ = el.class_list().add_1("vu");
        }
        return;
    };
    rappel.forget();
    for el in outils::tous("[data-revele], .compte") {
        obs.observe(&el);
    }
}

/// Fait défiler un nombre de 0 à `data-compte-vers`, en ne touchant qu'à la
/// valeur du nœud texte existant — jamais aux nœuds eux-mêmes.
fn compter(el: HtmlElement) {
    let Some(texte) = el.first_child() else {
        return;
    };
    let final_ = texte.node_value().unwrap_or_default();
    let Some(cible) = el
        .get_attribute("data-compte-vers")
        .and_then(|v| v.parse::<f64>().ok())
    else {
        return;
    };
    let centimes = el.has_attribute("data-centimes");
    let prefixe = el.get_attribute("data-prefixe").unwrap_or_default();
    let suffixe = el.get_attribute("data-suffixe").unwrap_or_default();
    let debut = maintenant();
    const DUREE: f64 = 1600.0;

    let boucle = Boucle::new(move |_| {
        let t = ((maintenant() - debut) / DUREE).min(1.0);
        if t >= 1.0 {
            texte.set_node_value(Some(&final_));
            return false;
        }
        // Sortie exponentielle : vite d'abord, puis l'arrivée se pose.
        let e = 1.0 - 2f64.powf(-10.0 * t);
        let v = cible * e;
        let affiche = if centimes {
            let c = v.round() as u32;
            format!("{prefixe}{},{:02}{suffixe}", c / 100, c % 100)
        } else {
            format!("{prefixe}{}{suffixe}", v.round() as u32)
        };
        texte.set_node_value(Some(&affiche));
        true
    });
    boucle.relancer();
}
