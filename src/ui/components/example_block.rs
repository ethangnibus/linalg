use bevy::{
    ui::FocusPolicy,
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    // winit::WinitSettings,
};

use crate::ui::{
    util::theme,
    subsection_cameras,
    root,
    view::{self, UiResizeEvent},
};
use super::example_header;
use super::example_footer;

#[derive(Component)]
pub struct ExampleSkeletonCorner {
    pub crew_id: u8,
}

#[derive(Component)]
pub struct ExampleBlock {
    pub crew_id: u8,
}

pub fn spawn(
    commands: &mut Commands,
    theme: &theme::CurrentTheme,
    film_crew_entity: Entity,
    height: Val,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    mut images: &mut ResMut<Assets<Image>>,
    view_list_entity: Entity,
    crew_id: u8,
) {
    let example_block = new(commands, crew_id);
    commands.entity(view_list_entity).push_children(&[example_block]);

    example_header::spawn(
        commands,
        theme,
        example_block,
        crew_id,
        format!(" Example {}", crew_id).as_str(),
    );

    subsection_cameras::setup_camera(
        commands,
        theme,
        film_crew_entity,
        Val::Px(400.0),
        meshes,
        materials,
        images,
        example_block,
        crew_id,
    );

    example_footer::spawn(
        commands,
        theme,
        example_block,
        crew_id,
        format!(" Press \"*\" on the top right to activate").as_str(),
    );
    
    
}

fn new(commands: &mut Commands, crew_id: u8) -> Entity {
    return commands.spawn((
        ExampleBlock {
            crew_id: crew_id,
        },
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Auto,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
    )).id();
}


pub fn example_skeleton_color_system(
    mut camera_selection_reader: EventReader<subsection_cameras::CameraSelectionEvent>,
    mut camera_selection_color_reader: EventReader<subsection_cameras::CameraSelectionColorEvent>,
    mut example_skeleton_query: Query<(&mut BorderColor, &mut theme::ColorFunction, &ExampleSkeletonCorner), With<ExampleSkeletonCorner>>,
    theme: Res<theme::CurrentTheme>,
) {
    let theme = theme.as_ref();

    for camera_selection_color_event in camera_selection_color_reader.read() {
        for (mut border_color, mut color_function, mut skeleton_corner) in example_skeleton_query.iter_mut() {
            if skeleton_corner.crew_id == camera_selection_color_event.crew_id {
                *border_color = (camera_selection_color_event.color_function)(theme).into();
            }
        }
    }

    for camera_selection_event in camera_selection_reader.read() {
        // println!("camera_selection_event {:?}", camera_selection_event);
        for (mut border_color, mut color_function, mut skeleton_corner) in example_skeleton_query.iter_mut() {
            if skeleton_corner.crew_id == camera_selection_event.crew_id {
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


pub fn fullscreen_event_system (
    mut fullscreen_reader: EventReader<subsection_cameras::FullscreenEvent>,
    mut fullscreen_node_query: Query<(Entity, &mut FocusPolicy), With<root::FullscreenNode>>,

    mut example_block_query: Query<(Entity, &ExampleBlock), With<ExampleBlock>>,
    mut example_header_query: Query<(Entity, &example_header::ExampleHeader), With<example_header::ExampleHeader>>,
    mut example_footer_query: Query<(Entity, &example_footer::ExampleFooter), With<example_footer::ExampleFooter>>,
    mut camera_banner_query: Query<(Entity, &subsection_cameras::CameraBackgroundBanner, &mut Style), With<subsection_cameras::CameraBackgroundBanner>>,



    mut ui_resize_writer: EventWriter<view::UiResizeEvent>,
    mut commands: Commands,
) {
    // Fixme: maybe move the 6 nexted for loops lmaooooo
    // Not that it would matter tho it's not like this is gonna
    // scale or anything
    for fullscreen_event in fullscreen_reader.read() {
        println!("fullscreen maximize: {:?}", fullscreen_event.maximize);
        for (camera_banner_entity, camera_banner, mut camera_banner_style) in camera_banner_query.iter_mut() {
            if camera_banner.crew_id != fullscreen_event.crew_id { continue };
            for (footer_entity, footer) in example_footer_query.iter() {
                if footer.crew_id != fullscreen_event.crew_id { continue };
                for (header_entity, header) in example_header_query.iter() {
                    if header.crew_id != fullscreen_event.crew_id { continue };
                    if fullscreen_event.maximize {
                        // add to fullscreen node
                        for (fullscreen_node_entity, mut fullscreen_node_focus_policy) in fullscreen_node_query.iter_mut() {
                            commands.entity(fullscreen_node_entity).push_children(&[
                                header_entity,
                                camera_banner_entity,
                                footer_entity,
                            ]);
                            *fullscreen_node_focus_policy = FocusPolicy::Block;

                            camera_banner_style.flex_grow = 3.0;
                        }
                    } else {
                        // add to example_block
                        for (example_block_entity, example_block) in example_block_query.iter() {
                            if example_block.crew_id != fullscreen_event.crew_id { continue };
                            commands.entity(example_block_entity).push_children(&[
                                header_entity,
                                camera_banner_entity,
                                footer_entity,
                            ]);
                        }
                        // remove focus block
                        for (_fullscreen_node_entity, mut fullscreen_node_focus_policy) in fullscreen_node_query.iter_mut() {
                            *fullscreen_node_focus_policy = FocusPolicy::Pass;
                        }
                    }
                }
            }
        }
        ui_resize_writer.send(UiResizeEvent);
    }
}

//             // if camera_banner.crew_id != fullscreen_event.crew_id { continue };
//             for (example_block_entity, example_block) in example_block_query.iter() {
//                 if example_block.crew_id != fullscreen_event.crew_id { continue };
//                 for fullscreen_node_entity in fullscreen_node_query.iter() {

//                     if fullscreen_event.maximize {
//                         commands.entity(fullscreen_node_entity).push_children(&[example_block_entity]);
//                     } else {
//                         commands.entity(fullscreen_node_entity).clear_children(); // remember to add children back to viewlist
//                     }
                    
//                 }
//                 // example_block_style.width = Val::Vw(100.0);
//                 // example_block_style.height = Val::Vh(100.0);
//                 ui_resize_writer.send(view::UiResizeEvent);
//             }
//         }
//     }
// }