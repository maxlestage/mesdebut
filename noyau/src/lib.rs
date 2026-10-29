//! Le noyau de ReNeuro : les thèmes, les questions, la mémoire de l'apprenant
//! et la journée planifiée.
//!
//! Il n'existe qu'en un exemplaire, et sert partout :
//!
//! - le **web** (`site/`) l'utilise directement, compilé en WebAssembly ;
//! - l'**iPhone** et l'**Apple Watch** (`ios/`) l'appellent par son interface C
//!   ([`ffi`], fonctionnalité `ffi`), compilé en bibliothèque statique par
//!   `xcode.sh` pendant la construction Xcode.
//!
//! Du Rust pur, sans dépendance : il se teste avec `cargo test`. Ce qui reste
//! propre à chaque plateforme, c'est l'interface et le stockage (SQLite et les
//! réglages sur l'iPhone, `localStorage` dans le navigateur).

pub mod alea;
pub mod casse_tete;
pub mod categories;
pub mod donnees;
pub mod hanoi;
pub mod journee;
pub mod logique;
pub mod memoire;
pub mod planifier;
pub mod question;
pub mod questions;
pub mod revision;

#[cfg(feature = "ffi")]
pub mod ffi;

/// Les étoiles et le mot de la fin d'un quiz : on encourage, on ne sanctionne pas.
pub fn mot_de_la_fin(score: u32, total: u32) -> (&'static str, &'static str) {
    let ratio = if total == 0 {
        0.0
    } else {
        score as f64 / total as f64
    };
    if ratio >= 1.0 {
        ("⭐⭐⭐⭐⭐", "Score parfait, tu es un champion ! 🏆")
    } else if ratio >= 0.8 {
        ("⭐⭐⭐⭐", "Excellent travail, continue comme ça ! 🎉")
    } else if ratio >= 0.6 {
        ("⭐⭐⭐", "Bien joué ! Encore un petit effort ! 💪")
    } else if ratio >= 0.4 {
        ("⭐⭐", "Pas mal ! Entraîne-toi encore un peu ! 🙂")
    } else {
        (
            "⭐",
            "Courage, réessaie : c'est en s'entraînant qu'on apprend ! 🌱",
        )
    }
}

/// La marque du pluriel : `format!("{n} notion{s} maîtrisée{s}", s = s(n))`.
pub fn s(n: u32) -> &'static str {
    if n > 1 { "s" } else { "" }
}

/// « 3 notions maîtrisées », « 1 notion maîtrisée »
pub fn notions_maitrisees(n: u32) -> String {
    format!("{n} notion{s} maîtrisée{s}", s = s(n))
}
