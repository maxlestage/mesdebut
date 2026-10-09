//! Les sections de la page, dans l'ordre où elles s'affichent.

mod bande;
mod entete;
mod hero;
mod manifeste;
mod pied;
mod planification;
mod principes;
mod supports;
mod tarif;
mod themes;

pub use bande::Bande;
pub use entete::Entete;
pub use hero::Hero;
pub use manifeste::Manifeste;
pub use pied::Pied;
pub use planification::Planification;
pub use principes::Principes;
pub use supports::Supports;
pub use tarif::Tarif;
pub use themes::Themes;

use yew::prelude::*;

/// Découpe un texte en mots, en notant ceux qui sont en accent : ce qui est
/// entre `*`, un mot ou plusieurs.
pub fn decouper(texte: &str) -> Vec<(String, bool)> {
    let mut dans_accent = false;
    texte
        .split(' ')
        .filter(|m| !m.is_empty())
        .map(|brut| {
            let ouvre = brut.starts_with('*');
            let ferme = brut[usize::from(ouvre)..].contains('*');
            let accent = dans_accent || ouvre;
            dans_accent = accent && !ferme;
            (brut.replace('*', ""), accent)
        })
        .collect()
}

/// Chaque mot dans son masque, pour qu'ils montent l'un après l'autre. Le texte
/// entier reste lisible d'un bloc par les lecteurs d'écran (voir `Titre`).
pub fn mots(texte: &str, depart: usize) -> Html {
    let morceaux = decouper(texte);
    let n = morceaux.len();
    morceaux
        .into_iter()
        .enumerate()
        .map(|(i, (mot, accent))| {
            let style = format!("--i:{}", depart + i);
            html! {
                <>
                    <span class="mot">
                        <span class={classes!("mot-in", accent.then_some("accent"))} {style}>{mot}</span>
                    </span>
                    { if i + 1 < n { " " } else { "" } }
                </>
            }
        })
        .collect()
}

/// Retire les marques d'accent, pour l'étiquette lue par les lecteurs d'écran.
pub fn sans_marques(texte: &str) -> String {
    texte.replace('*', "")
}

#[derive(Properties, PartialEq)]
pub struct TitreProps {
    pub texte: AttrValue,
    #[prop_or_default]
    pub class: Classes,
}

/// Un titre de section qui se révèle mot à mot en entrant à l'écran.
#[function_component(Titre)]
pub fn titre(props: &TitreProps) -> Html {
    html! {
        <h2 class={classes!("titre", props.class.clone())} data-revele="" aria-label={sans_marques(&props.texte)}>
            <span aria-hidden="true">{ mots(&props.texte, 0) }</span>
        </h2>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn accents(texte: &str) -> Vec<&'static str> {
        decouper(texte)
            .into_iter()
            .map(|(_, a)| if a { "A" } else { "." })
            .collect()
    }

    #[test]
    fn un_mot_en_accent() {
        assert_eq!(accents("Un prix, *tout* compris."), [".", ".", "A", "."]);
    }

    #[test]
    fn plusieurs_mots_en_accent() {
        assert_eq!(
            accents("De la poche *au poignet*."),
            [".", ".", ".", "A", "A"]
        );
        assert_eq!(decouper("*au poignet*.")[1].0, "poignet.");
    }

    #[test]
    fn la_ponctuation_suit_l_accent() {
        assert_eq!(
            decouper("on *répond*, on"),
            [
                ("on".to_owned(), false),
                ("répond,".to_owned(), true),
                ("on".to_owned(), false),
            ]
        );
    }

    #[test]
    fn aucune_marque_ne_reste() {
        for texte in [crate::donnees::MANIFESTE, "Comment c'est *construit*."] {
            assert!(decouper(texte).iter().all(|(m, _)| !m.contains('*')));
        }
    }
}
