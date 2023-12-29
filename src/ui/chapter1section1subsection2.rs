use bevy::prelude::*;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (mouse_scroll, routing_system));
    }
}