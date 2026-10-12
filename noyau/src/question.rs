//! Une question de quiz, et ce qui l'illustre.

use super::alea;

#[derive(Clone, Debug, PartialEq)]
pub struct Question {
    pub enonce: String,
    pub reponse: String,
    /// Les mauvaises réponses proposées.
    pub leurres: Vec<String>,
    /// Réponse et leurres, mélangés : l'ordre des boutons.
    pub options: Vec<String>,
    /// Ce qui distingue deux questions : deux questions de même clé ne sont
    /// jamais posées dans le même quiz, et la mémoire suit chaque clé.
    pub cle: String,
    /// Le thème d'où vient la question, en mode Mélange.
    pub categorie: Option<&'static str>,
    pub illustration: Option<Illustration>,
}

/// Ce que l'on montre sous l'énoncé.
#[derive(Clone, Debug, PartialEq)]
pub enum Illustration {
    /// Des billes à compter, `par_rang` par rangée ; une couleur par rangée pour
    /// voir les dizaines.
    Billes {
        nombre: u32,
        par_rang: u32,
        couleur_par_rang: bool,
    },
    /// Une pastille de couleur (`#rrggbb`).
    Pastille(&'static str),
    /// Une forme géométrique, par son nom.
    Forme(&'static str),
    /// Une horloge à aiguilles.
    Horloge { heures: u32, minutes: u32 },
    /// Les étapes d'une activité ; `trou` marque celle qui manque.
    Etapes {
        etapes: Vec<String>,
        trou: Option<usize>,
    },
}

impl Question {
    pub fn new(
        enonce: impl Into<String>,
        reponse: impl Into<String>,
        leurres: Vec<String>,
    ) -> Self {
        let enonce = enonce.into();
        let reponse = reponse.into();
        let mut question = Question {
            cle: enonce.clone(),
            enonce,
            reponse,
            leurres,
            options: Vec::new(),
            categorie: None,
            illustration: None,
        };
        question.rebattre();
        question
    }

    pub fn cle(mut self, cle: impl Into<String>) -> Self {
        self.cle = cle.into();
        self
    }

    pub fn avec(mut self, illustration: Illustration) -> Self {
        self.illustration = Some(illustration);
        self
    }

    /// Remélange les boutons (une question ratée revient dans un autre ordre).
    pub fn rebattre(&mut self) {
        let mut options = Vec::with_capacity(self.leurres.len() + 1);
        options.push(self.reponse.clone());
        options.extend(self.leurres.iter().cloned());
        alea::melanger(&mut options);
        self.options = options;
    }

    /// La clé sous laquelle la mémoire range la question.
    pub fn cle_memoire(&self, categorie: &str) -> String {
        format!("{}:{}", self.categorie.unwrap_or(categorie), self.cle)
    }
}

/// Première lettre en majuscule, accents compris : « été » → « Été ».
pub fn majuscule(texte: &str) -> String {
    let mut lettres = texte.chars();
    match lettres.next() {
        Some(premiere) => premiere.to_uppercase().chain(lettres).collect(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn majuscule_garde_les_accents() {
        assert_eq!(majuscule("été"), "Été");
        assert_eq!(majuscule("œuf"), "Œuf");
        assert_eq!(majuscule(""), "");
    }

    #[test]
    fn les_options_contiennent_la_reponse_et_les_leurres() {
        let q = Question::new("1 + 1 = ?", "2", vec!["3".into(), "4".into(), "5".into()]);
        let mut options = q.options.clone();
        options.sort();
        assert_eq!(options, ["2", "3", "4", "5"]);
        assert_eq!(q.cle, "1 + 1 = ?");
    }
}
