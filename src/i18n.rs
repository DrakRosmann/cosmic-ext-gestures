// SPDX-License-Identifier: GPL-3.0-only

//! The app's words, in Portuguese or English as the session's language is.

pub struct Strings {
    pub title: &'static str,
    pub three_fingers: &'static str,
    pub four_fingers: &'static str,
    pub up: &'static str,
    pub down: &'static str,
    pub left: &'static str,
    pub right: &'static str,
    pub nothing: &'static str,
    pub switch_workspace: &'static str,
    pub overview: &'static str,
    pub workspaces_overview: &'static str,
    pub app_library: &'static str,
    pub launcher: &'static str,
    pub close: &'static str,
    pub command: &'static str,
    pub command_placeholder: &'static str,
    pub nothing_hint: &'static str,
    pub switch_hint: &'static str,
    pub presets: &'static str,
    pub gnome: &'static str,
    pub gnome_about: &'static str,
    pub cosmic: &'static str,
    pub cosmic_about: &'static str,
    pub active: &'static str,
    pub inactive: &'static str,
    pub inactive_about: &'static str,
    pub checking: &'static str,
}

const PT: Strings = Strings {
    title: "Gestos do touchpad",
    three_fingers: "Três dedos",
    four_fingers: "Quatro dedos",
    up: "Deslizar para cima",
    down: "Deslizar para baixo",
    left: "Deslizar para a esquerda",
    right: "Deslizar para a direita",
    nothing: "Nada",
    switch_workspace: "Trocar de espaço de trabalho",
    overview: "Visão geral, depois apps (GNOME)",
    workspaces_overview: "Espaços de trabalho",
    app_library: "Biblioteca de apps",
    launcher: "Iniciador",
    close: "Voltar",
    command: "Executar comando",
    command_placeholder: "Comando, como cosmic-term",
    nothing_hint: "Com todas as direções em Nada, o gesto vai para o app sob o cursor.",
    switch_hint: "Trocar de espaço acompanha os dedos, na direção em que os espaços estão (horizontal ou vertical).",
    presets: "Predefinições",
    gnome: "Como no GNOME",
    gnome_about: "Três ou quatro dedos: para cima abre a visão geral e depois os apps, para baixo volta, para os lados troca de espaço",
    cosmic: "Padrão do COSMIC",
    cosmic_about: "Quatro dedos trocam de espaço; três dedos ficam para os apps",
    active: "Gestos configuráveis ativos no compositor",
    inactive: "O compositor em uso não tem os gestos configuráveis",
    inactive_about: "Instale o cosmic-comp com o patch (compositor/install.sh) e entre na sessão de novo. Até lá, as escolhas ficam salvas.",
    checking: "Verificando o compositor…",
};

const EN: Strings = Strings {
    title: "Touchpad Gestures",
    three_fingers: "Three fingers",
    four_fingers: "Four fingers",
    up: "Swipe up",
    down: "Swipe down",
    left: "Swipe left",
    right: "Swipe right",
    nothing: "Nothing",
    switch_workspace: "Switch workspace",
    overview: "Overview, then apps (GNOME)",
    workspaces_overview: "Workspaces",
    app_library: "App library",
    launcher: "Launcher",
    close: "Go back",
    command: "Run command",
    command_placeholder: "A command, like cosmic-term",
    nothing_hint: "With every direction set to Nothing, the swipe goes to the app under the pointer.",
    switch_hint: "Switching workspaces follows the fingers, along the way the workspaces are laid out (horizontal or vertical).",
    presets: "Presets",
    gnome: "Like GNOME",
    gnome_about: "Three or four fingers: up opens the overview and then the apps, down goes back, sideways switches workspace",
    cosmic: "COSMIC's default",
    cosmic_about: "Four fingers switch workspace; three fingers are left to apps",
    active: "Configurable gestures are on in the compositor",
    inactive: "The compositor running has no configurable gestures",
    inactive_about: "Install the patched cosmic-comp (compositor/install.sh) and log in again. Meanwhile your choices are kept.",
    checking: "Checking the compositor…",
};

pub fn strings() -> &'static Strings {
    static STRINGS: std::sync::OnceLock<&'static Strings> = std::sync::OnceLock::new();
    STRINGS.get_or_init(|| {
        let lang = ["LC_ALL", "LC_MESSAGES", "LANG"]
            .iter()
            .find_map(|var| std::env::var(var).ok().filter(|v| !v.is_empty()))
            .unwrap_or_default();
        if lang.starts_with("pt") { &PT } else { &EN }
    })
}
