//! Les tours de Hanoï : déplacer toute une pile de disques de la première tige
//! à la dernière, un disque à la fois, sans jamais poser un grand sur un petit.
//!
//! Un mini-jeu de réflexion, et non un quiz : il faut prévoir plusieurs coups
//! à l'avance. Le noyau garde les règles ; le web et l'iPhone dessinent les
//! tiges et transmettent les coups.

/// Un niveau : combien de disques.
pub struct Niveau {
    pub id: u8,
    pub emoji: &'static str,
    pub libelle: &'static str,
    pub disques: u8,
}

pub const NIVEAUX: [Niveau; 3] = [
    Niveau {
        id: 1,
        emoji: "🌱",
        libelle: "3 disques",
        disques: 3,
    },
    Niveau {
        id: 2,
        emoji: "🌿",
        libelle: "4 disques",
        disques: 4,
    },
    Niveau {
        id: 3,
        emoji: "🌳",
        libelle: "5 disques",
        disques: 5,
    },
];

pub fn niveau(id: u8) -> Option<&'static Niveau> {
    NIVEAUX.iter().find(|n| n.id == id)
}

/// Les trois tiges et les coups joués. Un disque est sa taille, de 1 (le plus
/// petit) à `disques` ; chaque tige va du bas vers le haut.
#[derive(Clone, Debug, PartialEq)]
pub struct Hanoi {
    pub tours: [Vec<u8>; 3],
    pub coups: u32,
    pub disques: u8,
}

impl Hanoi {
    /// Toute la pile sur la première tige, la plus grande en bas.
    pub fn new(disques: u8) -> Self {
        Hanoi {
            tours: [(1..=disques).rev().collect(), Vec::new(), Vec::new()],
            coups: 0,
            disques,
        }
    }

    /// Le disque du haut de la tige `de` peut-il aller sur `vers` ?
    pub fn peut(&self, de: usize, vers: usize) -> bool {
        if de == vers || de > 2 || vers > 2 {
            return false;
        }
        match (self.tours[de].last(), self.tours[vers].last()) {
            (None, _) => false,
            (Some(_), None) => true,
            (Some(disque), Some(dessous)) => disque < dessous,
        }
    }

    /// Joue un coup. `false` s'il est interdit : rien ne change alors.
    pub fn deplacer(&mut self, de: usize, vers: usize) -> bool {
        if !self.peut(de, vers) || self.gagne() {
            return false;
        }
        let disque = self.tours[de].pop().unwrap();
        self.tours[vers].push(disque);
        self.coups += 1;
        true
    }

    /// Toute la pile est arrivée sur la dernière tige.
    pub fn gagne(&self) -> bool {
        self.tours[2].len() == self.disques as usize
    }

    /// Le moins de coups possible : 2ⁿ − 1.
    pub fn minimum(&self) -> u32 {
        (1u32 << self.disques) - 1
    }

    /// Les étoiles et le mot de la fin, selon les coups joués : le minimum
    /// mérite trois étoiles, mais finir est déjà une réussite.
    pub fn bilan(&self) -> (&'static str, String) {
        let (coups, minimum) = (self.coups, self.minimum());
        if coups <= minimum {
            (
                "⭐⭐⭐",
                format!("Parfait : {coups} coups, le moins possible ! 🏆"),
            )
        } else if coups * 2 <= minimum * 3 {
            (
                "⭐⭐",
                format!("Bravo ! {coups} coups — le minimum est {minimum}. 🎉"),
            )
        } else {
            (
                "⭐",
                format!(
                    "Réussi en {coups} coups ! Le minimum est {minimum} : rejoue pour t'en approcher. 💪"
                ),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// La solution classique, récursive : elle doit gagner en 2ⁿ − 1 coups.
    fn resoudre(jeu: &mut Hanoi, n: u8, de: usize, vers: usize, par: usize) {
        if n == 0 {
            return;
        }
        resoudre(jeu, n - 1, de, par, vers);
        assert!(jeu.deplacer(de, vers), "{jeu:?}");
        resoudre(jeu, n - 1, par, vers, de);
    }

    #[test]
    fn la_solution_classique_gagne_au_minimum() {
        for niveau in &NIVEAUX {
            let mut jeu = Hanoi::new(niveau.disques);
            assert!(!jeu.gagne());
            resoudre(&mut jeu, niveau.disques, 0, 2, 1);
            assert!(jeu.gagne());
            assert_eq!(jeu.coups, jeu.minimum());
            assert_eq!(jeu.bilan().0, "⭐⭐⭐");
        }
    }

    #[test]
    fn un_grand_disque_ne_va_pas_sur_un_petit() {
        let mut jeu = Hanoi::new(3);
        assert!(jeu.deplacer(0, 2)); // le 1 sur la troisième tige
        assert!(!jeu.deplacer(0, 2), "le 2 sur le 1");
        assert!(!jeu.deplacer(1, 0), "tige vide");
        assert!(!jeu.deplacer(0, 0), "sur place");
        assert!(!jeu.deplacer(0, 3), "tige inexistante");
        assert_eq!(jeu.coups, 1);
        assert_eq!(jeu.tours, [vec![3, 2], vec![], vec![1]]);
    }

    #[test]
    fn plus_aucun_coup_une_fois_gagne() {
        let mut jeu = Hanoi::new(1);
        assert!(jeu.deplacer(0, 2));
        assert!(jeu.gagne());
        assert!(!jeu.deplacer(2, 1));
        assert_eq!(jeu.coups, 1);
    }

    #[test]
    fn le_bilan_encourage() {
        let mut jeu = Hanoi::new(3);
        jeu.coups = 10;
        assert_eq!(jeu.bilan().0, "⭐⭐");
        jeu.coups = 30;
        assert_eq!(jeu.bilan().0, "⭐");
    }
}
