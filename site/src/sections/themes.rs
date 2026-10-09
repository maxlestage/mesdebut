use yew::prelude::*;

use crate::donnees::CATEGORIES;
use crate::sections::Titre;

#[function_component(Themes)]
pub fn themes() -> Html {
    let avec_niveaux = CATEGORIES.iter().filter(|c| !c.niveaux.is_empty()).count();

    html! {
        <section id="themes" class="section">
            <p class="etiquette" data-revele="">{"(02) — Les thèmes"}</p>
            <Titre texte="Seize thèmes, *dix* questions à la fois." />
            <p class="intro" data-revele="">
                {"Chaque thème se révise d'abord, puis se travaille en quiz de dix questions."}
            </p>

            <ul class="grille">
                { for CATEGORIES.iter().enumerate().map(|(i, c)| {
                    let style = format!(
                        "--a:{};--b:{};--i:{}",
                        c.degrade.0, c.degrade.1, i % 4
                    );
                    html! {
                        <li key={c.key} data-revele="" {style}>
                            <article class="carte-theme" data-incline="" title={c.title}>
                                <span class="carte-reflet" aria-hidden="true"></span>
                                <span class="emoji" aria-hidden="true">{c.emoji}</span>
                                <span class="nom">{c.label}</span>
                                if !c.niveaux.is_empty() {
                                    <span class="niveaux">{format!("{} niveaux", c.niveaux.len())}</span>
                                }
                            </article>
                        </li>
                    }
                }) }
            </ul>

            <p class="note" data-revele="">
                {format!("{avec_niveaux} thèmes ont plusieurs niveaux. L'heure en a quatre, des heures pleines \
                  à la minute près — soit ")}
                <strong class="compte" data-compte-vers="720">{"720"}</strong>
                {" horaires différents à lire sur une horloge à aiguilles."}
            </p>
        </section>
    }
}
