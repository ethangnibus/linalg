use bevy::prelude::*;


pub const BERKELEY_GOLD: Color = Color::rgb(1.0, 0.7, 0.1);

#[derive(Resource)]
pub enum CurrentTheme {
    Light,
    Dark,
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
            return Color::BLACK;
        }
        CurrentTheme::Dark => {
            return BERKELEY_GOLD;
        }
    }
}

pub fn sidebar_color(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Light => {
            return BERKELEY_GOLD;
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