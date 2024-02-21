use bevy::{
    prelude::*,
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
};

pub fn get(commands: &mut Commands, view_list_entity: Entity) {
    let text_item = (
        TextBundle::from_section(
            format!("TODO: Remember to implement this page!"),
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
                left: Val::Px(4.0),
                right: Val::Px(4.0),
                top: Val::Px(4.0),
                bottom: Val::Px(8.0),
            },
            border: UiRect {
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(4.0),
            },
            ..default()
        },
        background_color: Color::rgb(0.3, 0.1, 0.1).into(),
        border_color: Color::rgb(0.1, 0.1, 0.1).into(),

        ..default()
    };

    let text_item = commands.spawn(text_item).id();
    let page_item = commands.spawn(page_item).id();

    commands.entity(page_item).push_children(&[text_item]);
    commands.entity(view_list_entity).push_children(&[page_item]);
}