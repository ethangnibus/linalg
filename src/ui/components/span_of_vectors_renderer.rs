use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    // winit::WinitSettings,
};

pub fn spawn(commands: &mut Commands) -> Entity {

    // make body under definition header
    let body_content = commands.spawn(
        NodeBundle {
        style: Style {
            width: Val::Percent(50.0),
            height: Val::Percent(100.0),
            padding: UiRect {
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
            },
            border: UiRect {
                left: Val::Px(4.0),
                right: Val::Px(4.0),
                top: Val::Px(4.0),
                bottom: Val::Px(4.0),
            },
            flex_direction: FlexDirection::Column,
            justify_items: JustifyItems::Start,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgb(1.0 / 3.0, 0.7 / 3.0, 0.1 / 3.0).into(),
        border_color: Color::rgb(0.1, 0.1, 0.1).into(),
        ..default()
    }).id();

    return body_content;
}