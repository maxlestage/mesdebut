//! La page de présentation, `/`.
//!
//! Rendue une fois pour toutes en HTML par l'exportateur : le contenu est dans
//! la page, lisible sans JavaScript et par les moteurs de recherche. Le module
//! WebAssembly l'anime ensuite (`mouvement`), sans jamais créer ni déplacer un
//! nœud : il pose des classes, des variables CSS et des transformations.

pub mod donnees;
mod sections;
mod texte;

use active::prelude::*;

/// Les fichiers construits, tels que la page les référence.
pub struct Ressources<'a> {
    pub css: &'a str,
    pub js: &'a str,
    pub wasm: &'a str,
}

pub const DESCRIPTION: &str = "ReNeuro est une application de quiz pour réapprendre les bases : les jours, \
    l'heure, le calcul, la planification. Sur iPhone, Apple Watch et dans le navigateur.";

/// Ce que la page ajoute à `<head>`, en plus des balises SEO.
const TETE: &str = concat!(
    r##"<meta name="theme-color" content="#0c0b14"/>"##,
    r#"<meta name="color-scheme" content="dark"/>"#,
    r#"<link rel="icon" type="image/svg+xml" href="icon.svg"/>"#,
    r#"<link rel="preconnect" href="https://fonts.googleapis.com"/>"#,
    r#"<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin=""/>"#,
    r#"<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Archivo:wdth,wght@62..125,100..900&amp;family=Fraunces:ital,opsz,wght@1,9..144,300..600&amp;display=swap"/>"#,
    // Le rideau ne joue son compte qu'une fois par session : décidé avant le
    // premier pixel, pour qu'il ne clignote pas au retour sur la page.
    r#"<script>try{if(sessionStorage.getItem("reneuro-rideau"))document.documentElement.classList.add("rideau-vu")}catch(e){}</script>"#,
    r#"<noscript><style>#rideau{display:none}</style></noscript>"#,
);

pub fn page(r: &Ressources<'_>) -> String {
    Document::new(corps())
        .lang("fr")
        .viewport("width=device-width, initial-scale=1, viewport-fit=cover")
        .seo(
            Seo::new()
                .title("ReNeuro — réapprendre les bases, en s'amusant")
                .description(DESCRIPTION)
                .site_name("ReNeuro")
                .og_type("website"),
        )
        .stylesheet(r.css.to_string())
        .head(TETE)
        .client(r.js.to_string(), r.wasm.to_string())
        .render()
}

fn corps() -> Node {
    fragment([
        // Le rideau couvre aussi le chargement du .wasm. Si celui-ci ne démarre
        // jamais, il se lève seul (animation « secours » de site.css).
        div()
            .id("rideau")
            .attr("aria-hidden", "true")
            .child(
                div()
                    .class("rideau-centre")
                    .child(
                        img()
                            .attr("src", "icon.svg")
                            .attr("alt", "")
                            .attr("width", "72")
                            .attr("height", "72"),
                    )
                    .child(span().class("rideau-nom").text("ReNeuro")),
            )
            .child(span().class("rideau-compte"))
            .into(),
        div().class("grain").attr("aria-hidden", "true").into(),
        div()
            .class("progression")
            .attr("aria-hidden", "true")
            .child(span())
            .into(),
        div().class("curseur").attr("aria-hidden", "true").into(),
        div()
            .class("curseur-point")
            .attr("aria-hidden", "true")
            .into(),
        sections::entete(),
        main()
            .child(sections::hero())
            .child(sections::manifeste())
            .child(sections::bande())
            .child(sections::themes())
            .child(sections::planification())
            .child(sections::supports())
            .child(sections::principes())
            .child(sections::tarif())
            .into(),
        sections::pied(),
    ])
}

/// Anime la page déjà rendue (navigateur seulement).
#[cfg(target_arch = "wasm32")]
pub fn animer() {
    let html = crate::outils::document()
        .document_element()
        .expect("<html> introuvable");
    // Sous « réduire les animations », rien ne part caché, donc rien n'attend
    // d'être révélé. Le rideau couvre la page jusque-là : rien ne clignote.
    let classe = if crate::outils::mouvement_reduit() {
        "calme"
    } else {
        "anime"
    };
    let _ = html.class_list().add_1(classe);
    crate::mouvement::demarrer();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rendu() -> String {
        page(&Ressources {
            css: "assets/site-0.css",
            js: "./assets/reneuro-0.js",
            wasm: "./assets/reneuro-0_bg.wasm",
        })
    }

    #[test]
    fn la_page_dit_l_essentiel_sans_javascript() {
        let html = rendu();
        assert!(html.starts_with(r#"<!DOCTYPE html><html lang="fr">"#));
        assert_eq!(html.matches("<title>").count(), 1);
        assert!(html.contains("<title>ReNeuro — réapprendre les bases, en s&#x27;amusant</title>"));
        assert_eq!(html.matches(r#"property="og:title""#).count(), 1);
        for c in crate::moteur::categories::CATEGORIES {
            assert!(
                html.contains(&c.libelle.replace('\'', "&#x27;")),
                "{}",
                c.libelle
            );
        }
        assert!(html.contains("4,99 €"));
        assert!(html.contains(r#"href="app""#), "un lien vers l'application");
        assert!(html.contains(r#"import init from "./assets/reneuro-0.js""#));
    }

    #[test]
    fn aucun_identifiant_en_double() {
        let html = rendu();
        let mut ids: Vec<&str> = html
            .split(" id=\"")
            .skip(1)
            .map(|s| &s[..s.find('"').unwrap()])
            .collect();
        let n = ids.len();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), n);
    }
}
