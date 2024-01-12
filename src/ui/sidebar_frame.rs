use bevy::prelude::*;

// Marker for UI node
#[derive(Component)]
pub struct SidebarFrame;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, temp);
    }
}

// pub fn setup(mut commands: Commands) {
//     //
//     println!("sidebar_frame.rs");
// }

pub fn setup(commands: &mut Commands, width: f32, height: f32) -> Entity {
    let sidebar_frame = new(width, height);
    return commands.spawn(sidebar_frame).id();
}

pub fn new(width: f32, height: f32) -> NodeBundle {
    return NodeBundle {
        style: Style {
            width: Val::Percent(width),
            height: Val::Percent(100.0),
            flex_grow: 1.0,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        background_color: Color::rgb(0.0, 0.0, 1.0).into(),
        ..default()
    };
}

fn temp() {}
