// SPDX-License-Identifier: GPL-3.0-only

//! Which touchpad swipes do what, in the `io.github.DrakRosmann.CosmicGestures`
//! config the patched cosmic-comp reads. The same as its
//! `src/input/gestures/config.rs`: keep them alike.

use cosmic::cosmic_config;
use serde::{Deserialize, Serialize};

pub const CONFIG_ID: &str = "io.github.DrakRosmann.CosmicGestures";
pub const CONFIG_VERSION: u64 = 1;

/// The way the fingers move.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SwipeDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum GestureAction {
    /// The next or previous workspace, following the fingers, along the
    /// workspaces' layout.
    SwitchWorkspace,
    /// As in GNOME: the workspaces overview, and from it the app library.
    Overview,
    WorkspacesOverview,
    AppLibrary,
    Launcher,
    /// Close the app library, the launcher or the overview, whichever is open.
    Close,
    /// Run a command.
    Command(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GestureBinding {
    pub fingers: u32,
    pub direction: SwipeDirection,
    pub action: GestureAction,
}

/// GNOME's: three or four fingers up for the overview and the app library,
/// down to go back, sideways for the workspaces.
pub fn defaults() -> Vec<GestureBinding> {
    let mut bindings = Vec::new();
    for fingers in [3, 4] {
        for (direction, action) in [
            (SwipeDirection::Up, GestureAction::Overview),
            (SwipeDirection::Down, GestureAction::Close),
            (SwipeDirection::Left, GestureAction::SwitchWorkspace),
            (SwipeDirection::Right, GestureAction::SwitchWorkspace),
        ] {
            bindings.push(GestureBinding {
                fingers,
                direction,
                action,
            });
        }
    }
    bindings
}

pub fn load(config: &cosmic_config::Config) -> Vec<GestureBinding> {
    use cosmic_config::ConfigGet;
    config.get("bindings").unwrap_or_else(|_| defaults())
}

pub fn context() -> Option<cosmic_config::Config> {
    cosmic_config::Config::new(CONFIG_ID, CONFIG_VERSION).ok()
}

pub fn save(config: &cosmic_config::Config, bindings: &[GestureBinding]) {
    use cosmic_config::ConfigSet;
    if let Err(err) = config.set("bindings", bindings) {
        eprintln!("cannot save the gestures: {err}");
    }
}
