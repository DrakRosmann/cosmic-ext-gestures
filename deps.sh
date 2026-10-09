#!/bin/sh
# Install what building cosmic-comp and the app needs, with the system's
# package manager (apt, dnf or pacman), if anything is missing; and check that
# Rust is new enough.
#   ./deps.sh          # just the libraries and tools
#   ./deps.sh 1.93     # and Rust at least this version
set -e

# The libraries, by their pkg-config names.
libs="libinput libseat libudev gbm egl xkbcommon pixman-1 wayland-server libdisplay-info fontconfig xcb"

missing=
for tool in git cc pkg-config cmake cargo; do
    command -v "$tool" >/dev/null 2>&1 || missing="$missing $tool"
done
if command -v pkg-config >/dev/null 2>&1; then
    for lib in $libs; do
        pkg-config --exists "$lib" || missing="$missing $lib"
    done
fi

if [ -n "$missing" ]; then
    echo "Missing:$missing"
    if command -v apt-get >/dev/null 2>&1; then
        # Debian, Ubuntu, Pop!_OS, Mint...
        sudo apt-get install -y git build-essential pkgconf cmake cargo rustc \
            libinput-dev libseat-dev libudev-dev libsystemd-dev libgbm-dev libegl-dev \
            libxkbcommon-dev libpixman-1-dev libwayland-dev libdisplay-info-dev \
            libfontconfig-dev libxcb1-dev
    elif command -v dnf >/dev/null 2>&1; then
        # Fedora
        sudo dnf install -y git gcc gcc-c++ pkgconf-pkg-config cmake cargo rust \
            libinput-devel libseat-devel systemd-devel mesa-libgbm-devel mesa-libEGL-devel \
            libxkbcommon-devel pixman-devel wayland-devel libdisplay-info-devel \
            fontconfig-devel libxcb-devel
    elif command -v pacman >/dev/null 2>&1; then
        # Arch, Manjaro, EndeavourOS...
        rust=rust
        # rustup provides cargo too; don't swap one for the other.
        command -v rustup >/dev/null 2>&1 && rust=
        sudo pacman -S --needed --noconfirm git base-devel pkgconf cmake $rust \
            libinput seatd systemd-libs mesa libglvnd libxkbcommon pixman wayland \
            libdisplay-info fontconfig libxcb
    else
        echo "Unknown package manager: install the development files of these yourself." >&2
        exit 1
    fi
fi

# Distributions often ship an older Rust than cosmic-comp asks for.
if [ -n "$1" ]; then
    have=$(rustc --version | cut -d' ' -f2)
    if [ "$(printf '%s\n%s\n' "$1" "$have" | sort -V | head -n1)" != "$1" ]; then
        if command -v rustup >/dev/null 2>&1; then
            rustup update stable
        else
            echo "Rust $have is too old: $1 or newer is needed. Install it with rustup" >&2
            echo "(https://rustup.rs), then run this again." >&2
            exit 1
        fi
    fi
fi
