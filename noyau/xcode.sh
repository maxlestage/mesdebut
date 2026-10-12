#!/bin/sh
# Compile le noyau Rust pour ce qu'Xcode est en train de construire.
#
# Appelé par la phase « Noyau Rust » des cibles ReNeuro (iPhone) et
# ReNeuroWatch. Xcode lui donne la plateforme ($PLATFORM_NAME) et les
# architectures ($ARCHS) ; il compile une bibliothèque statique par
# architecture, les réunit avec lipo dans $DERIVED_FILE_DIR/noyau/, où l'édition
# de liens va la chercher (LIBRARY_SEARCH_PATHS). Swift voit le noyau par la
# module map de noyau/include (SWIFT_INCLUDE_PATHS).
#
# Il faut Rust, installé avec rustup (https://rustup.rs). Les cibles Apple
# s'ajoutent toutes seules au premier passage. Seules les montres d'avant la
# Series 9 (arm64_32, pour un envoi sur TestFlight) demandent Rust nightly,
# installé lui aussi au besoin.
set -eu

[ "${ACTION:-build}" = clean ] && exit 0

NOYAU="$(cd "$(dirname "$0")" && pwd)"
SORTIE="${DERIVED_FILE_DIR:?à lancer depuis Xcode}/noyau"
RUSTUP_HOME="${RUSTUP_HOME:-$HOME/.rustup}"
CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
# Xcode n'a pas le PATH du terminal : on va chercher rustup là où il s'installe.
BIN="$CARGO_HOME/bin"

if [ ! -x "$BIN/rustup" ]; then
  echo "error: le noyau de ReNeuro est en Rust : installer rustup (https://rustup.rs), puis relancer la construction." >&2
  exit 1
fi

# La cible Rust d'une architecture de la plateforme construite.
cible_rust() {
  case "$PLATFORM_NAME/$1" in
    iphoneos/arm64) echo aarch64-apple-ios ;;
    iphonesimulator/arm64) echo aarch64-apple-ios-sim ;;
    iphonesimulator/x86_64) echo x86_64-apple-ios ;;
    watchos/arm64) echo aarch64-apple-watchos ;;
    watchos/arm64_32) echo arm64_32-apple-watchos ;;
    watchsimulator/arm64) echo aarch64-apple-watchos-sim ;;
    *)
      echo "error: le noyau ne se construit pas pour $PLATFORM_NAME/$1" >&2
      exit 1
      ;;
  esac
}

# Rust dans un environnement propre : celui d'Xcode (SDKROOT, CC…) viserait
# l'iPhone jusque dans les programmes de construction de la bibliothèque
# standard, qui tournent sur le Mac.
rust() {
  env -i HOME="$HOME" TMPDIR="${TMPDIR:-/tmp}" PATH="$BIN:/usr/bin:/bin:/usr/sbin:/sbin" \
    RUSTUP_HOME="$RUSTUP_HOME" CARGO_HOME="$CARGO_HOME" CARGO_TARGET_DIR="$NOYAU/target" \
    "$@"
}

mkdir -p "$SORTIE"
BIBLIOTHEQUES=""
for ARCH in $ARCHS; do
  CIBLE="$(cible_rust "$ARCH")"
  # La version minimale du système, celle de la cible Xcode.
  case "$CIBLE" in
    *-ios*) VERSION="IPHONEOS_DEPLOYMENT_TARGET=$IPHONEOS_DEPLOYMENT_TARGET" ;;
    *) VERSION="WATCHOS_DEPLOYMENT_TARGET=$WATCHOS_DEPLOYMENT_TARGET" ;;
  esac
  if [ "$CIBLE" = arm64_32-apple-watchos ]; then
    # Cible sans bibliothèque standard précompilée : nightly la recompile.
    if ! rust rustup run nightly rustc --version >/dev/null 2>&1 \
      || ! rust rustup component list --toolchain nightly --installed | grep -q '^rust-src'; then
      echo "note: installation de Rust nightly (montres arm64_32)"
      rust rustup toolchain install nightly --profile minimal --component rust-src
    fi
    set -- +nightly rustc -Z build-std=std,panic_abort
  else
    rust rustup target add --toolchain stable "$CIBLE" >/dev/null
    set -- +stable rustc
  fi
  rust env "$VERSION" "$BIN/cargo" "$@" --quiet --manifest-path "$NOYAU/Cargo.toml" \
    --lib --crate-type staticlib --features ffi --release --target "$CIBLE"
  BIBLIOTHEQUES="$BIBLIOTHEQUES $NOYAU/target/$CIBLE/release/libreneuro_noyau.a"
done

# shellcheck disable=SC2086 # une bibliothèque par architecture
xcrun lipo -create $BIBLIOTHEQUES -output "$SORTIE/libreneuro_noyau.a"
echo "noyau : $SORTIE/libreneuro_noyau.a ($ARCHS)"
