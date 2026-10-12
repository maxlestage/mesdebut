# ReNeuro sur le web 🌐

Le site de présentation **et** l'application complète, écrits en **Rust** avec
[Active](https://github.com/maxlestage/Active) et compilés en **WebAssembly**.
Pensés **mobile first** : la mise en page est écrite pour un téléphone, les
écrans larges ne sont traités que dans les media queries.

| Adresse | Quoi | Rendu |
| --- | --- | --- |
| `/` | la présentation, sombre et animée dans l'esprit de [zamocorp.com](https://zamocorp.com) | HTML écrit par Active à la construction, puis animé par le module WebAssembly |
| `/app` | l'application : les seize thèmes, la révision, les quiz, la mémoire, « Ma journée » | entièrement dans le navigateur, par Active (routeur, état, `<head>`) |

C'est un projet **indépendant** de l'application iOS : ses propres dépendances,
son propre build, sa propre sortie statique. Il en reprend fidèlement le
fonctionnement (voir plus bas).

## Démarrer

Il faut Rust et la cible WebAssembly :

```bash
rustup target add wasm32-unknown-unknown

cd site
./construire.sh                       # tout le site dans site/dist/
cargo test                            # moteur, pages, écrans
cargo clippy --all-targets            # côté machine (exportateur, tests)
cargo clippy --target wasm32-unknown-unknown   # côté navigateur
```

Pour l'essayer, servir `dist/` comme en production, depuis la racine du dépôt :
`npm ci && npm run build && npm start`, puis <http://localhost:3000>.

`./construire.sh` est ce que lancent Heroku et le CI. Si Rust manque, il
l'installe lui-même **hors du dépôt** (dans `${TMPDIR:-/tmp}`), et télécharge
`wasm-bindgen` à la version exacte du `Cargo.lock`. Il enchaîne trois étapes :

1. la bibliothèque, compilée en WebAssembly (profil `wasm-release`) ;
2. `wasm-bindgen`, qui écrit le chargeur JavaScript du module ;
3. l'**exportateur** (`src/bin/exporter.rs`), compilé pour la machine, qui rend
   les deux pages avec Active, nomme chaque fichier d'après une empreinte de son
   contenu et copie `public/`.

```
dist/
  index.html                 la présentation (chemins relatifs)
  app/index.html             la coquille de l'application, servie pour toute adresse /app/…
  app/manifest.webmanifest   l'application s'installe sur l'écran d'accueil…
  app/sw.js                  … et fonctionne hors ligne
  assets/                    .wasm, .js et .css à empreinte, gardés en cache un an
  icon.svg, sw.js, app/*.png copiés de public/
```

Un seul module WebAssembly sert les deux pages : au démarrage, il regarde s'il
trouve `#app` (l'application) ou non (la présentation à animer).

## L'application, `/app`

Chaque écran a sa propre adresse : on y revient avec le bouton « précédent », on
la garde en favori, on la partage. Le serveur envoie toujours la même coquille ;
le routeur d'Active affiche l'écran de l'adresse, et le titre de l'onglet suit.

| Adresse | Écran |
| --- | --- |
| `/app/` | le menu des seize thèmes |
| `/app/heure` | « Réviser d'abord » ou « Faire le quiz » |
| `/app/heure/reviser` | la révision (liste, grille de lettres ou accordéon) |
| `/app/heure/niveaux` | le choix du niveau |
| `/app/heure/quiz/2` | le quiz, au niveau 2 |
| `/app/progres` | « 📊 Mes progrès » |
| `/app/journee` | « 🗓️ Ma journée », le planificateur |

Ce qu'elle reprend de l'application iOS, à l'identique :

- **le moteur de questions** (`src/moteur/`) : mêmes énoncés, mêmes leurres, mêmes
  niveaux, le mode Mélange entrelacé ;
- **la répétition espacée dans la partie** : une question ratée revient 3
  questions plus loin, puis 5 après ;
- **la mémoire de l'apprenant** : boîtes de Leitner (1 à 5), intervalles de 1, 2,
  4, 7 et 15 jours ; les notions à réviser reviennent en priorité dans les thèmes
  à ensemble fini ; les séances sont historisées ;
- **« Ma journée »** : tâches libres ou activités connues, heures qui
  s'enchaînent, chronomètre (un seul à la fois), bilan des estimations, et
  l'historique des 60 derniers jours avec la tendance des dix derniers.

Là où l'iPhone a SQLite et les réglages utilisateur, le navigateur a le
`localStorage` : la mémoire (`reneuro-memoire`), la journée (`reneuro-journee`)
et l'historique des estimations (`reneuro-estimations`), chacun sous sa clé, en
texte, une ligne par enregistrement. Rien ne quitte l'appareil.

Les horloges, formes et étapes sont dessinées en SVG, les billes en CSS. Le
retour tactile utilise la vibration là où le navigateur la permet (Android).
Le thème suit celui de l'appareil, clair ou sombre.

L'application est une **PWA** : `active::pwa` écrit son manifeste et son service
worker. Une fois ouverte, elle fonctionne sans réseau ; chaque déploiement qui
change un de ses fichiers installe un nouveau service worker, qui remplace
l'ancien cache.

Les chemins de l'application sont **absolus** (`/app/…`, `/assets/…`) : elle se
dépose à la racine d'un domaine. La présentation, elle, garde des chemins
relatifs.

## La présentation, `/`

Rendue une fois pour toutes par l'exportateur : tout le texte est dans le HTML,
lisible sans JavaScript et par les moteurs de recherche. Les seize thèmes y
viennent du moteur de l'application (`moteur::categories`) : la page présente
exactement ce que l'application propose, et chaque carte y mène.

| Effet | Où |
| --- | --- |
| Rideau qui compte de 0 à 100 %, puis se lève (une fois par session) | CSS `@property` · `mouvement/rideau.rs` |
| Grain de film sur toute la page | CSS |
| Poussière qui se rassemble en toque au défilement, balayage de lumière, grains écartés par le pointeur | `mouvement/poussiere.rs` (canvas) |
| Titre géant révélé ligne par ligne, titres de section mot à mot | CSS · `mouvement/revelation.rs` |
| Manifeste qui s'allume mot à mot au défilement | `mouvement/defilement.rs` |
| Bande des seize thèmes qui défile, et penche avec la vitesse du défilement | CSS · `mouvement/defilement.rs` |
| Nombres qui défilent jusqu'à leur valeur (720, +18 %, 4,99 €) | `mouvement/revelation.rs` |
| Curseur à anneau, boutons aimantés, cartes qui s'inclinent, lueur du tarif | `mouvement/pointeur.rs` (souris seulement) |
| Entête qui s'efface en descendant, barre de progression, parallaxes | `mouvement/defilement.rs` |

Trois règles tiennent l'ensemble :

- **Les animations ne touchent pas au HTML rendu.** Elles posent des classes,
  des variables CSS et la propriété `translate`, jamais un nœud créé, déplacé ou
  supprimé. Le seul retiré est le rideau.
- **Rien n'est caché d'avance.** Un élément ne part masqué que sous `html.anime`,
  posé par le module WebAssembly pendant que le rideau couvre encore la page.
  Sous « réduire les animations » (`html.calme`), la page est en place, la toque
  est dessinée immobile, et la bande ne défile pas.
- **Toujours un repli.** Sans JavaScript, le rideau n'apparaît pas et la page se
  lit telle quelle ; si le module ne démarre jamais, le rideau se lève seul.

Les polices sont Archivo (variable, axes de graisse et de chasse) et Fraunces
italique pour les mots d'accent, chargées depuis Google Fonts.

`public/sw.js` n'est pas le service worker de l'application (lui est servi à
`/app/sw.js`). C'est un **service worker d'extinction**, servi à `/sw.js`, qui
retire celui que la toute première version web avait laissé à la racine du
domaine. Voir le [README principal](../README.md).

## Structure

```
construire.sh           # construction complète, outils compris (Heroku, CI)
styles/site.css         # la présentation : mise en forme et animations CSS
styles/appli.css        # l'application : clair et sombre, mobile first
public/                 # icon.svg, sw.js (extinction), app/*.png (icônes PWA)
src/
  lib.rs                # point d'entrée du module : application ou présentation
  bin/exporter.rs       # écrit dist/ : pages, empreintes, manifeste, service worker
  moteur/               # Rust pur, testé avec cargo test
    categories.rs       #   les seize thèmes et leurs niveaux
    donnees.rs          #   jours, mois, couleurs, formes, nombres et heures en lettres
    question.rs         #   une question et son illustration
    questions.rs        #   la fabrique de questions, la composition d'un quiz
    planifier.rs        #   les onze activités du quotidien et leurs questions
    revision.rs         #   le contenu des écrans « Réviser d'abord »
    memoire.rs          #   boîtes de Leitner, résumé, statistiques par thème
    journee.rs          #   le planificateur, le chronomètre, la tendance, les dates
  appli/                # l'application, en composants Active
    mod.rs              #   routes, coquille, manifeste, service worker
    ecrans.rs           #   menu, thème, niveaux, révision, progrès
    quiz.rs             #   le quiz et l'écran de fin
    journee.rs          #   « Ma journée »
    dessins.rs          #   horloge, billes, formes, pastilles, étapes
    stock.rs            #   ce qui est gardé sur l'appareil
    plateforme.rs       #   heure, stockage, minuteries (et leurs équivalents hors navigateur)
  vitrine/              # la présentation, en vues Active rendues à la construction
  mouvement/            # les animations de la présentation (navigateur seulement)
  outils.rs             # accès au navigateur pour les animations
```

## Les thèmes

`src/moteur/categories.rs` liste les seize thèmes. Ils reprennent ceux de
l'application iOS : les deux listes sont écrites séparément, donc en ajouter un
ici suppose de le retrouver là-bas, et inversement.
