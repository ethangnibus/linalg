use std::iter::Zip;

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
use bevy_inspector_egui::egui::Align;

use crate::ui::{
    fullscreen_camera::FullscreenCameraBanner, root, subsection_cameras, util::theme, view::{self, UiResizeEvent}
};
use bevy::render::view::RenderLayers;
use bevy_mod_picking::prelude::*;
use crate::TextbookCamera;

use super::example_header;
use super::example_footer;
use super::super::fullscreen_camera;
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
    // let example_block = new(commands, crew_id);
    // commands.entity(view_list_entity).push_children(&[example_block]);
    let camera_banner = subsection_cameras::setup_camera(
        commands,
        theme,
        film_crew_entity,
        Val::Px(500.0),
        meshes,
        materials,
        images,
        view_list_entity,
        crew_id,
    );

    let example_skeleton_banner = commands.spawn((
        ExampleSkeletonBanner {
            crew_id: crew_id,
        },
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                // flex_grow: 3.0,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            focus_policy: FocusPolicy::Pass,
            ..default()
        },
        Pickable::IGNORE,
        RenderLayers::layer(crew_id),
    )).id();
    // commands.entity(example_block).push_children(&[example_skeleton_banner]);
    commands.entity(camera_banner).push_children(&[example_skeleton_banner]);

    example_header::spawn(
        commands,
        theme,
        example_skeleton_banner,
        crew_id,
        format!(" Example {}", crew_id).as_str(),
    );

    example_footer::spawn(
        commands,
        theme,
        example_skeleton_banner,
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
                height: Val::Px(500.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            focus_policy: FocusPolicy::Pass,
            ..default()
        },
        
        RenderLayers::layer(crew_id),
        Pickable::IGNORE,
        
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
                        *border_color = theme::sidebar_collapsed_color(theme).into();
                        color_function.border = theme::sidebar_collapsed_color;
                    },
                }
                
            }
            else {
                *border_color = theme::sidebar_collapsed_color(theme).into();
                color_function.border = theme::sidebar_collapsed_color;
            }
        }
    }
}

#[derive(Component)]
pub struct ExampleSkeletonBanner {
    pub crew_id: u8,
}

pub fn fullscreen_event_system (
    mut fullscreen_reader: EventReader<subsection_cameras::FullscreenEvent>,
    // mut fullscreen_node_query: Query<(Entity, &mut FocusPolicy), With<root::FullscreenNode>>,
    // mut root_node_query: Query<&mut Visibility, With<root::Root>>,

    // mut example_block_query: Query<(Entity, &ExampleBlock), With<ExampleBlock>>,
    // mut example_header_query: Query<(Entity, &example_header::ExampleHeader), With<example_header::ExampleHeader>>,
    // mut example_footer_query: Query<(Entity, &example_footer::ExampleFooter), With<example_footer::ExampleFooter>>,
    // mut camera_banner_query: Query<(Entity, &subsection_cameras::CameraBackgroundBanner, &mut Style), With<subsection_cameras::CameraBackgroundBanner>>,
    mut example_banner_query: Query<(Entity, &ExampleSkeletonBanner), With<ExampleSkeletonBanner>>,
    camera_banner_query: Query<(Entity, &subsection_cameras::CameraBackgroundBanner), With<subsection_cameras::CameraBackgroundBanner>>,

    mut ui_resize_writer: EventWriter<view::UiResizeEvent>,

    mut root_entity_visibility_query: Query<&mut Visibility, With<root::Root>>,


    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,

    mut textbook_camera: Query<(Entity), With<TextbookCamera>>,
    // mut mini_camera_query: Query<(&subsection_cameras::MiniCamera, &subsection_cameras::PanOrbitCamera), With<subsection_cameras::MiniCamera>>,

    // mut fullscreen_banner_query: Query<&mut Visibility, (Without<fullscreen_camera::FullscreenCameraBanner>, With<fullscreen_camera::TextbookCameraBanner>)>,
) {
    for fullscreen_event in fullscreen_reader.read() {
        let camera = textbook_camera.single();
        let mut root_entity_visibility = root_entity_visibility_query.single_mut();

        if fullscreen_event.maximize {

            // add render layer to main camera
            commands.entity(camera).insert(
                RenderLayers::layer(fullscreen_event.crew_id),
            );

            for (camera_banner_entity, camera_banner) in camera_banner_query.iter() {
                if camera_banner.crew_id != fullscreen_event.crew_id { continue }
                for (example_banner_entity, example_banner) in example_banner_query.iter() {
                    if example_banner.crew_id != fullscreen_event.crew_id { continue }
                    commands.entity(camera_banner_entity).remove_children(&[example_banner_entity]);
                }
            }
            *root_entity_visibility = Visibility::Hidden;
        } else {
            // remove render layer from main camera
            commands.entity(camera).remove::<RenderLayers>();

            for (camera_banner_entity, camera_banner) in camera_banner_query.iter() {
                if camera_banner.crew_id != fullscreen_event.crew_id { continue }
                for (example_banner_entity, example_banner) in example_banner_query.iter() {
                    if example_banner.crew_id != fullscreen_event.crew_id { continue }
                    
                    commands.entity(camera_banner_entity).push_children(&[example_banner_entity]);
                }
            }
            *root_entity_visibility = Visibility::Inherited;
        }
        ui_resize_writer.send(UiResizeEvent);
    }
}
