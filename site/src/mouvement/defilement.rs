//! Tout ce qui suit le défilement, dans une seule boucle qui ne tourne que
//! lorsqu'on défile : la barre de progression, l'entête qui s'efface, l'avancée
//! de l'ouverture, le manifeste qui s'allume, les parallaxes et l'inclinaison de
//! la bande selon la vitesse.

use std::cell::Cell;
use std::rc::Rc;

use web_sys::HtmlElement;

use super::Etat;
use crate::outils::{self, Boucle, classe, fenetre, hauteur_ecran, style};

pub fn installer(etat: Rc<Etat>, toque: Boucle) -> Boucle {
    let html = outils::document().document_element().expect("<html>");
    let barre = outils::un(".progression span");
    let entete = outils::un(".entete");
    let hero = outils::un(".hero");
    let bande = outils::un(".bande-defile");
    let manifeste = outils::un(".manifeste-texte");
    let mots: Vec<HtmlElement> = outils::tous(".manifeste-texte .mot-in");
    let parallaxes: Vec<(HtmlElement, f64)> = outils::tous("[data-parallaxe]")
        .into_iter()
        .filter_map(|el| {
            let k = el.get_attribute("data-parallaxe")?.parse().ok()?;
            Some((el, k))
        })
        .collect();

    let dernier_y = Cell::new(f64::NAN);
    let inclinaison = Cell::new(0.0f64);
    let allumes = Cell::new(usize::MAX);

    let boucle = Boucle::new(move |_| {
        let y = fenetre().scroll_y().unwrap_or(0.0);
        let h = hauteur_ecran();
        let avant = dernier_y.replace(y);
        let vitesse = if avant.is_nan() { 0.0 } else { y - avant };

        // Barre de progression de la page.
        if let Some(barre) = &barre {
            let total = (html.scroll_height() as f64 - h).max(1.0);
            style(
                barre,
                "transform",
                &format!("scaleX({:.4})", (y / total).clamp(0.0, 1.0)),
            );
        }

        // L'entête s'efface en descendant, revient dès qu'on remonte.
        if let Some(entete) = &entete {
            if y < 80.0 || vitesse < -4.0 {
                classe(entete, "cachee", false);
            } else if vitesse > 4.0 {
                classe(entete, "cachee", true);
            }
            classe(entete, "pleine", y > 40.0);
        }

        // L'ouverture : la scène est collée pendant (hauteur − écran) pixels.
        if let Some(hero) = &hero {
            let r = hero.get_bounding_client_rect();
            let p = (-r.top() / (r.height() - h).max(1.0)).clamp(0.0, 1.0);
            if (p - etat.ouverture.get()).abs() > 1e-4 {
                etat.ouverture.set(p);
                style(hero, "--p", &format!("{p:.4}"));
            }
            if r.bottom() > 0.0 {
                toque.relancer();
            }
        }

        // Le manifeste s'allume mot à mot, de l'entrée en bas d'écran au milieu.
        if let Some(m) = &manifeste {
            let r = m.get_bounding_client_rect();
            let a = ((h * 0.85 - r.top()) / (r.height() + h * 0.3)).clamp(0.0, 1.0);
            let n = (a * mots.len() as f64).round() as usize;
            if n != allumes.replace(n) {
                for (i, mot) in mots.iter().enumerate() {
                    classe(mot, "allume", i < n);
                }
            }
        }

        // Parallaxes : un léger décalage selon la distance au centre de l'écran.
        // Grand écran seulement : sur un téléphone, il ferait sortir les icônes
        // de leur carte.
        let parallaxes = if outils::largeur_ecran() >= 820.0 {
            &parallaxes[..]
        } else {
            &[]
        };
        for (el, k) in parallaxes {
            let r = el.get_bounding_client_rect();
            if r.bottom() < -200.0 || r.top() > h + 200.0 {
                continue;
            }
            let ecart = r.top() + r.height() / 2.0 - h / 2.0;
            style(el, "translate", &format!("0 {:.1}px", ecart * k));
        }

        // La bande penche avec la vitesse, puis se redresse.
        let cible = (vitesse * 0.12).clamp(-7.0, 7.0);
        let i = inclinaison.get() + (cible - inclinaison.get()) * 0.18;
        inclinaison.set(i);
        if let Some(bande) = &bande {
            style(bande, "--incline", &format!("{i:.3}deg"));
        }

        // Une image de plus tant que la page bouge ou que la bande se redresse.
        vitesse.abs() > 0.1 || i.abs() > 0.02
    });

    let fen: web_sys::EventTarget = fenetre().into();
    for evenement in ["scroll", "resize"] {
        let b = boucle.clone();
        outils::ecouter::<web_sys::Event>(&fen, evenement, move |_| b.relancer());
    }
    boucle.relancer();
    boucle
}
