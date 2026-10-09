use yew::prelude::*;

#[function_component(Pied)]
pub fn pied() -> Html {
    html! {
        <footer class="pied">
            <p class="pied-marque" aria-hidden="true" data-revele="">
                {"Re"}<em class="accent">{"Neuro"}</em>
            </p>
            <div class="pied-bas">
                <p class="credits">{"Conçu et développé par Maxime Nathan Lestage"}</p>
                <p class="mention">
                    {"ReNeuro est une application d'entraînement, pas un dispositif médical, et ne remplace \
                      aucun suivi ni aucune rééducation."}
                </p>
                <a class="retour" href="#haut" data-aimant="">{"Revenir en haut ↑"}</a>
            </div>
        </footer>
    }
}
