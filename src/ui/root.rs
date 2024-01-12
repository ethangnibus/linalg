use super::navbar;
use super::navbar_frame;
use super::under_navbar;
use super::util::theme;
use bevy::{
    ui::FocusPolicy,
    prelude::*,
};

// Marker for Root UI node
#[derive(Component)]
pub struct Root;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // Add the setup_ui system as a startup system
        app.add_plugins(navbar_frame::SystemsPlugin)
            .add_plugins(navbar::SystemsPlugin)
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
    // Make ECS for root and navbar
    // return entities
    let navbar_frame = navbar_frame::setup(commands, 100.0, 100.0);
    let navbar_height: f32 = 8.0; // in percentage
    let navbar = navbar::setup(commands, theme, navbar_height);
    let under_navbar = under_navbar::setup(commands, 100.0, 100.0 - navbar_height);

    // make root parent of navbar and under_navbar
    commands
        .entity(navbar_frame)
        .push_children(&[
            navbar,
            under_navbar,
        ]);
}

