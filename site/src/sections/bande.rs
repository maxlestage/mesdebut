use yew::prelude::*;

use crate::donnees::CATEGORIES;

/// La bande des thèmes, qui défile sans fin et s'incline avec la vitesse du
/// défilement. Deux copies se suivent pour que la boucle ne se voie pas.
#[function_component(Bande)]
pub fn bande() -> Html {
    let copie = |cachee: bool| {
        html! {
            <ul class="bande-piste" aria-hidden={cachee.then_some("true")}>
                { for CATEGORIES.iter().map(|c| html! {
                    <li>
                        <span class="bande-emoji" aria-hidden="true">{c.emoji}</span>
                        {c.label}
                        <span class="bande-sep" aria-hidden="true">{"✦"}</span>
                    </li>
                }) }
            </ul>
        }
    };

    html! {
        <div class="bande" aria-label="Les seize thèmes">
            <div class="bande-defile" data-defile-voulu="">
                { copie(false) }
                { copie(true) }
            </div>
        </div>
    }
}
