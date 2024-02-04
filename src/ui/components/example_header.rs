use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    }, input::mouse::{MouseScrollUnit, MouseWheel}, prelude::*, render::view
    // winit::WinitSettings,
};
use bevy_inspector_egui::egui::Align;

use crate::ui::{subsection_cameras, util::{
    theme,
    style,
}};

use super::example_block;


#[derive(Component)]
pub struct SelectionButton {
    pub crew_id: u8,
    pub is_selected: bool,
}

#[derive(Component)]
pub struct SelectionButtonText {
    pub crew_id: u8,
    pub is_selected: bool,
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
                top: Val::Px(4.0),
                bottom: Val::Px(8.0),
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
        example_block::ExampleSkeletonCorner { crew_id: crew_id },
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
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                // align_content: AlignContent::FlexStart,

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
        example_block::ExampleSkeletonCorner { crew_id: crew_id },
    )).id();

    let selection_button = selection_button(commands, theme, crew_id);
    commands.entity(skeleton_right).push_children(&[selection_button]);


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



pub fn selection_button(
    commands: &mut Commands,
    theme: &theme::CurrentTheme,
    crew_id: u8,
) -> Entity {
    let background_banner = commands
        .spawn((
            theme::ColorFunction {
                background: theme::navbar_background_color,
                border: theme::swiper_background_color,
            },
            ButtonBundle {
                style: Style {
                    // height: Val::Percent(height),
                    height: style::BUTTON_HEIGHT,
                    aspect_ratio: Some(1.0),

                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border: UiRect {
                        left: Val::Px(2.0),
                        right: Val::Px(2.0),
                        top: Val::Px(2.0),
                        bottom: Val::Px(2.0),
                    },
                    overflow: Overflow::clip(),
                    ..default()
                },
                visibility: Visibility::Inherited,
                focus_policy: bevy::ui::FocusPolicy::Pass,
                background_color: theme::navbar_background_color(theme).into(),
                border_color: theme::swiper_background_color(theme).into(),
                ..default()
            },
            SelectionButton{
                crew_id: crew_id,
                is_selected: false,
            },
        ))
        .id();

    let text = commands
        .spawn((
            theme::ColorFunction {
                background: theme::swiper_background_color,
                border: theme::swiper_background_color,
            },
            TextBundle::from_section(
                "*",
                TextStyle {
                    font_size: 50.0,
                    color: theme::swiper_background_color(theme).into(),
                    ..default()
                },
            ),
            SelectionButtonText {
                crew_id: crew_id,
                is_selected: false,
            },
        ))
        .id();

    commands.entity(background_banner).push_children(&[text]);

    return background_banner;
}



pub fn selection_button_interation(
    interaction_query: Query<(&SelectionButton, &Interaction), (Changed<Interaction>, With<SelectionButton>)>,
    mut camera_selection_writer: EventWriter<subsection_cameras::CameraSelectionEvent>,
    mut camera_selection_color_writer: EventWriter<subsection_cameras::CameraSelectionColorEvent>,
) {
    for (selection_button, interaction) in interaction_query.iter() {
        match interaction {
            Interaction::Pressed => {
                if selection_button.is_selected { 
                    camera_selection_writer.send(
                        subsection_cameras::CameraSelectionEvent {
                            crew_id: selection_button.crew_id,
                            select_this_camera: false,
                        }
                    )
                } else {
                    camera_selection_writer.send(
                        subsection_cameras::CameraSelectionEvent {
                            crew_id: selection_button.crew_id,
                            select_this_camera: true,
                        }
                    )
                }
            },
            Interaction::Hovered => {
                match selection_button.is_selected {
                    true => {
                        camera_selection_color_writer.send(
                            subsection_cameras::CameraSelectionColorEvent {
                                crew_id: selection_button.crew_id,
                                color_function: theme::swiper_background_color,
                            }
                        )
                    }
                    false => {
                        camera_selection_color_writer.send(
                            subsection_cameras::CameraSelectionColorEvent {
                                crew_id: selection_button.crew_id,
                                color_function: theme::sidebar_color,
                            }
                        )
                    }
                }
            },
            Interaction::None => {
                match selection_button.is_selected {
                    true => {
                        camera_selection_color_writer.send(
                            subsection_cameras::CameraSelectionColorEvent {
                                crew_id: selection_button.crew_id,
                                color_function: theme::sidebar_color,
                            }
                        )
                    }
                    false => {
                        camera_selection_color_writer.send(
                            subsection_cameras::CameraSelectionColorEvent {
                                crew_id: selection_button.crew_id,
                                color_function: theme::swiper_background_color,
                            }
                        )
                    }
                }
            }
        }
    }
}


pub fn selection_button_color_system(
    mut camera_selection_reader: EventReader<subsection_cameras::CameraSelectionEvent>,
    mut camera_selection_color_reader: EventReader<subsection_cameras::CameraSelectionColorEvent>,
    mut selection_button_query: Query<(&mut BorderColor, &mut theme::ColorFunction, &SelectionButton), With<SelectionButton>>,
    theme: Res<theme::CurrentTheme>,
) {
    let theme = theme.as_ref();

    for camera_selection_color_event in camera_selection_color_reader.read() {
        for (mut border_color, mut color_function, mut selection_button) in selection_button_query.iter_mut() {
            if selection_button.crew_id == camera_selection_color_event.crew_id {
                *border_color = (camera_selection_color_event.color_function)(theme).into();
            }
        }
    }

    for camera_selection_event in camera_selection_reader.read() {
        // println!("camera_selection_event {:?}", camera_selection_event);
        for (mut border_color, mut color_function, mut selection_button) in selection_button_query.iter_mut() {
            if selection_button.crew_id == camera_selection_event.crew_id {
                match camera_selection_event.select_this_camera {
                    true => {
                        *border_color = theme::sidebar_color(theme).into();
                        color_function.border = theme::sidebar_color;
                    },
                    false => {
                        *border_color = theme::swiper_background_color(theme).into();
                        color_function.border = theme::swiper_background_color;
                    },
                }
                
            }
            else {
                *border_color = theme::swiper_background_color(theme).into();
                color_function.border = theme::swiper_background_color;
            }
        }
    }
}

pub fn selection_button_text_color_system(
    mut camera_selection_reader: EventReader<subsection_cameras::CameraSelectionEvent>,
    mut camera_selection_color_reader: EventReader<subsection_cameras::CameraSelectionColorEvent>,
    mut selection_button_text_query: Query<(&mut Text, &mut theme::ColorFunction, &SelectionButtonText), With<SelectionButtonText>>,
    theme: Res<theme::CurrentTheme>,
) {
    let theme = theme.as_ref();

    for camera_selection_color_event in camera_selection_color_reader.read() {
        for (mut text, mut color_function, mut selection_button) in selection_button_text_query.iter_mut() {
            if selection_button.crew_id == camera_selection_color_event.crew_id {
                text.sections[0].style.color = (camera_selection_color_event.color_function)(theme).into();
            }
        }
    }

    for camera_selection_event in camera_selection_reader.read() {
        for (mut text, mut color_function, mut selection_button) in selection_button_text_query.iter_mut() {
            if selection_button.crew_id == camera_selection_event.crew_id {
                match camera_selection_event.select_this_camera {
                    true => {
                        text.sections[0].style.color = theme::sidebar_color(theme).into();
                        color_function.background = theme::sidebar_color;
                        text.sections[0].value = String::from("=");
                        color_function.border = theme::sidebar_color;
                    },
                    false => {
                        text.sections[0].style.color = theme::swiper_background_color(theme).into();
                        color_function.background = theme::swiper_background_color;
                        text.sections[0].value = String::from("*");
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