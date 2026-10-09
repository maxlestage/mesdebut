use yew::prelude::*;

/// La barre du haut : elle s'efface quand on descend, revient quand on remonte.
#[function_component(Entete)]
pub fn entete() -> Html {
    html! {
        <header class="entete">
            <a class="marque" href="#haut" data-aimant="">
                <img src="icon.svg" alt="" width="28" height="28" />
                <span>{"ReNeuro"}</span>
            </a>
            <nav aria-label="Sections">
                <a href="#themes">{"Thèmes"}</a>
                <a href="#planifier">{"Planifier"}</a>
                <a href="#tarif">{"Tarif"}</a>
            </nav>
        </header>
    }
}
