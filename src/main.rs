use bevy::prelude::*;
// use bevy::window::WindowResized;
mod ui;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                title: "linalg".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(bevy_svg::prelude::SvgPlugin)
        .add_plugins(ui::SetupUiPlugin)
        .add_systems(Startup, setup_camera)
        // .add_systems(Update, (on_resize_system, toggle_resolution))
        .run();
}

/// Marker component for the text that displays the current resolution.
#[derive(Component)]
struct ResolutionText;

// Spawns the camera that draws UI
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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

// /// This system shows how to respond to a window being resized.
// /// Whenever the window is resized, the text will update with the new resolution.
// fn on_resize_system(
//     mut q: Query<&mut Text, With<ResolutionText>>,
//     mut resize_reader: EventReader<WindowResized>,
// ) {
//     // let mut text = q.single_mut();
//     // for e in resize_reader.read() {
//     //     // When resolution is being changed
//     //     text.sections[0].value = format!("{:.1} x {:.1}", e.width, e.height);
//     // }
// }
