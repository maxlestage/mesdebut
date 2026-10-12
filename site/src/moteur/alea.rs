//! Le hasard des questions : un générateur xorshift, sans dépendance.
//!
//! Il n'a pas à être imprévisible, seulement à varier d'une partie à l'autre.
//! Le navigateur l'amorce une fois au démarrage ; les tests l'amorcent avec une
//! valeur fixe pour rejouer exactement les mêmes tirages.

use std::cell::Cell;

thread_local! {
    static ETAT: Cell<u64> = const { Cell::new(0x9E37_79B9_7F4A_7C15) };
}

/// Amorce le générateur. Une graine nulle bloquerait xorshift : elle est remplacée.
pub fn amorcer(graine: u64) {
    ETAT.with(|e| {
        e.set(if graine == 0 {
            0x2545_F491_4F6C_DD1D
        } else {
            graine
        })
    });
}

fn suivant() -> u64 {
    ETAT.with(|e| {
        let mut x = e.get();
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        e.set(x);
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    })
}

/// Un entier entre `bas` et `haut`, bornes comprises.
pub fn entre(bas: i32, haut: i32) -> i32 {
    debug_assert!(bas <= haut, "intervalle vide : {bas}..={haut}");
    let largeur = (i64::from(haut) - i64::from(bas) + 1) as u64;
    bas + (suivant() % largeur) as i32
}

/// Un indice dans `0..n` (`n` > 0).
pub fn indice(n: usize) -> usize {
    (suivant() % n as u64) as usize
}

pub fn pile_ou_face() -> bool {
    suivant() & 1 == 1
}

/// Un élément pris au hasard.
pub fn un_de<T: Copy>(liste: &[T]) -> T {
    liste[indice(liste.len())]
}

/// Mélange de Fisher-Yates, en place.
pub fn melanger<T>(liste: &mut [T]) {
    for i in (1..liste.len()).rev() {
        liste.swap(i, indice(i + 1));
    }
}

/// Une copie mélangée.
pub fn melange<T: Clone>(liste: &[T]) -> Vec<T> {
    let mut copie = liste.to_vec();
    melanger(&mut copie);
    copie
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reste_dans_ses_bornes_et_les_atteint() {
        amorcer(42);
        let mut vus = [false; 5];
        for _ in 0..500 {
            let n = entre(3, 7);
            assert!((3..=7).contains(&n));
            vus[(n - 3) as usize] = true;
        }
        assert!(vus.iter().all(|&v| v));
    }

    #[test]
    fn un_melange_garde_les_memes_elements() {
        amorcer(7);
        let mut l = melange(&[1, 2, 3, 4, 5, 6]);
        l.sort_unstable();
        assert_eq!(l, [1, 2, 3, 4, 5, 6]);
    }
}
