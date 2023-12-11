use bevy::{prelude::*, window::WindowResized};

fn main() {
    App::new()
        .insert_resource(ResolutionSettings {
            large: Vec2::new(1920.0, 1080.0),
            medium: Vec2::new(800.0, 600.0),
            small: Vec2::new(640.0, 360.0),
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                title: "linalg".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_camera, setup_ui))
        .add_systems(Update, (on_resize_system, toggle_resolution))
        .run();
}

/// Marker component for the text that displays the current resolution.
#[derive(Component)]
struct ResolutionText;

/// Stores the various window-resolutions we can select between.
#[derive(Resource)]
struct ResolutionSettings {
    large: Vec2,
    medium: Vec2,
    small: Vec2,
}

// Spawns the camera that draws UI
fn setup_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}

// Spawns the UI
fn setup_ui(mut cmd: Commands) {
    // Root node that fills entire background
    cmd.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        background_color: Color::rgb(1.0, 0.0, 0.0).into(),
        ..default()
    })
    .with_children(|root| {
        // Navbar Start
        root.spawn(NodeBundle {
            style: Style {
                height: Val::Percent(10.0),
                ..default()
            },
            background_color: Color::rgb(0.0, 1.0, 0.0).into(),
            ..default()
        })
        .with_children(|navbar| {
            // Text where we display current resolution
            navbar.spawn((
                TextBundle::from_section(
                    "Resolution",
                    TextStyle {
                        font_size: 30.0,
                        ..default()
                    },
                ),
                ResolutionText,
            ));
        });
        // Navbar End

        // Under Navbar Start
        root.spawn(NodeBundle {
            style: Style {
                height: Val::Percent(80.0),
                ..default()
            },
            background_color: Color::rgb(0.0, 0.0, 1.0).into(),
            ..default()
        })
        .with_children(|under_navbar| {
            // Sidebar Start
            under_navbar
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(20.),
                        ..default()
                    },
                    background_color: Color::rgb(0.2, 0.2, 0.2).into(),
                    ..default()
                })
                .with_children(|sidebar| {
                    sidebar.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(60.0),
                            ..default()
                        },
                        background_color: Color::rgb(0.6, 0.6, 0.6).into(),
                        ..default()
                    });
                });
            // Sidebar End
        });
        // Under Navbar End
    });
}

/// This system shows how to request the window to a new resolution
fn toggle_resolution(
    keys: Res<Input<KeyCode>>,
    mut windows: Query<&mut Window>,
    resolution: Res<ResolutionSettings>,
) {
    let mut window = windows.single_mut();

    if keys.just_pressed(KeyCode::Key1) {
        let res = resolution.small;
        window.resolution.set(res.x, res.y);
    }
    if keys.just_pressed(KeyCode::Key2) {
        let res = resolution.medium;
        window.resolution.set(res.x, res.y);
    }
    if keys.just_pressed(KeyCode::Key3) {
        let res = resolution.large;
        window.resolution.set(res.x, res.y);
    }
}

/// This system shows how to respond to a window being resized.
/// Whenever the window is resized, the text will update with the new resolution.
fn on_resize_system(
    mut q: Query<&mut Text, With<ResolutionText>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    let mut text = q.single_mut();
    for e in resize_reader.read() {
        // When resolution is being changed
        text.sections[0].value = format!("{:.1} x {:.1}", e.width, e.height);
    }
}
