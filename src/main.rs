#![allow(warnings)]
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::window::WindowResized;
pub mod ui;
pub mod pages;

use ui::view::UiResizeEvent;
use bevy_inspector_egui::quick::WorldInspectorPlugin; // FIXME: REMOVE IN PROD



fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        // .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                title: "linalg".to_string(),
                present_mode: bevy::window::PresentMode::AutoNoVsync, // Fixme: Remove if not needed for window interaction
                ..default()
            }),
            ..default()
        }))
        .add_plugins(WorldInspectorPlugin::new()) // FIXME: REMOVE IN PROD
        // .add_plugins(bevy_svg::prelude::SvgPlugin)
        .add_plugins(ui::SetupUiPlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(Update, on_resize_system)
        // .add_systems(Update, (on_resize_system, toggle_resolution))
        .run();
}

/// Marker component for the text that displays the current resolution.
#[derive(Component)]
struct ResolutionText;

#[derive(Component)]
pub struct Main2dCamera;


// Spawns the camera that draws UI
fn setup_camera(mut commands: Commands) {
    let camera_render_layer = RenderLayers::layer(1);
    commands.spawn((
        // subsection_cameras::PanOrbitCamera {
        //     focus: pan_orbit_camera.focus,
        //     radius: pan_orbit_camera.radius,
        //     upside_down: pan_orbit_camera.upside_down,
        // },
        Camera3dBundle {
            transform: Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                // Renders the right camera after the left camera, which has a default priority of 0
                order: 0,
                ..default()
            },
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::BLACK), // remember to change this based on theme
                ..default()
            },
            ..default()
        },
        // camera_render_layer,
        camera_render_layer,
        UiCameraConfig { show_ui: false },
    ));

    commands.spawn((
        Main2dCamera,
        Camera2dBundle {
            camera: Camera {
                // Renders the right camera after the left camera, which has a default priority of 0
                order: 0,
                ..default()
            },
        ..default()
    }));
}

// /// This system shows how to request the window to a new resolution
// fn toggle_resolution(keys: Res<Input<KeyCode>>, mut windows: Query<&mut Window>) {
//     // let mut window = windows.single_mut();

//     // if keys.just_pressed(KeyCode::Key1) {
//     //     let res = resolution.small;
//     //     window.resolution.set(res.x, res.y);
//     // }
//     // if keys.just_pressed(KeyCode::Key2) {
//     //     let res = resolution.medium;
//     //     window.resolution.set(res.x, res.y);
//     // }
//     // if keys.just_pressed(KeyCode::Key3) {
//     //     let res = resolution.large;
//     //     window.resolution.set(res.x, res.y);
//     // }
// }

/// This system shows how to respond to a window being resized.
/// Whenever the window is resized, the text will update with the new resolution.
fn on_resize_system(
    // mut q: Query<&mut Text, With<ResolutionText>>,
    mut resize_reader: EventReader<WindowResized>,
    mut resize_event_writer: EventWriter<UiResizeEvent>,
) {
    for _event in resize_reader.read() {
        resize_event_writer.send(UiResizeEvent);
    }
    
    // let mut text = q.single_mut();
    // for e in resize_reader.read() {
    //     // When resolution is being changed
    //     text.sections[0].value = format!("{:.1} x {:.1}", e.width, e.height);
    // }
}
