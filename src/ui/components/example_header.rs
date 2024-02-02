use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    }, input::mouse::{MouseScrollUnit, MouseWheel}, prelude::*, render::view
    // winit::WinitSettings,
};

use crate::ui::util::theme;

pub fn spawn(commands: &mut Commands, theme: &theme::CurrentTheme, view_list_entity: Entity, text: &str) {

    let background_banner = commands.spawn((
        theme::ColorFunction {
            background: theme::background_color,
            border: theme::background_color,
        },
        NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Auto,
            min_height: Val::Px(50.0),
            border: UiRect {
                left: Val::Px(8.0),
                right: Val::Px(8.0),
                top: Val::Px(4.0),
                bottom: Val::Px(4.0),
            },
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        background_color: theme::background_color(theme).into(),
        border_color: theme::background_color(theme).into(),
        ..default()
    })).id();

    let skeleton_left = commands.spawn((
        theme::ColorFunction {
            background: theme::background_color,
            border: theme::swiper_background_color,
        },
        NodeBundle {
            style: Style {
                // width: Val::Percent(100.0), // FIXME: fix so navbar isnt weird when going to 1.2.4
                height: Val::Percent(100.0),
                aspect_ratio: Some(1.0),
                border: UiRect {
                    left: Val::Px(4.0),
                    right: Val::Px(0.0),
                    top: Val::Px(4.0),
                    bottom: Val::Px(0.0),
                },
                ..default()
            },
            background_color: theme::background_color(theme).into(),
            border_color: theme::swiper_background_color(theme).into(),
            ..default()
        },
    )).id();

    let skeleton_right = commands.spawn((
        theme::ColorFunction {
            background: theme::background_color,
            border: theme::swiper_background_color,
        },
        NodeBundle {
            style: Style {
                // width: Val::Percent(100.0), // FIXME: fix so navbar isnt weird when going to 1.2.4
                height: Val::Percent(100.0),
                aspect_ratio: Some(1.0),
                border: UiRect {
                    left: Val::Px(0.0),
                    right: Val::Px(4.0),
                    top: Val::Px(4.0),
                    bottom: Val::Px(0.0),
                },
                ..default()
            },
            background_color: theme::background_color(theme).into(),
            border_color: theme::swiper_background_color(theme).into(),
            ..default()
        },
    )).id();


    // make the text that appears on the banner
    let text_bundle = commands.spawn((
        theme::ColorFunction {
            background: theme::sidebar_color,
            border: theme::sidebar_color,
        },
        TextBundle::from_section(
            text,
            TextStyle {
                font_size: 24.,
                color: theme::sidebar_color(theme).into(),
                ..default()
            },
        ).with_style(
            Style {
                padding: UiRect {
                    left: Val::Px(20.0),
                    right: Val::Px(4.0),
                    top: Val::Px(4.0),
                    bottom: Val::Px(4.0),
                },
                justify_self: JustifySelf::Start,
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                ..default()
            }
        ),
        Label,
        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
    )).id();

    // commands.entity(text_banner).push_children(&[text_bundle]);

    commands.entity(background_banner).push_children(&[skeleton_left, skeleton_right, text_bundle]);
    commands.entity(view_list_entity).push_children(&[background_banner]);
}