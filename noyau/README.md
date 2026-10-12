# Le noyau de ReNeuro 🧠

Tout ce que ReNeuro **sait faire**, en un seul exemplaire, en Rust : les seize
thèmes, la fabrique de questions, la répétition espacée, les révisions et « Ma
journée ». Le web, l'iPhone et l'Apple Watch s'en servent tous les trois ; seuls
l'interface et le stockage restent propres à chaque plateforme.

```
                 ┌──────────── noyau/ (Rust) ────────────┐
                 │ thèmes · questions · mémoire · journée │
                 └───────┬───────────────────────┬───────┘
          crate Rust     │                       │   interface C (`ffi`)
                         ▼                       ▼
     site/ — WebAssembly, Active         ios/ — bibliothèque statique, SwiftUI
     localStorage                        SQLite, réglages utilisateur
```

Changer une question, un thème, une durée d'activité, la règle des boîtes ou
le message de la tendance, c'est changer **ce dossier**, et c'est tout : les
trois applications suivent à leur prochaine construction.

## Ce qu'il contient

| Fichier | |
| --- | --- |
| `categories.rs` | les seize thèmes, leurs niveaux et leurs couleurs |
| `donnees.rs` | jours, mois, couleurs, formes, nombres et heures en toutes lettres |
| `question.rs` | une question et ce qui l'illustre (billes, horloge, forme…) |
| `questions.rs` | la fabrique de questions, la composition d'un quiz, ce qui revient |
| `planifier.rs` | les onze activités du quotidien et leurs questions |
| `revision.rs` | le contenu des écrans « Réviser d'abord » |
| `memoire.rs` | les boîtes de Leitner, le résumé, les statistiques par thème |
| `journee.rs` | « Ma journée » : heures, chronomètre, bilan, historique, tendance |
| `alea.rs` | le hasard des questions (xorshift, sans dépendance) |
| `ffi.rs` | l'interface C, pour Swift (fonctionnalité `ffi`) |
| `include/` | `reneuro.h` et la module map : `import ReNeuroNoyauC` en Swift |
| `xcode.sh` | la phase « Noyau Rust » des cibles Xcode |

Pas une seule dépendance : le JSON de l'interface C s'écrit à la main.

## Vérifier

```bash
cd noyau
cargo test --features ffi
cargo clippy --all-targets --features ffi -- -D warnings
```

Le pont Swift se vérifie aussi sous Linux, sans Mac — voir
[`ios/pont/`](../ios/pont/verifier.sh) :

```bash
cargo rustc --manifest-path noyau/Cargo.toml --lib --crate-type staticlib --features ffi --release
docker run --rm -v "$PWD:/depot" -w /depot swift:6.1-noble ios/pont/verifier.sh noyau/target/release
```

Le workflow [`noyau.yml`](../.github/workflows/noyau.yml) fait les deux, et
compile en plus le noyau pour chaque architecture Apple.

## Côté web

`site/` en dépend directement (`reneuro-noyau = { path = "../noyau" }`) et le
compile en WebAssembly avec le reste de l'application. L'interface C n'y est
pas : la fonctionnalité `ffi` reste désactivée.

## Côté iPhone et montre

Swift appelle le noyau par deux fonctions C :

```c
char *reneuro_appel(const char *fonction, const char *arguments);
void reneuro_liberer(char *reponse);
```

Les arguments sont du texte (champs séparés par des tabulations, lignes par des
sauts de ligne), la réponse du JSON que lit `JSONDecoder`. La liste des
fonctions est dans `src/ffi.rs`. Côté Swift, `ReNeuroKit/Noyau.swift` fait
l'aller-retour, et les fichiers du moteur (`QuestionEngine`, `CategoryData`,
`PlanningData`, `MemoryStore`, `Services/PlanStore`) traduisent les réponses
dans les types que les vues utilisent.

Xcode compile le noyau lui-même : la phase « Noyau Rust » des cibles ReNeuro et
ReNeuroWatch lance `xcode.sh`, qui construit une bibliothèque statique par
architecture et les réunit avec `lipo`. Il faut Rust, installé avec
[rustup](https://rustup.rs) ; les cibles Apple s'ajoutent toutes seules au
premier build. Pour un envoi sur TestFlight, la montre se construit aussi pour
`arm64_32` (Apple Watch d'avant la Series 9) : cette cible n'a pas de
bibliothèque standard précompilée, Rust nightly la recompile et `xcode.sh`
l'installe au besoin.
