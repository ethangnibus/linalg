use bevy::prelude::*;


// pub const BERKELEY_GOLD: Color = Color::rgb(1.0, 0.7, 0.1);
pub const BERKELEY_GOLD: Color = Color::rgb(0.99, 0.71, 0.08);
pub const BERKELEY_DARK_GOLD: Color = Color::rgb(0.77, 0.51, 0.05);
pub const BERKELEY_BLUE: Color = Color::rgb(0.0, 0.19, 0.38);
pub const BERKELEY_LIGHT_BLUE: Color = Color::rgb(0.23, 0.49, 0.63);
pub const LIGHT_GRAY: Color = Color::rgb(0.9, 0.9, 0.9);
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

pub fn swiper_background_color(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Light => {
            return Color::rgb(0.85, 0.85, 0.85);
        }
        CurrentTheme::Dark => {
            return Color::rgb(0.15, 0.15, 0.15);
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

pub fn navbar_buttons_background_color(theme:&CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Light => {
            return BERKELEY_GOLD;
        }
        CurrentTheme::Dark => {
            return BERKELEY_GOLD;
        }
    }
}

pub fn sidebar_color(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Light => {
            return BERKELEY_LIGHT_BLUE;
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