use bevy::prelude::*;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, temp);
    }
}

// pub fn setup(mut commands: Commands) {
//     //
//     println!("sidebar.rs");
// }

pub fn setup(commands: &mut Commands, width: f32) -> Entity {
    let sidebar = new(width);
    return commands.spawn(sidebar).id();
}

pub fn new(width: f32) -> NodeBundle {
    return NodeBundle {
        style: Style {
            height: Val::Percent(100.0),
            width: Val::Percent(width),
            ..default()
        },
        background_color: Color::rgb(1.0, 0.0, 1.0).into(),
        ..default()
    };
}

fn temp() {}
