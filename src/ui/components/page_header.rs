use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    // winit::WinitSettings,
};

use crate::ui::util::theme;

pub fn spawn(commands: &mut Commands, theme: &theme::CurrentTheme, view_list_entity: Entity, text: &str) {

    // make banner behind the text
    let background_banner = commands.spawn((
        theme::ColorFunction {
            background: theme::sidebar_color,
            border: theme::background_color,
        },
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Auto,
                min_height: Val::Px(54.0),
                padding: UiRect {
                    left: Val::Px(4.0),
                    right: Val::Px(0.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(4.0),
                },
                border: UiRect {
                    left: Val::Px(0.0),
                    right: Val::Px(0.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(4.0),
                },
                justify_items: JustifyItems::Start,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: theme::sidebar_color(theme).into(),
            border_color: theme::background_color(theme).into(),
            ..default()
        },
    )).id();

    // make the text that appears on the banner
    let text_bundle = commands.spawn((
        theme::ColorFunction {
            background: theme::sidebar_header_text_color,
            border: theme::sidebar_header_text_color,
        },
        TextBundle::from_section(
            // format!(text),
            text,
            TextStyle {
                font_size: 30.,
                color: theme::sidebar_header_text_color(theme).into(),
                ..default()
            },
        ),
        Label,
        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
    )).id();

    commands.entity(background_banner).push_children(&[text_bundle]);
    commands.entity(view_list_entity).push_children(&[background_banner]);
}