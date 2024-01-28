use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, render::{
        camera::{
            self, CameraProjection, ComputedCameraValues, RenderTarget, ScalingMode, Viewport
        }, primitives::Frustum, render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        }, view::{RenderLayers, VisibleEntities}
    }, ui, ecs::event::ManualEventReader,
};
use super::{routes::RoutingEvent, view::UiResizeEvent};
use super::util::subsection::SubsectionGameEntity;
use super::theme;
use super::routes;
// use rand::Rng;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CameraSetupEvent>()
        .add_systems(Update, (
            // setup_new_camera.after(routes::routing_system),
            setup_new_camera,
            resize_camera_system,
            rotator_system,
            theme_change_node_color_change_system,
            delete_camera_system,
            delete_camera_texture_system,
        ));
    }
}

#[derive(Event)]
pub struct CameraSetupEvent;

#[derive(Component)]
pub struct CameraBackgroundBanner;

// Marks the first pass cube (rendered to a texture.)
#[derive(Component)]
pub struct FirstPassCube;

// Marks the main pass cube, to which the texture is applied.
#[derive(Component)]
struct MainPassCube;

// Shows us which mini camera this is
#[derive(Component)]
pub struct MiniCamera {
    pub number: u8,
}


#[derive(Component)]
pub struct FilmCrew {
    pub camera_entity: Entity,
    pub mesh_entity: Entity,
    pub light_entity: Entity,
}

pub fn setup_new_camera (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    // mini_camera_query: Query<(Entity), With<MiniCamera>>,
    mut camera_setup_reader: EventReader<CameraSetupEvent>,
    camera_banner_query: Query<(Entity, &Node, &UiImage), With<CameraBackgroundBanner>>,
    mut film_crew_query: Query<(Entity, &mut FilmCrew), With<FilmCrew>>,
    theme: Res<theme::CurrentTheme>,

    // world: &mut World, // probably wrong
    // mut camera_setup_reader: Local<ManualEventReader<CameraSetupEvent>>, // probably wrong
) {

    
    for (camera_banner_entity, node, ui_image) in camera_banner_query.iter() {
        for event in camera_setup_reader.read() {
            for (film_crew_entity, mut film_crew) in film_crew_query.iter_mut() {
                let size = node.size();
                let size = Extent3d {
                    width: size.x.ceil() as u32,
                    height: size.y.ceil() as u32,
                    ..default()
                };

                // remove old image handle from images
                images.remove(ui_image.texture.clone());

                // remove old UiImage
                commands.entity(camera_banner_entity).remove::<UiImage>();


                // remove old Camera
                let camera_entity = film_crew.camera_entity;
                commands.entity(film_crew_entity).remove_children(&[camera_entity]);
                commands.entity(camera_entity).despawn_recursive();
                film_crew.camera_entity = Entity::PLACEHOLDER;
                



                // create new image handle
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
                image.resize(size.clone()); // fill image.data with zeroes and change it's size to the correct size
                let image_handle = images.add(image);

                // create new UiImage
                let ui_image = UiImage { texture: image_handle.clone(), flip_x: false, flip_y: false };
                commands.entity(camera_banner_entity).insert(ui_image);
                
                // create new Camera
                let new_camera = commands.spawn(
                    (
                    Camera3dBundle {
                        camera_3d: Camera3d {
                            clear_color: ClearColorConfig::Custom(theme::background_color(&theme)),
                            ..default()
                        },
                        camera: Camera {
                            viewport: Some(Viewport {
                                physical_position: UVec2::new(0, 0),
                                physical_size: UVec2::new(
                                    size.width.clone(),
                                    size.height.clone(),
                                ),
                                ..default()
                            }),
                            // render before the "main pass" camera
                            order: 1,
                            target: RenderTarget::Image(image_handle),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
                            .looking_at(Vec3::ZERO, Vec3::Y),
                        ..default()
                    },
                    // UI config is a separate component
                    UiCameraConfig {
                        show_ui: false,
                    },
                    RenderLayers::layer(1),
                    MiniCamera{number: 0},
                )).id();

                film_crew.camera_entity = new_camera;
                commands.entity(film_crew_entity).push_children(&[new_camera]);
            }
        }
    }
}

fn delete_camera_system(
    mut commands: Commands,
    mini_camera_query: Query<(Entity), With<MiniCamera>>,
    mut routing_reader: EventReader<RoutingEvent>,
) {
    // for event in routing_reader.read() {
    //     for entity in mini_camera_query.iter() {
    //         commands.entity(entity).despawn();
    //     }
    // }
}

fn delete_camera_texture_system(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    camera_banner_query: Query<(&UiImage), With<CameraBackgroundBanner>>,
    mut routing_reader: EventReader<RoutingEvent>,
) {
    // for event in routing_reader.read() {
    //     for ui_image in camera_banner_query.iter() {
    //         // println!("Removing texture: {:?}", ui_image.texture.clone());
    //         images.remove(ui_image.texture.clone());
    //     }
    // }
}


use bevy::render::view::visibility::update_frusta;

fn resize_camera_system (
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    // mut mini_camera_query: Query<(Entity, &Camera, &mut Projection, &mut Frustum), With<MiniCamera>>,
    mut mini_camera_query: Query<(Entity, &Camera, &mut Projection), With<MiniCamera>>,
    mut film_crew_query: Query<(Entity, &mut FilmCrew), With<FilmCrew>>,

    // mut camera_banner_query: Query<(Entity, &Node, &UiImage), (With<CameraBackgroundBanner>, Changed<Node>)>,
    mut camera_banner_query: Query<(Entity, &Node, &UiImage), With<CameraBackgroundBanner>>,
    // mut proj_query: Query<&bevy::render::camera::OrthographicProjection, With<bevy::render::camera::OrthographicProjection>>,
    mut ui_resize_reader: EventReader<UiResizeEvent>,
    theme: Res<theme::CurrentTheme>,
) {
    
        
    for (minimap_entity, node, ui_image) in camera_banner_query.iter_mut() {
        for (film_crew_entity, mut film_crew) in film_crew_query.iter_mut() {
            for ev in ui_resize_reader.read() {
            
                // let size = node.size();
                // projection.update(size.y, size.x);
                // println!("Projection updated to {:?}", projection);
                // println!("x: {:?}, y: {:?}", size.x, size.y);

                // projection.update();
                // projection.aspect_ratio = 1.0;
                // println!("Aspect ratio: {:?}", projection.aspect_ratio);
                // projection.get_projection_matrix();

                // update_frusta(views); //Continue down this path ...
                // println!("Projection: {:?}", );
                
                // // for q in proj_query.iter_mut() {
                // //     println!("got the projection!");
                // //     q.
                // // }
                // // println!("UI RESIZE");
                // // make size for new image
                let size = node.size();
                let size = Extent3d {
                    width: size.x.ceil() as u32,
                    height: size.y.ceil() as u32,
                    ..default()
                };

                // remove old image handle from images
                images.remove(ui_image.texture.clone());

                // remove old UiImage
                commands.entity(minimap_entity).remove::<UiImage>();

                // delete old UiImage


                // remove old Camera
                // println!("Film_crew_entity: {:?}", film_crew_entity);
                // commands.entity(film_crew.camera_entity).despawn_recursive();
                // film_crew.camera_entity = Entity::PLACEHOLDER;

                let camera_entity = film_crew.camera_entity;
                commands.entity(film_crew_entity).remove_children(&[camera_entity]);
                commands.entity(camera_entity).despawn_recursive();
                film_crew.camera_entity = Entity::PLACEHOLDER;



                // create new image handle
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
                image.resize(size.clone()); // fill image.data with zeroes and change it's size to the correct size
                let image_handle = images.add(image);

                // create new UiImage
                let ui_image = UiImage { texture: image_handle.clone(), flip_x: false, flip_y: false };
                commands.entity(minimap_entity).insert(ui_image);
                
                // create new Camera
                let new_camera = commands.spawn(
                    (
                    Camera3dBundle {
                        camera_3d: Camera3d {
                            clear_color: ClearColorConfig::Custom(theme::background_color(&theme)),
                            ..default()
                        },
                        camera: Camera {
                            viewport: Some(Viewport {
                                physical_position: UVec2::new(0, 0),
                                physical_size: UVec2::new(
                                    size.width.clone(),
                                    size.height.clone(),
                                ),
                                ..default()
                            }),
                            // render before the "main pass" camera
                            order: 1,
                            target: RenderTarget::Image(image_handle),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
                            .looking_at(Vec3::ZERO, Vec3::Y),
                        ..default()
                    },
                    // UI config is a separate component
                    UiCameraConfig {
                        show_ui: false,
                    },
                    RenderLayers::layer(1),
                    MiniCamera{number: 0},
                )).id();
                film_crew.camera_entity = new_camera;

                commands.entity(film_crew_entity).push_children(&[new_camera]);

            }
        }
    }
}


fn theme_change_node_color_change_system(
    mut theme_change_reader: EventReader<theme::ThemeChangeEvent>,
    mut camera_3d_query: Query<(&mut Camera3d), With<Camera3d>>,
    theme: Res<theme::CurrentTheme>,
) {
    for event in theme_change_reader.read() {
        for (mut camera) in camera_3d_query.iter_mut() {
            camera.clear_color = ClearColorConfig::Custom(theme::background_color(&theme))
        }
    }
}

/// Rotates the inner cube (first pass)
fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<FirstPassCube>>) {
    for mut transform in &mut query {
        transform.rotate_x(1.5 * time.delta_seconds());
        transform.rotate_z(1.3 * time.delta_seconds());
    }
}