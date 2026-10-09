use yew::prelude::*;

use crate::sections::Titre;

struct Ligne {
    heure: &'static str,
    tache: &'static str,
    duree: &'static str,
    fait: bool,
    etat: &'static str,
}

const JOURNEE: &[Ligne] = &[
    Ligne {
        heure: "9 h 00",
        tache: "Prendre son médicament",
        duree: "2 → 2 min",
        fait: true,
        etat: "juste",
    },
    Ligne {
        heure: "9 h 02",
        tache: "Se faire un café",
        duree: "5 → 9 min",
        fait: false,
        etat: "ecart",
    },
    Ligne {
        heure: "9 h 11",
        tache: "Faire les courses",
        duree: "45 min",
        fait: false,
        etat: "",
    },
];

#[function_component(Planification)]
pub fn planification() -> Html {
    html! {
        <section id="planifier" class="section section-planifier">
            <div class="planifier-texte">
                <p class="etiquette" data-revele="">{"(03) — Planifier"}</p>
                <Titre texte="Organiser sa journée, pas *seulement* la raconter." />
                <p data-revele="">
                    {"Onze activités du quotidien sont décomposées en étapes\u{a0}: se faire un café, faire une \
                      lessive, prendre le bus. On apprend à les remettre dans l'ordre, à repérer l'étape qui \
                      manque, à estimer le temps qu'elles prennent."}
                </p>
                <p data-revele="">
                    {"Puis on passe à sa vraie journée. "}<strong>{"Ma journée"}</strong>
                    {" laisse écrire ses tâches, les ordonner, les cocher. Chacune démarre quand la précédente \
                      finit, donc l'heure de chaque tâche s'affiche\u{a0}: on voit tout de suite quand la journée \
                      déborde."}
                </p>
                <p data-revele="">
                    {"Un chronomètre compare le temps prévu au temps réellement passé, et un suivi sur dix jours \
                      montre si l'écart se resserre. C'est cet écart-là qu'on travaille."}
                </p>
            </div>

            <div class="exemple" data-revele="" data-parallaxe="-0.06">
                <p class="exemple-titre">{"Ma journée"}</p>
                { for JOURNEE.iter().enumerate().map(|(i, l)| html! {
                    <div class="exemple-ligne" style={format!("--i:{i}")}>
                        <span class="exemple-heure">{l.heure}</span>
                        <span class={classes!("exemple-tache", l.fait.then_some("fait"))}>{l.tache}</span>
                        <span class={classes!("exemple-duree", (!l.etat.is_empty()).then_some(l.etat))}>{l.duree}</span>
                    </div>
                }) }
                <p class="exemple-bilan">
                    {"Sur 5 jours\u{a0}: "}<strong class="compte" data-compte-vers="18" data-prefixe="+" data-suffixe="\u{a0}%">{"+18\u{a0}%"}</strong>
                    {" d'écart en moyenne (contre +45\u{a0}% avant) — tu progresses\u{a0}! 🌱"}
                </p>
            </div>
        </section>
    }
}
