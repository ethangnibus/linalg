use bevy::{math::vec4, prelude::*};
use bevy_mod_picking::prelude::*;

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
pub const CYBERPUNK_PINK: Color = Color::rgb(0.91764706,0.0,0.8509804);

pub const TRANSPARENT: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);

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

#[derive(Component, Copy, Clone, Debug)]
pub struct ColorFunction {
    pub background: fn(&CurrentTheme) -> Color,
    pub border: fn(&CurrentTheme) -> Color,
}


pub const HIGHLIGHT_TINT: Highlight<StandardMaterial> = Highlight {
    hovered: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + vec4(-0.2, -0.2, 0.4, 0.0),
        ..matl.to_owned()
    })),
    pressed: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + vec4(-0.3, -0.3, 0.5, 0.0),
        ..matl.to_owned()
    })),
    selected: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + vec4(-0.3, 0.2, -0.3, 0.0),
        ..matl.to_owned()
    })),
};



pub fn not_a_color(theme: &CurrentTheme) -> Color {
    return NOT_A_COLOR;
}

pub fn transparent(theme: &CurrentTheme) -> Color {
    return TRANSPARENT;
}

pub fn transparent_maker(theme: &CurrentTheme, color: fn(&CurrentTheme) -> Color) -> Color {
    let color = color(theme);
    return Color::rgba(
        color.r(),
        color.g(),
        color.b(),
        0.5
    );
}

pub fn vector_color_3d(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Matrix => {
            return MATRIX_LIGHT_GREEN;
        }
        _ => {
            return Color::rgb(0.0, 0.0, 1.0);
        }
    }
}

pub fn vector_color_3d_transparent(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Matrix => {
            return Color::rgba(
                MATRIX_LIGHT_GREEN.r(),
                MATRIX_LIGHT_GREEN.g(),
                MATRIX_LIGHT_GREEN.b(),
                0.05,
            );
        }
        _ => {
            return Color::rgba(0.0, 0.0, 1.0, 0.01);
        }
    }
}

pub fn line_color(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Matrix => {
            return MATRIX_LIGHT_GREEN;
        }
        _ => {
            return Color::rgb(0.35686275, 0.85882354, 0.85882354);
        }
    }
}

pub fn line_color_transparent(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Matrix => {
            return Color::rgba(
                MATRIX_LIGHT_GREEN.r(),
                MATRIX_LIGHT_GREEN.g(),
                MATRIX_LIGHT_GREEN.b(),
                0.5,
            );
        }
        _ => {
            return Color::rgba(0.35686275 * 1.2, 0.85882354 * 1.2, 0.85882354 * 1.2, 0.5);
        }
    }
}

pub fn line_alternate_color_1(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Matrix => {
            return MATRIX_LIGHT_GREEN;
        }
        _ => {
            return Color::rgb(0.95686275, 0.8, 0.38431373);
        }
    }
}
pub fn line_alternate_color_1_transparent(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Matrix => {
            return Color::rgba(
                MATRIX_LIGHT_GREEN.r(),
                MATRIX_LIGHT_GREEN.g(),
                MATRIX_LIGHT_GREEN.b(),
                0.05,
            );
        }
        _ => {
            return Color::rgba(0.95686275, 0.8, 0.38431373, 0.05);
        }
    }
}

pub fn line_alternate_color_2(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Matrix => {
            return MATRIX_LIGHT_GREEN;
        }
        _ => {
            return Color::rgb(0.67058825, 0.9372549,0.85882354);
        }
    }
}
pub fn line_alternate_color_2_transparent(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Matrix => {
            return Color::rgba(
                MATRIX_LIGHT_GREEN.r(),
                MATRIX_LIGHT_GREEN.g(),
                MATRIX_LIGHT_GREEN.b(),
                0.05,
            );
        }
        _ => {
            return Color::rgba(0.67058825, 0.9372549,0.85882354, 0.05);
        }
    }
}

pub fn line_alternate_color_3(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Matrix => {
            return MATRIX_LIGHT_GREEN;
        }
        _ => {
            return Color::rgb(0.827451, 0.18431373, 0.18431373);
        }
    }
}
pub fn line_alternate_color_3_transparent(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Matrix => {
            return Color::rgba(
                MATRIX_LIGHT_GREEN.r(),
                MATRIX_LIGHT_GREEN.g(),
                MATRIX_LIGHT_GREEN.b(),
                0.05,
            );
        }
        _ => {
            return Color::rgba(0.827451, 0.18431373, 0.18431373, 0.05);
        }
    }
}

pub fn planet_color(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Matrix => {
            return MATRIX_GREEN;
        }
        _ => {
            return BERKELEY_GOLD;
        }
    }
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
            return Color::rgb(0.1, 0.1, 0.1);
        }
        CurrentTheme::Matrix => {
            return MATRIX_DARK_GREEN;
        }
        CurrentTheme::Cyberpunk => {
            return CYBERPUNK_GREY;
        }
    }
}

pub fn sidebar_header_color(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Light => {
            return BERKELEY_GOLD;
        }
        CurrentTheme::Dark => {
            return BERKELEY_GOLD;
        }
        CurrentTheme::Matrix => {
            return MATRIX_DARK_GREEN;
        }
        CurrentTheme::Cyberpunk => {
            return CYBERPUNK_BLUE;
        }
    }
}

pub fn option_bar_header_color(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Light => {
            return BERKELEY_BLUE;
        }
        CurrentTheme::Dark => {
            return BERKELEY_BLUE;
        }
        CurrentTheme::Matrix => {
            return MATRIX_DARK_GREEN;
        }
        CurrentTheme::Cyberpunk => {
            return CYBERPUNK_PINK;
        }
    }
}

pub fn option_bar_header_text_color(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Light => {
            return Color::WHITE;
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
            return BERKELEY_GOLD;
        }
        CurrentTheme::Dark => {
            return BERKELEY_BLUE;
        }
        CurrentTheme::Matrix => {
            return MATRIX_LIGHT_GREEN;
        }
        CurrentTheme::Cyberpunk => {
            return CYBERPUNK_YELLOW;
        }
    }
}

pub fn navbar_swiper_color(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Light => {
            // return Color::rgb(0.5, 0.5, 0.5);
            return Color::BLACK;
        }
        CurrentTheme::Dark => {
            // return Color::rgb(0.5, 0.5, 0.5);
            return Color::rgb(0.7, 0.7, 0.7);
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
            // return LIGHT_GRAY;
            return Color::rgb(0.6, 0.6, 0.6);
        }
        CurrentTheme::Dark => {
            return Color::rgb(0.3, 0.3, 0.3);
            // return Color::GRAY;
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
            return Color::BLACK;
        }
        CurrentTheme::Dark => {
            return Color::BLACK;
        }
        CurrentTheme::Matrix => {
            return MATRIX_LIGHT_GREEN;
        }
        CurrentTheme::Cyberpunk => {
            return Color::BLACK;
        }
    }
}


pub fn sidebar_button_text_color(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Light => {
            return Color::BLACK;
        }
        CurrentTheme::Dark => {
            return Color::BLACK;
        }
        CurrentTheme::Matrix => {
            return MATRIX_LIGHT_GREEN;
        }
        CurrentTheme::Cyberpunk => {
            return Color::BLACK;
        }
    }
}

pub fn option_button_text_color(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Light => {
            return Color::WHITE;
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



pub fn page_header_color(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Light => {
            return BERKELEY_DARK_GOLD;
        }
        CurrentTheme::Dark => {
            return BERKELEY_DARK_GOLD;
        }
        CurrentTheme::Matrix => {
            return MATRIX_GREEN;
        }
        CurrentTheme::Cyberpunk => {
            return Color::RED;
        }
    }
}

pub fn cube_base_color(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Light => {
            return BERKELEY_DARK_GOLD;
        }
        CurrentTheme::Dark => {
            return BERKELEY_GOLD;
        }
        CurrentTheme::Matrix => {
            return MATRIX_GREEN;
        }
        CurrentTheme::Cyberpunk => {
            return CYBERPUNK_PINK;
        }
    }
}

pub fn cube_emissive_color(theme: &CurrentTheme) -> Color {
    let base_color = cube_base_color(theme);

    return Color::rgba(
        base_color.r(),
        base_color.g(),
        base_color.b(),
        0.1,
    );
}


pub fn axis_base_color(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Light => {
            return Color::BLACK;
        }
        CurrentTheme::Dark => {
            return Color::WHITE;
        }
        CurrentTheme::Matrix => {
            return MATRIX_GREEN;
        }
        CurrentTheme::Cyberpunk => {
            return Color::WHITE;
        }
    }
}

pub fn axis_emissive_color(theme: &CurrentTheme) -> Color {
    let base_color = axis_base_color(theme);

    return Color::rgba(
        base_color.r(),
        base_color.g(),
        base_color.b(),
        1.0,
    );
}


pub fn grid_base_color(theme: &CurrentTheme) -> Color {
    match theme {
        CurrentTheme::Light => {
            return Color::rgba(0.0, 0.0, 0.0, 0.2);
        }
        CurrentTheme::Dark => {
            return Color::rgba(1.0, 1.0, 1.0, 0.1);
        }
        CurrentTheme::Matrix => {
            return Color::rgba(
                MATRIX_GREEN.r(),
                MATRIX_GREEN.g(),
                MATRIX_GREEN.b(),
                0.5,
            );
        }
        CurrentTheme::Cyberpunk => {
            return Color::rgba(1.0, 1.0, 1.0, 0.1);
        }
    }
}

pub fn grid_emissive_color(theme: &CurrentTheme) -> Color {
    let base_color = grid_base_color(theme);

    return Color::rgba(
        base_color.r(),
        base_color.g(),
        base_color.b(),
        1.0,
    );
}