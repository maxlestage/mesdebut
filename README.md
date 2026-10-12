# ReNeuro 🎓

Réapprendre les bases — les jours, l'heure, le calcul, organiser sa journée — en
questions courtes.

Ce dépôt contient trois choses :

| | |
| --- | --- |
| [`noyau/`](noyau/) | le **noyau**, en Rust : thèmes, questions, répétition espacée, journée — un seul exemplaire pour le web, l'iPhone et la montre |
| [`site/`](site/) | **ReNeuro sur le web** : le site de présentation (`/`) et l'application complète (`/app`), en Rust avec [Active](https://github.com/maxlestage/Active), compilés en WebAssembly, mobile first |
| [`ios/`](ios/) | l'**application native**, en Swift / SwiftUI (iPhone et Apple Watch) |

## Un seul noyau

Ce que ReNeuro sait faire n'est écrit qu'une fois, dans [`noyau/`](noyau/README.md).
Le web le compile en WebAssembly ; l'iPhone et la montre l'appellent par une
interface C, compilée par Xcode à chaque build. Chaque plateforme ne garde que
son interface et son stockage. Ajouter un thème, changer une question ou la
règle des révisions se fait donc à un seul endroit, et les trois applications
suivent.

## Le web

Écrit en Rust avec [Active](https://github.com/maxlestage/Active) : composants,
état, routage et balises SEO, rendus à la construction ou dans le navigateur.

- **`/`, la présentation** — une page sombre et animée dans l'esprit de
  [zamocorp.com](https://zamocorp.com) : rideau qui compte jusqu'à 100 %,
  poussière qui se rassemble en toque au défilement, titres révélés mot à mot,
  manifeste qui s'allume, bande qui défile. Son HTML est écrit par Active à la
  construction : tout son contenu est lisible sans JavaScript.
- **`/app`, l'application** — les seize thèmes, la révision, les quiz de dix
  questions, la mémoire de ce qui résiste (boîtes de Leitner), « Mes progrès »
  et « Ma journée » avec son chronomètre et le suivi des estimations : tout ce
  que fait l'iPhone, dans le navigateur. Rendue par Active côté client, chaque
  écran a son adresse ; elle s'installe sur l'écran d'accueil et fonctionne
  hors ligne.

```bash
cd site
./construire.sh   # tout le site dans site/dist/ (installe Rust et wasm-bindgen s'il le faut)
cargo test        # moteur, pages, écrans
```

Puis, depuis la racine : `npm ci && npm start`, et <http://localhost:3000>.

`./construire.sh` produit un dossier **entièrement statique**. La présentation
garde des chemins relatifs ; l'application, qui répond à toute adresse
`/app/…`, se dépose à la racine d'un domaine. Le dépôt contient de quoi le
publier sur Heroku (voir plus bas).

Voir [`site/README.md`](site/README.md) pour le détail.

## Déployer sur Heroku

Un seul buildpack, `heroku/nodejs`, celui installé par défaut : rien à configurer
côté Heroku.

```bash
git push heroku master
```

Le buildpack construit le site (`heroku-postbuild`), puis `npm start` le sert avec
[`serve`](https://www.npmjs.com/package/serve), un serveur de fichiers statiques
du registre npm — aucun serveur n'est écrit ici. Le `Procfile` se contente de
`web: npm start`.

`serve.json` définit le comportement : toute adresse `/app/…` reçoit la
coquille de l'application (`app/index.html`), dont le routeur affiche l'écran ;
l'ancienne adresse `/presentation` redirige vers la racine ; les fichiers
d'assets sont gardés un an (leur nom porte une empreinte), et les pages comme
les service workers sont toujours revalidés. Une adresse inconnue hors de
`/app` répond 404.

`cleanUrls` y est **désactivé** volontairement. Activé — c'est le défaut de
`serve` —, il retire le `.html` avant même de regarder les redirections, ce qui
enchaînait `/index.html` → `/index` → `/`. Désactivé, nos propres redirections
s'appliquent et ramènent à l'adresse canonique en un seul saut. Conséquence :
`serve` ne cherche plus seul `index.html` dans un dossier, d'où la réécriture
explicite de `/`. L'option `--single` n'est pas utilisée : elle ajoute sa propre
réécriture de **toute** adresse vers `/index.html`, *avant* celles du fichier,
ce qui enverrait la présentation à la place de l'application.

Deux choses à savoir :

- Heroku n'a pas Rust. `npm run build` lance `site/construire.sh`, qui installe
  la chaîne d'outils (rustup, cible `wasm32-unknown-unknown`, wasm-bindgen) dans
  le dossier temporaire, **hors du slug** — rustup y pèserait ~600 Mo, au-delà de
  la limite de 500 Mo. La compilation (`target/`) y reste aussi : le slug ne
  garde que `site/dist/`. Compter deux minutes environ de construction : le
  module WebAssembly, puis l'exportateur qui écrit les pages.
- `serve` est une **dépendance de production** : sans cela, Heroku l'élaguerait
  après la construction et le démarrage échouerait.

Le `package.json` de la racine ne sert qu'à ça : construire `site/` et le servir.

### Sans terminal

Rien de ce qui précède n'exige la ligne de commande. Dans le tableau de bord
Heroku, onglet **Deploy** : connecter le dépôt GitHub, puis activer
**Automatic deploys** sur `master`. Chaque fusion se déploie alors toute seule.
Le bouton **Deploy Branch** juste en dessous permet un déploiement ponctuel.

Aucun buildpack à ajouter : `heroku/nodejs`, celui par défaut, suffit.

### Découvrir les erreurs avant le déploiement

Le workflow [`site-build.yml`](.github/workflows/site-build.yml) rejoue à chaque
PR **exactement** ce que fait Heroku — mêmes commandes, `NODE_ENV=production`,
élagage des dépendances de développement — puis démarre le serveur et vérifie ce
qu'il sert : les codes de réponse, les adresses de l'application, les
redirections en un saut, les en-têtes de cache, le module WebAssembly (servi en
`application/wasm`, gardé en cache), le manifeste et le contenu des pages.
Avant cela, il passe le code Rust à `cargo fmt`, `clippy` (pour le navigateur
et pour la machine) et `cargo test`.

Son résultat se lit depuis GitHub, sans rien installer.

### Fusion automatique

Toute PR vers `master` est fusionnée toute seule, en squash, dès que toutes
ses vérifications sont vertes et qu'elle n'a pas de conflit
([`fusion.yml`](.github/workflows/fusion.yml)). Avec le déploiement
automatique de Heroku sur `master`, une PR verte est donc en ligne quelques
minutes plus tard.

Pour garder une PR ouverte : la marquer comme brouillon, ou nommer sa branche
`wip/…`.

## L'application native

En Swift / SwiftUI : seize thèmes, une mémoire de l'apprenant en SQLite, une
activité en direct sur l'écran verrouillé, et une application Apple Watch.
Voir [`ios/README.md`](ios/README.md).

Elle est compilée à chaque changement par une intégration continue, et peut être
envoyée sur TestFlight. Voir [`ios/CI.md`](ios/CI.md).

Ses questions, ses révisions, sa répétition espacée et les calculs de « Ma
journée » viennent du noyau Rust, le même que l'application web : Xcode le
compile à chaque build, il faut donc Rust sur le Mac
([rustup](https://rustup.rs)). Le pont entre Swift et le noyau se vérifie aussi
sous Linux, sans Mac ([`ios/pont/`](ios/pont/verifier.sh)).

## Les icônes

Toutes les icônes viennent d'**un seul fichier vectoriel**,
[`icons/icon.svg`](icons/icon.svg), rasterisé par
[`icons/generate.ts`](icons/generate.ts) au cadrage propre à chaque plateforme
(cercle sur watchOS, squircle sur iOS). Voir [`icons/README.md`](icons/README.md).

## Un fichier à ne pas supprimer par mégarde

`site/public/sw.js` n'est pas le service worker de l'application — lui est écrit
par l'exportateur à `/app/sw.js`, avec la portée `/app/`. C'est un **service
worker d'extinction** : la toute première version web avait laissé le sien à la
racine du domaine, et il y répond encore, de mémoire cache, chez les personnes
qui l'avaient ouverte. Ce fichier prend sa place, vide les caches et le
désinscrit. À conserver tant que le site est déposé sur le même domaine qu'elle.

## Crédits

Conçu et développé par **Maxime Nathan Lestage**.
