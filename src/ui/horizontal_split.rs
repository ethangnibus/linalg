use bevy::prelude::*;

pub fn setup(commands: &mut Commands, width: f32, height: f32) -> Entity {
    let horizontal_split = new(width, height);
    return commands.spawn(horizontal_split).id();
}

pub fn new(width: f32, height: f32) -> NodeBundle {
    return NodeBundle {
        style: Style {
            width: Val::Percent(width),
            height: Val::Percent(height),
            flex_direction: FlexDirection::Row,
            ..default()
        },
        // background_color: Color::rgb(0.0, 0.0, 1.0).into(),
        ..default()
    };
}
