use bevy::prelude::*;

pub mod subsection_0;
pub mod subsection_1;
pub mod subsection_2;
pub mod subsection_3;
pub mod subsection_4;
pub mod subsection_5;
pub mod subsection_6;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // Add the setup_ui system as a startup system
        app.add_plugins(subsection_0::SystemsPlugin)
           .add_plugins(subsection_1::SystemsPlugin)
           .add_plugins(subsection_2::SystemsPlugin)
           .add_plugins(subsection_3::SystemsPlugin)
           .add_plugins(subsection_4::SystemsPlugin)
           .add_plugins(subsection_5::SystemsPlugin)
           .add_plugins(subsection_6::SystemsPlugin);
    }
}