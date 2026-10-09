#!/bin/sh
# Build and install the Gestures app for the current user (default) or under
# another prefix. The compositor is installed apart: compositor/install.sh.
set -e
prefix="${1:-$HOME/.local}"
id=io.github.DrakRosmann.CosmicGestures
cargo build --release
install -Dm0755 target/release/cosmic-ext-gestures "$prefix/bin/cosmic-ext-gestures"
mkdir -p "$prefix/share/applications"
sed "s|@bindir@|$prefix/bin|" "data/$id.desktop.in" > "$prefix/share/applications/$id.desktop"
install -Dm0644 "data/icons/scalable/apps/$id.svg" "$prefix/share/icons/hicolor/scalable/apps/$id.svg"
echo "Installed: open \"Touchpad Gestures\" from the app library."
