use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    }, input::mouse::{MouseScrollUnit, MouseWheel}, prelude::*, render::view
    // winit::WinitSettings,
};

use crate::ui::{
    util::{
        theme::{self, swiper_background_color},
        style,
    },
    subsection_cameras::{self, FullscreenColorEvent, FullscreenEvent},
};

use super::example_block;

#[derive(Component)]
pub struct SelectionTextDescription {
    crew_id: u8,
    is_selected: bool,
}

#[derive(Component)]
pub struct FullscreenButton {
    pub crew_id: u8,
    pub is_selected: bool,
}
pub enum FullscreenArrowType{
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Component)]
pub struct ExampleFooter {
    pub crew_id: u8,
}

#[derive(Component)]
pub struct FullscreenArrow {
    pub crew_id: u8,
    pub is_selected: bool,
    pub arrow_type: FullscreenArrowType,
}

pub fn spawn(commands: &mut Commands, theme: &theme::CurrentTheme, view_list_entity: Entity, crew_id: u8, text: &str) {
    let background_banner = commands.spawn((
        ExampleFooter {
            crew_id,
        },
        theme::ColorFunction {
            background: theme::background_color,
            border: theme::background_color,
        },
        NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Px(70.0),
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
            border: theme::sidebar_collapsed_color,
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
            border_color: theme::sidebar_collapsed_color(theme).into(),
            ..default()
        },
        example_block::ExampleSkeletonCorner { crew_id: crew_id },
    )).id();

    let skeleton_right = commands.spawn((
        theme::ColorFunction {
            background: theme::background_color,
            border: theme::sidebar_collapsed_color,
        },
        NodeBundle {
            style: Style {
                height: Val::Percent(100.0),
                aspect_ratio: Some(1.0),

                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,

                border: UiRect {
                    left: Val::Px(0.0),
                    right: Val::Px(4.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(4.0),
                },
                ..default()
            },
            background_color: theme::background_color(theme).into(),
            border_color: theme::sidebar_collapsed_color(theme).into(),
            ..default()
        },
        example_block::ExampleSkeletonCorner { crew_id: crew_id },
    )).id();

    let fullscreen_button = fullscreen_button(commands, theme, crew_id);
    commands.entity(skeleton_right).push_children(&[fullscreen_button]);


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

pub fn fullscreen_button(
    commands: &mut Commands,
    theme: &theme::CurrentTheme,
    crew_id: u8,
) -> Entity {
    let background_banner = commands
        .spawn((
            theme::ColorFunction {
                background: theme::navbar_background_color,
                border: theme::sidebar_collapsed_color,
            },
            ButtonBundle {
                style: Style {
                    // height: Val::Percent(height),
                    height: style::BUTTON_HEIGHT,
                    aspect_ratio: Some(1.0),
                    flex_direction: FlexDirection::Column,
                    // align_content: AlignContent::SpaceBetween,
                    justify_content: JustifyContent::SpaceBetween,
                    overflow: Overflow::clip(),
                    ..default()
                },
                visibility: Visibility::Inherited,
                focus_policy: bevy::ui::FocusPolicy::Pass,
                background_color: theme::navbar_background_color(theme).into(),
                border_color: theme::sidebar_collapsed_color(theme).into(),
                ..default()
            },
            FullscreenButton {
                crew_id: crew_id,
                is_selected: false,
            },
        ))
        .id();

    let top = commands
        .spawn((
            theme::ColorFunction {
                background: theme::navbar_background_color,
                border: theme::sidebar_collapsed_color,
            },
            NodeBundle {
                style: Style {
                    height: style::BUTTON_HEIGHT / 3.0,
                    width: Val::Percent(100.0),

                    flex_direction: FlexDirection::Row,
                    // align_content: AlignContent::SpaceBetween,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                visibility: Visibility::Inherited,
                focus_policy: bevy::ui::FocusPolicy::Pass,
                background_color: theme::navbar_background_color(theme).into(),
                border_color: theme::sidebar_collapsed_color(theme).into(),
                ..default()
            },
            // FullscreenButton {
            //     crew_id: crew_id,
            //     is_selected: false,
            // },
        ))
        .id();

    let top_left = commands
        .spawn((
            theme::ColorFunction {
                background: theme::navbar_background_color,
                border: theme::sidebar_collapsed_color,
            },
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.0),
                    aspect_ratio: Some(1.0),

                    border: UiRect {
                        left: Val::Px(2.0),
                        right: Val::Px(0.0),
                        top: Val::Px(2.0),
                        bottom: Val::Px(0.0),
                    },
                    ..default()
                },
                visibility: Visibility::Inherited,
                focus_policy: bevy::ui::FocusPolicy::Pass,
                background_color: theme::navbar_background_color(theme).into(),
                border_color: theme::sidebar_collapsed_color(theme).into(),
                ..default()
            },
            FullscreenArrow {
                crew_id: crew_id,
                is_selected: false,
                arrow_type: FullscreenArrowType::TopLeft,
            },
        ))
        .id();

    let top_right = commands
        .spawn((
            theme::ColorFunction {
                background: theme::navbar_background_color,
                border: theme::sidebar_collapsed_color,
            },
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.0),
                    aspect_ratio: Some(1.0),

                    border: UiRect {
                        left: Val::Px(0.0),
                        right: Val::Px(2.0),
                        top: Val::Px(2.0),
                        bottom: Val::Px(0.0),
                    },
                    ..default()
                },
                visibility: Visibility::Inherited,
                focus_policy: bevy::ui::FocusPolicy::Pass,
                background_color: theme::navbar_background_color(theme).into(),
                border_color: theme::sidebar_collapsed_color(theme).into(),
                ..default()
            },
            FullscreenArrow {
                crew_id: crew_id,
                is_selected: false,
                arrow_type: FullscreenArrowType::TopRight,
            },
        ))
        .id();

    let bottom = commands
        .spawn((
            theme::ColorFunction {
                background: theme::navbar_background_color,
                border: theme::sidebar_collapsed_color,
            },
            NodeBundle {
                style: Style {
                    height: style::BUTTON_HEIGHT / 3.0,
                    width: Val::Percent(100.0),

                    flex_direction: FlexDirection::Row,
                    // align_content: AlignContent::SpaceBetween,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                visibility: Visibility::Inherited,
                focus_policy: bevy::ui::FocusPolicy::Pass,
                background_color: theme::navbar_background_color(theme).into(),
                border_color: theme::sidebar_collapsed_color(theme).into(),
                ..default()
            },
            // FullscreenButton {
            //     crew_id: crew_id,
            //     is_selected: false,
            // },
        ))
        .id();

    let bottom_left = commands
        .spawn((
            theme::ColorFunction {
                background: theme::navbar_background_color,
                border: theme::sidebar_collapsed_color,
            },
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.0),
                    aspect_ratio: Some(1.0),

                    border: UiRect {
                        left: Val::Px(2.0),
                        right: Val::Px(0.0),
                        top: Val::Px(0.0),
                        bottom: Val::Px(2.0),
                    },
                    ..default()
                },
                visibility: Visibility::Inherited,
                focus_policy: bevy::ui::FocusPolicy::Pass,
                background_color: theme::navbar_background_color(theme).into(),
                border_color: theme::sidebar_collapsed_color(theme).into(),
                ..default()
            },
            FullscreenArrow {
                crew_id: crew_id,
                is_selected: false,
                arrow_type: FullscreenArrowType::BottomLeft,
            },
        ))
        .id();

    let bottom_right = commands
        .spawn((
            theme::ColorFunction {
                background: theme::navbar_background_color,
                border: theme::sidebar_collapsed_color,
            },
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.0),
                    aspect_ratio: Some(1.0),

                    border: UiRect {
                        left: Val::Px(0.0),
                        right: Val::Px(2.0),
                        top: Val::Px(0.0),
                        bottom: Val::Px(2.0),
                    },
                    ..default()
                },
                visibility: Visibility::Inherited,
                focus_policy: bevy::ui::FocusPolicy::Pass,
                background_color: theme::navbar_background_color(theme).into(),
                border_color: theme::sidebar_collapsed_color(theme).into(),
                ..default()
            },
            FullscreenArrow {
                crew_id: crew_id,
                is_selected: false,
                arrow_type: FullscreenArrowType::BottomRight,
            },
        ))
        .id();
    
    commands.entity(top).push_children(&[top_left, top_right]);
    commands.entity(bottom).push_children(&[bottom_left, bottom_right]);
    commands.entity(background_banner).push_children(&[top, bottom]);

    return background_banner;
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
                        text.sections[0].style.color = theme::sidebar_collapsed_color(theme).into();
                        text.sections[0].value = String::from(" Press \"*\" at the top right to interact with the scene");
                        color_function.border = theme::sidebar_collapsed_color;
                    },
                }
                
            }
            else {
                text.sections[0].style.color = theme::sidebar_collapsed_color(theme).into();
                color_function.border = theme::sidebar_collapsed_color;
            }
        }
    }
}


pub fn fullscreen_button_interaction_system(
    mut interaction_query: Query<(&Interaction, &mut FullscreenButton), (Changed<Interaction>, With<FullscreenButton>)>,
    mut fullscreen_writer: EventWriter<subsection_cameras::FullscreenEvent>,
    mut fullscreen_color_writer: EventWriter<subsection_cameras::FullscreenColorEvent>,
) {
    for (interaction, mut fullscreen_button) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                match fullscreen_button.is_selected {
                    true => {
                        fullscreen_writer.send(
                            FullscreenEvent{
                                crew_id: fullscreen_button.crew_id,
                                maximize: false,
                            }
                        )
                    }
                    false => {
                        fullscreen_writer.send(
                            FullscreenEvent{
                                crew_id: fullscreen_button.crew_id,
                                maximize: true,
                            }
                        )
                    }
                }
                fullscreen_button.is_selected = !fullscreen_button.is_selected;
            }
            Interaction::Hovered => {
                // println!("hovered");
                match fullscreen_button.is_selected {
                    true => {
                        fullscreen_color_writer.send(
                            FullscreenColorEvent {
                                crew_id: fullscreen_button.crew_id,
                                color_function: theme::sidebar_collapsed_color,
                            }
                        )
                    }
                    false => {
                        fullscreen_color_writer.send(
                            FullscreenColorEvent {
                                crew_id: fullscreen_button.crew_id,
                                color_function: theme::sidebar_color,
                            }
                        )
                    }
                }
            }
            Interaction::None => {
                match fullscreen_button.is_selected {
                    true => {
                        fullscreen_color_writer.send(
                            FullscreenColorEvent {
                                crew_id: fullscreen_button.crew_id,
                                color_function: theme::sidebar_color,
                            }
                        )
                    }
                    false => {
                        fullscreen_color_writer.send(
                            FullscreenColorEvent {
                                crew_id: fullscreen_button.crew_id,
                                color_function: theme::sidebar_collapsed_color,
                            }
                        )
                    }
                }
            }
        }
    }
}

pub fn fullscreen_button_color_system (
    mut fullscreen_reader: EventReader<subsection_cameras::FullscreenEvent>,
    mut fullscreen_color_reader: EventReader<subsection_cameras::FullscreenColorEvent>,
    mut fullscreen_arrow_query: Query<(&FullscreenArrow, &mut Style, &mut BorderColor, &mut theme::ColorFunction), With<FullscreenArrow>>,

    theme: Res<theme::CurrentTheme>,
) {
    let theme = theme.as_ref();

    for fullscreen_color_event in fullscreen_color_reader.read() {
        // println!("fullscreen_color_event {:?}", fullscreen_color_event);
        for (fullscreen_arrow, style, mut border_color, color_function) in fullscreen_arrow_query.iter_mut() {
            if fullscreen_arrow.crew_id != fullscreen_color_event.crew_id { continue };
            // println!("before coloring");
            *border_color = (fullscreen_color_event.color_function)(theme).into();
        }
    }

    for fullscreen_event in fullscreen_reader.read() {
        for (fullscreen_arrow, mut style, mut border_color, mut color_function) in fullscreen_arrow_query.iter_mut() {
            if fullscreen_arrow.crew_id != fullscreen_event.crew_id { continue }
            match fullscreen_event.maximize {
                true => {
                    *border_color = theme::sidebar_collapsed_color(theme).into();
                    color_function.border = theme::sidebar_color;
                    match fullscreen_arrow.arrow_type {
                        FullscreenArrowType::TopLeft => {
                            style.border.top = Val::Px(0.0);
                            style.border.bottom = Val::Px(2.0);
                            style.border.left = Val::Px(0.0);
                            style.border.right = Val::Px(2.0);
                        }
                        FullscreenArrowType::TopRight => {
                            style.border.top = Val::Px(0.0);
                            style.border.bottom = Val::Px(2.0);
                            style.border.left = Val::Px(2.0);
                            style.border.right = Val::Px(0.0);
                        }
                        FullscreenArrowType::BottomLeft => {
                            style.border.top = Val::Px(2.0);
                            style.border.bottom = Val::Px(0.0);
                            style.border.left = Val::Px(0.0);
                            style.border.right = Val::Px(2.0);
                        }
                        FullscreenArrowType::BottomRight => {
                            style.border.top = Val::Px(2.0);
                            style.border.bottom = Val::Px(0.0);
                            style.border.left = Val::Px(2.0);
                            style.border.right = Val::Px(0.0);
                        }
                    }
                }
                false => {
                    *border_color = theme::sidebar_color(theme).into();
                    color_function.border = theme::sidebar_collapsed_color;
                    match fullscreen_arrow.arrow_type {
                        FullscreenArrowType::TopLeft => {
                            style.border.top = Val::Px(2.0);
                            style.border.bottom = Val::Px(0.0);
                            style.border.left = Val::Px(2.0);
                            style.border.right = Val::Px(0.0);
                        }
                        FullscreenArrowType::TopRight => {
                            style.border.top = Val::Px(2.0);
                            style.border.bottom = Val::Px(0.0);
                            style.border.left = Val::Px(0.0);
                            style.border.right = Val::Px(2.0);
                        }
                        FullscreenArrowType::BottomLeft => {
                            style.border.top = Val::Px(0.0);
                            style.border.bottom = Val::Px(2.0);
                            style.border.left = Val::Px(2.0);
                            style.border.right = Val::Px(0.0);
                        }
                        FullscreenArrowType::BottomRight => {
                            style.border.top = Val::Px(0.0);
                            style.border.bottom = Val::Px(2.0);
                            style.border.left = Val::Px(0.0);
                            style.border.right = Val::Px(2.0);
                        }
                    }
                }
            }
        }

    }
}