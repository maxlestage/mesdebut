//! L'application, `/app` : rendue entièrement dans le navigateur par Active.
//!
//! Le serveur envoie toujours la même coquille (`coquille`) ; le routeur
//! d'Active lit l'adresse et affiche l'écran qui lui correspond, sans
//! recharger la page. Chaque écran a sa propre adresse : on peut y revenir avec
//! le bouton « précédent », la garder en favori ou la partager.
//!
//! | Adresse | Écran |
//! | --- | --- |
//! | `/app/` | le menu des dix-huit thèmes |
//! | `/app/heure` | réviser ou faire le quiz |
//! | `/app/heure/reviser` | la révision |
//! | `/app/heure/niveaux` | le choix du niveau |
//! | `/app/heure/quiz/2` | le quiz, au niveau 2 |
//! | `/app/progres` | mes progrès |
//! | `/app/journee` | ma journée, le planificateur |
//! | `/app/hanoi` | les tours de Hanoï : le choix du nombre de disques |
//! | `/app/hanoi/2` | les tours de Hanoï, au niveau 2 |

mod dessins;
mod ecrans;
mod hanoi;
mod journee;
pub mod plateforme;
mod quiz;
mod stock;

use active::prelude::*;
use active::pwa::{Manifest, ServiceWorker};

use crate::moteur::categories::{Categorie, categorie};

/// L'adresse du menu. Avec la barre finale : c'est aussi la portée du service
/// worker et l'adresse de départ de l'application installée.
pub const ACCUEIL: &str = "/app/";

#[derive(Clone, Copy)]
pub enum Route {
    Menu,
    Progres,
    Journee,
    HanoiNiveaux,
    Hanoi(u8),
    Theme(&'static Categorie),
    Revision(&'static Categorie),
    Niveaux(&'static Categorie),
    Quiz(&'static Categorie, u8),
    Introuvable,
}

/// L'écran d'une adresse.
pub fn route(chemin: &str) -> Route {
    let Some(reste) = chemin.strip_prefix("/app") else {
        return Route::Introuvable;
    };
    let morceaux: Vec<&str> = reste.split('/').filter(|m| !m.is_empty()).collect();
    match morceaux.as_slice() {
        [] => Route::Menu,
        ["progres"] => Route::Progres,
        ["journee"] => Route::Journee,
        ["hanoi"] => Route::HanoiNiveaux,
        ["hanoi", n] => match n.parse::<u8>() {
            Ok(n) if crate::moteur::hanoi::niveau(n).is_some() => Route::Hanoi(n),
            _ => Route::Introuvable,
        },
        [cle, suite @ ..] => {
            let Some(c) = categorie(cle) else {
                return Route::Introuvable;
            };
            match suite {
                [] => Route::Theme(c),
                ["reviser"] if c.revision => Route::Revision(c),
                ["niveaux"] if c.a_des_niveaux() => Route::Niveaux(c),
                ["quiz"] => Route::Quiz(c, 1),
                ["quiz", n] => match n.parse::<u8>() {
                    Ok(n) if c.niveaux.iter().any(|niv| niv.id == n) => Route::Quiz(c, n),
                    _ => Route::Introuvable,
                },
                _ => Route::Introuvable,
            }
        }
    }
}

/// Où mène un thème choisi au menu : la révision d'abord s'il en a une, sinon
/// le choix du niveau, sinon le quiz directement (le Mélange).
pub fn adresse_theme(c: &Categorie) -> String {
    if c.revision {
        format!("/app/{}", c.cle)
    } else {
        adresse_quiz_ou_niveaux(c)
    }
}

/// Le quiz, en passant par le choix du niveau quand il y en a plusieurs.
pub fn adresse_quiz_ou_niveaux(c: &Categorie) -> String {
    if c.a_des_niveaux() {
        format!("/app/{}/niveaux", c.cle)
    } else {
        format!("/app/{}/quiz", c.cle)
    }
}

pub fn seo_du_site() -> Seo {
    Seo::new()
        .site_name("ReNeuro")
        .title_template("%s · ReNeuro")
        .description(
            "Réapprendre les bases en questions courtes : les jours, l'heure, le calcul, les couleurs, \
             organiser sa journée. Dix-huit thèmes, une mémoire de ce qui résiste.",
        )
}

/// Le manifeste d'application web : `/app` s'installe sur l'écran d'accueil.
pub fn manifeste() -> Manifest {
    Manifest::new("ReNeuro")
        .short_name("ReNeuro")
        .description("Réapprendre les bases, en s'amusant.")
        .id(ACCUEIL)
        .start_url(ACCUEIL)
        .scope(ACCUEIL)
        .display("standalone")
        .theme_color("#667eea")
        .background_color("#667eea")
        .lang("fr")
        .categories(["education", "health"])
        .icon("/app/pwa-192.png", "192x192", "image/png")
        .icon("/app/pwa-512.png", "512x512", "image/png")
        .maskable_icon("/app/pwa-maskable-512.png", "512x512", "image/png")
}

/// Le service worker : l'application fonctionne hors ligne une fois ouverte.
/// `version` change avec ce qu'il garde, et remplace alors l'ancien cache.
pub fn service_worker(version: &str, fichiers: &[&str]) -> ServiceWorker {
    ServiceWorker::new("reneuro-app", version)
        .precache(fichiers.iter().copied())
        .offline_page(ACCUEIL)
}

/// La page que le serveur envoie pour toute adresse de l'application.
pub fn coquille(css: &str, js: &str, wasm: &str) -> String {
    // le même cadre que l'application : rien ne bouge quand elle démarre, et
    // l'accueil reste à portée même sans JavaScript
    let attente = div().id("app").child(cadre(
        main()
            .class("carte attente")
            .child(
                img()
                    .attr("src", "/icon.svg")
                    .attr("alt", "")
                    .attr("width", "72")
                    .attr("height", "72"),
            )
            .child(p().text("Chargement de ReNeuro…"))
            .child(noscript().child(p().text(
                "ReNeuro a besoin de JavaScript et de WebAssembly pour fonctionner. \
                             Active-les dans ton navigateur, puis recharge la page.",
            ))),
    ));
    Document::new(attente)
        .lang("fr")
        .viewport("width=device-width, initial-scale=1, viewport-fit=cover")
        // le titre de chaque écran suivra le modèle, une fois l'application lancée
        .seo(
            seo_du_site()
                .title_template("%s")
                .title("ReNeuro — l'application")
                .canonical(ACCUEIL),
        )
        .stylesheet(css.to_string())
        .head(r#"<link rel="icon" type="image/svg+xml" href="/icon.svg"/>"#)
        .manifest("/app/manifest.webmanifest", &manifeste())
        .service_worker("/app/sw.js")
        .client(js.to_string(), wasm.to_string())
        .render()
}

/// Monte l'application dans `#app` (navigateur seulement).
#[cfg(target_arch = "wasm32")]
pub fn lancer() {
    active::mount_to("#app", racine());
}

/// La vue racine : le fond, la carte, et l'écran de l'adresse courante.
#[cfg(target_arch = "wasm32")]
fn racine() -> Node {
    let chemin = active::location();
    cadre(
        main()
            .class("carte")
            // répété ici pour que le titre garde son modèle quand l'écran change
            .child(seo_du_site())
            .children_dyn(move || vec![ecran(route(&chemin.get()))]),
    )
}

/// Le cadre de chaque écran : la barre qui ramène à l'accueil du site, puis la
/// carte de l'écran.
pub fn cadre(carte: Element) -> Node {
    div()
        .class("ecran")
        .child(
            header().class("entete").child(
                // la présentation n'appartient pas à l'application : un
                // chargement complet, pas le routeur
                a().class("vers-accueil")
                    .href("/")
                    .attr("rel", "external")
                    .attr("aria-label", "Revenir à l'accueil de ReNeuro")
                    .child(span().class("fleche").attr("aria-hidden", "true").text("←"))
                    .child(
                        img()
                            .attr("src", "/icon.svg")
                            .attr("alt", "")
                            .attr("width", "24")
                            .attr("height", "24"),
                    )
                    .child(span().text("Accueil")),
            ),
        )
        .child(carte)
        .into()
}

pub fn ecran(r: Route) -> Node {
    match r {
        Route::Menu => ecrans::menu(),
        Route::Progres => ecrans::progres(),
        Route::Journee => journee::journee(),
        Route::HanoiNiveaux => hanoi::niveaux(),
        Route::Hanoi(n) => hanoi::page(n),
        Route::Theme(c) => ecrans::theme(c),
        Route::Revision(c) => ecrans::revision(c),
        Route::Niveaux(c) => ecrans::niveaux(c),
        Route::Quiz(c, n) => quiz::page(c, n),
        Route::Introuvable => ecrans::introuvable(),
    }
}

// ---------------------------------------------------------------------------
// Les briques communes des écrans

pub(crate) fn titre(texte: impl Into<active::Str>) -> Element {
    h1().class("titre-ecran").text(texte)
}

pub(crate) fn sous_titre(texte: impl Into<active::Str>) -> Element {
    p().class("sous-titre").text(texte)
}

/// Un grand bouton qui mène à un autre écran.
pub(crate) fn gros_lien(href: impl Into<active::Str>, texte: impl Into<active::Str>) -> Element {
    a().class("gros-bouton").href(href).text(texte)
}

pub(crate) fn gros_bouton(
    texte: impl Into<active::Str>,
    action: impl Fn(Event) + 'static,
) -> Element {
    button()
        .class("gros-bouton")
        .attr("type", "button")
        .on_click(action)
        .text(texte)
}

pub(crate) fn retour(href: impl Into<active::Str>, texte: impl Into<active::Str>) -> Element {
    a().class("retour").href(href).text(texte)
}

pub(crate) fn retour_menu() -> Element {
    retour(ACCUEIL, "← Retour au menu")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nom(r: Route) -> String {
        match r {
            Route::Menu => "menu".into(),
            Route::Progres => "progres".into(),
            Route::Journee => "journee".into(),
            Route::HanoiNiveaux => "hanoi".into(),
            Route::Hanoi(n) => format!("hanoi:{n}"),
            Route::Theme(c) => format!("theme:{}", c.cle),
            Route::Revision(c) => format!("revision:{}", c.cle),
            Route::Niveaux(c) => format!("niveaux:{}", c.cle),
            Route::Quiz(c, n) => format!("quiz:{}:{n}", c.cle),
            Route::Introuvable => "introuvable".into(),
        }
    }

    #[test]
    fn chaque_adresse_a_son_ecran() {
        let cas = [
            ("/app", "menu"),
            ("/app/", "menu"),
            ("/app/progres", "progres"),
            ("/app/journee/", "journee"),
            ("/app/hanoi", "hanoi"),
            ("/app/hanoi/3", "hanoi:3"),
            ("/app/hanoi/4", "introuvable"),
            ("/app/heure", "theme:heure"),
            ("/app/heure/reviser", "revision:heure"),
            ("/app/heure/niveaux", "niveaux:heure"),
            ("/app/heure/quiz/4", "quiz:heure:4"),
            ("/app/heure/quiz/5", "introuvable"),
            ("/app/jours/quiz", "quiz:jours:1"),
            ("/app/jours/niveaux", "introuvable"),
            ("/app/addition/reviser", "introuvable"),
            ("/app/inconnu", "introuvable"),
            ("/autre", "introuvable"),
        ];
        for (chemin, attendu) in cas {
            assert_eq!(nom(route(chemin)), attendu, "{chemin}");
        }
    }

    #[test]
    fn un_theme_mene_a_la_revision_aux_niveaux_ou_au_quiz() {
        let c = |cle| categorie(cle).unwrap();
        assert_eq!(adresse_theme(c("jours")), "/app/jours");
        assert_eq!(adresse_theme(c("addition")), "/app/addition/niveaux");
        assert_eq!(adresse_theme(c("melange")), "/app/melange/quiz");
        // chaque adresse produite mène bien à un écran
        for c in crate::moteur::categories::CATEGORIES {
            for a in [adresse_theme(c), adresse_quiz_ou_niveaux(c)] {
                assert!(!matches!(route(&a), Route::Introuvable), "{a}");
            }
        }
    }

    #[test]
    fn chaque_ecran_se_rend() {
        let mut chemins = vec![
            "/app/".to_string(),
            "/app/progres".into(),
            "/app/journee".into(),
            "/app/hanoi".into(),
            "/app/hanoi/1".into(),
            "/app/hanoi/3".into(),
            "/app/x".into(),
        ];
        for c in crate::moteur::categories::CATEGORIES {
            chemins.push(format!("/app/{}", c.cle));
            chemins.push(adresse_quiz_ou_niveaux(c));
            if c.revision {
                chemins.push(format!("/app/{}/reviser", c.cle));
            }
        }
        for chemin in chemins {
            let html = ecran(route(&chemin)).render();
            assert!(html.contains("titre-ecran"), "{chemin} : {html}");
        }
    }

    #[test]
    fn chaque_ecran_a_son_bouton_accueil() {
        let html = cadre(
            main()
                .class("carte")
                .child(ecran(route("/app/heure/quiz/2"))),
        )
        .render();
        assert!(html.starts_with(r#"<div class="ecran"><header class="entete"><a class="vers-accueil" href="/" rel="external""#), "{html}");
        // la coquille servie avant le démarrage a le même cadre
        let coquille = coquille("/a.css", "/a.js", "/a.wasm");
        assert!(coquille.contains(r#"<div id="app"><div class="ecran"><header class="entete"><a class="vers-accueil" href="/""#));
    }

    #[test]
    fn le_menu_renvoie_a_active() {
        let html = ecran(Route::Menu).render();
        assert!(html.contains(r#"<a href="https://maxlestage.github.io/Active/">Active</a>"#));
    }

    #[test]
    fn la_coquille_est_installable() {
        let html = coquille(
            "/assets/appli-0.css",
            "/assets/reneuro-0.js",
            "/assets/reneuro-0_bg.wasm",
        );
        assert!(html.contains(r#"<div id="app">"#));
        assert!(html.contains(r#"<link rel="manifest" href="/app/manifest.webmanifest"/>"#));
        assert!(
            html.contains(r#"navigator.serviceWorker.register(\"/app/sw.js\")"#)
                || html.contains("/app/sw.js")
        );
        assert!(manifeste().to_json().contains(r#""start_url":"/app/""#));
    }
}
