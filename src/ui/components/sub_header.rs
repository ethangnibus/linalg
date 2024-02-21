use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    }, input::mouse::{MouseScrollUnit, MouseWheel}, prelude::*, render::view
    // winit::WinitSettings,
};
use bevy_inspector_egui::egui::Align;

use crate::ui::util::theme;

pub fn spawn(commands: &mut Commands, theme: &theme::CurrentTheme, view_list_entity: Entity, text: &str) {

    let background_banner = commands.spawn((
        theme::ColorFunction {
            background: theme::swiper_background_color,
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
                top: Val::Px(28.0),
                bottom: Val::Px(0.0),
            },
            padding: UiRect {
                left: Val::Px(12.0),
                right: Val::Px(12.0),
                top: Val::Px(4.0),
                bottom: Val::Px(4.0),
            },
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Start,
            
            ..default()
        },
        background_color: theme::swiper_background_color(theme).into(),
        border_color: theme::background_color(theme).into(),
        ..default()
    })).id();

    // make the text that appears on the banner
    let text_bundle = commands.spawn((
        theme::ColorFunction {
            background: theme::text_color,
            border: theme::text_color,
        },
        TextBundle::from_section(
            text,
            TextStyle {
                font_size: 24.,
                color: theme::text_color(theme).into(),
                ..default()
            },
        ),
        Label,
        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
    )).id();

    // commands.entity(text_banner).push_children(&[text_bundle]);

    commands.entity(background_banner).push_children(&[text_bundle]);
    commands.entity(view_list_entity).push_children(&[background_banner]);
}