use bevy::prelude::*;

pub mod partial;
pub mod example_0;
pub mod example_1;
pub mod example_2;
pub mod example_3;
pub mod example_4;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // Add the setup_ui system as a startup system
        app.add_plugins(example_0::SystemsPlugin);
        app.add_plugins(example_1::SystemsPlugin);
        app.add_plugins(example_2::SystemsPlugin);
        app.add_plugins(example_3::SystemsPlugin);
        app.add_plugins(example_4::SystemsPlugin);
    }
}