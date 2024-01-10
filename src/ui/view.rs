use bevy::{
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

    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*, ui,
    // winit::WinitSettings,
};
use bevy_svg::prelude::*;
use bevy_prototype_lyon::prelude::*;
use super::scrollable_page;
use super::util::SubsectionEntity;
use super::routes;

// Marker for UI node
#[derive(Component)]
pub struct View;

#[derive(Component, Default)]
pub struct ViewList {
    position: f32,
}

#[derive(Event)]
pub struct UiResizeEvent;


#[derive(Component)]
pub struct MyMinimapCamera;

#[derive(Event)]
pub struct SvgLoadEvent{
    pub entity: Entity,
    pub file_name: String,
}

#[derive(Component)]
pub struct SvgHolder;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(routes::SystemsPlugin)
        .add_event::<SvgLoadEvent>()
        .add_event::<UiResizeEvent>()
        // .add_systems(Startup, spawn_svg)
        .add_plugins(ShapePlugin)
        .add_systems(Update, (mouse_scroll, resize_camera_system, setup_new_camera, rotator_system));
    }
}

pub fn setup(commands: &mut Commands) -> Entity {
    let view = new();
    let view = commands.spawn(view).id();
    
    let page_items = page_items(commands);
    let view_list = scrollable_page::get_page();
    let view_list = commands.spawn((ViewList::default(), view_list)).id();
    
    commands.entity(view_list).push_children(&page_items);
    commands.entity(view).push_children(&[view_list]);

    return view;
}

pub fn new() -> (View, ButtonBundle) {
    return (View, ButtonBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_self: AlignSelf::Stretch,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            overflow: Overflow::clip_y(),
            ..default()
        },
        background_color: Color::rgb(0.0, 1.0, 0.0).into(),
        ..default()
    });
}

pub fn page_items(commands: &mut Commands) -> Vec<Entity> {
    let mut page_items = Vec::new();
    for i in 0..3 {
        let text_item = (
            TextBundle::from_section(
                format!("Page Item: {i}"),
                TextStyle {
                    font_size: 20.,
                    ..default()
                },
            ),
            Label,
            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
        );
        let page_item = NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(200.0),
                padding: UiRect {
                    left: Val::Px(0.0),
                    right: Val::Px(0.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(4.0),
                },
                ..default()
            },
            background_color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..default()
        };

        let inner_item = NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                // justify_items: JustifyItems::Center,
                ..default()
            },
            background_color: Color::rgb(0.4, 0.4, 0.4).into(),
            ..default()
        };

        let text_item = commands.spawn(text_item).id();
        let inner_item = commands.spawn(inner_item).id();
        let page_item = commands.spawn(page_item).id();

        commands.entity(inner_item).push_children(&[text_item]);
        commands.entity(page_item).push_children(&[inner_item]);

        page_items.push(page_item);
    }
    // let svg_window = get_svg(commands);
    // page_items.push(svg_window);

    return page_items;
}

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

use std::f32::consts::PI;

fn setup_new_camera (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut new_camera_event: EventReader<SvgLoadEvent>,
    minimap_query: Query<(Entity, &Node), With<MyMinimapCamera>>,
) {
    for (entity, node) in minimap_query.iter() {
        for ev in new_camera_event.read() {
            let size = node.size();
            let size = Extent3d {
                width: size.x.ceil() as u32,
                height: size.y.ceil() as u32,
                ..default()
            };
            // println!("{:?}", size.width);
            // println!("{:?}", size.height);
        
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
            ));

            // Light
            // NOTE: Currently lights are shared between passes - see https://github.com/bevyengine/bevy/issues/3462
            commands.spawn(PointLightBundle {
                point_light: PointLight {
                    intensity: 100.0,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
                ..default()
            });

            commands.spawn(
                (
                Camera3dBundle {
                    camera_3d: Camera3d {
                        clear_color: ClearColorConfig::Custom(Color::WHITE),
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

fn resize_camera_system (
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut mini_camera_query: Query<(Entity, &Camera), With<MiniCamera>>,
    mut minimap_query: Query<(Entity, &Node, &UiImage), (With<MyMinimapCamera>, Changed<Node>)>,
    mut ui_resize_reader: EventReader<UiResizeEvent>,
) {
    for (minimap_entity, node, ui_image) in minimap_query.iter_mut() {
        for (camera_entity, camera) in mini_camera_query.iter_mut() {
            for ev in ui_resize_reader.read() {
                let size = node.size();
                let size = Extent3d {
                    width: size.x.ceil() as u32,
                    height: size.y.ceil() as u32,
                    ..default()
                };

                
                // remove old image handle from images
                println!("image handle: {:?}", ui_image.texture.clone());
                images.remove(ui_image.texture.clone());

                // remove old UiImage
                commands.entity(minimap_entity).remove::<UiImage>();

                // remove old Camera
                println!("image handle: {:?}", camera_entity);
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
                            clear_color: ClearColorConfig::Custom(Color::WHITE),
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

/// Rotates the inner cube (first pass)
fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<FirstPassCube>>) {
    for mut transform in &mut query {
        transform.rotate_x(1.5 * time.delta_seconds());
        transform.rotate_z(1.3 * time.delta_seconds());
    }
}


fn mouse_scroll(
    mut interaction_query: Query<
        &Interaction,
        With<View>
    >,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ViewList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                for mouse_wheel_event in mouse_wheel_events.read() {
                    for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
                        let items_height = list_node.size().y;
                        let container_height = query_node.get(parent.get()).unwrap().size().y;
            
                        let max_scroll = (items_height - container_height).max(0.);
            
                        let dy = match mouse_wheel_event.unit {
                            MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                            MouseScrollUnit::Pixel => mouse_wheel_event.y,
                        };
            
                        scrolling_list.position += dy;
                        scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
                        style.top = Val::Px(scrolling_list.position);
                    }
                }
            }
            _ => {}
        }
    }
}



