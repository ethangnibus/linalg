use bevy::prelude::*;

pub fn setup(commands: &mut Commands, height: f32) -> Entity {
    let navbar = new(height);
    return commands.spawn(navbar).id();
}

pub fn new(height: f32) -> NodeBundle {
    return NodeBundle {
        style: Style {
            height: Val::Percent(height),
            width: Val::Percent(100.0),
            ..default()
        },
        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
        ..default()
    };
}
