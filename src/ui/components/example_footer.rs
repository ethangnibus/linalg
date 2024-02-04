use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    }, input::mouse::{MouseScrollUnit, MouseWheel}, prelude::*, render::view
    // winit::WinitSettings,
};

use crate::ui::{
    util::theme,
    subsection_cameras,
};

use super::example_block;

#[derive(Component)]
pub struct SelectionTextDescription {
    crew_id: u8,
    is_selected: bool,
}

pub fn spawn(commands: &mut Commands, theme: &theme::CurrentTheme, view_list_entity: Entity, crew_id: u8, text: &str) {
    let background_banner = commands.spawn((
        theme::ColorFunction {
            background: theme::background_color,
            border: theme::background_color,
        },
        NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Auto,
            min_height: Val::Px(70.0),
            border: UiRect {
                left: Val::Px(8.0),
                right: Val::Px(8.0),
                top: Val::Px(8.0),
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
                height: Val::Percent(100.0),
                aspect_ratio: Some(1.0),
                border: UiRect {
                    left: Val::Px(4.0),
                    right: Val::Px(0.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(4.0),
                },
                ..default()
            },
            background_color: theme::background_color(theme).into(),
            border_color: theme::swiper_background_color(theme).into(),
            ..default()
        },
        example_block::ExampleSkeletonCorner { crew_id: crew_id },
    )).id();

    let skeleton_right = commands.spawn((
        theme::ColorFunction {
            background: theme::background_color,
            border: theme::swiper_background_color,
        },
        NodeBundle {
            style: Style {
                height: Val::Percent(100.0),
                aspect_ratio: Some(1.0),
                border: UiRect {
                    left: Val::Px(0.0),
                    right: Val::Px(4.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(4.0),
                },
                ..default()
            },
            background_color: theme::background_color(theme).into(),
            border_color: theme::swiper_background_color(theme).into(),
            ..default()
        },
        example_block::ExampleSkeletonCorner { crew_id: crew_id },
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
                overflow: Overflow::clip(),
                justify_self: JustifySelf::Start,
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                ..default()
            }
        ),
        SelectionTextDescription {
            crew_id: crew_id,
            is_selected: false,
        },
        Label,
        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
    )).id();

    commands.entity(background_banner).push_children(&[skeleton_left, skeleton_right, text_bundle]);

    let space_under = commands.spawn((
        theme::ColorFunction {
            background: theme::background_color,
            border: theme::background_color,
        },
        NodeBundle {
            style: Style {
                height: Val::Px(10.0),
                width: Val::Px(100.0),
                ..default()
            },
            ..default()
        }

    )).id();
    commands.entity(view_list_entity).push_children(&[background_banner, space_under]);
}


pub fn selection_text_description_color_system(
    mut camera_selection_reader: EventReader<subsection_cameras::CameraSelectionEvent>,
    mut camera_selection_color_reader: EventReader<subsection_cameras::CameraSelectionColorEvent>,
    mut selection_text_description_query: Query<(&mut Text, &mut theme::ColorFunction, &SelectionTextDescription), With<SelectionTextDescription>>,
    theme: Res<theme::CurrentTheme>,
) {
    let theme = theme.as_ref();

    for camera_selection_color_event in camera_selection_color_reader.read() {
        for (mut text, mut color_function, mut selection_text_description) in selection_text_description_query.iter_mut() {
            if selection_text_description.crew_id == camera_selection_color_event.crew_id {
                text.sections[0].style.color = (camera_selection_color_event.color_function)(theme).into();
            }
        }
    }

    for camera_selection_event in camera_selection_reader.read() {
        for (mut text, mut color_function, mut selection_text_description) in selection_text_description_query.iter_mut() {
            if selection_text_description.crew_id == camera_selection_event.crew_id {
                match camera_selection_event.select_this_camera {
                    true => {
                        text.sections[0].style.color = theme::sidebar_color(theme).into();
                        text.sections[0].value = String::from(" Press \"=\" to return to scrolling");
                        color_function.border = theme::sidebar_color;
                    },
                    false => {
                        text.sections[0].style.color = theme::swiper_background_color(theme).into();
                        text.sections[0].value = String::from(" Press \"*\" at the top right to interact with the scene");
                        color_function.border = theme::swiper_background_color;
                    },
                }
                
            }
            else {
                text.sections[0].style.color = theme::swiper_background_color(theme).into();
                color_function.border = theme::swiper_background_color;
            }
        }
    }
}