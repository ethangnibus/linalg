use super::horizontal_split;
use super::scrollable_page;
use super::sidebar;
use bevy::prelude::*;

// Returns root node
pub fn setup(commands: &mut Commands, width: f32, height: f32) -> Entity {
    // Make ECS for root and navbar
    // return entities
    let under_navbar = horizontal_split::setup(commands, width, height);

    let sidebar_width: f32 = 20.0; // in percentage
    let sidebar = sidebar::setup(commands, sidebar_width);
    let scrollable_page = scrollable_page::setup(commands);

    // make under_navbar parent of sidebar and scrollable_page
    commands
        .entity(under_navbar)
        .push_children(&[sidebar, scrollable_page]);

    return under_navbar;
}
