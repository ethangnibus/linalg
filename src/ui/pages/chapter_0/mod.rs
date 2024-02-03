use bevy::prelude::*;

pub mod section_0;
pub mod section_1;
pub mod section_2;
pub mod section_3;
pub mod section_4;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // Add the setup_ui system as a startup system
        app.add_plugins(section_0::SystemsPlugin)
        .add_plugins(section_1::SystemsPlugin)
        .add_plugins(section_2::SystemsPlugin)
        .add_plugins(section_3::SystemsPlugin)
        .add_plugins(section_4::SystemsPlugin);
    }
}