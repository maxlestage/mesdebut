use yew::prelude::*;

use crate::donnees::PRINCIPES;
use crate::sections::Titre;

#[function_component(Principes)]
pub fn principes() -> Html {
    html! {
        <section class="section section-principes">
            <p class="etiquette" data-revele="">{"(05) — La méthode"}</p>
            <Titre texte="Comment c'est *construit*." />
            <p class="intro" data-revele="">
                {"Six principes issus des sciences cognitives, qui décident de la forme des exercices."}
            </p>
            <ol class="principes">
                { for PRINCIPES.iter().enumerate().map(|(i, p)| html! {
                    <li data-revele="" style={format!("--i:{}", i % 2)}>
                        <span class="principe-numero" aria-hidden="true">{format!("{:02}", i + 1)}</span>
                        <div>
                            <h3>{p.titre}</h3>
                            <p>{p.texte}</p>
                        </div>
                    </li>
                }) }
            </ol>
        </section>
    }
}
