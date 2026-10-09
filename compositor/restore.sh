#!/bin/sh
# Put the system's cosmic-comp and cosmic-workspaces back, as install.sh kept
# them. Then log out and back in. Reinstalling the packages does the same:
# sudo dnf reinstall cosmic-comp cosmic-workspaces, sudo apt install
# --reinstall cosmic-comp cosmic-workspaces or sudo pacman -S cosmic-comp
# cosmic-workspaces.
set -e
restored=
for name in cosmic-comp cosmic-workspaces; do
    bin=/usr/bin/$name
    [ -x "$bin" ] || bin=$(command -v "$name") || continue
    if [ -f "$bin.orig" ]; then
        sudo mv "$bin.orig" "$bin"
        restored="$restored $name"
    fi
done
if [ -z "$restored" ]; then
    echo "Nothing to put back: no cosmic-comp.orig or cosmic-workspaces.orig." >&2
    exit 1
fi
echo "Restored:$restored. Log out and back in."
