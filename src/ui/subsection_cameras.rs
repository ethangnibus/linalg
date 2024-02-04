

use super::components::example_block;
use super::components::example_header;
use super::option_bar;
use super::routes;
use super::theme;
use super::under_navbar;
use super::util::style;
use super::util::subsection;
use super::util::subsection::SubsectionGameEntity;
use super::{routes::RoutingEvent, view::UiResizeEvent};
use bevy::input::mouse::MouseMotion;
use bevy::input::mouse::MouseWheel;
use bevy::utils::intern;
use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    ecs::event::ManualEventReader,
    prelude::*,
    render::{
        camera::{
            self, CameraProjection, ComputedCameraValues, RenderTarget, ScalingMode, Viewport,
        },
        primitives::Frustum,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::{RenderLayers, VisibleEntities},
    },
    ui::FocusPolicy,
};
// use rand::Rng;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CameraSetupEvent>()
           .add_event::<CameraSelectionEvent>()
           .add_event::<CameraSelectionColorEvent>()
           .add_systems(
            Update,
            (
                // setup_new_camera.after(routes::routing_system),
                resize_camera_system,
                theme_change_node_color_change_system,
                pan_orbit_camera,
                camera_selection_system.before(pan_orbit_camera),
                camera_background_focus_policy_system,
                example_block::example_skeleton_color_system,
                example_header::selection_button_interation,
                example_header::selection_button_color_system,
            ),
        );
    }
}

#[derive(Event)]
pub struct CameraSetupEvent;

#[derive(Event, Debug)]
pub struct CameraSelectionEvent {
    pub crew_id: u8,
    pub select_this_camera: bool,
}

#[derive(Event)]
pub struct CameraSelectionColorEvent;

#[derive(Component)]
pub struct CameraBackgroundBanner {
    pub crew_id: u8,
    pub is_selected: bool,
}



// Marks the main pass cube, to which the texture is applied.
#[derive(Component)]
struct MainPassCube;

// Shows us which mini camera this is
#[derive(Component)]
pub struct MiniCamera {
    pub crew_id: u8,
}

#[derive(Component)]
pub struct FilmCrew;

pub fn setup_light(
    commands: &mut Commands,
    film_crew_entity: Entity,
) {
    // Light
    // NOTE: Currently lights are shared between passes - see https://github.com/bevyengine/bevy/issues/3462
    let light = commands
        .spawn((
            PointLightBundle {
                point_light: PointLight {
                    intensity: 100.0,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
                ..default()
            },
            subsection::SubsectionGameEntity,
        ))
        .id();
    commands.entity(film_crew_entity).push_children(&[light]);
}

pub fn setup_camera(
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
    // make banner behind the text
    let camera_banner = commands
        .spawn((
            // theme::ColorFunction { // lol don't use this it makes everything pitch black in dark mode
            //     background: theme::background_color,
            //     border: theme::background_color,
            // },
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: height,
                    padding: style::NO_BORDER,
                    border: style::NO_BORDER,
                    justify_items: JustifyItems::Start,
                    align_items: AlignItems::Center,
                    ..default()
                },
                focus_policy: FocusPolicy::Pass,
                background_color: Color::WHITE.into(), // FIXME: Change to background color and change to white when camera loads
                ..default()
            },
            CameraBackgroundBanner {
                crew_id: crew_id,
                is_selected: false,
            },
            // image,
        ))
        .id();

    commands
        .entity(view_list_entity)
        .push_children(&[camera_banner]);
    let size = Extent3d {
        width: 1000,
        height: 1000,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: size.clone(),
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size.clone());
    let image_handle = images.add(image);

    let ui_image = UiImage {
        texture: image_handle.clone(),
        flip_x: false,
        flip_y: false,
    };

    commands.entity(camera_banner).insert(ui_image); // FIXME: this gets added multiple times.. bad

    let crew_render_layer = RenderLayers::layer(crew_id);


    // let translation = Vec3::new(-2.0, 2.5, 5.0);
    let translation = Vec3::new(0.0, 0.0, 15.0);
    let radius = translation.length();

    
    let camera = commands
        .spawn((
            PanOrbitCamera {
                radius,
                ..Default::default()
            },
            Camera3dBundle {
                camera_3d: Camera3d {
                    clear_color: ClearColorConfig::Custom(theme::background_color(&theme)),
                    ..default()
                },
                camera: Camera {
                    viewport: Some(Viewport {
                        physical_position: UVec2::new(0, 0),
                        physical_size: UVec2::new(size.width.clone(), size.height.clone()),
                        ..default()
                    }),
                    // render before the "main pass" camera
                    order: 1,
                    target: RenderTarget::Image(image_handle),
                    ..default()
                },
                transform: Transform::from_translation(translation)
                .looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()

            },
            // UI config is a separate component
            UiCameraConfig { show_ui: false },
            crew_render_layer,
            MiniCamera {
                crew_id: crew_id,
            },
        ))
        .id();

    commands.entity(film_crew_entity).push_children(&[camera]);
}

use bevy::render::view::visibility::update_frusta;

pub fn resize_camera_system(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    // mut mini_camera_query: Query<(Entity, &Camera, &mut Projection, &mut Frustum), With<MiniCamera>>,
    mut mini_camera_query: Query<
        (Entity, &mut Camera, &MiniCamera, &mut Projection),
        With<MiniCamera>,
    >,
    // mut film_crew_query: Query<(Entity, &mut FilmCrew), With<FilmCrew>>,

    // mut camera_banner_query: Query<(Entity, &Node, &UiImage), (With<CameraBackgroundBanner>, Changed<Node>)>,
    mut camera_banner_query: Query<
        (Entity, &Node, &UiImage, &CameraBackgroundBanner),
        (With<CameraBackgroundBanner>),
    >,
    // mut proj_query: Query<&bevy::render::camera::OrthographicProjection, With<bevy::render::camera::OrthographicProjection>>,
    mut ui_resize_reader: EventReader<UiResizeEvent>,
    theme: Res<theme::CurrentTheme>,
) {
    for ev in ui_resize_reader.read() {
        for (entity, node, mut ui_image, camera_background_banner) in camera_banner_query.iter_mut()
        {
            for (mini_camera_entity, mut camera, mini_camera, mut projection) in
                mini_camera_query.iter_mut()
            {
                if mini_camera.crew_id != camera_background_banner.crew_id {
                    continue;
                }
                // get size of the node that the camera renders to
                let size = node.size();
                let size = Extent3d {
                    width: size.x.ceil() as u32,
                    height: size.y.ceil() as u32,
                    ..default()
                };

                // resize image
                let image = images.get_mut(ui_image.texture.clone()).unwrap(); // FIXME: Change this from unwrap so there's no panic
                image.resize(size);
                // println!("image aspect ratio {:?}", image.aspect_ratio());
                // println!("image size: width: {:?}, height: {:?}", image.width(), image.height());

                // update projection's aspect ratio
                projection.update(size.height as f32, size.width as f32);
                // println!("Projection aspect ratio \n{:?}", projection);

                // resize viewport's physical size
                let mut viewport = camera.viewport.as_mut().unwrap();
                viewport.physical_size.x = size.width;
                viewport.physical_size.y = size.height;
            }
        }
    }

    // for (minimap_entity, node, ui_image) in camera_banner_query.iter_mut() {
    //     for (film_crew_entity, mut film_crew) in film_crew_query.iter_mut() {
    //         for ev in ui_resize_reader.read() {
    //             println!("Image resized");
    //             // let size = node.size();
    //             // projection.update(size.y, size.x);
    //             // println!("Projection updated to {:?}", projection);
    //             // println!("x: {:?}, y: {:?}", size.x, size.y);

    //             // projection.update();
    //             // projection.aspect_ratio = 1.0;
    //             // println!("Aspect ratio: {:?}", projection.aspect_ratio);
    //             // projection.get_projection_matrix();

    //             // update_frusta(views); //Continue down this path ...
    //             // println!("Projection: {:?}", );

    //             // // for q in proj_query.iter_mut() {
    //             // //     println!("got the projection!");
    //             // //     q.
    //             // // }
    //             // // println!("UI RESIZE");
    //             // // make size for new image
    //             let size = node.size();
    //             let size = Extent3d {
    //                 width: size.x.ceil() as u32,
    //                 height: size.y.ceil() as u32,
    //                 ..default()
    //             };

    //             // remove old image handle from images
    //             images.remove(ui_image.texture.clone());

    //             // remove old UiImage
    //             commands.entity(minimap_entity).remove::<UiImage>();

    //             // delete old UiImage

    //             // remove old Camera
    //             // println!("Film_crew_entity: {:?}", film_crew_entity);
    //             // commands.entity(film_crew.camera_entity).despawn_recursive();
    //             // film_crew.camera_entity = Entity::PLACEHOLDER;

    //             let camera_entity = film_crew.camera_entity;

    //             println!("Film Crew Entity: {:?},\n Camera Entity: {:?}", film_crew_entity, camera_entity);
    //             commands.entity(camera_entity).despawn_recursive();
    //             commands.entity(film_crew_entity).remove_children(&[camera_entity]);

    //             film_crew.camera_entity = Entity::PLACEHOLDER;

    //             // create new image handle
    //             let mut image = Image {
    //                 texture_descriptor: TextureDescriptor {
    //                     label: None,
    //                     size: size.clone(),
    //                     dimension: TextureDimension::D2,
    //                     format: TextureFormat::Bgra8UnormSrgb,
    //                     mip_level_count: 1,
    //                     sample_count: 1,
    //                     usage: TextureUsages::TEXTURE_BINDING
    //                         | TextureUsages::COPY_DST
    //                         | TextureUsages::RENDER_ATTACHMENT,
    //                     view_formats: &[],
    //                 },
    //                 ..default()
    //             };
    //             image.resize(size.clone()); // fill image.data with zeroes and change it's size to the correct size
    //             let image_handle = images.add(image);

    //             // create new UiImage
    //             let ui_image = UiImage { texture: image_handle.clone(), flip_x: false, flip_y: false };
    //             commands.entity(minimap_entity).insert(ui_image);

    //             // create new Camera
    //             let new_camera = commands.spawn(
    //                 (
    //                 Camera3dBundle {
    //                     camera_3d: Camera3d {
    //                         clear_color: ClearColorConfig::Custom(theme::background_color(&theme)),
    //                         ..default()
    //                     },
    //                     camera: Camera {
    //                         viewport: Some(Viewport {
    //                             physical_position: UVec2::new(0, 0),
    //                             physical_size: UVec2::new(
    //                                 size.width.clone(),
    //                                 size.height.clone(),
    //                             ),
    //                             ..default()
    //                         }),
    //                         // render before the "main pass" camera
    //                         order: 1,
    //                         target: RenderTarget::Image(image_handle),
    //                         ..default()
    //                     },
    //                     transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
    //                         .looking_at(Vec3::ZERO, Vec3::Y),
    //                     ..default()
    //                 },
    //                 // UI config is a separate component
    //                 UiCameraConfig {
    //                     show_ui: false,
    //                 },
    //                 RenderLayers::layer(1),
    //                 MiniCamera{number: 0},
    //             )).id();
    //             film_crew.camera_entity = new_camera;
    //             println!("New camera entity: {:?}", new_camera);
    //             commands.entity(film_crew_entity).push_children(&[new_camera]);
    //         }
    //     }
    // }
}

fn theme_change_node_color_change_system(
    mut theme_change_reader: EventReader<theme::ThemeChangeEvent>,
    mut camera_3d_query: Query<(&mut Camera3d), With<Camera3d>>,
    theme: Res<theme::CurrentTheme>,
) {
    for event in theme_change_reader.read() {
        // println!("theme change");
        for (mut camera) in camera_3d_query.iter_mut() {
            camera.clear_color = ClearColorConfig::Custom(theme::background_color(&theme))
        }
    }
}




















/// Tags an entity as capable of panning and orbiting.
#[derive(Component)]
struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}

fn camera_selection_system (
    mut camera_selection_writer: EventWriter<CameraSelectionEvent>,
    interaction_query: Query<(&CameraBackgroundBanner, &Interaction), (Changed<Interaction>, With<CameraBackgroundBanner>)>,
)
{
    for (camera_banner, interaction) in interaction_query.iter() {
        match interaction {
            Interaction::Pressed => {
                if camera_banner.is_selected { continue };
                
                camera_selection_writer.send(
                    CameraSelectionEvent {
                        crew_id: camera_banner.crew_id,
                        select_this_camera: true,
                    }
                )
            }
            _ => {
            }
        }
    }
}


fn camera_background_focus_policy_system(
    mut camera_selection_reader: EventReader<CameraSelectionEvent>,
    mut camera_banner_query: Query<(&mut CameraBackgroundBanner, &mut FocusPolicy), With<CameraBackgroundBanner>>,
    mut selection_button_query: Query<&mut example_header::SelectionButton, With<example_header::SelectionButton>>,
) {
    for camera_selection_event in camera_selection_reader.read() {
        println!("selected camera {:?}", camera_selection_event.crew_id);
        for (mut camera_banner, mut focus_policy) in camera_banner_query.iter_mut() {
            if camera_banner.crew_id != camera_selection_event.crew_id { continue };
            for mut selection_button in selection_button_query.iter_mut() {
                if selection_button.crew_id != camera_selection_event.crew_id { continue };
                
                if camera_selection_event.select_this_camera {
                    *focus_policy = FocusPolicy::Block;
                    camera_banner.is_selected = true;
                    selection_button.is_selected = true;
                } else {
                    *focus_policy = FocusPolicy::Pass;
                    camera_banner.is_selected = false;
                    selection_button.is_selected = false;
                }

                // } else {
                //     *focus_policy = FocusPolicy::Pass;
                //     camera_banner.is_selected = false;
                //     selection_button.is_selected = false;
                // }
            }
        }
        
    }
}




/// Pan the camera with middle mouse click, zoom with scroll wheel, orbit with right mouse click.
fn pan_orbit_camera(
    // windows: Res<Window>,
    camera_banner_query: Query<(&CameraBackgroundBanner, &Node, &Interaction), With<CameraBackgroundBanner>>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<(&MiniCamera, &mut PanOrbitCamera, &mut Transform, &Projection)>,
) {
    
    for (camera_background_banner, camera_banner_node, interaction) in camera_banner_query.iter() {
        match interaction {
            Interaction::None => {
            }
            _ => {

                // change input mapping for orbit and panning here
                // let orbit_button = MouseButton::Right;
                // let pan_button = MouseButton::Middle;
                let orbit_button = MouseButton::Left;
                let pan_button = MouseButton::Middle;

                let mut pan = Vec2::ZERO;
                let mut rotation_move = Vec2::ZERO;
                let mut scroll = 0.0;
                let mut orbit_button_changed = false;

                if input_mouse.pressed(orbit_button) {
                    for ev in ev_motion.iter() {
                        rotation_move += ev.delta;
                    }
                } else if input_mouse.pressed(pan_button) {
                    // Pan only if we're not rotating at the moment
                    for ev in ev_motion.iter() {
                        pan += ev.delta;
                    }
                }
                for ev in ev_scroll.iter() {
                    scroll += ev.y;
                }
                if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
                    orbit_button_changed = true;
                }
                    for (mini_camera, mut pan_orbit, mut transform, projection) in query.iter_mut() {
                        if camera_background_banner.crew_id != mini_camera.crew_id {continue};
                        if !camera_background_banner.is_selected { continue }

                        if orbit_button_changed {
                            // only check for upside down when orbiting started or ended this frame
                            // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
                            let up = transform.rotation * Vec3::Y;
                            pan_orbit.upside_down = up.y <= 0.0;
                        }

                        let mut any = false;
                        if rotation_move.length_squared() > 0.0 {
                            any = true;
                            // let window = get_primary_window_size(&windows);
                            let window = camera_banner_node.size();
                            let delta_x = {
                                let delta = rotation_move.x / window.x * std::f32::consts::PI * 1.0; // slow horizontal rotation
                                if pan_orbit.upside_down { -delta } else { delta }
                            };
                            let delta_y = rotation_move.y / window.y * std::f32::consts::PI * 1.0; // slow vertical rotation;
                            let yaw = Quat::from_rotation_y(-delta_x);
                            let pitch = Quat::from_rotation_x(-delta_y);
                            transform.rotation = yaw * transform.rotation; // rotate around global y axis
                            transform.rotation = transform.rotation * pitch; // rotate around local x axis
                        } else if pan.length_squared() > 0.0 {
                            any = true;
                            // make panning distance independent of resolution and FOV,
                            // let window = get_primary_window_size(&windows);
                            let window = camera_banner_node.size();
                            if let Projection::Perspective(projection) = projection {
                                pan *= (Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window);
                            }
                            // translate by local axes
                            let right = transform.rotation * Vec3::X * -pan.x;
                            let up = transform.rotation * Vec3::Y * pan.y;
                            // make panning proportional to distance away from focus point
                            let translation = (right + up) * pan_orbit.radius;
                            pan_orbit.focus += translation;
                        } else if scroll.abs() > 0.0 {
                            any = true;
                            pan_orbit.radius -= scroll * pan_orbit.radius * 0.001;
                            // dont allow zoom to reach zero or you get stuck
                            pan_orbit.radius = f32::max(pan_orbit.radius, 12.0); // min radius
                            pan_orbit.radius = f32::min(pan_orbit.radius, 100.0); // max radius
                        }

                        if any {
                            // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
                            // parent = x and y rotation
                            // child = z-offset
                            let rot_matrix = Mat3::from_quat(transform.rotation);
                            transform.translation = pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
                        }
                    }
                }
                
            }
        }
    // consume any remaining events, so they don't pile up if we don't need them
    // (and also to avoid Bevy warning us about not checking events every frame update)
    ev_motion.clear();
    ev_scroll.clear();
}

// fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
//     let window = windows.get_primary().unwrap();
//     let window = Vec2::new(window.width() as f32, window.height() as f32);
//     window
// }



