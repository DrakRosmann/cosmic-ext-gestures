#!/bin/sh
# Build cosmic-comp, the version the system has, with the gestures patch, and
# put it in place of the system's, keeping that one as cosmic-comp.orig.
#   compositor/install.sh          # the version from rpm
#   compositor/install.sh 1.8.0    # or say which
# Then log out and back in. compositor/restore.sh puts the original back.
set -e
here=$(cd "$(dirname "$0")" && pwd)
version=${1:-$(rpm -q --qf '%{VERSION}' cosmic-comp 2>/dev/null || true)}
if [ -z "$version" ]; then
    echo "Which cosmic-comp version? Like: $0 1.8.0" >&2
    exit 1
fi
patch="$here/cosmic-comp-$version-gestures.patch"
if [ ! -f "$patch" ]; then
    echo "There is no patch for cosmic-comp $version yet." >&2
    exit 1
fi

src="${XDG_CACHE_HOME:-$HOME/.cache}/cosmic-ext-gestures/cosmic-comp-$version"
if [ ! -d "$src" ]; then
    git clone --depth 1 --branch "epoch-$version" https://github.com/pop-os/cosmic-comp "$src"
fi
# A clean tree takes the patch; one already patched keeps it.
if git -C "$src" diff --quiet; then
    git -C "$src" apply "$patch"
fi
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
