use bevy::prelude::*;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, temp);
    }
}

// pub fn setup(mut commands: Commands) {
//     //
//     println!("vertical_split.rs");
// }

pub fn setup(commands: &mut Commands, width: f32, height: f32) -> Entity {
    let vertical_split = new(width, height);
    return commands.spawn(vertical_split).id();
}

pub fn new(width: f32, height: f32) -> NodeBundle {
    return NodeBundle {
        style: Style {
            width: Val::Percent(width),
            height: Val::Percent(height),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        // background_color: Color::rgb(0.0, 1.0, 0.0).into(),
        ..default()
    };
}

pub fn temp() {}
