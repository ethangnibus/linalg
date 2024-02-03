use bevy::prelude::*;

pub mod partial;
pub mod example_1;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // Add the setup_ui system as a startup system
        app.add_plugins(example_1::SystemsPlugin);
    }
}