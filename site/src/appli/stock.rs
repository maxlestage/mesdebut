//! Ce que l'application garde sur l'appareil : la mémoire de l'apprenant, la
//! journée planifiée et l'historique des estimations.
//!
//! La mémoire est faite de lignes qu'on met à jour une à une ; la journée est
//! un petit document remplacé en bloc. Les deux vivent dans le `localStorage`,
//! chacune sous sa clé, comme l'application iOS les sépare entre SQLite et
//! les réglages.

use std::cell::RefCell;

use super::plateforme;
use crate::moteur::journee::{self, JourEstime, Journee};
use crate::moteur::memoire::Memoire;

const CLE_MEMOIRE: &str = "reneuro-memoire";
const CLE_JOURNEE: &str = "reneuro-journee";
const CLE_ESTIMATIONS: &str = "reneuro-estimations";

thread_local! {
    static MEMOIRE: RefCell<Option<Memoire>> = const { RefCell::new(None) };
}

/// Lit la mémoire (chargée au premier accès).
pub fn memoire<R>(f: impl FnOnce(&Memoire) -> R) -> R {
    MEMOIRE.with(|m| {
        let mut m = m.borrow_mut();
        f(m.get_or_insert_with(charger_memoire))
    })
}

/// Change la mémoire, puis l'enregistre.
pub fn modifier_memoire<R>(f: impl FnOnce(&mut Memoire) -> R) -> R {
    MEMOIRE.with(|m| {
        let mut m = m.borrow_mut();
        let memoire = m.get_or_insert_with(charger_memoire);
        let r = f(memoire);
        plateforme::ecrire(CLE_MEMOIRE, &memoire.exporter());
        r
    })
}

fn charger_memoire() -> Memoire {
    plateforme::lire(CLE_MEMOIRE)
        .map(|t| Memoire::importer(&t))
        .unwrap_or_default()
}

/// La journée du jour et l'historique des estimations. Au changement de date,
/// la journée révolue part à l'historique, qui est enregistré aussitôt.
pub fn charger_journee() -> (Journee, Vec<JourEstime>) {
    let journee = plateforme::lire(CLE_JOURNEE);
    let historique = plateforme::lire(CLE_ESTIMATIONS);
    let (j, h) = journee::charger(
        journee.as_deref(),
        historique.as_deref(),
        &plateforme::aujourdhui(),
    );
    if journee.as_deref().is_some_and(|t| !t.starts_with(&j.date)) {
        enregistrer_historique(&h);
        enregistrer_journee(&j);
    }
    (j, h)
}

pub fn enregistrer_journee(j: &Journee) {
    plateforme::ecrire(CLE_JOURNEE, &j.exporter());
}

fn enregistrer_historique(h: &[JourEstime]) {
    plateforme::ecrire(CLE_ESTIMATIONS, &journee::exporter_historique(h));
}
