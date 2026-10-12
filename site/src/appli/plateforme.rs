//! Ce que l'application demande au navigateur : l'heure, le stockage, les
//! minuteries, la vibration.
//!
//! Hors du navigateur (tests, exportateur), les mêmes fonctions existent et ne
//! font rien d'irréversible : le stockage vit en mémoire, les minuteries ne
//! partent jamais. L'application se compile et se rend donc aussi en natif.

/// Millisecondes depuis 1970.
pub fn maintenant_ms() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Date::now()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0.0, |d| d.as_secs_f64() * 1000.0)
    }
}

/// Secondes depuis 1970 : l'unité de la mémoire de l'apprenant.
pub fn maintenant_s() -> f64 {
    maintenant_ms() / 1000.0
}

/// La date du jour, dans le fuseau de l'appareil : `aaaa-mm-jj`.
pub fn aujourdhui() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        let d = js_sys::Date::new_0();
        crate::moteur::journee::iso(d.get_full_year() as i32, d.get_month() + 1, d.get_date())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        // en natif, le jour UTC suffit : seuls les tests y passent
        let jours = (maintenant_s() / 86_400.0).floor() as i64;
        let (a, m, j) = civil(jours);
        crate::moteur::journee::iso(a, m, j)
    }
}

/// L'heure et la minute de l'appareil.
pub fn heure_locale() -> (u32, u32) {
    #[cfg(target_arch = "wasm32")]
    {
        let d = js_sys::Date::new_0();
        (d.get_hours(), d.get_minutes())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let minutes = (maintenant_s() / 60.0) as u64 % 1440;
        ((minutes / 60) as u32, (minutes % 60) as u32)
    }
}

/// Jour civil d'un nombre de jours depuis 1970 (algorithme de Howard Hinnant).
#[cfg(not(target_arch = "wasm32"))]
fn civil(jours: i64) -> (i32, u32, u32) {
    let z = jours + 719_468;
    let ere = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let j = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let a = yoe + ere * 400 + i64::from(m <= 2);
    (a as i32, m, j)
}

#[cfg(target_arch = "wasm32")]
fn stockage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok()?
}

#[cfg(not(target_arch = "wasm32"))]
thread_local! {
    static STOCKAGE: std::cell::RefCell<std::collections::HashMap<String, String>> = Default::default();
}

/// Lit une valeur gardée sur l'appareil.
pub fn lire(cle: &str) -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        stockage()?.get_item(cle).ok().flatten()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        STOCKAGE.with(|s| s.borrow().get(cle).cloned())
    }
}

/// Garde une valeur sur l'appareil. Un stockage plein ou interdit (navigation
/// privée) n'empêche pas de jouer : la valeur est simplement perdue.
pub fn ecrire(cle: &str, valeur: &str) {
    #[cfg(target_arch = "wasm32")]
    if let Some(s) = stockage() {
        let _ = s.set_item(cle, valeur);
    }
    #[cfg(not(target_arch = "wasm32"))]
    STOCKAGE.with(|s| s.borrow_mut().insert(cle.to_string(), valeur.to_string()));
}

/// Lance `f` dans `ms` millisecondes.
pub fn plus_tard(ms: i32, f: impl FnOnce() + 'static) {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        let rappel = wasm_bindgen::closure::Closure::once_into_js(f);
        if let Some(w) = web_sys::window() {
            let _ =
                w.set_timeout_with_callback_and_timeout_and_arguments_0(rappel.unchecked_ref(), ms);
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (ms, f);
}

/// Lance `f` chaque seconde, jusqu'à ce que le composant qui l'a demandé
/// disparaisse.
pub fn chaque_seconde(f: impl Fn() + 'static) {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        let rappel = wasm_bindgen::closure::Closure::<dyn Fn()>::new(f);
        let Some(w) = web_sys::window() else { return };
        let Ok(id) = w.set_interval_with_callback_and_timeout_and_arguments_0(
            rappel.as_ref().unchecked_ref(),
            1000,
        ) else {
            return;
        };
        active::on_cleanup(move || {
            if let Some(w) = web_sys::window() {
                w.clear_interval_with_handle(id);
            }
            drop(rappel);
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = f;
}

/// Un bref retour tactile, là où le navigateur le permet (Android) ; ailleurs, rien.
pub fn vibrer(ms: u32) {
    #[cfg(target_arch = "wasm32")]
    if let Some(w) = web_sys::window() {
        let _ = w.navigator().vibrate_with_duration(ms);
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = ms;
}

/// Demande confirmation avant un geste qu'on ne peut pas défaire.
pub fn confirmer(question: &str) -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|w| w.confirm_with_message(question).ok())
            .unwrap_or(false)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = question;
        false
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    #[test]
    fn jours_civils() {
        assert_eq!(super::civil(0), (1970, 1, 1));
        assert_eq!(super::civil(20_738), (2026, 10, 12));
    }
}
