//! Le rideau de index.html : il compte jusqu'à 100 en CSS (`@property`), puis
//! l'application le lève. Une fois par session seulement ; ensuite il s'efface
//! dès que la page est prête.

use crate::outils::{self, document, maintenant, plus_tard};

/// Durée du compte dans site.css (`.rideau-compte`), plus un temps de pose.
const COMPTE_MS: f64 = 1900.0;
const CLE: &str = "reneuro-rideau";

pub fn lever(calme: bool, ensuite: impl FnOnce() + 'static) {
    let html = document().document_element().expect("<html>");
    let deja_vu = html.class_list().contains("rideau-vu");
    let attente = if calme || deja_vu {
        0.0
    } else {
        (COMPTE_MS - maintenant()).max(0.0)
    };

    plus_tard(attente as i32, move || {
        if let Ok(Some(session)) = outils::fenetre().session_storage() {
            let _ = session.set_item(CLE, "1");
        }
        let _ = html.class_list().add_1("pret");
        if let Some(rideau) = document().get_element_by_id("rideau") {
            let _ = rideau.class_list().add_1("leve");
            plus_tard(1400, move || rideau.remove());
        }
        ensuite();
    });
}
