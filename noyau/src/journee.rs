//! « Ma journée » : le planificateur, son chronomètre et le suivi des estimations.
//!
//! On écrit ses tâches ou on prend une activité connue, on les ordonne, on les
//! coche. Chaque tâche démarre quand la précédente se termine : l'heure de
//! chacune s'affiche, et la journée se voit déborder. Le chronomètre compare le
//! temps prévu au temps réellement passé ; c'est cet écart qu'on travaille.
//!
//! Même logique que `PlannerView.swift` de l'application iOS.

use super::memoire::propre;
use super::planifier::{ajoute_minutes, format_heure};

/// Jours d'estimations gardés : c'est une tendance qu'on regarde, pas une archive.
const HISTORIQUE_MAX: usize = 60;

pub const DUREES: [u32; 7] = [5, 10, 15, 20, 30, 45, 60];

#[derive(Clone, Debug, PartialEq)]
pub struct Tache {
    pub id: u32,
    pub libelle: String,
    /// Durée prévue, en minutes.
    pub duree: u32,
    pub fait: bool,
    /// L'activité connue d'où vient la tâche, pour en dérouler les étapes.
    pub routine: Option<String>,
    /// Instant du démarrage (millisecondes depuis 1970) tant que le chrono tourne.
    pub demarre_a: Option<f64>,
    /// Minutes réellement passées, une fois le chrono arrêté.
    pub reel: Option<u32>,
}

impl Tache {
    /// Minutes écoulées depuis le démarrage, arrondies à la plus proche.
    pub fn ecoulees(&self, maintenant_ms: f64) -> u32 {
        self.demarre_a.map_or(0, |debut| {
            ((maintenant_ms - debut) / 60_000.0).round().max(0.0) as u32
        })
    }

    /// L'estimation était-elle juste, à 20 % près (2 minutes au moins) ? `None`
    /// tant que la tâche n'a pas été chronométrée.
    pub fn estimation_juste(&self) -> Option<bool> {
        let reel = self.reel?;
        let marge = (self.duree as f64 * 0.2).max(2.0);
        Some((reel as f64 - self.duree as f64).abs() <= marge)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Journee {
    /// `aaaa-mm-jj` : la journée repart à zéro le lendemain.
    pub date: String,
    pub debut_h: u32,
    pub debut_m: u32,
    pub taches: Vec<Tache>,
}

/// Le prévu et le réel d'une journée, pour les tâches chronométrées.
#[derive(Clone, Debug, PartialEq)]
pub struct JourEstime {
    pub date: String,
    pub nb: u32,
    pub prevu: u32,
    pub reel: u32,
}

impl JourEstime {
    /// Écart en pour cent entre le réel et le prévu.
    pub fn ecart_pct(&self) -> i32 {
        if self.prevu == 0 {
            0
        } else {
            ((self.reel as f64 - self.prevu as f64) / self.prevu as f64 * 100.0).round() as i32
        }
    }

    pub fn juste(&self) -> bool {
        self.ecart_pct().abs() <= 10
    }
}

impl Journee {
    pub fn new(date: &str) -> Self {
        Journee {
            date: date.to_string(),
            debut_h: 9,
            debut_m: 0,
            taches: Vec::new(),
        }
    }

    /// L'heure de début de chaque tâche : chacune commence quand la précédente finit.
    pub fn horaires(&self) -> Vec<String> {
        let (mut h, mut m) = (self.debut_h, self.debut_m);
        self.taches
            .iter()
            .map(|t| {
                let debut = format_heure(h, m);
                (h, m) = ajoute_minutes(h, m, t.duree as i32);
                debut
            })
            .collect()
    }

    pub fn total(&self) -> u32 {
        self.taches.iter().map(|t| t.duree).sum()
    }

    pub fn fin(&self) -> (u32, u32) {
        ajoute_minutes(self.debut_h, self.debut_m, self.total() as i32)
    }

    pub fn faites(&self) -> usize {
        self.taches.iter().filter(|t| t.fait).count()
    }

    pub fn chrono_en_cours(&self) -> bool {
        self.taches.iter().any(|t| t.demarre_a.is_some())
    }

    /// Ajoute une tâche ; `false` si son libellé est vide.
    pub fn ajoute(&mut self, libelle: &str, duree: u32, routine: Option<&str>) -> bool {
        let libelle = libelle.trim();
        if libelle.is_empty() {
            return false;
        }
        let id = self.taches.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        self.taches.push(Tache {
            id,
            libelle: libelle.to_string(),
            duree,
            fait: false,
            routine: routine.map(str::to_string),
            demarre_a: None,
            reel: None,
        });
        true
    }

    fn arrete_tache(t: &mut Tache, maintenant_ms: f64) {
        if t.demarre_a.is_some() {
            t.reel = Some(t.ecoulees(maintenant_ms));
            t.demarre_a = None;
        }
    }

    /// Démarre le chrono d'une tâche, et arrête celui qui tournait : on fait une
    /// chose à la fois, c'est justement ce qu'on apprend ici.
    pub fn demarre(&mut self, id: u32, maintenant_ms: f64) {
        for t in &mut self.taches {
            Self::arrete_tache(t, maintenant_ms);
        }
        if let Some(t) = self.taches.iter_mut().find(|t| t.id == id) {
            t.demarre_a = Some(maintenant_ms);
        }
    }

    pub fn arrete(&mut self, id: u32, maintenant_ms: f64) {
        if let Some(t) = self.taches.iter_mut().find(|t| t.id == id) {
            Self::arrete_tache(t, maintenant_ms);
        }
    }

    /// Coche ou décoche. Cocher une tâche en cours arrête son chrono.
    pub fn bascule(&mut self, id: u32, maintenant_ms: f64) {
        if let Some(t) = self.taches.iter_mut().find(|t| t.id == id) {
            if !t.fait {
                Self::arrete_tache(t, maintenant_ms);
            }
            t.fait = !t.fait;
        }
    }

    /// Échange une tâche avec sa voisine (`sens` = -1 vers le haut, 1 vers le bas).
    pub fn deplace(&mut self, id: u32, sens: i32) {
        let Some(i) = self.taches.iter().position(|t| t.id == id) else {
            return;
        };
        let j = i as i32 + sens;
        if (0..self.taches.len() as i32).contains(&j) {
            self.taches.swap(i, j as usize);
        }
    }

    pub fn retire(&mut self, id: u32) {
        self.taches.retain(|t| t.id != id);
    }

    pub fn bilan(&self) -> JourEstime {
        let mesurees: Vec<&Tache> = self.taches.iter().filter(|t| t.reel.is_some()).collect();
        JourEstime {
            date: self.date.clone(),
            nb: mesurees.len() as u32,
            prevu: mesurees.iter().map(|t| t.duree).sum(),
            reel: mesurees.iter().filter_map(|t| t.reel).sum(),
        }
    }

    /// Le bilan des estimations du jour, formulé sans jamais reprocher un
    /// dépassement : le voir suffit à progresser.
    pub fn bilan_estimation(&self) -> Option<String> {
        let b = self.bilan();
        if b.nb == 0 {
            return None;
        }
        let s = if b.nb > 1 { "s" } else { "" };
        let debut = format!(
            "Sur {} tâche{s} chronométrée{s} : {} min prévues, {} min réelles",
            b.nb, b.prevu, b.reel
        );
        let ecart = b.reel as f64 - b.prevu as f64;
        Some(if ecart.abs() <= (b.prevu as f64 * 0.15).max(3.0) {
            format!("{debut} — tes estimations sont justes ! 🎯")
        } else if ecart > 0.0 {
            format!("{debut} — tu as tendance à sous-estimer, c'est très courant. 🌱")
        } else {
            format!("{debut} — tu prends moins de temps que prévu.")
        })
    }

    /// Première ligne : la date et l'heure de début ; puis une ligne par tâche.
    pub fn exporter(&self) -> String {
        let mut out = format!("{}\t{}\t{}\n", self.date, self.debut_h, self.debut_m);
        for t in &self.taches {
            out.push_str(&format!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                t.id,
                propre(&t.libelle),
                t.duree,
                u8::from(t.fait),
                t.routine.as_deref().unwrap_or(""),
                t.demarre_a.map(|d| d.to_string()).unwrap_or_default(),
                t.reel.map(|r| r.to_string()).unwrap_or_default(),
            ));
        }
        out
    }

    pub fn importer(texte: &str) -> Option<Self> {
        let mut lignes = texte.lines();
        let entete: Vec<&str> = lignes.next()?.split('\t').collect();
        let [date, h, m] = entete.as_slice() else {
            return None;
        };
        let mut journee = Journee {
            date: date.to_string(),
            debut_h: h.parse().ok().filter(|h| *h < 24)?,
            debut_m: m.parse().ok().filter(|m| *m < 60)?,
            taches: Vec::new(),
        };
        for ligne in lignes {
            let champs: Vec<&str> = ligne.split('\t').collect();
            let [id, libelle, duree, fait, routine, demarre, reel] = champs.as_slice() else {
                continue;
            };
            let (Ok(id), Ok(duree)) = (id.parse(), duree.parse()) else {
                continue;
            };
            journee.taches.push(Tache {
                id,
                libelle: libelle.to_string(),
                duree,
                fait: *fait == "1",
                routine: (!routine.is_empty()).then(|| routine.to_string()),
                demarre_a: demarre.parse().ok(),
                reel: reel.parse().ok(),
            });
        }
        Some(journee)
    }
}

/// La journée gardée, et l'historique des estimations mis à jour : au changement
/// de date, la journée révolue part à l'historique et une journée vide commence.
pub fn charger(
    journee: Option<&str>,
    historique: Option<&str>,
    aujourdhui: &str,
) -> (Journee, Vec<JourEstime>) {
    let mut hist = historique.map(importer_historique).unwrap_or_default();
    let journee = match journee.and_then(Journee::importer) {
        Some(j) if j.date == aujourdhui => j,
        Some(ancienne) => {
            archiver(&mut hist, &ancienne);
            Journee::new(aujourdhui)
        }
        None => Journee::new(aujourdhui),
    };
    (journee, hist)
}

/// Range une journée révolue dans l'historique, si elle a été chronométrée.
pub fn archiver(hist: &mut Vec<JourEstime>, journee: &Journee) {
    archiver_jour(hist, journee.bilan());
}

/// Range le bilan d'une journée révolue dans l'historique, s'il a été
/// chronométré : un jour par date, dans l'ordre, les 60 derniers seulement.
pub fn archiver_jour(hist: &mut Vec<JourEstime>, jour: JourEstime) {
    if jour.nb == 0 {
        return;
    }
    hist.retain(|j| j.date != jour.date);
    hist.push(jour);
    hist.sort_by(|a, b| a.date.cmp(&b.date));
    if hist.len() > HISTORIQUE_MAX {
        hist.drain(..hist.len() - HISTORIQUE_MAX);
    }
}

/// Les jours passés, complétés par aujourd'hui tel qu'il va.
pub fn avec_aujourdhui(hist: &[JourEstime], journee: &Journee) -> Vec<JourEstime> {
    let mut jours: Vec<JourEstime> = hist
        .iter()
        .filter(|j| j.date != journee.date)
        .cloned()
        .collect();
    let ce_jour = journee.bilan();
    if ce_jour.nb > 0 {
        jours.push(ce_jour);
    }
    jours
}

/// Une barre de la tendance des estimations.
#[derive(Clone, Debug, PartialEq)]
pub struct Barre {
    pub date: String,
    /// « dim. 30 », ou « aujourd'hui ».
    pub libelle: String,
    pub ecart_pct: i32,
    pub juste: bool,
    /// Longueur de la barre, de 0 à 1 : toutes à la même échelle.
    pub part: f64,
    /// « +25 % », ou « ✓ » quand le prévu tombe juste.
    pub valeur: String,
}

/// Les barres des dix derniers jours, à la même échelle (60 % d'écart au moins,
/// pour qu'un petit écart ne remplisse pas la barre).
pub fn barres(jours: &[JourEstime], aujourdhui: &str) -> Vec<Barre> {
    let derniers = &jours[jours.len().saturating_sub(10)..];
    let pire = derniers
        .iter()
        .map(|j| j.ecart_pct().abs())
        .max()
        .unwrap_or(0)
        .max(60);
    derniers
        .iter()
        .map(|j| {
            let ecart = j.ecart_pct();
            Barre {
                date: j.date.clone(),
                libelle: date_courte(&j.date, aujourdhui),
                ecart_pct: ecart,
                juste: j.juste(),
                part: (ecart.abs() as f64 / pire as f64).min(1.0),
                valeur: if ecart == 0 {
                    "✓".to_string()
                } else {
                    format!("{} %", signe(ecart))
                },
            }
        })
        .collect()
}

pub fn exporter_historique(hist: &[JourEstime]) -> String {
    hist.iter()
        .map(|j| format!("{}\t{}\t{}\t{}\n", j.date, j.nb, j.prevu, j.reel))
        .collect()
}

pub fn importer_historique(texte: &str) -> Vec<JourEstime> {
    texte
        .lines()
        .filter_map(|ligne| {
            let champs: Vec<&str> = ligne.split('\t').collect();
            let [date, nb, prevu, reel] = champs.as_slice() else {
                return None;
            };
            Some(JourEstime {
                date: date.to_string(),
                nb: nb.parse().ok()?,
                prevu: prevu.parse().ok()?,
                reel: reel.parse().ok()?,
            })
        })
        .collect()
}

fn moyenne(jours: &[JourEstime]) -> Option<i32> {
    let prevu: u32 = jours.iter().map(|j| j.prevu).sum();
    let reel: u32 = jours.iter().map(|j| j.reel).sum();
    (prevu > 0).then(|| ((reel as f64 - prevu as f64) / prevu as f64 * 100.0).round() as i32)
}

pub fn signe(n: i32) -> String {
    if n > 0 {
        format!("+{n}")
    } else {
        n.to_string()
    }
}

/// Ce que disent les derniers jours : les cinq récents comparés aux cinq d'avant.
pub fn tendance(jours: &[JourEstime]) -> String {
    let n = jours.len();
    let recents = &jours[n.saturating_sub(5)..];
    let avant = &jours[n.saturating_sub(10)..n.saturating_sub(5)];
    let Some(m_recents) = moyenne(recents) else {
        return String::new();
    };
    let s = if recents.len() > 1 { "s" } else { "" };
    let mut texte = format!(
        "Sur {} jour{s} : {} % d'écart en moyenne",
        recents.len(),
        signe(m_recents)
    );
    if m_recents.abs() <= 10 {
        texte.push_str(" — tes estimations sont fiables 🎯");
    } else if let Some(m_avant) = moyenne(avant).filter(|m| m_recents.abs() < m.abs()) {
        texte.push_str(&format!(
            " (contre {} % avant) — tu progresses ! 🌱",
            signe(m_avant)
        ));
    } else if m_recents > 0 {
        texte.push_str(" — tu sous-estimes encore un peu");
    }
    texte
}

// ---------------------------------------------------------------------------
// Dates

pub const JOURS_SEMAINE: [&str; 7] = [
    "lundi", "mardi", "mercredi", "jeudi", "vendredi", "samedi", "dimanche",
];
const JOURS_COURTS: [&str; 7] = ["lun.", "mar.", "mer.", "jeu.", "ven.", "sam.", "dim."];
const MOIS_NOMS: [&str; 12] = [
    "janvier",
    "février",
    "mars",
    "avril",
    "mai",
    "juin",
    "juillet",
    "août",
    "septembre",
    "octobre",
    "novembre",
    "décembre",
];

/// Le jour de la semaine d'une date du calendrier grégorien, 0 = lundi.
pub fn jour_semaine(annee: i32, mois: u32, jour: u32) -> usize {
    // algorithme de Sakamoto (0 = dimanche), ramené à 0 = lundi
    const T: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let a = if mois < 3 { annee - 1 } else { annee };
    let d = (a + a / 4 - a / 100 + a / 400 + T[(mois - 1) as usize] + jour as i32).rem_euclid(7);
    ((d + 6) % 7) as usize
}

fn lire_date(date: &str) -> Option<(i32, u32, u32)> {
    let mut parts = date.split('-');
    let a = parts.next()?.parse().ok()?;
    let m: u32 = parts.next()?.parse().ok()?;
    let j: u32 = parts.next()?.parse().ok()?;
    ((1..=12).contains(&m) && (1..=31).contains(&j)).then_some((a, m, j))
}

pub fn iso(annee: i32, mois: u32, jour: u32) -> String {
    format!("{annee:04}-{mois:02}-{jour:02}")
}

/// « lundi 12 octobre »
pub fn date_lisible(date: &str) -> String {
    match lire_date(date) {
        Some((a, m, j)) => format!(
            "{} {j} {}",
            JOURS_SEMAINE[jour_semaine(a, m, j)],
            MOIS_NOMS[(m - 1) as usize]
        ),
        None => date.to_string(),
    }
}

/// « dim. 30 », ou « aujourd'hui ».
pub fn date_courte(date: &str, aujourdhui: &str) -> String {
    if date == aujourdhui {
        return "aujourd'hui".into();
    }
    match lire_date(date) {
        Some((a, m, j)) => format!("{} {j}", JOURS_COURTS[jour_semaine(a, m, j)]),
        None => date.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MIN: f64 = 60_000.0;

    fn journee() -> Journee {
        let mut j = Journee::new("2026-10-12");
        j.ajoute("Se faire un café", 5, Some("cafe"));
        j.ajoute("  Lire  ", 30, None);
        assert!(!j.ajoute("   ", 10, None));
        j
    }

    #[test]
    fn chaque_tache_commence_quand_la_precedente_finit() {
        let mut j = journee();
        j.debut_h = 23;
        j.debut_m = 50;
        assert_eq!(j.horaires(), ["23 h 50", "23 h 55"]);
        assert_eq!(j.fin(), (0, 25));
        assert_eq!(j.taches[1].libelle, "Lire");
    }

    #[test]
    fn une_chose_a_la_fois() {
        let mut j = journee();
        j.demarre(1, 0.0);
        j.demarre(2, 9.0 * MIN);
        assert_eq!(j.taches[0].reel, Some(9));
        assert_eq!(j.taches[0].demarre_a, None);
        assert_eq!(j.taches[0].estimation_juste(), Some(false));
        // cocher une tâche en cours l'arrête
        j.bascule(2, 40.0 * MIN);
        assert!(j.taches[1].fait);
        assert_eq!(j.taches[1].reel, Some(31));
        assert_eq!(j.taches[1].estimation_juste(), Some(true));
        assert!(!j.chrono_en_cours());
        let b = j.bilan();
        assert_eq!((b.nb, b.prevu, b.reel), (2, 35, 40));
        assert!(j.bilan_estimation().unwrap().contains("justes"));
    }

    #[test]
    fn deplacer_et_retirer() {
        let mut j = journee();
        j.deplace(2, -1);
        assert_eq!(j.taches[0].id, 2);
        j.deplace(2, -1); // déjà en tête : rien ne bouge
        assert_eq!(j.taches[0].id, 2);
        j.retire(2);
        assert_eq!(j.taches.len(), 1);
    }

    #[test]
    fn la_journee_s_ecrit_et_se_relit() {
        let mut j = journee();
        j.demarre(1, 1234.5);
        j.arrete(2, 0.0);
        j.taches[1].libelle = "avec\ttab".into();
        let relue = Journee::importer(&j.exporter()).unwrap();
        assert_eq!(relue.taches[0], j.taches[0]);
        assert_eq!(relue.taches[1].libelle, "avec tab");
    }

    #[test]
    fn le_lendemain_la_journee_part_a_l_historique() {
        let mut hier = Journee::new("2026-10-11");
        hier.ajoute("Courses", 45, None);
        hier.demarre(1, 0.0);
        hier.arrete(1, 60.0 * MIN);
        let (j, hist) = charger(Some(&hier.exporter()), None, "2026-10-12");
        assert_eq!(j, Journee::new("2026-10-12"));
        assert_eq!(
            hist,
            [JourEstime {
                date: "2026-10-11".into(),
                nb: 1,
                prevu: 45,
                reel: 60
            }]
        );
        assert_eq!(importer_historique(&exporter_historique(&hist)), hist);
        // une journée jamais chronométrée ne laisse pas de trace
        let (_, hist) = charger(
            Some(&Journee::new("2026-10-10").exporter()),
            None,
            "2026-10-12",
        );
        assert!(hist.is_empty());
    }

    #[test]
    fn la_tendance_compare_les_cinq_derniers_jours_aux_cinq_d_avant() {
        let jour = |i: u32, reel| JourEstime {
            date: iso(2026, 10, i),
            nb: 1,
            prevu: 100,
            reel,
        };
        let mut jours: Vec<_> = (1..=5).map(|i| jour(i, 150)).collect();
        jours.extend((6..=10).map(|i| jour(i, 125)));
        assert_eq!(
            tendance(&jours),
            "Sur 5 jours : +25 % d'écart en moyenne (contre +50 % avant) — tu progresses ! 🌱"
        );
        assert!(tendance(&[jour(1, 105)]).ends_with("fiables 🎯"));
    }

    #[test]
    fn les_barres_partagent_une_echelle() {
        let jour = |d: u32, reel| JourEstime {
            date: iso(2026, 10, d),
            nb: 1,
            prevu: 100,
            reel,
        };
        let jours: Vec<_> = (1..=12).map(|d| jour(d, 100 + d * 10)).collect();
        let b = barres(&jours, "2026-10-12");
        assert_eq!(b.len(), 10, "les dix derniers jours");
        assert_eq!(b[0].date, "2026-10-03");
        assert_eq!(b[9].libelle, "aujourd'hui");
        assert_eq!(
            (b[9].ecart_pct, b[9].part, b[9].valeur.as_str()),
            (120, 1.0, "+120 %")
        );
        assert_eq!(b[0].part, 0.25);
        assert_eq!(barres(&[jour(1, 100)], "x")[0].valeur, "✓");
    }

    #[test]
    fn dates() {
        assert_eq!(jour_semaine(2026, 10, 12), 0); // un lundi
        assert_eq!(jour_semaine(2000, 1, 1), 5); // un samedi
        assert_eq!(date_lisible("2026-10-12"), "lundi 12 octobre");
        assert_eq!(date_courte("2026-08-30", "2026-10-12"), "dim. 30");
        assert_eq!(date_courte("2026-10-12", "2026-10-12"), "aujourd'hui");
    }
}
