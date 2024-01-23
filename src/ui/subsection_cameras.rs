use bevy::{
    prelude::*,
    core_pipeline::clear_color::ClearColorConfig,
    render::{
        camera::{
            ComputedCameraValues,
            RenderTarget,
            Viewport,
        },
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
};
use super::{routes::RoutingEvent, view::UiResizeEvent};
use super::util::subsection::SubsectionGameEntity;
use super::theme;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SvgLoadEvent>()
        .add_systems(Update, (
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
pub struct SvgLoadEvent{
    pub entity: Entity,
    pub file_name: String,
}

#[derive(Component)]
pub struct MyMinimapCamera;

// Marks the first pass cube (rendered to a texture.)
#[derive(Component)]
struct FirstPassCube;

// Marks the main pass cube, to which the texture is applied.
#[derive(Component)]
struct MainPassCube;

// Shows us which mini camera this is
#[derive(Component)]
pub struct MiniCamera {
    number: u8,
}



fn setup_new_camera (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut new_camera_event: EventReader<SvgLoadEvent>,
    minimap_query: Query<(Entity, &Node), With<MyMinimapCamera>>,
    theme: Res<theme::CurrentTheme>,
    
) {
    for (entity, node) in minimap_query.iter() {
        for ev in new_camera_event.read() {
            println!("entity id: {:?}", entity);
            let size = node.size();
            let size = Extent3d {
                width: size.x.ceil() as u32,
                height: size.y.ceil() as u32,
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

            let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 4.0 }));
            let cube_material_handle = materials.add(StandardMaterial {
                base_color: Color::rgb(1.0, 0.75, 0.90),
                metallic: 20.0,
                reflectance: 0.02,
                unlit: false,
                ..default()
            });
            
            let image_handle = images.add(image);

            let ui_image = UiImage { texture: image_handle.clone(), flip_x: false, flip_y: false };
            commands.entity(entity).insert(ui_image);

            let first_pass_layer = RenderLayers::layer(1);
            // The cube that will be rendered to the texture.
            commands.spawn((
                PbrBundle {
                    mesh: cube_handle,
                    material: cube_material_handle,
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                    ..default()
                },
                FirstPassCube,
                first_pass_layer,
                SubsectionGameEntity,
            ));

            // Light
            // NOTE: Currently lights are shared between passes - see https://github.com/bevyengine/bevy/issues/3462
            commands.spawn((
                PointLightBundle {
                    point_light: PointLight {
                        intensity: 100.0,
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
                    ..default()
                },
                SubsectionGameEntity,
            ));

            commands.spawn(
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
                first_pass_layer,
                MiniCamera{number: 0},
            ));

            // TODO: remember to make a delete system for all game objects and image textures when you leave the page :)
        }
    }
}

fn delete_camera_system(
    mut commands: Commands,
    mini_camera_query: Query<(Entity), With<MiniCamera>>,
    mut routing_reader: EventReader<RoutingEvent>,
) {
    for event in routing_reader.read() {
        for entity in mini_camera_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}

fn delete_camera_texture_system(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut routing_reader: EventReader<RoutingEvent>,
) {
    // for event in routing_reader.read() {
    //     for (id, image) in images.iter_mut() {
    //         images.remove(id);
    //     }
    // }
}

fn resize_camera_system (
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut mini_camera_query: Query<(Entity, &Camera), With<MiniCamera>>,
    mut minimap_query: Query<(Entity, &Node, &UiImage), (With<MyMinimapCamera>, Changed<Node>)>,
    mut ui_resize_reader: EventReader<UiResizeEvent>,
    theme: Res<theme::CurrentTheme>,
) {
    for (minimap_entity, node, ui_image) in minimap_query.iter_mut() {
        for (camera_entity, camera) in mini_camera_query.iter_mut() {
            for ev in ui_resize_reader.read() {
                println!("UI RESIZE");
                // make size for new image
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
                commands.entity(camera_entity).despawn();



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
                commands.spawn(
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
                ));

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
        println!("theme change");
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