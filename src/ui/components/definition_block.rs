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
    let title_text = "Span of vectors";

    // make banner behind the definition node
    let background_banner = commands.spawn(
        NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Px(200.0),
            padding: UiRect {
                left: Val::Px(4.0),
                right: Val::Px(4.0),
                top: Val::Px(4.0),
                bottom: Val::Px(4.0),
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
        background_color: Color::rgb(0.2, 0.2, 0.0).into(),
        border_color: Color::rgb(0.1, 0.1, 0.1).into(),
        ..default()
    }).id();

    // definition flag
    let definition_flag = commands.spawn(
        NodeBundle {
        style: Style {
            width: Val::Px(200.0),
            height: Val::Px(100.0),
            // min_height: Val::Px(100.0),
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
            flex_direction: FlexDirection::Row,
            justify_items: JustifyItems::Start,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgb(5.0, 0.1, 0.1).into(),
        // border_color: Color::rgb(0.1, 0.1, 0.1).into(),
        ..default()
    }).id();

    // fake background
    let fake_background = commands.spawn(
        NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Px(30.0),
            // min_height: Val::Px(100.0),
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
            flex_direction: FlexDirection::Row,
            justify_items: JustifyItems::Start,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgb(0.1, 0.3, 0.1).into(),
        // border_color: Color::rgb(0.1, 0.1, 0.1).into(),
        ..default()
    }).id();

    // definition banner
    let definition_banner = commands.spawn(
        NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Auto,
            // min_height: Val::Px(100.0),
            padding: UiRect {
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(12.0),
                bottom: Val::Px(12.0),
            },
            border: UiRect {
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
            },
            flex_direction: FlexDirection::Row,
            justify_items: JustifyItems::Start,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgb(0.1, 0.1, 1.0).into(),
        // border_color: Color::rgb(0.1, 0.1, 0.1).into(),
        ..default()
    }).id();

    // make the text that appears on the banner
    let text_bundle = commands.spawn((
        TextBundle::from_section(
            // format!(text),
            text,
            TextStyle {
                font_size: 24.,
                ..default()
            },
        ),
        Label,
        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
    )).id();

    commands.entity(definition_banner).push_children(&[text_bundle]);
    commands.entity(background_banner).push_children(&[fake_background, definition_banner, definition_flag]);
    
    return background_banner;
}