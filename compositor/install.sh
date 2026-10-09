#!/bin/sh
# Build cosmic-comp and cosmic-workspaces, the versions the system has, with
# this project's patches, and put them in place of the system's, keeping those
# as cosmic-comp.orig and cosmic-workspaces.orig:
#   cosmic-comp: the gestures, the animations (windows, and the shell's
#     layers), and an overview that copies workspaces and windows only when
#     they change;
#   cosmic-workspaces (the overview): copying with the GPU on Intel too.
#   compositor/install.sh          # the versions installed (rpm, dpkg or pacman)
#   compositor/install.sh 1.8.0    # or say which (of both)
# Then log out and back in. compositor/restore.sh puts the originals back.
set -e
here=$(cd "$(dirname "$0")" && pwd)
# In the patched builds, so that a patched one is never kept as the original.
marker=io.github.DrakRosmann.CosmicGestures

# An installed package's version, as upstream numbers it: without the
# packaging's epoch ("1:") and release ("-1", "~22.04"...).
installed_version() {
    if command -v rpm >/dev/null 2>&1 && rpm -q "$1" >/dev/null 2>&1; then
        rpm -q --qf '%{VERSION}' "$1"
    elif command -v dpkg-query >/dev/null 2>&1 \
        && dpkg-query -W -f '${Version}' "$1" >/dev/null 2>&1; then
        dpkg-query -W -f '${Version}' "$1"
    elif command -v pacman >/dev/null 2>&1 && pacman -Q "$1" >/dev/null 2>&1; then
        pacman -Q "$1" | cut -d' ' -f2
    fi | sed -e 's/^[0-9]*://' -e 's/[-~+].*//'
}

# The patches for $1 at version $2, in order, or nothing if one is missing.
patches_for() {
    case $1 in
        cosmic-comp) names="gestures animations layer-animations overview" ;;
        cosmic-workspaces) names="overview" ;;
    esac
    list=
    for name in $names; do
        patch="$here/$1-$2-$name.patch"
        [ -f "$patch" ] || return 0
        list="$list $patch"
    done
    echo "$list"
}

# Builds $1 (from the GitHub repository $2) at version $3 with the patches
# after; the binary ends up in $src/target/release.
build() {
    name=$1 repo=$2 version=$3
    shift 3
    src="${XDG_CACHE_HOME:-$HOME/.cache}/cosmic-ext-gestures/$name-$version"
    if [ ! -d "$src" ]; then
        command -v git >/dev/null 2>&1 || "$here/../deps.sh"
        git clone --depth 1 --branch "epoch-$version" "https://github.com/pop-os/$repo" "$src"
    fi
    # Patched from a clean tree each time, so updated patches take. The build
    # in target/ is kept.
    git -C "$src" reset --quiet --hard
    git -C "$src" clean --quiet -fd
    for patch; do
        git -C "$src" apply "$patch"
    done
    "$here/../deps.sh" "$(sed -n 's/^rust-version = "\(.*\)"/\1/p' "$src/Cargo.toml")"
    (cd "$src" && cargo build --release)
}

# Puts the build of $1 in place of the system's.
put() {
    bin=/usr/bin/$1
    [ -x "$bin" ] || bin=$(command -v "$1")
    # Only the system's own is kept, never one already patched.
    if ! grep -q "$marker" "$bin"; then
        sudo cp -a "$bin" "$bin.orig"
    fi
    # Swapped in whole: a running program keeps the file it started from.
    sudo install -m0755 "$src/target/release/$1" "$bin.new"
    sudo mv "$bin.new" "$bin"
}

comp_version=${1:-$(installed_version cosmic-comp)}
if [ -z "$comp_version" ]; then
    echo "Which cosmic-comp version? Like: $0 1.8.0" >&2
    exit 1
fi
comp_patches=$(patches_for cosmic-comp "$comp_version")
if [ -z "$comp_patches" ]; then
    echo "There are no patches for cosmic-comp $comp_version yet; there are for:" >&2
    ls "$here" | sed -n 's/^cosmic-comp-\(.*\)-gestures\.patch$/  \1/p' >&2
    exit 1
fi

# The overview is optional: without patches for its version, it stays as it is.
ws_version=${1:-$(installed_version cosmic-workspaces)}
ws_patches=
if [ -n "$ws_version" ]; then
    ws_patches=$(patches_for cosmic-workspaces "$ws_version")
    [ -n "$ws_patches" ] || echo "No patch for cosmic-workspaces $ws_version: leaving it as it is."
fi

# shellcheck disable=SC2086 # the lists are of paths without spaces
build cosmic-comp cosmic-comp "$comp_version" $comp_patches
put cosmic-comp
comp_bin=$bin
if [ -n "$ws_patches" ]; then
    # shellcheck disable=SC2086
    build cosmic-workspaces cosmic-workspaces-epoch "$ws_version" $ws_patches
    put cosmic-workspaces
fi

echo "Installed. Log out and back in to use it."
echo "If the session doesn't start: Ctrl+Alt+F3, log in, and run"
echo "  sudo mv $comp_bin.orig $comp_bin"
