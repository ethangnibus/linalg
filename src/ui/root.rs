use super::navbar;
use super::under_navbar;
use super::vertical_split;
use bevy::prelude::*;

// Returns root node
pub fn setup(commands: &mut Commands) -> Entity {
    // Make ECS for root and navbar
    // return entities
    let root = vertical_split::setup(commands, 100.0, 100.0);

    let navbar_height: f32 = 10.0; // in percentage
    let navbar = navbar::setup(commands, navbar_height);
    let under_navbar = under_navbar::setup(commands, 100.0, 100.0 - navbar_height);

    // make root parent of navbar and under_navbar
    commands.entity(root).push_children(&[navbar, under_navbar]);

    return root;
}
