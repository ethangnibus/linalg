use bevy::prelude::*;

// pub const BERKELEY_GOLD: Color = Color::rgb(1.0, 0.7, 0.1);
pub const NOT_A_COLOR: Color = Color::rgba(1.0, 0.0, 0.0, 0.0);
pub const BERKELEY_GOLD: Color = Color::rgb(0.99, 0.71, 0.08);
pub const BERKELEY_DARK_GOLD: Color = Color::rgb(0.77, 0.51, 0.05);
pub const BERKELEY_BLUE: Color = Color::rgb(0.0, 0.19, 0.38);
pub const BERKELEY_LIGHT_BLUE: Color = Color::rgb(0.23, 0.49, 0.63);
pub const LIGHT_GRAY: Color = Color::rgb(0.55, 0.55, 0.55);

// Matrix theme
pub const MATRIX_LIGHT_GREEN: Color = Color::rgb(0.0, 0.66, 0.12);
pub const MATRIX_GREEN: Color = Color::rgb(0.0, 0.33, 0.06);
pub const MATRIX_DARK_GREEN: Color = Color::rgb(0.0, 0.15, 0.0);

// Cyberpunk theme
pub const CYBERPUNK_YELLOW: Color = Color::rgb(0.952, 0.901, 0.0);
pub const CYBERPUNK_BLUE: Color = Color::rgb(0.015, 0.855, 0.965);
pub const CYBERPUNK_RED: Color = Color::rgb(1.0, 0.0, 0.235);
pub const CYBERPUNK_GREY: Color = Color::rgb(0.333, 0.294, 0.255);

#[derive(Resource, Clone, Copy, PartialEq)]
pub enum CurrentTheme {
    Light,
    Dark,
    Matrix,
    Cyberpunk,
}

#[derive(Event)]
pub struct ThemeChangeEvent;

#[derive(Component)]
pub struct ThemeButton {
    pub next_theme: CurrentTheme,
}

#[derive(Component)]
pub struct ThemeButtonText {
    pub next_theme: CurrentTheme,
}

#[derive(Component)]
pub struct ThemeButtonLine {
    pub next_theme: CurrentTheme,
}

#[derive(Component, Copy, Clone)]
pub struct ColorFunction {
    pub background: fn(&CurrentTheme) -> Color,
    pub border: fn(&CurrentTheme) -> Color,
}

pub fn not_a_color(theme: &CurrentTheme) -> Color {
    return NOT_A_COLOR;
}

pub fn background_color(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Light => {
            return Color::WHITE;
        }
        CurrentTheme::Dark => {
            return Color::BLACK;
        }
        CurrentTheme::Matrix => {
            return Color::BLACK;
        }
        CurrentTheme::Cyberpunk => {
            return Color::BLACK;
        }
    }
}

pub fn swiper_background_color(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Light => {
            return Color::rgb(0.85, 0.85, 0.85);
        }
        CurrentTheme::Dark => {
            return Color::rgb(0.15, 0.15, 0.15);
        }
        CurrentTheme::Matrix => {
            return MATRIX_DARK_GREEN;
        }
        CurrentTheme::Cyberpunk => {
            return CYBERPUNK_GREY;
        }
    }
}

pub fn text_color(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Light => {
            return Color::BLACK;
        }
        CurrentTheme::Dark => {
            return Color::WHITE;
        }
        CurrentTheme::Matrix => {
            return MATRIX_LIGHT_GREEN;
        }
        CurrentTheme::Cyberpunk => {
            return Color::WHITE;
        }
    }
}

pub fn navbar_background_color(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Light => {
            return Color::WHITE;
        }
        CurrentTheme::Dark => {
            return Color::BLACK;
        }
        CurrentTheme::Matrix => {
            return Color::BLACK;
        }
        CurrentTheme::Cyberpunk => {
            return Color::BLACK;
        }
    }
}

pub fn navbar_text_color(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Light => {
            return BERKELEY_BLUE;
        }
        CurrentTheme::Dark => {
            return BERKELEY_GOLD;
        }
        CurrentTheme::Matrix => {
            return MATRIX_LIGHT_GREEN;
        }
        CurrentTheme::Cyberpunk => {
            return CYBERPUNK_YELLOW;
        }
    }
}

pub fn navbar_buttons_background_color(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Light => {
            return BERKELEY_GOLD;
        }
        CurrentTheme::Dark => {
            return BERKELEY_GOLD;
        }
        CurrentTheme::Matrix => {
            return MATRIX_LIGHT_GREEN;
        }
        CurrentTheme::Cyberpunk => {
            return CYBERPUNK_YELLOW;
        }
    }
}

pub fn sidebar_color(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Light => {
            return BERKELEY_BLUE;
        }
        CurrentTheme::Dark => {
            return BERKELEY_GOLD;
        }
        CurrentTheme::Matrix => {
            return MATRIX_LIGHT_GREEN;
        }
        CurrentTheme::Cyberpunk => {
            return CYBERPUNK_YELLOW;
        }
    }
}

pub fn sidebar_collapsed_color(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Light => {
            return LIGHT_GRAY;
        }
        CurrentTheme::Dark => {
            return Color::GRAY;
        }
        CurrentTheme::Matrix => {
            return MATRIX_GREEN;
        }
        CurrentTheme::Cyberpunk => {
            return CYBERPUNK_RED;
        }
    }
}

pub fn sidebar_header_text_color(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Light => {
            return Color::WHITE;
        }
        CurrentTheme::Dark => {
            return Color::BLACK;
        }
        CurrentTheme::Matrix => {
            return Color::BLACK;
        }
        CurrentTheme::Cyberpunk => {
            return Color::BLACK;
        }
    }
}
