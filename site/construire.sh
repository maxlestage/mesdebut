#!/usr/bin/env bash
# Construit le site (Rust → WebAssembly) dans site/dist/.
#
# Appelé par `npm run build`, donc par Heroku (heroku-postbuild) et par le
# workflow site-build.yml. Ni l'un ni l'autre n'a Rust : s'il manque, ce script
# installe la chaîne d'outils et trunk *hors* du dossier construit, dans
# ${TMPDIR:-/tmp} — sinon les ~600 Mo de rustup finiraient dans le slug Heroku,
# limité à 500 Mo.
set -euo pipefail

# Versions fixées, relevées sur crates.io et les publications GitHub.
TRUNK_VERSION=0.21.14
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

TRUNK="$(command -v trunk || true)"
if [ -z "$TRUNK" ] || ! "$TRUNK" --version | grep -q "$TRUNK_VERSION"; then
  TRUNK="$OUTILS/trunk-$TRUNK_VERSION/trunk"
  if [ ! -x "$TRUNK" ]; then
    echo "→ téléchargement de trunk $TRUNK_VERSION"
    mkdir -p "$(dirname "$TRUNK")"
    curl -sSfL "https://github.com/trunk-rs/trunk/releases/download/v$TRUNK_VERSION/trunk-x86_64-unknown-linux-gnu.tar.gz" \
      | tar -xz -C "$(dirname "$TRUNK")"
  fi
fi

cd "$ICI"
# Sur Heroku, la compilation et les outils que trunk télécharge restent eux
# aussi hors du slug.
if [ -n "${STACK:-}" ]; then
  export CARGO_TARGET_DIR="$OUTILS/target" XDG_CACHE_HOME="$OUTILS/cache"
fi
# trunk télécharge lui-même wasm-bindgen (à la version du Cargo.lock) et wasm-opt.
"$TRUNK" build --release --locked
