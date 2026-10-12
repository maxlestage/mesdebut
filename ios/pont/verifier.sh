#!/usr/bin/env bash
# Vérifie le pont entre Swift et le noyau Rust, sous Linux : sans Mac, donc
# sans les minutes macOS de l'intégration continue.
#
# Compile les fichiers sans interface de l'app — ReNeuroKit (le pont, le moteur
# vu de Swift, la mémoire SQLite) et Services/PlanStore.swift — avec
# main.swift, contre la bibliothèque statique du noyau, puis lance les
# vérifications. Les vues SwiftUI, elles, restent au build Xcode.
#
#   ios/pont/verifier.sh <dossier de libreneuro_noyau.a>
#
# Dans le conteneur officiel de Swift (swift:6.1-noble), par exemple :
#   cargo rustc --manifest-path noyau/Cargo.toml --lib --crate-type staticlib --features ffi --release
#   docker run --rm -v "$PWD:/depot" -w /depot swift:6.1-noble ios/pont/verifier.sh noyau/target/release
set -euo pipefail

ICI="$(cd "$(dirname "$0")" && pwd)"
APP="$ICI/../ReNeuro"
RACINE="$ICI/../.."
BIB="$(cd "${1:?dossier contenant libreneuro_noyau.a}" && pwd)"
T="$(mktemp -d)"

# SwiftUI n'existe pas ici : ces fichiers n'en utilisent que `Color`, imité par Shim.swift.
for f in Noyau QuestionEngine CategoryData Models PlanningData MemoryStore; do
  sed 's/^import SwiftUI$/import Foundation/' "$APP/ReNeuroKit/$f.swift" > "$T/$f.swift"
done
cp "$APP/ReNeuro/Services/PlanStore.swift" "$ICI/Shim.swift" "$ICI/main.swift" "$T/"

# SQLite : le module système qu'Apple fournit, recréé pour Linux.
if [ ! -f /usr/include/sqlite3.h ]; then
  apt-get update -qq && apt-get install -y -qq libsqlite3-dev >/dev/null
fi
mkdir -p "$T/sqlite" "$T/lib" "$T/home"
cat > "$T/sqlite/module.modulemap" <<'MODULE'
module SQLite3 [system] {
    header "/usr/include/sqlite3.h"
    link "sqlite3"
    export *
}
MODULE
ln -sf "$(ls /usr/lib/*-linux-gnu/libsqlite3.so.0 | head -1)" "$T/lib/libsqlite3.so"

# Les bibliothèques système dont la bibliothèque standard de Rust a besoin
# (`rustc --print native-static-libs`).
swiftc -swift-version 5 -o "$T/pont" "$T"/*.swift \
  -I "$RACINE/noyau/include" -I "$T/sqlite" -L "$BIB" -L "$T/lib" \
  -lgcc_s -lutil -lrt -lpthread -lm -ldl

HOME="$T/home" "$T/pont"
