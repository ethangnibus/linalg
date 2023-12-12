use bevy::prelude::*;

// Marker for UI node
#[derive(Component)]
struct NavbarFrame;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, temp);
    }
}

// pub fn setup(mut commands: Commands) {
//     //
//     println!("navbar_frame.rs");
// }

pub fn setup(commands: &mut Commands, width: f32, height: f32) -> Entity {
    let navbar_frame = new(width, height);
    return commands.spawn(navbar_frame).id();
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
