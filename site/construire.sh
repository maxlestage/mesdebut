#!/usr/bin/env bash
# Construit le site (Rust → WebAssembly + HTML) dans site/dist/.
#
# Appelé par `npm run build`, donc par Heroku (heroku-postbuild) et par le
# workflow site-build.yml. Ni l'un ni l'autre n'a Rust : s'il manque, ce script
# installe la chaîne d'outils *hors* du dossier construit, dans
# ${TMPDIR:-/tmp} — sinon les ~600 Mo de rustup finiraient dans le slug Heroku,
# limité à 500 Mo.
#
# Trois étapes :
#   1. la bibliothèque, compilée en WebAssembly ;
#   2. wasm-bindgen, qui écrit le chargeur JavaScript du module ;
#   3. l'exportateur, compilé pour la machine, qui rend les pages avec Active,
#      nomme les fichiers d'après leur empreinte et copie public/.
set -euo pipefail

CIBLE=wasm32-unknown-unknown

ICI="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUTILS="${RENEURO_OUTILS:-${TMPDIR:-/tmp}/reneuro-outils}"

if ! command -v cargo >/dev/null 2>&1 || ! command -v rustup >/dev/null 2>&1; then
  export RUSTUP_HOME="$OUTILS/rustup" CARGO_HOME="$OUTILS/cargo"
  export PATH="$CARGO_HOME/bin:$PATH"
  if ! command -v cargo >/dev/null 2>&1; then
    echo "→ installation de Rust dans $OUTILS"
    mkdir -p "$OUTILS"
    # rustup-init directement : c'est ce que le script de sh.rustup.rs télécharge.
    curl --proto '=https' --tlsv1.2 -sSfL -o "$OUTILS/rustup-init" \
      https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init
    chmod +x "$OUTILS/rustup-init"
    "$OUTILS/rustup-init" -y --profile minimal --default-toolchain stable --no-modify-path
  fi
fi
rustup target add "$CIBLE" >/dev/null

cd "$ICI"

# wasm-bindgen doit être exactement à la version de la bibliothèque du même
# nom dans Cargo.lock : on la lit là, plutôt que de la répéter ici.
WB_VERSION="$(awk '/^name = "wasm-bindgen"$/ { getline; gsub(/version = |"/, ""); print; exit }' Cargo.lock)"
WASM_BINDGEN="$(command -v wasm-bindgen || true)"
if [ -z "$WASM_BINDGEN" ] || ! "$WASM_BINDGEN" --version | grep -q " $WB_VERSION\$"; then
  WASM_BINDGEN="$OUTILS/wasm-bindgen-$WB_VERSION/wasm-bindgen"
  if [ ! -x "$WASM_BINDGEN" ]; then
    echo "→ téléchargement de wasm-bindgen $WB_VERSION"
    mkdir -p "$(dirname "$WASM_BINDGEN")"
    curl -sSfL "https://github.com/wasm-bindgen/wasm-bindgen/releases/download/$WB_VERSION/wasm-bindgen-$WB_VERSION-x86_64-unknown-linux-musl.tar.gz" \
      | tar -xz -C "$(dirname "$WASM_BINDGEN")" --strip-components=1 \
          "wasm-bindgen-$WB_VERSION-x86_64-unknown-linux-musl/wasm-bindgen"
  fi
fi

# Sur Heroku, la compilation reste elle aussi hors du slug.
if [ -n "${STACK:-}" ]; then
  export CARGO_TARGET_DIR="$OUTILS/target"
fi
CIBLES="${CARGO_TARGET_DIR:-$ICI/target}"

echo "→ module WebAssembly"
cargo build --locked --lib --target "$CIBLE" --profile wasm-release

echo "→ chargeur JavaScript (wasm-bindgen $WB_VERSION)"
rm -rf "$CIBLES/pkg"
"$WASM_BINDGEN" --target web --no-typescript --out-dir "$CIBLES/pkg" \
  "$CIBLES/$CIBLE/wasm-release/reneuro_site.wasm"

echo "→ pages"
cargo run --locked --release --bin exporter -- "$CIBLES/pkg" "$ICI/dist"
