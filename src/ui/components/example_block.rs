use bevy::{
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
    view,
};
use super::example_header;
use super::example_footer;

#[derive(Component)]
pub struct ExampleSkeletonCorner {
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
    let example_block = new(commands);
    commands.entity(view_list_entity).push_children(&[example_block]);

    example_header::spawn(
        commands,
        theme,
        view_list_entity,
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
        view_list_entity,
        crew_id,
    );

    example_footer::spawn(
        commands,
        theme,
        view_list_entity,
        crew_id,
        format!(" Press \"*\" on the top right to activate").as_str(),
    );
    
    
}

fn new(commands: &mut Commands) -> Entity {
    return commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(60.0),
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
    mut camera_banner_query: Query<(Entity, &subsection_cameras::CameraBackgroundBanner, &mut Style), With<subsection_cameras::CameraBackgroundBanner>>,
    mut fullscreen_node_query: Query<Entity, With<root::FullscreenNode>>,

    mut ui_resize_writer: EventWriter<view::UiResizeEvent>,
    mut commands: Commands,
) {
    for fullscreen_event in fullscreen_reader.read() {
        println!("fullscreen maximize: {:?}", fullscreen_event.maximize);
        for (camera_banner_entity, camera_banner, mut style) in camera_banner_query.iter_mut() {
            
            if camera_banner.crew_id == fullscreen_event.crew_id {
                println!("node: {:?}", style);
                // *z_index = ZIndex::Global(2);
                for fullscreen_node_entity in fullscreen_node_query.iter() {
                    commands.entity(fullscreen_node_entity).push_children(&[camera_banner_entity]);
                }
                style.width = Val::Vw(100.0);
                style.height = Val::Vh(100.0);
                ui_resize_writer.send(view::UiResizeEvent);
        
            }
        }
    }
}