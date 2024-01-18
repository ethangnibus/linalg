use bevy::prelude::*;


pub const BERKELEY_GOLD: Color = Color::rgb(1.0, 0.7, 0.1);
pub const BERKELEY_BLUE: Color = Color::rgb(0.0, 0.19, 0.38);
pub const NOT_A_COLOR: Color = Color::rgba(1.0, 0.0, 0.0, 0.0);


#[derive(Resource, Clone, Copy)]
pub enum CurrentTheme {
    Light,
    Dark,
}

#[derive(Event)]
pub struct ThemeChangeEvent;

#[derive(Component)]
pub struct ThemeButton {
    pub next_theme: CurrentTheme,
}

#[derive(Component)]
pub struct ColorFunction {
    pub background: fn(&CurrentTheme) -> Color,
    pub border: fn(&CurrentTheme) -> Color,
}

pub fn background_color(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Light => {
            return Color::WHITE;
        }
        CurrentTheme::Dark => {
            return Color::BLACK;
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
    }
}

pub fn sidebar_collapsed_color(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Light => {
            return Color::GRAY;
        }
        CurrentTheme::Dark => {
            return Color::GRAY;
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
    }
}