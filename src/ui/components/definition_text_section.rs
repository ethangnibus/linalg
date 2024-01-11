use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    // winit::WinitSettings,
};

pub fn spawn(commands: &mut Commands, text: &str) -> Entity {

    // make body under definition header
    let body_content = commands.spawn(
        NodeBundle {
        style: Style {
            width: Val::Px(200.0),
            flex_grow: 1.0,
            height: Val::Auto,
            padding: UiRect {
                left: Val::Px(4.0),
                right: Val::Px(8.0),
                top: Val::Px(4.0),
                bottom: Val::Px(4.0),
            },
            border: UiRect {
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
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

    let text = commands.spawn((
        TextBundle::from_section(
            // format!(text),
            text,
            TextStyle {
                font_size: 20.,
                ..default()
            },
        ),
        Label,
        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
    )).id();

    commands.entity(body_content).push_children(&[text]);

    return body_content;
}