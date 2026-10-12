//! Les animations de la page, dans l'esprit de zamocorp.com : un rideau qui
//! compte jusqu'à 100, de la poussière qui se rassemble en toque au défilement,
//! des titres qui montent mot à mot, un manifeste qui s'allume, une bande qui
//! défile, un curseur à anneau et des boutons aimantés.
//!
//! Règle : rien ici ne crée, ne supprime ni ne déplace un nœud de la page rendue.
//! On pose des classes, des variables CSS et `translate` ; le seul nœud retiré
//! est le rideau.
//!
//! Sous « réduire les animations », seul le rideau se lève et la toque se
//! dessine, immobile : le reste de la page est déjà en place (classe `calme`).

mod defilement;
mod pointeur;
mod poussiere;
mod revelation;
mod rideau;

use std::cell::Cell;
use std::rc::Rc;

use crate::outils;

/// Ce que les animations partagent : où en est l'ouverture, où est le pointeur.
#[derive(Default)]
pub struct Etat {
    /// Avancée du défilement dans l'ouverture, de 0 à 1.
    pub ouverture: Cell<f64>,
    /// Position du pointeur dans la fenêtre ; loin hors champ tant qu'il n'a pas bougé.
    pub pointeur: Cell<(f64, f64)>,
}

pub fn demarrer() {
    let calme = outils::mouvement_reduit();
    let etat = Rc::new(Etat {
        ouverture: Cell::new(0.0),
        pointeur: Cell::new((-1e4, -1e4)),
    });

    let toque = poussiere::installer(etat.clone(), calme);

    if calme {
        rideau::lever(true, || {});
        return;
    }

    let defile = defilement::installer(etat.clone(), toque);
    if outils::pointeur_fin() {
        pointeur::installer(etat);
    }
    // Les révélations attendent que le rideau soit levé : sinon le titre
    // monterait derrière lui, sans être vu.
    rideau::lever(false, move || {
        revelation::installer();
        defile.relancer();
    });
}
