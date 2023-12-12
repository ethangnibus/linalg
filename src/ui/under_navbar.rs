use super::scrollable_page;
use super::sidebar;
use super::sidebar_frame;
use bevy::prelude::*;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(sidebar_frame::SystemsPlugin)
            .add_plugins(sidebar::SystemsPlugin)
            .add_plugins(scrollable_page::SystemsPlugin);
    }
}

// pub fn setup(mut commands: Commands) {
//     //
//     println!("under_navbar.rs");
// }

// Returns root node
pub fn setup(commands: &mut Commands, width: f32, height: f32) -> Entity {
    // Make ECS for root and navbar
    // return entities
    let under_navbar = sidebar_frame::setup(commands, width, height);

    let sidebar_width: f32 = 20.0; // in percentage
    let sidebar = sidebar::setup(commands, sidebar_width);
    let scrollable_page = scrollable_page::setup(commands);

    // make under_navbar parent of sidebar and scrollable_page
    commands
        .entity(under_navbar)
        .push_children(&[sidebar, scrollable_page]);

    return under_navbar;
}
