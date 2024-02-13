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
use bevy::render::view::RenderLayers;
use bevy_mod_picking::prelude::*;
use crate::Main2dCamera;

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


pub fn fullscreen_event_system (
    mut fullscreen_reader: EventReader<subsection_cameras::FullscreenEvent>,
    // mut fullscreen_node_query: Query<(Entity, &mut FocusPolicy), With<root::FullscreenNode>>,
    // mut root_node_query: Query<&mut Visibility, With<root::Root>>,

    // mut example_block_query: Query<(Entity, &ExampleBlock), With<ExampleBlock>>,
    // mut example_header_query: Query<(Entity, &example_header::ExampleHeader), With<example_header::ExampleHeader>>,
    // mut example_footer_query: Query<(Entity, &example_footer::ExampleFooter), With<example_footer::ExampleFooter>>,
    // mut camera_banner_query: Query<(Entity, &subsection_cameras::CameraBackgroundBanner, &mut Style), With<subsection_cameras::CameraBackgroundBanner>>,



    mut ui_resize_writer: EventWriter<view::UiResizeEvent>,


    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,

    mut main_2d_camera: Query<(Entity), With<Main2dCamera>>,
    // mut mini_camera_query: Query<(&subsection_cameras::MiniCamera, &subsection_cameras::PanOrbitCamera), With<subsection_cameras::MiniCamera>>,

    // mut fullscreen_banner_query: Query<&mut Visibility, (Without<fullscreen_camera::FullscreenCameraBanner>, With<fullscreen_camera::TextbookCameraBanner>)>,
) {
    for fullscreen_event in fullscreen_reader.read() {
        for camera in main_2d_camera.iter_mut() {
            commands.entity(camera).insert(
                RenderLayers::layer(1),
            );
        }
    }
    



    // // Fixme: maybe move the 6 nexted for loops lmaooooo
    // // Not that it would matter tho it's not like this is gonna
    // // scale or anything
    // for fullscreen_event in fullscreen_reader.read() {
    //     // for (main_camera, mut main_camera_visibility) in main_2d_camera.iter_mut() {
    //     //     // commands.entity(main_camera).despawn();
    //     //     // *main_camera_visibility = Visibility::Hidden;
    //     // }
    //     // commands.spawn((
    //     //     PbrBundle {
    //     //         mesh: meshes.add(Mesh::from(shape::Plane::from_size(5.0))),
    //     //         material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //     //         ..default()
    //     //     },
    //     //     PickableBundle::default(), // Optional: adds selection, highlighting, and helper components.
    //     // ));
    //     // commands.spawn((
    //     //     PbrBundle {
    //     //         mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     //         material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //     //         transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //     //         ..default()
    //     //     },
    //     //     PickableBundle::default(), // Optional: adds selection, highlighting, and helper components.
    //     // ));
    //     // commands.spawn(PointLightBundle {
    //     //     point_light: PointLight {
    //     //         intensity: 1500.0,
    //     //         shadows_enabled: true,
    //     //         ..default()
    //     //     },
    //     //     transform: Transform::from_xyz(4.0, 8.0, -4.0),
    //     //     ..default()
    //     // });



    //     for mut fullscreen_banner_visibility in fullscreen_banner_query.iter_mut() {
    //         *fullscreen_banner_visibility = Visibility::Inherited;
    //     }
    //     // for (mini_camera, pan_orbit_camera) in mini_camera_query.iter() {
    //     //     if mini_camera.crew_id != fullscreen_event.crew_id { continue };
            
    //     //     // let camera_render_layer = RenderLayers::layer(fullscreen_event.crew_id);

    //     //     // let translation = Vec3::new(12.0, 7.0, 12.0);
    //     //     // let radius = translation.length();

    //     //     // commands.spawn((
    //     //     //     // subsection_cameras::PanOrbitCamera {
    //     //     //     //     focus: pan_orbit_camera.focus,
    //     //     //     //     radius: pan_orbit_camera.radius,
    //     //     //     //     upside_down: pan_orbit_camera.upside_down,
    //     //     //     // },
    //     //     //     Camera3dBundle {
    //     //     //         transform: Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     //     //         camera: Camera {
    //     //     //             // Renders the right camera after the left camera, which has a default priority of 0
    //     //     //             order: 1,
    //     //     //             ..default()
    //     //     //         },
    //     //     //         ..default()
    //     //     //     },
    //     //     //     camera_render_layer,
    //     //     //     UiCameraConfig { show_ui: false },
    //     //     // ));
    //     // }




    //     // for (camera_banner_entity, camera_banner, mut camera_banner_style) in camera_banner_query.iter_mut() {
    //     //     if camera_banner.crew_id != fullscreen_event.crew_id { continue };
    //     //     for (footer_entity, footer) in example_footer_query.iter() {
    //     //         if footer.crew_id != fullscreen_event.crew_id { continue };
    //     //         for (header_entity, header) in example_header_query.iter() {
    //     //             if header.crew_id != fullscreen_event.crew_id { continue };
    //     //             for mut root_visibility in root_node_query.iter_mut() {
    //     //                 if fullscreen_event.maximize {
    //     //                     // add to fullscreen node
    //     //                     for (fullscreen_node_entity, mut fullscreen_node_focus_policy) in fullscreen_node_query.iter_mut() {
    //     //                         commands.entity(fullscreen_node_entity).push_children(&[
    //     //                             header_entity,
    //     //                             camera_banner_entity,
    //     //                             footer_entity,
    //     //                         ]);
    //     //                         *fullscreen_node_focus_policy = FocusPolicy::Block;
    
    //     //                         camera_banner_style.flex_grow = 3.0;
    //     //                     }
    //     //                     *root_visibility = Visibility::Hidden;
    //     //                 } else {
    //     //                     // add to example_block
    //     //                     for (example_block_entity, example_block) in example_block_query.iter() {
    //     //                         if example_block.crew_id != fullscreen_event.crew_id { continue };
    //     //                         commands.entity(example_block_entity).push_children(&[
    //     //                             header_entity,
    //     //                             camera_banner_entity,
    //     //                             footer_entity,
    //     //                         ]);
    //     //                     }
    //     //                     // remove focus block
    //     //                     for (_fullscreen_node_entity, mut fullscreen_node_focus_policy) in fullscreen_node_query.iter_mut() {
    //     //                         *fullscreen_node_focus_policy = FocusPolicy::Pass;
    //     //                     }
    //     //                     *root_visibility = Visibility::Inherited;
    //     //                 }
    //     //             }
    //     //         }
    //     //     }
    //     // }
        // ui_resize_writer.send(UiResizeEvent);
    // } 
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