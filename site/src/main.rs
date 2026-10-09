//! Site de présentation de ReNeuro, en Yew compilé en WebAssembly.
//!
//! Yew rend la page ; le module `mouvement` l'anime ensuite. La séparation est
//! stricte : les animations ne créent ni ne déplacent aucun nœud rendu par Yew,
//! elles ne posent que des classes, des variables CSS et des transformations.

mod app;
mod donnees;
mod mouvement;
mod outils;
mod sections;

fn main() {
    let document = outils::document();
    let html = document.document_element().expect("<html> introuvable");

    // Décidé avant le premier rendu : sous « réduire les animations », rien ne
    // part caché, donc rien n'attend d'être révélé.
    let classe = if outils::mouvement_reduit() {
        "calme"
    } else {
        "anime"
    };
    let _ = html.class_list().add_1(classe);

    let racine = document
        .get_element_by_id("racine")
        .expect("#racine introuvable");
    // Le contenu de repli de index.html laisse la place à l'application.
    racine.set_inner_html("");
    yew::Renderer::<app::App>::with_root(racine).render();
}
