use bevy::prelude::*;

#[derive(Resource)]
pub enum CurrentTheme {
    // An `enum` variant may either be `unit-like`,
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