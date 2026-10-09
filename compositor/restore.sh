#!/bin/sh
# Put the system's cosmic-comp back, as install.sh kept it. Then log out and
# back in. (Reinstalling the package does the same: sudo dnf reinstall cosmic-comp.)
set -e
bin=$(command -v cosmic-comp)
if [ ! -f "$bin.orig" ]; then
    echo "No $bin.orig to put back." >&2
    exit 1
fi
sudo mv "$bin.orig" "$bin"
echo "Restored. Log out and back in."
