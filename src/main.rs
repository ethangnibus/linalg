#![allow(warnings)]
use bevy::prelude::*;
use bevy::window::WindowResized;
mod ui;

use ui::view::UiResizeEvent;
use bevy_inspector_egui::quick::WorldInspectorPlugin; // FIXME: REMOVE IN PROD


fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        // .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                title: "linalg".to_string(),
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

// Spawns the camera that draws UI
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            // Renders the right camera after the left camera, which has a default priority of 0
            order: 0,
            ..default()
        },
        ..default()
    });
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
