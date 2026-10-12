//! ReNeuro sur le web, écrit avec [Active](https://github.com/maxlestage/Active).
//!
//! Deux pages, une seule bibliothèque :
//!
//! - la **présentation** (`/`), rendue en HTML par l'exportateur
//!   (`src/bin/exporter.rs`) puis animée dans le navigateur par `mouvement` ;
//! - l'**application** (`/app`), rendue entièrement dans le navigateur par
//!   Active, avec son routeur, sa mémoire et son planificateur.
//!
//! Le même module WebAssembly sert les deux : au démarrage, il regarde sur
//! quelle page il se trouve.

pub mod appli;
pub mod moteur;
pub mod vitrine;

#[cfg(target_arch = "wasm32")]
mod mouvement;
#[cfg(target_arch = "wasm32")]
mod outils;

/// Point d'entrée du module WebAssembly.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn demarrer() {
    // le hasard des questions change à chaque visite
    moteur::alea::amorcer((js_sys::Math::random() * u64::MAX as f64) as u64);
    if outils::document().get_element_by_id("app").is_some() {
        appli::lancer();
    } else {
        vitrine::animer();
    }
}
