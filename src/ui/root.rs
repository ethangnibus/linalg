use super::navbar;
use super::navbar_frame;
use super::under_navbar;
use bevy::prelude::*;

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

// // Returns root node
pub fn setup(commands: &mut Commands) -> Entity {
    // Make ECS for root and navbar
    // return entities
    let root = navbar_frame::setup(commands, 100.0, 100.0);

    let navbar_height: f32 = 10.0; // in percentage
    let navbar = navbar::setup(commands, navbar_height);
    let under_navbar = under_navbar::setup(commands, 100.0, 100.0 - navbar_height);

    // make root parent of navbar and under_navbar
    commands.entity(root).push_children(&[navbar, under_navbar]);

    return root;
}
