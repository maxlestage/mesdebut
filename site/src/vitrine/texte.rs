//! Les textes qui se révèlent mot à mot.

use active::prelude::*;

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
/// entier reste lisible d'un bloc par les lecteurs d'écran (`aria-label`).
pub fn mots(texte: &str) -> Vec<Node> {
    let morceaux = decouper(texte);
    let n = morceaux.len();
    let mut noeuds = Vec::with_capacity(n * 2);
    for (i, (mot, accent)) in morceaux.into_iter().enumerate() {
        let mut dedans = span().class("mot-in").style(format!("--i:{i}"));
        if accent {
            dedans = dedans.class("accent");
        }
        noeuds.push(span().class("mot").child(dedans.text(mot)).into());
        if i + 1 < n {
            noeuds.push(text(" "));
        }
    }
    noeuds
}

/// Retire les marques d'accent, pour l'étiquette lue par les lecteurs d'écran.
pub fn sans_marques(texte: &str) -> String {
    texte.replace('*', "")
}

/// Un titre de section qui se révèle mot à mot en entrant à l'écran.
pub fn titre(texte: &str) -> Element {
    h2().class("titre")
        .attr("data-revele", "")
        .attr("aria-label", sans_marques(texte))
        .child(span().attr("aria-hidden", "true").children(mots(texte)))
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
                ("on".to_owned(), false)
            ]
        );
    }

    #[test]
    fn aucune_marque_ne_reste() {
        for texte in [
            super::super::donnees::MANIFESTE,
            "Comment c'est *construit*.",
        ] {
            assert!(decouper(texte).iter().all(|(m, _)| !m.contains('*')));
        }
    }

    #[test]
    fn un_titre_rendu() {
        let html = Node::from(titre("Un *prix*.")).render();
        assert_eq!(
            html,
            concat!(
                r#"<h2 class="titre" data-revele="" aria-label="Un prix."><span aria-hidden="true">"#,
                r#"<span class="mot"><span class="mot-in" style="--i:0">Un</span></span> "#,
                r#"<span class="mot"><span class="mot-in accent" style="--i:1">prix.</span></span>"#,
                "</span></h2>"
            )
        );
    }
}
