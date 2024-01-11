use bevy::prelude::*;

use super::util::theme;

// Marker for UI node
#[derive(Component)]
pub struct Navbar;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, temp);
    }
}

// pub fn setup(mut commands: Commands) {
//     //
//     println!("navbar.rs");
// }

pub fn setup(commands: &mut Commands, theme: &theme::CurrentTheme, height: f32) -> Entity {
    let navbar = new(theme, height);

    let navbar = commands.spawn(navbar).id();

    return navbar;
}

pub fn new(theme: &theme::CurrentTheme, height: f32) -> NodeBundle {
    return NodeBundle {
        style: Style {
            // height: Val::Percent(height),
            height: Val::Px(64.0),
            width: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            border: UiRect {
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(4.0),
            },
            overflow: Overflow::clip(),
            ..default()
        },
        background_color: theme::navbar_background_color(theme).into(),
        border_color: Color:: rgb(0.1, 0.1, 0.1).into(),
        ..default()
    };
}

fn temp() {}
