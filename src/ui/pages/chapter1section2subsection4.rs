use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    // winit::WinitSettings,
};

pub fn get_page(commands: &mut Commands) -> Entity {
    let text_item = (
        TextBundle::from_section(
            format!("Chaper 1, Section 2, Subsection 4: Spans"),
            TextStyle {
                font_size: 20.,
                ..default()
            },
        ),
        Label,
        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
    );
    let page_item = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Px(200.0),
            padding: UiRect {
                left: Val::Px(2.0),
                right: Val::Px(4.0),
                top: Val::Px(0.0),
                bottom: Val::Px(4.0),
            },
            ..default()
        },
        background_color: Color::rgb(0.1, 0.1, 0.1).into(),
        ..default()
    };

    let inner_item = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            // justify_items: JustifyItems::Center,
            ..default()
        },
        background_color: Color::rgb(0.4, 0.6, 0.4).into(),
        ..default()
    };

    let text_item = commands.spawn(text_item).id();
    let inner_item = commands.spawn(inner_item).id();
    let page_item = commands.spawn(page_item).id();

    commands.entity(inner_item).push_children(&[text_item]);
    commands.entity(page_item).push_children(&[inner_item]);
    
    return page_item;
}