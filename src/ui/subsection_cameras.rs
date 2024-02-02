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
use super::under_navbar;
use super::option_bar;
use super::util::style;
use super::util::subsection;
use super::components::example_header;
// use rand::Rng;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CameraSetupEvent>()
        .add_systems(Update, (
            // setup_new_camera.after(routes::routing_system),
            
            resize_camera_system,
            rotator_system,
            theme_change_node_color_change_system,
        ));
    }
}

#[derive(Event)]
pub struct CameraSetupEvent;

#[derive(Component)]
pub struct CameraBackgroundBanner {
    pub crew_id: u8,
}

// Marks the first pass cube (rendered to a texture.)
#[derive(Component)]
pub struct FirstPassCube;

// Marks the main pass cube, to which the texture is applied.
#[derive(Component)]
struct MainPassCube;

// Shows us which mini camera this is
#[derive(Component)]
pub struct MiniCamera {
    pub crew_id: u8,
}


#[derive(Component)]
pub struct FilmCrew {
    pub crew_id: u8,
    pub camera_entity: Entity,
    pub mesh_entity: Entity,
    pub light_entity: Entity,
}

pub fn setup_camera(
    commands: &mut Commands,
    theme: &theme::CurrentTheme,
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
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: height,
                    padding: style::NO_BORDER,
                    border: style::NO_BORDER,
                    justify_items: JustifyItems::Start,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::WHITE.into(), // FIXME: Change to background color and change to white when camera loads
                ..default()
            },
            CameraBackgroundBanner {
                crew_id: crew_id,
            },
            // image,
        )
    ).id();

    commands.entity(view_list_entity).push_children(&[camera_banner]);
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

    let ui_image = UiImage { texture: image_handle.clone(), flip_x: false, flip_y: false };
    
    commands.entity(camera_banner).insert(ui_image); // FIXME: this gets added multiple times.. bad

    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 4.0 }));
    let cube_material_handle = if crew_id == 0 {
        materials.add(StandardMaterial {
            base_color: Color::rgb(1.0, 0.75, 0.90),
            metallic: 20.0,
            reflectance: 0.02,
            unlit: false,
            ..default()
        })
    } else {
        materials.add(StandardMaterial {
            base_color: Color::rgb(0.0, 0.1, 0.90),
            metallic: 20.0,
            reflectance: 0.02,
            unlit: false,
            ..default()
        })
    };

    let crew_render_layer = RenderLayers::layer(crew_id);

    let camera = commands.spawn(
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
        crew_render_layer,
        MiniCamera{crew_id: crew_id},
    )).id();

    // commands.entity(camera_banner_entity).push_children(&[camera]);

    // The cube that will be rendered to the texture.
    let cube = commands.spawn((
        PbrBundle {
            mesh: cube_handle,
            material: cube_material_handle,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..default()
        },
        FirstPassCube,
        crew_render_layer,
        subsection::SubsectionGameEntity,
    )).id();

    // Light
    // NOTE: Currently lights are shared between passes - see https://github.com/bevyengine/bevy/issues/3462
    let light = commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                intensity: 100.0,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
            ..default()
        },
        subsection::SubsectionGameEntity,
    )).id();

    let film_crew = commands.spawn((
        FilmCrew {
            crew_id: crew_id,
            camera_entity: camera,
            mesh_entity: cube,
            light_entity: light,
        },
        SpatialBundle{
            ..default()
        },
    )).id();
    commands.entity(film_crew).push_children(&[camera, cube, light]);
}


use bevy::render::view::visibility::update_frusta;

pub fn resize_camera_system (
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    // mut mini_camera_query: Query<(Entity, &Camera, &mut Projection, &mut Frustum), With<MiniCamera>>,
    mut mini_camera_query: Query<(Entity, &mut Camera, &MiniCamera, &mut Projection), With<MiniCamera>>,
    // mut film_crew_query: Query<(Entity, &mut FilmCrew), With<FilmCrew>>,

    // mut camera_banner_query: Query<(Entity, &Node, &UiImage), (With<CameraBackgroundBanner>, Changed<Node>)>,
    mut camera_banner_query: Query<(Entity, &Node, &UiImage, &CameraBackgroundBanner), (With<CameraBackgroundBanner>)>,
    // mut proj_query: Query<&bevy::render::camera::OrthographicProjection, With<bevy::render::camera::OrthographicProjection>>,
    mut ui_resize_reader: EventReader<UiResizeEvent>,
    theme: Res<theme::CurrentTheme>,
) {
    for ev in ui_resize_reader.read() {
        for (entity, node, mut ui_image, camera_background_banner) in camera_banner_query.iter_mut() {
            for (mini_camera_entity, mut camera, mini_camera, mut projection) in mini_camera_query.iter_mut() {
            
                if mini_camera.crew_id != camera_background_banner.crew_id {continue}
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

/// Rotates the inner cube (first pass)
fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<FirstPassCube>>) {
    for mut transform in &mut query {
        transform.rotate_x(1.5 * time.delta_seconds());
        transform.rotate_z(1.3 * time.delta_seconds());
    }
}