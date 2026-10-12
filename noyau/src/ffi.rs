//! L'interface C du noyau, pour l'iPhone et la montre (fonctionnalité `ffi`).
//!
//! Deux fonctions seulement, déclarées dans `include/reneuro.h` :
//!
//! ```c
//! char *reneuro_appel(const char *fonction, const char *arguments);
//! void reneuro_liberer(char *reponse);
//! ```
//!
//! `fonction` nomme ce qu'on demande (`"quiz"`, `"categories"`…). Les
//! `arguments` sont du texte : des champs séparés par des tabulations, des
//! enregistrements par des sauts de ligne — le format qu'écrit déjà la mémoire
//! web. La réponse est du JSON, que Swift lit avec `JSONDecoder` ; en cas
//! d'erreur, `{"erreur":"…"}`. Une interface étroite et du texte des deux côtés :
//! rien à maintenir en double quand le noyau s'enrichit.
//!
//! Chaque fonction est décrite dans [`appeler`].

use std::cell::Cell;
use std::collections::HashMap;
use std::ffi::{CStr, CString, c_char};
use std::hash::{BuildHasher, Hasher};

use crate::categories::CATEGORIES;
use crate::journee::{self, DUREES, JourEstime, Journee};
use crate::memoire::{self, MAITRISE};
use crate::planifier::ROUTINES;
use crate::question::{Illustration, Question};
use crate::questions::{self, EtatNotion, NB_QUESTIONS};
use crate::revision::{Fiche, Revision, revision};
use crate::{alea, mot_de_la_fin};

/// Appelle une fonction du noyau. Ne renvoie jamais NULL ; la réponse est à
/// rendre avec [`reneuro_liberer`].
///
/// # Safety
///
/// `fonction` et `arguments` sont NULL ou des chaînes C terminées par un zéro,
/// valides le temps de l'appel.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn reneuro_appel(
    fonction: *const c_char,
    arguments: *const c_char,
) -> *mut c_char {
    // SAFETY : garanti par l'appelant (voir la documentation de la fonction).
    let lire = |p: *const c_char| {
        if p.is_null() {
            String::new()
        } else {
            unsafe { CStr::from_ptr(p) }.to_string_lossy().into_owned()
        }
    };
    let (fonction, arguments) = (lire(fonction), lire(arguments));
    let reponse = std::panic::catch_unwind(|| appeler(&fonction, &arguments))
        .unwrap_or_else(|_| Err(format!("le noyau a échoué sur « {fonction} »")))
        .unwrap_or_else(|erreur| objet(&[("erreur", chaine(&erreur))]));
    // le JSON n'a jamais de zéro : les caractères de contrôle y sont échappés
    CString::new(reponse).unwrap_or_default().into_raw()
}

/// Rend la mémoire d'une réponse de [`reneuro_appel`].
///
/// # Safety
///
/// `reponse` est NULL, ou un pointeur rendu par [`reneuro_appel`] et pas encore libéré.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn reneuro_liberer(reponse: *mut c_char) {
    if !reponse.is_null() {
        // SAFETY : la chaîne vient de `CString::into_raw` (voir la documentation).
        drop(unsafe { CString::from_raw(reponse) });
    }
}

thread_local! {
    static AMORCE: Cell<bool> = const { Cell::new(false) };
}

/// Le hasard des questions change à chaque lancement : la première fois, le
/// générateur est amorcé avec des clés aléatoires du système.
fn amorcer_si_besoin() {
    if !AMORCE.with(|a| a.replace(true)) {
        let mut h = std::collections::hash_map::RandomState::new().build_hasher();
        h.write_u64(0x5EED);
        alea::amorcer(h.finish());
    }
}

/// Exécute une fonction : ce que fait réellement [`reneuro_appel`], en Rust.
///
/// | Fonction | Arguments | Réponse |
/// | --- | --- | --- |
/// | `amorcer` | `graine` | `null` (rejoue les mêmes tirages, pour les tests) |
/// | `constantes` | | `{nb_questions, maitrise, durees}` |
/// | `categories` | | les seize thèmes |
/// | `revision` | `cle` | ce que montre « Réviser d'abord », ou `null` |
/// | `quiz` | `categorie niveau maintenant`, puis une ligne `cle boite due` par notion connue | les questions |
/// | `bravo` | | une félicitation |
/// | `fin` | `score total` | `{etoiles, message}` |
/// | `retours` | `index nombre` | où revient une question ratée |
/// | `noter` | `boite serie juste maintenant` | `{boite, serie, due}` |
/// | `routines` | | les onze activités du quotidien |
/// | `journee` | `debut_h debut_m`, puis une ligne `duree fait reel` par tâche | horaires, fin, bilan |
/// | `ecoulees` | `debut_ms maintenant_ms` | minutes écoulées, arrondies |
/// | `archiver` | le jour `date nb prevu reel`, puis l'historique, une ligne par jour | l'historique |
/// | `tendance` | `aujourdhui`, puis une ligne `date nb prevu reel` par jour | `{barres, message}` |
/// | `date_lisible` | `aaaa-mm-jj` | « lundi 12 octobre » |
///
/// Les instants sont en secondes depuis 1970 (`maintenant`), sauf ceux du
/// chronomètre, en millisecondes. Les booléens s'écrivent `0` ou `1`.
pub fn appeler(fonction: &str, arguments: &str) -> Result<String, String> {
    amorcer_si_besoin();
    let mut lignes = arguments.lines();
    let mut tete = Champs::new(lignes.next().unwrap_or(""));
    Ok(match fonction {
        "amorcer" => {
            alea::amorcer(tete.nombre("graine")?);
            "null".into()
        }
        "constantes" => objet(&[
            ("nb_questions", NB_QUESTIONS.to_string()),
            ("maitrise", MAITRISE.to_string()),
            ("durees", tableau(DUREES.iter().map(u32::to_string))),
        ]),
        "categories" => tableau(CATEGORIES.iter().map(|c| {
            objet(&[
                ("cle", chaine(c.cle)),
                ("emoji", chaine(c.emoji)),
                ("libelle", chaine(c.libelle)),
                ("titre", chaine(c.titre)),
                ("revision", c.revision.to_string()),
                (
                    "niveaux",
                    tableau(c.niveaux.iter().map(|n| {
                        objet(&[
                            ("id", n.id.to_string()),
                            ("emoji", chaine(n.emoji)),
                            ("libelle", chaine(n.libelle)),
                        ])
                    })),
                ),
                (
                    "degrade",
                    tableau([chaine(c.degrade.0), chaine(c.degrade.1)]),
                ),
            ])
        })),
        "revision" => revision(tete.texte()).map_or_else(|| "null".into(), |r| revision_json(&r)),
        "quiz" => {
            let categorie = tete.texte().to_string();
            let niveau: u8 = tete.nombre("niveau")?;
            let maintenant: f64 = tete.nombre("maintenant")?;
            let mut etats = HashMap::new();
            for ligne in lignes {
                let mut c = Champs::new(ligne);
                let cle = c.texte().to_string();
                etats.insert(
                    cle,
                    EtatNotion {
                        boite: c.nombre("boite")?,
                        due: c.nombre("due")?,
                    },
                );
            }
            tableau(
                questions::quiz(&categorie, niveau, &etats, maintenant)
                    .iter()
                    .map(question_json),
            )
        }
        "bravo" => chaine(questions::bravo()),
        "fin" => {
            let (etoiles, message) = mot_de_la_fin(tete.nombre("score")?, tete.nombre("total")?);
            objet(&[("etoiles", chaine(etoiles)), ("message", chaine(message))])
        }
        "retours" => {
            let (index, nombre) = (tete.nombre("index")?, tete.nombre("nombre")?);
            tableau(
                questions::retours(index, nombre)
                    .iter()
                    .map(usize::to_string),
            )
        }
        "noter" => {
            let (boite, serie) = (tete.nombre("boite")?, tete.nombre("serie")?);
            let juste = tete.booleen("juste")?;
            let (boite, serie, due) =
                memoire::apres_reponse(boite, serie, juste, tete.nombre("maintenant")?);
            objet(&[
                ("boite", boite.to_string()),
                ("serie", serie.to_string()),
                ("due", decimal(due)),
            ])
        }
        "routines" => tableau(ROUTINES.iter().map(|r| {
            objet(&[
                ("cle", chaine(r.cle)),
                ("emoji", chaine(r.emoji)),
                ("libelle", chaine(r.libelle)),
                ("duree", r.duree.to_string()),
                ("etapes", tableau(r.etapes.iter().map(|e| chaine(e)))),
            ])
        })),
        "journee" => {
            let mut j = Journee::new("");
            (j.debut_h, j.debut_m) = (tete.nombre("debut_h")?, tete.nombre("debut_m")?);
            for ligne in lignes {
                let mut c = Champs::new(ligne);
                j.ajoute("tâche", c.nombre("duree")?, None);
                let t = j.taches.last_mut().expect("tâche ajoutée");
                t.fait = c.booleen("fait")?;
                t.reel = c.optionnel("reel")?;
            }
            journee_json(&j)
        }
        "ecoulees" => {
            let (debut, maintenant): (f64, f64) =
                (tete.nombre("debut_ms")?, tete.nombre("maintenant_ms")?);
            let t = journee::Tache {
                id: 0,
                libelle: String::new(),
                duree: 0,
                fait: false,
                routine: None,
                demarre_a: Some(debut),
                reel: None,
            };
            t.ecoulees(maintenant).to_string()
        }
        "archiver" => {
            let jour = jour_estime(&mut tete)?;
            let mut hist = lignes
                .map(|l| jour_estime(&mut Champs::new(l)))
                .collect::<Result<Vec<_>, _>>()?;
            journee::archiver_jour(&mut hist, jour);
            tableau(hist.iter().map(jour_json))
        }
        "tendance" => {
            let aujourdhui = tete.texte().to_string();
            let jours = lignes
                .map(|l| jour_estime(&mut Champs::new(l)))
                .collect::<Result<Vec<_>, _>>()?;
            objet(&[
                (
                    "barres",
                    tableau(journee::barres(&jours, &aujourdhui).iter().map(|b| {
                        objet(&[
                            ("date", chaine(&b.date)),
                            ("libelle", chaine(&b.libelle)),
                            ("ecart_pct", b.ecart_pct.to_string()),
                            ("juste", b.juste.to_string()),
                            ("part", decimal(b.part)),
                            ("valeur", chaine(&b.valeur)),
                        ])
                    })),
                ),
                ("message", chaine(&journee::tendance(&jours))),
            ])
        }
        "date_lisible" => chaine(&journee::date_lisible(tete.texte())),
        autre => return Err(format!("fonction inconnue : « {autre} »")),
    })
}

/// Les champs d'une ligne, lus un à un.
struct Champs<'a> {
    champs: std::str::Split<'a, char>,
}

impl<'a> Champs<'a> {
    fn new(ligne: &'a str) -> Self {
        Champs {
            champs: ligne.split('\t'),
        }
    }

    fn texte(&mut self) -> &'a str {
        self.champs.next().unwrap_or("")
    }

    fn nombre<T: std::str::FromStr>(&mut self, nom: &str) -> Result<T, String> {
        let texte = self.texte();
        texte
            .trim()
            .parse()
            .map_err(|_| format!("{nom} : nombre attendu, reçu « {texte} »"))
    }

    fn booleen(&mut self, nom: &str) -> Result<bool, String> {
        match self.texte() {
            "1" | "true" => Ok(true),
            "0" | "false" | "" => Ok(false),
            autre => Err(format!("{nom} : 0 ou 1 attendu, reçu « {autre} »")),
        }
    }

    /// Vide : `None`.
    fn optionnel<T: std::str::FromStr>(&mut self, nom: &str) -> Result<Option<T>, String> {
        let texte = self.texte();
        if texte.is_empty() {
            Ok(None)
        } else {
            texte
                .parse()
                .map(Some)
                .map_err(|_| format!("{nom} : nombre attendu, reçu « {texte} »"))
        }
    }
}

fn jour_estime(c: &mut Champs<'_>) -> Result<JourEstime, String> {
    Ok(JourEstime {
        date: c.texte().to_string(),
        nb: c.nombre("nb")?,
        prevu: c.nombre("prevu")?,
        reel: c.nombre("reel")?,
    })
}

// ---------------------------------------------------------------------------
// JSON, écrit à la main : le noyau reste sans dépendance.

fn chaine(texte: &str) -> String {
    let mut out = String::with_capacity(texte.len() + 2);
    out.push('"');
    for c in texte.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Un nombre JSON : jamais `NaN` ni l'infini, qui n'en sont pas.
fn decimal(x: f64) -> String {
    if x.is_finite() {
        format!("{x}")
    } else {
        "0".into()
    }
}

fn objet(champs: &[(&str, String)]) -> String {
    let corps: Vec<String> = champs
        .iter()
        .map(|(cle, valeur)| format!("{}:{valeur}", chaine(cle)))
        .collect();
    format!("{{{}}}", corps.join(","))
}

fn tableau(valeurs: impl IntoIterator<Item = String>) -> String {
    format!("[{}]", valeurs.into_iter().collect::<Vec<_>>().join(","))
}

fn optionnel(valeur: Option<String>) -> String {
    valeur.unwrap_or_else(|| "null".into())
}

fn illustration_json(i: &Illustration) -> String {
    match i {
        Illustration::Billes {
            nombre,
            par_rang,
            couleur_par_rang,
        } => objet(&[
            ("genre", chaine("billes")),
            ("nombre", nombre.to_string()),
            ("par_rang", par_rang.to_string()),
            ("couleur_par_rang", couleur_par_rang.to_string()),
        ]),
        Illustration::Pastille(hex) => {
            objet(&[("genre", chaine("pastille")), ("hex", chaine(hex))])
        }
        Illustration::Forme(nom) => objet(&[("genre", chaine("forme")), ("nom", chaine(nom))]),
        Illustration::Horloge { heures, minutes } => objet(&[
            ("genre", chaine("horloge")),
            ("heures", heures.to_string()),
            ("minutes", minutes.to_string()),
        ]),
        Illustration::Etapes { etapes, trou } => objet(&[
            ("genre", chaine("etapes")),
            ("etapes", tableau(etapes.iter().map(|e| chaine(e)))),
            ("trou", optionnel(trou.map(|t| t.to_string()))),
        ]),
    }
}

fn question_json(q: &Question) -> String {
    objet(&[
        ("enonce", chaine(&q.enonce)),
        ("reponse", chaine(&q.reponse)),
        ("leurres", tableau(q.leurres.iter().map(|l| chaine(l)))),
        ("options", tableau(q.options.iter().map(|o| chaine(o)))),
        ("cle", chaine(&q.cle)),
        ("categorie", optionnel(q.categorie.map(chaine))),
        (
            "illustration",
            optionnel(q.illustration.as_ref().map(illustration_json)),
        ),
    ])
}

fn fiche_json(f: &Fiche) -> String {
    objet(&[
        ("libelle", chaine(&f.libelle)),
        ("detail", optionnel(f.detail.as_deref().map(chaine))),
        ("numero", optionnel(f.numero.map(|n| n.to_string()))),
        ("image", optionnel(f.image.as_ref().map(illustration_json))),
    ])
}

fn revision_json(r: &Revision) -> String {
    match r {
        Revision::Liste {
            titre,
            intro,
            fiches,
        } => objet(&[
            ("genre", chaine("liste")),
            ("titre", chaine(titre)),
            ("intro", optionnel(intro.map(chaine))),
            ("fiches", tableau(fiches.iter().map(fiche_json))),
        ]),
        Revision::Lettres { titre, lettres } => objet(&[
            ("genre", chaine("lettres")),
            ("titre", chaine(titre)),
            ("lettres", tableau(lettres.iter().map(|l| chaine(l)))),
        ]),
        Revision::Accordeon {
            titre,
            intro,
            groupes,
        } => objet(&[
            ("genre", chaine("accordeon")),
            ("titre", chaine(titre)),
            ("intro", optionnel(intro.map(chaine))),
            (
                "groupes",
                tableau(groupes.iter().map(|g| {
                    objet(&[
                        ("libelle", chaine(&g.libelle)),
                        ("fiches", tableau(g.fiches.iter().map(fiche_json))),
                    ])
                })),
            ),
        ]),
    }
}

fn journee_json(j: &Journee) -> String {
    let (fh, fm) = j.fin();
    let b = j.bilan();
    let faites = j.faites();
    objet(&[
        ("horaires", tableau(j.horaires().iter().map(|h| chaine(h)))),
        ("total", j.total().to_string()),
        ("fin", chaine(&crate::planifier::format_heure(fh, fm))),
        ("faites", faites.to_string()),
        (
            "terminee",
            (!j.taches.is_empty() && faites == j.taches.len()).to_string(),
        ),
        (
            "estimations",
            tableau(
                j.taches
                    .iter()
                    .map(|t| optionnel(t.estimation_juste().map(|e| e.to_string()))),
            ),
        ),
        ("bilan", jour_json(&b)),
        (
            "bilan_estimation",
            optionnel(j.bilan_estimation().as_deref().map(chaine)),
        ),
    ])
}

fn jour_json(j: &JourEstime) -> String {
    objet(&[
        ("date", chaine(&j.date)),
        ("nb", j.nb.to_string()),
        ("prevu", j.prevu.to_string()),
        ("reel", j.reel.to_string()),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn appel(fonction: &str, arguments: &str) -> String {
        let f = CString::new(fonction).unwrap();
        let a = CString::new(arguments).unwrap();
        // SAFETY : deux chaînes C valides, la réponse est rendue aussitôt.
        unsafe {
            let r = reneuro_appel(f.as_ptr(), a.as_ptr());
            let texte = CStr::from_ptr(r).to_string_lossy().into_owned();
            reneuro_liberer(r);
            texte
        }
    }

    #[test]
    fn les_seize_themes() {
        let json = appel("categories", "");
        assert!(json.starts_with(r##"[{"cle":"melange","emoji":"🧠","libelle":"Mélange","titre":"Quiz mélangé","revision":false,"niveaux":[],"degrade":["#26a69a","#00695c"]}"##), "{json}");
        assert_eq!(json.matches(r#""cle":"#).count(), 16);
    }

    #[test]
    fn un_quiz_guide_par_la_memoire() {
        appel("amorcer", "1");
        let json = appel("quiz", "couleurs\t1\t100\ncouleurs:pastille:rouge\t1\t10\n");
        assert!(
            json.starts_with(r#"[{"enonce":"Quelle est cette couleur ?","reponse":"Rouge""#),
            "{json}"
        );
        assert!(json.contains(r##""illustration":{"genre":"pastille","hex":"#e53935"}"##));
    }

    #[test]
    fn noter_et_retours() {
        assert_eq!(
            appel("noter", "3\t2\t1\t0"),
            r#"{"boite":4,"serie":3,"due":604800}"#
        );
        assert_eq!(
            appel("noter", "1\t5\t0\t10"),
            r#"{"boite":1,"serie":0,"due":86410}"#
        );
        assert_eq!(appel("retours", "9\t10"), "[10,11]");
        assert_eq!(
            appel("fin", "10\t10"),
            r#"{"etoiles":"⭐⭐⭐⭐⭐","message":"Score parfait, tu es un champion ! 🏆"}"#
        );
    }

    #[test]
    fn la_journee_et_sa_tendance() {
        let json = appel("journee", "23\t50\n5\t1\t9\n30\t0\t\n");
        assert_eq!(
            json,
            concat!(
                r#"{"horaires":["23 h 50","23 h 55"],"total":35,"fin":"0 h 25","faites":1,"terminee":false,"#,
                r#""estimations":[false,null],"bilan":{"date":"","nb":1,"prevu":5,"reel":9},"#,
                r#""bilan_estimation":"Sur 1 tâche chronométrée : 5 min prévues, 9 min réelles — tu as tendance à sous-estimer, c'est très courant. 🌱"}"#
            )
        );
        assert_eq!(appel("ecoulees", "0\t150000"), "3");
        assert_eq!(
            appel(
                "archiver",
                "2026-10-02\t1\t10\t12\n2026-10-03\t1\t20\t20\n2026-10-02\t9\t9\t9"
            ),
            r#"[{"date":"2026-10-02","nb":1,"prevu":10,"reel":12},{"date":"2026-10-03","nb":1,"prevu":20,"reel":20}]"#
        );
        let t = appel(
            "tendance",
            "2026-10-12\n2026-10-11\t1\t100\t150\n2026-10-12\t1\t100\t100",
        );
        assert!(t.contains(r#""libelle":"dim. 11","ecart_pct":50,"juste":false,"part":0.8333333333333334,"valeur":"+50 %""#), "{t}");
        assert!(t.contains(r#""message":"Sur 2 jours : +25 % d'écart en moyenne — tu sous-estimes encore un peu""#), "{t}");
        assert_eq!(appel("date_lisible", "2026-10-12"), r#""lundi 12 octobre""#);
    }

    #[test]
    fn les_erreurs_sont_du_json() {
        assert_eq!(
            appel("inconnue", ""),
            r#"{"erreur":"fonction inconnue : « inconnue »"}"#
        );
        assert_eq!(
            appel("fin", "x"),
            r#"{"erreur":"score : nombre attendu, reçu « x »"}"#
        );
        assert_eq!(appel("revision", "addition"), "null");
        // les guillemets et les sauts de ligne sont échappés
        assert_eq!(chaine("a\"b\n\u{1}"), r#""a\"b\n\u0001""#);
    }
}
