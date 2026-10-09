// SPDX-License-Identifier: GPL-3.0-only

//! Gestures: sets which touchpad swipes do what, for a cosmic-comp built with
//! the patch in `compositor/`.

mod gestures;
mod i18n;

use cosmic::{
    ApplicationExt,
    Element, Task, app,
    cosmic_config,
    iced::{Alignment, Length, Size},
    theme,
    widget::{self, settings},
};

use gestures::{GestureAction, GestureBinding, SwipeDirection};
use i18n::strings;

const APP_ID: &str = "io.github.DrakRosmann.CosmicGestures";

fn main() -> cosmic::iced::Result {
    let settings = app::Settings::default().size(Size::new(620.0, 820.0));
    app::run::<App>(settings, ())
}

const FINGERS: [u32; 2] = [3, 4];
const DIRECTIONS: [SwipeDirection; 4] = [
    SwipeDirection::Up,
    SwipeDirection::Down,
    SwipeDirection::Left,
    SwipeDirection::Right,
];

/// The choices in each list, in order; `None` is no binding.
fn choice(index: usize, command: String) -> Option<GestureAction> {
    match index {
        1 => Some(GestureAction::SwitchWorkspace),
        2 => Some(GestureAction::Overview),
        3 => Some(GestureAction::WorkspacesOverview),
        4 => Some(GestureAction::AppLibrary),
        5 => Some(GestureAction::Launcher),
        6 => Some(GestureAction::Close),
        7 => Some(GestureAction::Command(command)),
        _ => None,
    }
}

fn index_of(action: Option<&GestureAction>) -> usize {
    match action {
        None => 0,
        Some(GestureAction::SwitchWorkspace) => 1,
        Some(GestureAction::Overview) => 2,
        Some(GestureAction::WorkspacesOverview) => 3,
        Some(GestureAction::AppLibrary) => 4,
        Some(GestureAction::Launcher) => 5,
        Some(GestureAction::Close) => 6,
        Some(GestureAction::Command(_)) => 7,
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Preset {
    Gnome,
    Cosmic,
}

#[derive(Clone, Debug)]
pub enum Message {
    Select(u32, SwipeDirection, usize),
    /// A command being typed, saved as it is typed.
    Command(u32, SwipeDirection, String),
    Preset(Preset),
    /// Whether the running compositor knows these gestures.
    Compositor(bool),
}

pub struct App {
    core: app::Core,
    config: Option<cosmic_config::Config>,
    bindings: Vec<GestureBinding>,
    choices: Vec<&'static str>,
    compositor: Option<bool>,
}

impl App {
    fn action(&self, fingers: u32, direction: SwipeDirection) -> Option<&GestureAction> {
        self.bindings
            .iter()
            .find(|b| b.fingers == fingers && b.direction == direction)
            .map(|b| &b.action)
    }

    fn set(&mut self, fingers: u32, direction: SwipeDirection, action: Option<GestureAction>) {
        self.bindings
            .retain(|b| !(b.fingers == fingers && b.direction == direction));
        if let Some(action) = action {
            // Kept in a steady order, which the config file shows.
            let at = self
                .bindings
                .iter()
                .position(|b| {
                    (b.fingers, DIRECTIONS.iter().position(|d| *d == b.direction))
                        > (fingers, DIRECTIONS.iter().position(|d| *d == direction))
                })
                .unwrap_or(self.bindings.len());
            self.bindings.insert(
                at,
                GestureBinding {
                    fingers,
                    direction,
                    action,
                },
            );
        }
        self.save();
    }

    fn save(&self) {
        if let Some(config) = &self.config {
            gestures::save(config, &self.bindings);
        }
    }

    fn status(&self) -> Element<'_, Message> {
        let t = strings();
        let (icon, title, about) = match self.compositor {
            None => ("content-loading-symbolic", t.checking, None),
            Some(true) => ("emblem-ok-symbolic", t.active, None),
            Some(false) => ("dialog-warning-symbolic", t.inactive, Some(t.inactive_about)),
        };
        let mut text = widget::column::with_capacity(2).push(widget::text::body(title));
        if let Some(about) = about {
            text = text.push(widget::text::caption(about));
        }
        widget::container(
            widget::row::with_children(vec![
                widget::icon::from_name(icon).size(24).into(),
                text.width(Length::Fill).into(),
            ])
            .spacing(theme::spacing().space_s)
            .align_y(Alignment::Center),
        )
        .padding(theme::spacing().space_s)
        .class(theme::Container::Card)
        .width(Length::Fill)
        .into()
    }

    fn fingers_section(&self, fingers: u32) -> Element<'_, Message> {
        let t = strings();
        let title = if fingers == 3 { t.three_fingers } else { t.four_fingers };
        let mut section = settings::section().title(title);
        for direction in DIRECTIONS {
            let label = match direction {
                SwipeDirection::Up => t.up,
                SwipeDirection::Down => t.down,
                SwipeDirection::Left => t.left,
                SwipeDirection::Right => t.right,
            };
            let action = self.action(fingers, direction);
            section = section.add(settings::item(
                label,
                widget::dropdown(
                    self.choices.as_slice(),
                    Some(index_of(action)),
                    move |index| Message::Select(fingers, direction, index),
                ),
            ));
            if let Some(GestureAction::Command(command)) = action {
                section = section.add(settings::item_row(vec![
                    widget::text_input(t.command_placeholder, command.clone())
                        .on_input(move |text| Message::Command(fingers, direction, text))
                        .width(Length::Fill)
                        .into(),
                ]));
            }
        }
        section.into()
    }

    fn presets(&self) -> Element<'_, Message> {
        let t = strings();
        let preset = |title: &'static str, about: &'static str, preset: Preset| {
            settings::item_row(vec![
                widget::column::with_children(vec![
                    widget::text::body(title).into(),
                    widget::text::caption(about).into(),
                ])
                .width(Length::Fill)
                .into(),
                widget::button::standard(title).on_press(Message::Preset(preset)).into(),
            ])
            .align_y(Alignment::Center)
        };
        settings::section()
            .title(t.presets)
            .add(preset(t.gnome, t.gnome_about, Preset::Gnome))
            .add(preset(t.cosmic, t.cosmic_about, Preset::Cosmic))
            .into()
    }
}

/// Whether the running cosmic-comp was built with the gestures patch: it
/// carries the config's name.
fn compositor_patched() -> bool {
    let Ok(processes) = std::fs::read_dir("/proc") else {
        return false;
    };
    let needle = gestures::CONFIG_ID.as_bytes();
    processes.flatten().any(|process| {
        let comm = std::fs::read_to_string(process.path().join("comm")).unwrap_or_default();
        comm.trim() == "cosmic-comp"
            && std::fs::read(process.path().join("exe"))
                .is_ok_and(|binary| binary.windows(needle.len()).any(|w| w == needle))
    })
}

impl cosmic::Application for App {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = APP_ID;

    fn core(&self) -> &app::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut app::Core {
        &mut self.core
    }

    fn init(core: app::Core, _flags: ()) -> (Self, Task<cosmic::Action<Message>>) {
        let t = strings();
        let config = gestures::context();
        let bindings = config.as_ref().map(gestures::load).unwrap_or_else(gestures::defaults);
        let mut app = App {
            core,
            config,
            bindings,
            choices: vec![
                t.nothing,
                t.switch_workspace,
                t.overview,
                t.workspaces_overview,
                t.app_library,
                t.launcher,
                t.close,
                t.command,
            ],
            compositor: None,
        };
        app.set_header_title(t.title.to_owned());
        let title = match app.core.main_window_id() {
            Some(id) => app.set_window_title(t.title.to_owned(), id),
            None => Task::none(),
        };
        let check = Task::future(async {
            let patched = tokio::task::spawn_blocking(compositor_patched)
                .await
                .unwrap_or(false);
            cosmic::Action::App(Message::Compositor(patched))
        });
        (app, Task::batch([title, check]))
    }

    fn update(&mut self, message: Message) -> Task<cosmic::Action<Message>> {
        match message {
            Message::Select(fingers, direction, index) => {
                // A command kept when picking it again.
                let command = match self.action(fingers, direction) {
                    Some(GestureAction::Command(command)) => command.clone(),
                    _ => String::new(),
                };
                self.set(fingers, direction, choice(index, command));
            }
            Message::Command(fingers, direction, command) => {
                self.set(fingers, direction, Some(GestureAction::Command(command)));
            }
            Message::Preset(preset) => {
                self.bindings = match preset {
                    Preset::Gnome => gestures::defaults(),
                    Preset::Cosmic => [SwipeDirection::Left, SwipeDirection::Right]
                        .into_iter()
                        .map(|direction| GestureBinding {
                            fingers: 4,
                            direction,
                            action: GestureAction::SwitchWorkspace,
                        })
                        .collect(),
                };
                self.save();
            }
            Message::Compositor(patched) => self.compositor = Some(patched),
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let t = strings();
        let spacing = theme::spacing();
        let mut column = widget::column::with_capacity(6)
            .push(self.status())
            .push(self.presets());
        for fingers in FINGERS {
            column = column.push(self.fingers_section(fingers));
        }
        column = column
            .push(widget::text::caption(t.switch_hint))
            .push(widget::text::caption(t.nothing_hint))
            .spacing(spacing.space_m)
            .padding([spacing.space_s, spacing.space_l]);
        widget::scrollable(widget::container(column).max_width(760).center_x(Length::Fill))
            .into()
    }
}
