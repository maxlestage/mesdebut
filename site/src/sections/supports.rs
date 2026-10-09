use yew::prelude::*;

use crate::donnees::SUPPORTS;
use crate::sections::Titre;

#[function_component(Supports)]
pub fn supports() -> Html {
    html! {
        <section class="section">
            <p class="etiquette" data-revele="">{"(04) — Les écrans"}</p>
            <Titre texte="De la poche *au poignet*." />
            <p class="intro" data-revele="">{"Le même contenu, la même progression, sur deux écrans."}</p>
            <ul class="supports">
                { for SUPPORTS.iter().enumerate().map(|(i, s)| html! {
                    <li data-revele="" style={format!("--i:{i}")}>
                        <span class="support-emoji" aria-hidden="true"
                              data-parallaxe={if i == 0 { "0.08" } else { "-0.08" }}>{s.emoji}</span>
                        <div>
                            <h3>{s.nom}</h3>
                            <p>{s.detail}</p>
                            <p class="support-note">{s.note}</p>
                        </div>
                    </li>
                }) }
            </ul>
        </section>
    }
}
