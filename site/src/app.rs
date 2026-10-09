use yew::prelude::*;

use crate::mouvement;
use crate::sections::{
    Bande, Entete, Hero, Manifeste, Pied, Planification, Principes, Supports, Tarif, Themes,
};

#[function_component(App)]
pub fn app() -> Html {
    // Une fois la page dans le DOM, et une seule : les animations s'y accrochent.
    use_effect_with((), |_| {
        mouvement::demarrer();
    });

    html! {
        <>
            <div class="progression" aria-hidden="true"><span></span></div>
            <div class="curseur" aria-hidden="true"></div>
            <div class="curseur-point" aria-hidden="true"></div>
            <Entete />
            <main>
                <Hero />
                <Manifeste />
                <Bande />
                <Themes />
                <Planification />
                <Supports />
                <Principes />
                <Tarif />
            </main>
            <Pied />
        </>
    }
}
