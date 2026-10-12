//! La mémoire de l'apprenant : ce qu'il sait, ce qui résiste.
//!
//! Chaque notion suit un système de boîtes de Leitner : une bonne réponse la
//! fait monter d'une boîte (révision plus espacée), une erreur la fait
//! redescendre. C'est de la répétition espacée qui survit d'une séance à
//! l'autre, et d'un jour à l'autre.
//!
//! Même modèle que la base SQLite de l'application iOS (`MemoryStore.swift`).
//! Ici, la mémoire s'écrit en texte, une ligne par notion, pour tenir dans le
//! `localStorage` du navigateur sans dépendance de sérialisation.

use std::collections::{BTreeMap, HashMap};

use super::questions::EtatNotion;

/// Intervalles de révision en jours, par boîte (1 à 5).
const INTERVALLES: [f64; 5] = [1.0, 2.0, 4.0, 7.0, 15.0];
const JOUR: f64 = 86_400.0;
/// Une notion est maîtrisée à partir de cette boîte.
pub const MAITRISE: u8 = 4;

/// Ce que devient une notion après une réponse : sa boîte, sa série de bonnes
/// réponses, et sa prochaine révision (secondes depuis 1970). Une bonne réponse
/// la fait monter d'une boîte (jusqu'à 5), une erreur redescendre (jusqu'à 1).
///
/// C'est toute la règle de la répétition espacée : la mémoire web s'en sert, et
/// la base SQLite de l'iPhone aussi, par l'interface C.
pub fn apres_reponse(boite: u8, serie: u32, juste: bool, maintenant: f64) -> (u8, u32, f64) {
    let (boite, serie) = if juste {
        ((boite.max(1) + 1).min(5), serie + 1)
    } else {
        (boite.saturating_sub(1).max(1), 0)
    };
    (
        boite,
        serie,
        maintenant + INTERVALLES[(boite - 1) as usize] * JOUR,
    )
}
/// Séances gardées : de quoi suivre une tendance, sans remplir le stockage.
const SEANCES_MAX: usize = 500;

#[derive(Clone, Debug, PartialEq)]
pub struct Notion {
    pub categorie: String,
    pub vues: u32,
    pub justes: u32,
    pub fausses: u32,
    pub serie: u32,
    pub boite: u8,
    /// Prochaine révision, en secondes depuis 1970.
    pub due: f64,
    pub derniere: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Seance {
    pub categorie: String,
    pub score: u32,
    pub total: u32,
    pub date: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Memoire {
    pub notions: BTreeMap<String, Notion>,
    pub seances: Vec<Seance>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Resume {
    /// Réponses données en tout.
    pub repondues: u32,
    pub justes: u32,
    /// Notions bien ancrées (boîte ≥ 4).
    pub maitrisees: u32,
    /// Notions dont la date de révision est passée.
    pub a_reviser: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StatTheme {
    pub vues: u32,
    pub justes: u32,
    pub maitrisees: u32,
}

impl StatTheme {
    /// Taux de réussite, en pour cent.
    pub fn reussite(&self) -> u32 {
        if self.vues == 0 {
            0
        } else {
            (self.justes as f64 / self.vues as f64 * 100.0).round() as u32
        }
    }
}

impl Memoire {
    /// Note une réponse et déplace la notion d'une boîte. `maintenant` en secondes.
    pub fn note(&mut self, cle: &str, categorie: &str, juste: bool, maintenant: f64) {
        let notion = self
            .notions
            .entry(cle.to_string())
            .or_insert_with(|| Notion {
                categorie: categorie.to_string(),
                vues: 0,
                justes: 0,
                fausses: 0,
                serie: 0,
                boite: 1,
                due: 0.0,
                derniere: 0.0,
            });
        notion.vues += 1;
        if juste {
            notion.justes += 1;
        } else {
            notion.fausses += 1;
        }
        (notion.boite, notion.serie, notion.due) =
            apres_reponse(notion.boite, notion.serie, juste, maintenant);
        notion.derniere = maintenant;
    }

    /// Garde le résultat d'un quiz terminé.
    pub fn termine(&mut self, categorie: &str, score: u32, total: u32, maintenant: f64) {
        self.seances.push(Seance {
            categorie: categorie.to_string(),
            score,
            total,
            date: maintenant,
        });
        if self.seances.len() > SEANCES_MAX {
            self.seances.drain(..self.seances.len() - SEANCES_MAX);
        }
    }

    pub fn resume(&self, maintenant: f64) -> Resume {
        let mut r = Resume::default();
        for n in self.notions.values() {
            r.repondues += n.vues;
            r.justes += n.justes;
            r.maitrisees += u32::from(n.boite >= MAITRISE);
            r.a_reviser += u32::from(n.due > 0.0 && n.due <= maintenant);
        }
        r
    }

    /// Statistiques par thème, pour l'écran des progrès.
    pub fn par_theme(&self) -> HashMap<String, StatTheme> {
        let mut out: HashMap<String, StatTheme> = HashMap::new();
        for n in self.notions.values() {
            let s = out.entry(n.categorie.clone()).or_default();
            s.vues += n.vues;
            s.justes += n.justes;
            s.maitrisees += u32::from(n.boite >= MAITRISE);
        }
        out
    }

    /// Ce que la mémoire sait de chaque notion d'un thème, par clé complète.
    pub fn etats(&self, categorie: &str) -> HashMap<String, EtatNotion> {
        self.notions
            .iter()
            .filter(|(_, n)| n.categorie == categorie)
            .map(|(cle, n)| {
                (
                    cle.clone(),
                    EtatNotion {
                        boite: n.boite,
                        due: n.due,
                    },
                )
            })
            .collect()
    }

    /// Une ligne par notion (`n`) et par séance (`s`), champs séparés par des tabulations.
    pub fn exporter(&self) -> String {
        let mut out = String::new();
        for (cle, n) in &self.notions {
            out.push_str(&format!(
                "n\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                propre(cle),
                propre(&n.categorie),
                n.vues,
                n.justes,
                n.fausses,
                n.serie,
                n.boite,
                n.due,
                n.derniere
            ));
        }
        for s in &self.seances {
            out.push_str(&format!(
                "s\t{}\t{}\t{}\t{}\n",
                propre(&s.categorie),
                s.score,
                s.total,
                s.date
            ));
        }
        out
    }

    /// Relit ce qu'a écrit [`exporter`](Memoire::exporter). Une ligne illisible
    /// est ignorée : mieux vaut perdre une notion que toute la mémoire.
    pub fn importer(texte: &str) -> Self {
        let mut m = Memoire::default();
        for ligne in texte.lines() {
            let champs: Vec<&str> = ligne.split('\t').collect();
            match champs.as_slice() {
                [
                    "n",
                    cle,
                    categorie,
                    vues,
                    justes,
                    fausses,
                    serie,
                    boite,
                    due,
                    derniere,
                ] => {
                    let lu = (|| {
                        Some(Notion {
                            categorie: categorie.to_string(),
                            vues: vues.parse().ok()?,
                            justes: justes.parse().ok()?,
                            fausses: fausses.parse().ok()?,
                            serie: serie.parse().ok()?,
                            boite: boite.parse::<u8>().ok()?.clamp(1, 5),
                            due: due.parse().ok()?,
                            derniere: derniere.parse().ok()?,
                        })
                    })();
                    if let Some(n) = lu {
                        m.notions.insert(cle.to_string(), n);
                    }
                }
                ["s", categorie, score, total, date] => {
                    if let (Ok(score), Ok(total), Ok(date)) =
                        (score.parse(), total.parse(), date.parse())
                    {
                        m.seances.push(Seance {
                            categorie: categorie.to_string(),
                            score,
                            total,
                            date,
                        });
                    }
                }
                _ => {}
            }
        }
        m
    }
}

/// Une valeur sans tabulation ni saut de ligne, qui casseraient le format.
pub(crate) fn propre(texte: &str) -> String {
    texte.replace(['\t', '\n', '\r'], " ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn les_boites_montent_et_descendent() {
        let mut m = Memoire::default();
        for _ in 0..6 {
            m.note("jours:x", "jours", true, 1000.0);
        }
        let n = &m.notions["jours:x"];
        assert_eq!(n.boite, 5);
        assert_eq!(n.due, 1000.0 + 15.0 * JOUR);
        m.note("jours:x", "jours", false, 2000.0);
        let n = &m.notions["jours:x"];
        assert_eq!((n.boite, n.serie, n.fausses, n.vues), (4, 0, 1, 7));
        m.note("jours:y", "jours", false, 0.0);
        assert_eq!(
            m.notions["jours:y"].boite, 1,
            "jamais sous la première boîte"
        );
    }

    #[test]
    fn resume_et_themes() {
        let mut m = Memoire::default();
        for _ in 0..3 {
            m.note("jours:a", "jours", true, 0.0);
        }
        m.note("mois:b", "mois", false, 0.0);
        let r = m.resume(JOUR * 1.5);
        assert_eq!(
            r,
            Resume {
                repondues: 4,
                justes: 3,
                maitrisees: 1,
                a_reviser: 1
            }
        );
        let t = m.par_theme();
        assert_eq!(t["jours"].reussite(), 100);
        assert_eq!(t["mois"].reussite(), 0);
    }

    #[test]
    fn s_ecrit_et_se_relit() {
        let mut m = Memoire::default();
        m.note("heure:heure:3:15", "heure", true, 12.5);
        m.note(
            "planifier:plan:intrus:cafe:Se laver",
            "planifier",
            false,
            13.0,
        );
        m.termine("heure", 8, 12, 99.0);
        let relue = Memoire::importer(&m.exporter());
        assert_eq!(relue, m);
        assert_eq!(
            Memoire::importer("n\tcassée\n\ns\tx\t1\t2\t3"),
            Memoire {
                notions: BTreeMap::new(),
                seances: vec![Seance {
                    categorie: "x".into(),
                    score: 1,
                    total: 2,
                    date: 3.0
                }],
            }
        );
    }
}
