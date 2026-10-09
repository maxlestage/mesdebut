use yew::prelude::*;

use crate::donnees::OFFRE;
use crate::sections::Titre;

#[function_component(Tarif)]
pub fn tarif() -> Html {
    html! {
        <section id="tarif" class="section section-tarif">
            <p class="etiquette" data-revele="">{"(06) — Le tarif"}</p>
            <Titre texte="Un prix, *tout* compris." />
            <div class="carte-tarif" data-revele="" data-lueur="">
                <p class="tarif-prix">
                    <strong class="compte" data-compte-vers={OFFRE.centimes.to_string()}
                            data-centimes="" data-suffixe="\u{a0}€">{OFFRE.prix}</strong>
                    <span class="tarif-periode">{OFFRE.periode}</span>
                </p>
                <ul class="tarif-inclus">
                    { for OFFRE.inclus.iter().map(|ligne| html! { <li key={*ligne}>{*ligne}</li> }) }
                </ul>
                // Annonce, pas bouton : rien à ouvrir tant que l'application n'est pas
                // publiée, et le site n'offre pas d'accès gratuit.
                <p class="tarif-dispo">{OFFRE.disponibilite}</p>
            </div>
        </section>
    }
}
