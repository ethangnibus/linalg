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

    // make banner behind the text
    let background_banner = commands.spawn(
        NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Auto,
            min_height: Val::Px(50.0),
            padding: UiRect {
                left: Val::Px(4.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(4.0),
            },
            border: UiRect {
                left: Val::Px(4.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(4.0),
            },
            flex_direction: FlexDirection::Column,
            justify_items: JustifyItems::Start,
            align_items: AlignItems::Start,
            ..default()
        },
        background_color: Color::rgb(0.1, 0.1, 0.1).into(),
        border_color: Color::rgb(0.1, 0.1, 0.1).into(),
        ..default()
    }).id();

    // make the text that appears on the banner
    let text_bundle = commands.spawn((
        TextBundle::from_section(
            // format!(text),
            text,
            TextStyle {
                font_size: 24.,
                color: Color::rgb(1.0, 0.7, 0.1).into(),
                ..default()
            },
        ),
        Label,
        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
    )).id();

    let horizontal_bar = commands.spawn(
        NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Px(1.0),
            padding: UiRect {
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
            },
            border: UiRect {
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
            },
            justify_items: JustifyItems::Start,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgb(1.0, 1.0, 1.0).into(),
        border_color: Color::rgb(0.1, 0.1, 0.1).into(),
        ..default()
    }).id();

    commands.entity(background_banner).push_children(&[horizontal_bar, text_bundle]);
    
    return background_banner;
}