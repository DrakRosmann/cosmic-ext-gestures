#!/bin/sh
# Build cosmic-comp, the version the system has, with the patches there are
# for it (the gestures, the animations of windows and of the shell's layers,
# Super and scrolling for workspaces), and put it in place of the system's,
# keeping that one as cosmic-comp.orig.
#   compositor/install.sh          # the version installed (rpm, dpkg or pacman)
#   compositor/install.sh 1.8.0    # or say which
# Then log out and back in. compositor/restore.sh puts the original back.
set -e
here=$(cd "$(dirname "$0")" && pwd)

# The installed cosmic-comp's version, as upstream numbers it: without the
# packaging's epoch ("1:") and release ("-1", "~22.04"...).
installed_version() {
    if command -v rpm >/dev/null 2>&1 && rpm -q cosmic-comp >/dev/null 2>&1; then
        rpm -q --qf '%{VERSION}' cosmic-comp
    elif command -v dpkg-query >/dev/null 2>&1 \
        && dpkg-query -W -f '${Version}' cosmic-comp >/dev/null 2>&1; then
        dpkg-query -W -f '${Version}' cosmic-comp
    elif command -v pacman >/dev/null 2>&1 && pacman -Q cosmic-comp >/dev/null 2>&1; then
        pacman -Q cosmic-comp | cut -d' ' -f2
    fi | sed -e 's/^[0-9]*://' -e 's/[-~+].*//'
}

version=${1:-$(installed_version)}
if [ -z "$version" ]; then
    echo "Which cosmic-comp version? Like: $0 1.8.0" >&2
    exit 1
fi
patches=$(ls "$here"/cosmic-comp-"$version"-*.patch 2>/dev/null || true)
if [ -z "$patches" ]; then
    echo "There is no patch for cosmic-comp $version yet; there are for:" >&2
    ls "$here" | sed -n 's/^cosmic-comp-\([0-9.]*\)-.*\.patch$/  \1/p' | sort -u >&2
    exit 1
fi
echo "Patches for cosmic-comp $version:"
for patch in $patches; do
    echo "  $(basename "$patch")"
done

src="${XDG_CACHE_HOME:-$HOME/.cache}/cosmic-ext-gestures/cosmic-comp-$version"
if [ ! -d "$src" ]; then
    command -v git >/dev/null 2>&1 || "$here/../deps.sh"
    git clone --depth 1 --branch "epoch-$version" https://github.com/pop-os/cosmic-comp "$src"
fi
# Patched from a clean tree each time, so updated patches take. The build in
# target/ is kept.
git -C "$src" reset --quiet --hard
git -C "$src" clean --quiet -fd
for patch in $patches; do
    git -C "$src" apply "$patch"
done
"$here/../deps.sh" "$(sed -n 's/^rust-version = "\(.*\)"/\1/p' "$src/Cargo.toml")"
(cd "$src" && cargo build --release)

bin=$(command -v cosmic-comp)
marker=io.github.DrakRosmann.CosmicGestures
# Only the system's own is kept, never one already patched.
if ! grep -q "$marker" "$bin"; then
    sudo cp -a "$bin" "$bin.orig"
fi
# Swapped in whole: the running compositor keeps the file it started from.
sudo install -m0755 "$src/target/release/cosmic-comp" "$bin.new"
sudo mv "$bin.new" "$bin"
echo "Installed. Log out and back in to use it."
echo "If the session doesn't start: Ctrl+Alt+F3, log in, and run"
echo "  sudo mv $bin.orig $bin"
