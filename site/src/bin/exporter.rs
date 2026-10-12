//! Écrit le site, entièrement statique, dans un dossier.
//!
//! ```sh
//! cargo run --release --bin exporter -- <pkg> <dist>
//! ```
//!
//! `<pkg>` contient la sortie de `wasm-bindgen` (le module WebAssembly et son
//! chargeur). L'exportateur y ajoute les feuilles de style, nomme chaque
//! fichier d'après une empreinte de son contenu (gardé en cache un an), rend
//! les deux pages avec Active et copie `public/`.
//!
//! ```text
//! dist/
//!   index.html                 la présentation, chemins relatifs
//!   app/index.html             la coquille de l'application, pour toute adresse /app/…
//!   app/manifest.webmanifest   l'application s'installe
//!   app/sw.js                  … et fonctionne hors ligne
//!   assets/                    .wasm, .js et .css à empreinte
//!   icon.svg, sw.js, app/*.png copiés de public/
//! ```

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use active::pwa::version;
use reneuro_site::{appli, vitrine};

const MODULE: &str = "reneuro_site";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [pkg, dist] = args.as_slice() else {
        eprintln!("usage : exporter <dossier wasm-bindgen> <dossier de sortie>");
        return ExitCode::FAILURE;
    };
    match exporter(Path::new(pkg), Path::new(dist)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("exporter : {e}");
            ExitCode::FAILURE
        }
    }
}

fn lire(chemin: &Path) -> Result<Vec<u8>, String> {
    fs::read(chemin).map_err(|e| format!("{} : {e}", chemin.display()))
}

fn ecrire(chemin: &Path, contenu: impl AsRef<[u8]>) -> Result<(), String> {
    if let Some(dossier) = chemin.parent() {
        fs::create_dir_all(dossier).map_err(|e| format!("{} : {e}", dossier.display()))?;
    }
    fs::write(chemin, contenu).map_err(|e| format!("{} : {e}", chemin.display()))
}

/// Écrit `contenu` dans `assets/` sous un nom à empreinte, et renvoie ce nom.
fn asset(dist: &Path, nom: &str, extension: &str, contenu: &[u8]) -> Result<String, String> {
    let fichier = format!("{nom}-{}{extension}", version([contenu]));
    ecrire(&dist.join("assets").join(&fichier), contenu)?;
    Ok(fichier)
}

fn copier(de: &Path, vers: &Path) -> Result<(), String> {
    for entree in fs::read_dir(de).map_err(|e| format!("{} : {e}", de.display()))? {
        let entree = entree.map_err(|e| e.to_string())?;
        let cible: PathBuf = vers.join(entree.file_name());
        if entree.path().is_dir() {
            copier(&entree.path(), &cible)?;
        } else {
            ecrire(&cible, lire(&entree.path())?)?;
        }
    }
    Ok(())
}

fn exporter(pkg: &Path, dist: &Path) -> Result<(), String> {
    let racine = Path::new(env!("CARGO_MANIFEST_DIR"));
    if dist.exists() {
        fs::remove_dir_all(dist).map_err(|e| format!("{} : {e}", dist.display()))?;
    }

    let chargeur = lire(&pkg.join(format!("{MODULE}.js")))?;
    let wasm = lire(&pkg.join(format!("{MODULE}_bg.wasm")))?;
    let site_css = lire(&racine.join("styles/site.css"))?;
    let appli_css = lire(&racine.join("styles/appli.css"))?;

    // Le .wasm d'abord : le chargeur porte son empreinte dans son propre nom,
    // et la page lui donne explicitement l'adresse du module.
    let wasm_nom = asset(dist, "reneuro", "_bg.wasm", &wasm)?;
    let js_nom = asset(dist, "reneuro", ".js", &chargeur)?;
    let site_css_nom = asset(dist, "site", ".css", &site_css)?;
    let appli_css_nom = asset(dist, "appli", ".css", &appli_css)?;

    // La présentation, à chemins relatifs : elle se dépose aussi dans un sous-dossier.
    let page = vitrine::page(&vitrine::Ressources {
        css: &format!("assets/{site_css_nom}"),
        js: &format!("./assets/{js_nom}"),
        wasm: &format!("./assets/{wasm_nom}"),
    });
    ecrire(&dist.join("index.html"), &page)?;

    // L'application : chemins absolus, puisque la même coquille répond à
    // /app/, /app/heure/quiz/2…
    let (css, js, wasm_url) = (
        format!("/assets/{appli_css_nom}"),
        format!("/assets/{js_nom}"),
        format!("/assets/{wasm_nom}"),
    );
    let coquille = appli::coquille(&css, &js, &wasm_url);
    let manifeste = appli::manifeste().to_json();
    ecrire(&dist.join("app/index.html"), &coquille)?;
    ecrire(&dist.join("app/manifest.webmanifest"), &manifeste)?;

    copier(&racine.join("public"), dist)?;

    let fichiers = [
        appli::ACCUEIL,
        "/app/manifest.webmanifest",
        "/icon.svg",
        "/app/pwa-192.png",
        css.as_str(),
        js.as_str(),
        wasm_url.as_str(),
    ];
    // une nouvelle version de l'un de ces fichiers installe un nouveau service worker
    let empreinte = version([
        coquille.as_bytes(),
        manifeste.as_bytes(),
        &appli_css,
        &chargeur,
        &wasm,
    ]);
    ecrire(
        &dist.join("app/sw.js"),
        appli::service_worker(&empreinte, &fichiers).to_js(),
    )?;

    println!("site écrit dans {}", dist.display());
    for f in [&wasm_nom, &js_nom, &site_css_nom, &appli_css_nom] {
        println!("  assets/{f}");
    }
    Ok(())
}
