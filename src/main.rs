#![allow(warnings)]
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy::render::camera::{RenderTarget, Viewport};
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::render::view::{visibility, RenderLayers};
use bevy::ui::FocusPolicy;
use bevy::window::{WindowResized, WindowResolution};
pub mod pages;
pub mod ui;
use bevy_mod_picking::picking_core::Pickable;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use std::f32::consts::PI;
use ui::util::{style, theme};

use bevy_inspector_egui::quick::WorldInspectorPlugin; // FIXME: REMOVE IN PROD
use std::f32::consts::TAU;
use ui::view::UiResizeEvent;
use ui::{fullscreen_camera, subsection_cameras};

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        // .insert_resource(Msaa::Sample4)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                title: "linalg".to_string(),
                present_mode: bevy::window::PresentMode::AutoNoVsync, // Fixme: Remove if not needed for window interaction
                ..default()
            }),
            ..default()
        }))
        .add_plugins(fullscreen_camera::SystemsPlugin)
        // .add_plugins(WorldInspectorPlugin::new()) // FIXME: REMOVE IN PROD
        // .add_plugins(bevy_svg::prelude::SvgPlugin)
        .add_plugins(ui::SetupUiPlugin)
        .add_systems(Startup, setup_cameras)
        .add_systems(Update, on_resize_system)
        // .add_systems(Update, (on_resize_system, toggle_resolution))
        .run();
}

/// Marker component for the text that displays the current resolution.
#[derive(Component)]
struct ResolutionText;

#[derive(Component)]
pub struct TextbookCamera;

fn setup_3d_camera(
    commands: &mut Commands,
    window: &Window,
    mut images: &mut ResMut<Assets<Image>>,
    theme: &theme::CurrentTheme,
) {
    // // let camera_render_layer = RenderLayers::layer(1);
    // // let translation = Vec3::new(12.0, 7.0, 12.0);
    // // let radius = translation.length();

    // // let fullscreen_camera_banner = commands.spawn((
    // //     // theme::ColorFunction { // lol don't use this it makes everything pitch black in dark mode
    // //     //     background: theme::background_color,
    // //     //     border: theme::background_color,
    // //     // },
    // //     fullscreen_camera::FullscreenCameraBanner,
    // //     ButtonBundle {
    // //         style: Style {
    // //             width: Val::Percent(100.0),
    // //             height: Val::Percent(100.0),
    // //             padding: style::NO_BORDER,
    // //             border: style::NO_BORDER,
    // //             ..default()
    // //         },
    // //         z_index: ZIndex::Global(1),
    // //         // focus_policy: FocusPolicy::Pass,
    // //         background_color: Color::WHITE.into(), // FIXME: Change to background color and change to white when camera loads
    // //         visibility: Visibility::Hidden,
    // //         ..default()
    // //     },
    // // )).id();

    // let size = Extent3d {
    //     width: window.resolution.physical_width(),
    //     height: window.resolution.physical_height(),
    //     ..default()
    // };

    // // // This is the texture that will be rendered to.
    // // let mut image = Image {
    // //     texture_descriptor: TextureDescriptor {
    // //         label: None,
    // //         size: size.clone(),
    // //         dimension: TextureDimension::D2,
    // //         format: TextureFormat::Bgra8UnormSrgb,
    // //         mip_level_count: 1,
    // //         sample_count: 1,
    // //         usage: TextureUsages::TEXTURE_BINDING
    // //             | TextureUsages::COPY_DST
    // //             | TextureUsages::RENDER_ATTACHMENT,
    // //         view_formats: &[],
    // //     },
    // //     ..default()
    // // };

    // // image.resize(size.clone());
    // // let image_handle = images.add(image);

    // // let ui_image = UiImage {
    // //     texture: image_handle.clone(),
    // //     flip_x: false,
    // //     flip_y: false,
    // // };

    // // commands.entity(fullscreen_camera_banner).insert(ui_image); // FIXME: this gets added multiple times.. bad

    // // let crew_render_layer = RenderLayers::layer(1);

    // // // let translation = Vec3::new(-2.0, 2.5, 5.0);
    // let translation = Vec3::new(12.0, 7.0, 12.0);
    // let radius = translation.length();

    // let camera = commands.spawn((
    //     // subsection_cameras::PanOrbitCamera {
    //     //     radius,
    //     //     ..Default::default()
    //     // },
    //     Camera3dBundle {
    //         camera_3d: Camera3d {
    //             clear_color: ClearColorConfig::Custom(theme::background_color(&theme)),
    //             ..default()
    //         },
    //         camera: Camera {
    //             // viewport: Some(Viewport {
    //             //     physical_position: UVec2::new(0, 0),
    //             //     physical_size: UVec2::new(size.width.clone(), size.height.clone()),
    //             //     ..default()
    //             // }),
    //             // render before the "main pass" camera
    //             order: 0,
    //             // target: RenderTarget::Image(image_handle),
    //             ..default()
    //         },
    //         transform: Transform::from_translation(translation)
    //         .looking_at(Vec3::ZERO, Vec3::Y),
    //         ..default()
    //     },
    //     // BloomSettings {
    //     //     intensity: 1.0,
    //     //     ..default()
    //     // },
    //     // UI config is a separate component
    //     UiCameraConfig { show_ui: false },
    //     // crew_render_layer,
    //     Pickable::IGNORE,
    // )).id();

    // // commands.spawn((
    // //     Camera3dBundle {
    // //         transform: Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    // //         camera: Camera {
    // //             // Renders the right camera after the left camera, which has a default priority of 0
    // //             order: 1,
    // //             ..default()
    // //         },
    // //         camera_3d: Camera3d {
    // //             clear_color: ClearColorConfig::Custom(Color::BLACK), // remember to change this based on theme
    // //             ..default()
    // //         },
    // //         ..default()
    // //     },
    // //     // camera_render_layer,
    // //     camera_render_layer,
    // //     UiCameraConfig { show_ui: true },
    // // ));
}

#[derive(Component)]
pub struct Spaceship;
// Spawns the camera that draws UI
fn setup_cameras(
    mut commands: Commands,
    mut windows: Query<&mut Window>,
    mut images: ResMut<Assets<Image>>,
    theme: Res<theme::CurrentTheme>,
    // mut gizmos: Gizmos,
    asset_server: Res<AssetServer>,
) {
    // let camera_render_layer = RenderLayers::layer(0);
    
    let window = windows.single();
    println!("window res: {:?}", window.resolution);

    // setup_3d_camera(&mut commands, &window, &mut images, theme.as_ref());
    // setup_2d_camera(&mut commands, &window, &mut images, theme.as_ref());

    // commands.spawn((
    //     Main2dCamera,
    //     Camera3dBundle {
    //         camera: Camera {
    //             // Renders the right camera after the left camera, which has a default priority of 0
    //             order: 0,
    //             ..default()
    //         },
    //     ..default()
    //     },
    //     UiCameraConfig { show_ui: false },
    // ));
    let translation = Vec3::new(12.0, 7.0, 12.0);
    let radius = translation.length();

    let crew_render_layer = RenderLayers::layer(1);
    // commands.spawn((
    //     PointLightBundle {
    //         point_light: PointLight {
    //             intensity: 2000.0,
    //             radius: 40.0,
    //             // shadows_enabled: true,
    //             ..default()
    //         },
    //         transform: Transform::from_xyz(-10.0, 5.0, 10.0),
    //         ..default()
    //     },
    //     crew_render_layer,
    // ));

    // // key light
    // commands.spawn((
    //     PointLightBundle {
    //         point_light: PointLight {
    //             intensity: 5000.0,
    //             radius: 40.0,
    //             shadows_enabled: true,
    //             ..default()
    //         },
    //         transform: Transform::from_xyz(8.0, 5.0, 10.0),
    //         ..default()
    //     },
    //     crew_render_layer,
    // ));
    //     commands.spawn(
    //         (DirectionalLightBundle {
    //             directional_light: DirectionalLight {
    //                 shadows_enabled: true,
    //                 illuminance: 50000.0,
    //                 ..default()
    //             },
    //             transform: Transform {
    //                 translation: Vec3 { x: 15.0, y: 8.0, z: 15.0 },
    //                 ..default()
    //             },
    //             ..default()
    //         },
    //         RenderLayers::layer(1),
    //     )
    // );
    // directional 'sun' light
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 20000.0,
                shadows_enabled: false,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 2.0, 0.0),
                rotation: Quat::from_rotation_z(-PI / 4.),
                ..default()
            },
            // The default cascade config is designed to handle large scenes.
            // As this example has a much smaller world, we can tighten the shadow
            // bounds for better visual quality.
            // cascade_shadow_config: CascadeShadowConfigBuilder {
            //     first_cascade_far_bound: 4.0,
            //     maximum_distance: 10.0,
            //     ..default()
            // }
            // .into(),
            ..default()
        },
        RenderLayers::layer(1),
    ));

    // // ambient light
    // commands.spawn(
    //     AmbientLight {
    //         color: Color::WHITE,
    //         brightness: 0.1,

    //     },
    // );
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });

    // // fill light
    // commands.spawn((
    //     PointLightBundle {
    //         point_light: PointLight {
    //             intensity: 5000.0,
    //             radius: 40.0,
    //             // shadows_enabled: true,
    //             ..default()
    //         },
    //         transform: Transform::from_xyz(-10.0, 5.0, 10.0),
    //         ..default()
    //     },
    //     crew_render_layer,
    // ));

    // // back light
    // commands.spawn((
    //     PointLightBundle {
    //         point_light: PointLight {
    //             intensity: 2000.0,
    //             // shadows_enabled: true,
    //             ..default()
    //         },
    //         transform: Transform::from_xyz(-10.0, 5.0, 10.0),
    //         ..default()
    //     },
    //     // crew_render_layer,
    // ));

    let camera = commands
        .spawn((
            PanOrbitCamera {
                // Set focal point (what the camera should look at)
                focus: Vec3::new(0.0, 0.0, 0.0),
                // Set the starting position, relative to focus (overrides camera's transform).
                alpha: Some(TAU / 8.0),
                beta: Some(TAU / 8.0),
                radius: Some(15.0),
                // Set limits on rotation and zoom
                // alpha_upper_limit: Some(TAU / 4.0),
                // alpha_lower_limit: Some(-TAU / 4.0),
                // beta_upper_limit: Some(TAU / 3.0),
                // beta_lower_limit: Some(-TAU / 3.0),
                zoom_upper_limit: Some(50.0),
                zoom_lower_limit: Some(1.0),
                // // Adjust sensitivity of controls
                // orbit_sensitivity: 1.5,
                // pan_sensitivity: 0.5,
                // zoom_sensitivity: 0.5,
                // Allow the camera to go upside down
                allow_upside_down: false,
                // // Change the controls (these match Blender)
                // button_orbit: MouseButton::Middle,
                // button_pan: MouseButton::Middle,
                // modifier_pan: Some(KeyCode::ShiftLeft),
                // // Reverse the zoom direction
                // reversed_zoom: true,
                ..default()
            },
            TextbookCamera,
            Camera3dBundle {
                camera_3d: Camera3d {
                    clear_color: ClearColorConfig::Custom(theme::background_color(&theme)),
                    ..default()
                },
                transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
            // BloomSettings {
            //     intensity: 1.0,
            //     ..default()
            // },
            // UI config is a separate component
            UiCameraConfig { show_ui: true },
            // crew_render_layer,
            Pickable::IGNORE,
        ))
        .id();
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
