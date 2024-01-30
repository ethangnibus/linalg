use super::navbar;
use super::under_navbar;
use super::util::theme;
use bevy::{
    ui::FocusPolicy,
    prelude::*,
};

// Marker for Root UI node
#[derive(Component)]
pub struct Root;

#[derive(Component)]
pub struct NavbarFrame;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // Add the setup_ui system as a startup system
        app.add_plugins(navbar::SystemsPlugin)
            .add_plugins(under_navbar::SystemsPlugin);
        // .add_systems(Startup, setup);
        // .add_systems(Update, (button_system, temporary));
    }
}

// pub fn setup(mut commands: Commands) {
//     println!("root.rs");
// }

// Returns root node
pub fn setup(commands: &mut Commands, theme: &theme::CurrentTheme) {
    let root = new(commands, 100.0, 100.0);

    let navbar_height: f32 = 8.0; // in percentage
    let navbar_holder = navbar::navbar_holder(commands, theme, navbar_height);
    let under_navbar = under_navbar::new(commands, 100.0, 100.0 - navbar_height);

    navbar::setup(commands, theme, navbar_height, navbar_holder); // setup navbar before undernavbar is made so view is scaled correctly
    under_navbar::setup(commands, theme, under_navbar);

    // make root parent of navbar and under_navbar
    commands
        .entity(root)
        .push_children(&[
            navbar_holder,
            under_navbar,
        ]);
}

pub fn new(commands: &mut Commands, width: f32, height: f32) -> Entity {
    return commands.spawn((
        NavbarFrame,
        NodeBundle {
            style: Style {
                width: Val::Percent(width),
                height: Val::Percent(height),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            // background_color: Color::rgb(0.0, 1.0, 0.0).into(),
            ..default()
        },
    )).id();
}