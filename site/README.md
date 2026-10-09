# Site de présentation de ReNeuro 🌐

Site vitrine de l'application, écrit en **Rust** avec [Yew](https://yew.rs) 0.23
et compilé en **WebAssembly** par [trunk](https://trunkrs.dev). Sombre et
cinématographique, ses animations s'inspirent de
[zamocorp.com](https://zamocorp.com). Il est pensé **mobile first** : la mise en
page est écrite pour un téléphone, les écrans larges sont traités dans les seules
media queries de `styles/site.css`.

C'est un projet **indépendant** de l'application iOS : ses propres dépendances,
son propre build, sa propre sortie statique.

## Démarrer

Il faut Rust, la cible WebAssembly et trunk :

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked trunk          # ou un binaire publié, voir construire.sh

cd site
trunk serve                           # serveur de développement, rechargement à chaud
./construire.sh                       # site statique dans site/dist/
cargo test                            # tests (découpage des titres)
cargo clippy --target wasm32-unknown-unknown
```

`./construire.sh` est ce que lancent Heroku et le CI. Si Rust ou trunk manquent,
il les installe lui-même **hors du dépôt** (dans `${TMPDIR:-/tmp}`), puis lance
`trunk build --release`. trunk télécharge de son côté `wasm-bindgen` (à la
version du `Cargo.lock`) et `wasm-opt`.

## Les animations

| Effet | Où |
| --- | --- |
| Rideau qui compte de 0 à 100 %, puis se lève (une fois par session) | `index.html`, CSS `@property` · `mouvement/rideau.rs` |
| Grain de film sur toute la page | CSS |
| Poussière qui se rassemble en toque au défilement, balayage de lumière, grains écartés par le pointeur | `mouvement/poussiere.rs` (canvas) |
| Titre géant révélé ligne par ligne, titres de section mot à mot | CSS · `mouvement/revelation.rs` |
| Manifeste qui s'allume mot à mot au défilement | `mouvement/defilement.rs` |
| Bande des seize thèmes qui défile, et penche avec la vitesse du défilement | CSS · `mouvement/defilement.rs` |
| Nombres qui défilent jusqu'à leur valeur (720, +18 %, 4,99 €) | `mouvement/revelation.rs` |
| Curseur à anneau, boutons aimantés, cartes qui s'inclinent, lueur du tarif | `mouvement/pointeur.rs` (souris seulement) |
| Entête qui s'efface en descendant, barre de progression, parallaxes | `mouvement/defilement.rs` |

Trois règles tiennent l'ensemble :

- **Les animations ne touchent pas à l'arbre de Yew.** Elles posent des classes,
  des variables CSS et la propriété `translate`, jamais un nœud créé, déplacé ou
  supprimé. Le seul retiré est le rideau, qui appartient à `index.html`.
- **Rien n'est caché d'avance.** Un élément ne part masqué que sous `html.anime`,
  posé par l'application. Sous « réduire les animations » (`html.calme`), la page
  est en place, la toque est dessinée immobile, et la bande ne défile pas.
- **Toujours un repli.** Sans JavaScript ou sans WebAssembly, `index.html` montre
  l'essentiel ; si l'application ne démarre jamais, le rideau se lève seul.

Les polices sont Archivo (variable, axes de graisse et de chasse) et Fraunces
italique pour les mots d'accent, chargées depuis Google Fonts.

## Déployer

`./construire.sh` produit un dossier `site/dist/` entièrement statique, à chemins
relatifs (`public_url = "./"` dans `Trunk.toml`) : il se dépose à la racine d'un
domaine comme dans un sous-dossier. Le `.wasm`, son chargeur et la feuille de
style vont dans `dist/assets/`, avec une empreinte dans leur nom, ce qui permet de
les garder en cache un an. Le dépôt contient de quoi publier sur Heroku avec le
seul buildpack Node — voir le [README principal](../README.md).

`public/sw.js` n'est pas le service worker du site — le site n'en a pas. C'est un
**service worker d'extinction**, servi à `/sw.js`, qui retire celui que
l'ancienne version web avait laissé à la racine du domaine. Voir le
[README principal](../README.md).

## Les thèmes

`src/donnees.rs` liste les seize thèmes présentés. Ils reprennent ceux de
l'application iOS, qui est la seule implémentation : les deux listes sont écrites
séparément, donc en ajouter un ici suppose de le retrouver là-bas.

## Structure

```
index.html              # métadonnées, rideau, grain, contenu de repli
Trunk.toml              # construction : sortie, chemins relatifs, wasm-opt
construire.sh           # construction complète, outils compris (Heroku, CI)
styles/site.css         # toute la mise en forme et les animations CSS
public/                 # icon.svg, sw.js (copiés tels quels)
src/
  main.rs               # point d'entrée : classe anime/calme, montage de Yew
  app.rs                # assemblage des sections
  donnees.rs            # thèmes, supports, principes, manifeste, offre
  outils.rs             # accès au navigateur, boucle d'animation
  sections/             # un composant Yew par section
  mouvement/            # les animations, branchées après le premier rendu
```
