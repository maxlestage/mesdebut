use yew::prelude::*;

use crate::donnees::MANIFESTE;
use crate::sections::{mots, sans_marques};

/// Un paragraphe qui s'allume mot à mot à mesure que l'on défile.
#[function_component(Manifeste)]
pub fn manifeste() -> Html {
    html! {
        <section class="manifeste" aria-label="Le principe">
            <p class="etiquette" data-revele="">{"(01) — Le principe"}</p>
            <p class="manifeste-texte" aria-label={sans_marques(MANIFESTE)}>
                <span aria-hidden="true">{ mots(MANIFESTE, 0) }</span>
            </p>
        </section>
    }
}
