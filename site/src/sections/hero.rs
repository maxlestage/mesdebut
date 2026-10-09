use yew::prelude::*;

use crate::donnees::OFFRE;

/// L'ouverture : une scène collée à l'écran pendant que l'on défile. La
/// poussière (`mouvement::poussiere`) s'y rassemble en toque, ligne après ligne
/// le titre monte.
#[function_component(Hero)]
pub fn hero() -> Html {
    let lignes: [(&str, &str); 3] = [
        ("Réapprendre", ""),
        ("les bases,", ""),
        ("en ", "s'amusant."),
    ];

    html! {
        <section id="haut" class="hero">
            <div class="hero-scene">
                <canvas class="poussiere" aria-hidden="true"></canvas>
                <div class="hero-contenu">
                    <p class="surtitre"><span class="puce"></span>{"Application iPhone & Apple Watch"}</p>
                    <h1 class="hero-titre" aria-label="Réapprendre les bases, en s'amusant.">
                        { for lignes.iter().enumerate().map(|(i, (texte, accent))| html! {
                            <span class="ligne" aria-hidden="true">
                                <span class="ligne-in" style={format!("--i:{i}")}>
                                    {*texte}
                                    if !accent.is_empty() { <em class="accent">{*accent}</em> }
                                </span>
                            </span>
                        }) }
                    </h1>
                    <div class="hero-bas">
                        <p class="sous-accroche">
                            {"Les jours, l'heure, le calcul, les couleurs, organiser sa journée\u{a0}: seize thèmes, \
                              en questions courtes."}
                        </p>
                        <div class="hero-actions">
                            // Pas d'accès direct à l'application depuis le site : l'offre est
                            // payante. Le seul geste possible est de parcourir les thèmes.
                            <a class="bouton" href="#themes" data-aimant="">
                                <span>{"Voir les thèmes"}</span>
                                <span class="bouton-fleche" aria-hidden="true">{"↓"}</span>
                            </a>
                            <p class="pastille-store">
                                {OFFRE.disponibilite}{" · "}<strong>{OFFRE.prix}{" "}{OFFRE.periode}</strong>
                            </p>
                        </div>
                    </div>
                </div>
                <div class="indice" aria-hidden="true">
                    <span>{"Faire défiler"}</span>
                    <span class="indice-trait"></span>
                </div>
            </div>
        </section>
    }
}
