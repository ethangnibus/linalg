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
use super::pages::*;

const BLACKSMITH_OUTLINE: &str = "m
210.49052,219.61666
c
-54.97575,-3.12045
-153.83891,-43.5046
-181.900067,-79.34483
41.944976,3.29834
143.100787,1.42313
185.138697,1.61897
l
6e-5,-0.003
c
41.78023,-0.87477
200.563,-0.4537
261.24529,0
0.085,7.05106
0.79737,22.71244
1.07386,32.86306
-42.04814,8.31883
-101.90702,24.33338
-128.45794,63.97855
-10.53308,31.59203
39.6912,45.827
74.62215,55.19132
1.14898,12.80889
2.62233,32.62936
2.46309,44.71853
-75.4682,-0.86499
-141.64601,-1.07063
-209.86695,-1.35786
-10.81491,-1.77566
-6.66734,-23.1495
-4.31819,-32.38456
5.44628,-16.65332
38.03788,-18.20507
28.06768,-83.12367
-7.29786,-2.58188
-23.92259,-1.83114
-28.06768,-2.15756";

const BLACKSMITH_DETAIL: &str = "m 213.72921,141.88787 -4e-5,80.1576";

const SHACK: &str = "m
254.47507,533.90714
28.03554,-31.1502
29.07393,-32.18938
30.11225,-26.99742
29.07391,-30.11185
28.03556,-34.26547
29.07391,-25.95885
28.03556,-29.0741
q
13.49859,-16.61388
21.80543,-21.80524
l
25.95885,-17.65243
q
20.76708,9.34498
26.9972,26.99742
6.2297,18.68994
25.95885,35.30382
l
34.26568,29.07411
31.15062,24.9205
26.9972,23.88213
24.92049,29.07412
28.03556,37.38075
q
12.46024,18.69016
22.84378,21.80522
11.4219,4.15218
28.03556,20.76687
m
-332.27326,332.27305
2.07692,-44.64881
v
-40.496
l
-6.23054,-39.45766
-3.11527,-42.57209
1.03835,-35.30383
6.23054,-46.72655
44.64922,-3.1161
38.4191,1.03627
30.11226,-1.03627
52.95605,3.1161
q
5.19218,20.76749
-2.0767,43.61128
-6.22972,22.84357
1.03835,41.53437
7.26806,18.68995
3.11527,39.45682
l
-6.23054,46.72656
q
-1.03836,25.95884
1.03835,35.30381
l
3.11527,42.5721
m
164.05971,-83.0681
-33.22711,-1.03629
-47.76428,1.03629
-4.15362,-32.18855
4.15362,-50.87956
34.26567,1.03628
48.80264,-1.03628
m
-498.40988,-83.06873
30.11226,4.15217
52.95606,-4.15217
3.11505,33.22774
q
-3.11505,11.42189
-3.11505,49.84099
l
-28.03557,1.03628
-55.03275,-1.03628";

const SHACK_WALLS: &str = "m
254.47507,866.18019
q
18.69037,-88.25945
8.30683,-113.17996
-9.34519,-24.92049
-8.30683,-52.95625
11.42188,-69.57013
0,-83.06873
v
-83.06811
l
-34.26568,42.57292
q
-8.30684,13.49862
-48.80263,40.49519
l
-49.841,-39.45683
Q
99.760328,557.78928
88.33844,533.90714
99.760328,499.64167
136.10271,475.75953
l
67.49301,-53.99462
57.10946,-62.30123
q
28.03557,-33.22712
57.10947,-53.9946
29.07391,-20.76688
55.03276,-58.14762
26.9972,-36.34218
62.30124,-59.18595
36.34239,-21.80524
47.76428,-45.68738
12.46024,-23.88276
20.76708,-23.88276
17.65201,12.46025
43.61086,52.95626
25.95885,40.49601
65.4163,66.45486
l
72.68478,55.03235
57.10946,58.14822
60.22453,60.22453
q
36.34259,31.1502
47.76427,60.22432
11.42191,29.07412
34.26569,45.68736
23.88214,17.65244
34.26589,16.61387
l
-43.61088,41.53437
-39.45764,41.53374
q
-17.65203,-26.99657
-38.4191,-40.49519
l
-44.64922,-42.57292
v
60.22453
105.91231
83.06811
h
-2.07671
q
-7.26826,4.15217
2.07671,83.0681";



// Marker for UI node
#[derive(Component)]
pub struct View;

#[derive(Component, Default)]
struct ViewList {
    position: f32,
}


// #[derive(Component)]
// struct SvgStruct;

#[derive(Event)]
pub struct RoutingEvent {
    pub chapter_number: u32,
    pub section_number: u32,
    pub subsection_number: u32,
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
        .add_event::<RoutingEvent>()
        .add_event::<SvgLoadEvent>()
        .add_event::<UiResizeEvent>()
        // .add_systems(Startup, spawn_svg)
        .add_plugins(ShapePlugin)
        .add_systems(Update, (mouse_scroll, routing_system, resize_camera_system, setup_new_camera, debug_minimap, rotator_system));
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
                    left: Val::Px(2.0),
                    right: Val::Px(4.0),
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


fn page_not_found(commands: &mut Commands, page_entities: &mut Vec<Entity>) {
    let text_item = (
        TextBundle::from_section(
            format!("TODO: Remember to implement this page!"),
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
                left: Val::Px(2.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(4.0),
            },
            border: UiRect {
                left: Val::Px(2.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(4.0),
            },
            ..default()
        },
        background_color: Color::rgb(0.3, 0.1, 0.1).into(),
        border_color: Color::rgb(0.1, 0.1, 0.1).into(),

        ..default()
    };

    let text_item = commands.spawn(text_item).id();
    let page_item = commands.spawn(page_item).id();

    commands.entity(page_item).push_children(&[text_item]);

    page_entities.push(page_item);
}



#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct BlacksmithMarker;

#[derive(Component)]
struct ToolShackMarker;

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

fn debug_minimap (
    // minimap_query: Query<(&Node, &Transform, &GlobalTransform), With<MyMinimapCamera>>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    minimap_query: Query<Entity, With<MyMinimapCamera>>,
) {
    // for entity in minimap_query.iter() {
    //     println!("Found image");
    //     let size = Extent3d {
    //         width: 512,
    //         height: 512,
    //         ..default()
    //     };
    
    //     // This is the texture that will be rendered to.
    //     let mut image = Image {
    //         texture_descriptor: TextureDescriptor {
    //             label: None,
    //             size,
    //             dimension: TextureDimension::D2,
    //             format: TextureFormat::Bgra8UnormSrgb,
    //             mip_level_count: 1,
    //             sample_count: 1,
    //             usage: TextureUsages::TEXTURE_BINDING
    //                 | TextureUsages::COPY_DST
    //                 | TextureUsages::RENDER_ATTACHMENT,
    //             view_formats: &[],
    //         },
    //         ..default()
    //     };
    
    //     // fill image.data with zeroes
    //     image.resize(size);
    //     let image_handle = images.add(image);

    
    //     let ui_image = UiImage { texture: image_handle.clone(), flip_x: false, flip_y: false };
    //     commands.entity(entity).insert(ui_image);

    //     let first_pass_layer = RenderLayers::layer(1);

    //     commands.spawn((
    //         Camera3dBundle {
    //             camera_3d: Camera3d {
    //                 clear_color: ClearColorConfig::Custom(Color::WHITE),
    //                 ..default()
    //             },
    //             camera: Camera {
    //                 // render before the "main pass" camera
    //                 order: -1,
    //                 target: RenderTarget::Image(image_handle),
    //                 ..default()
    //             },
    //             transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
    //                 .looking_at(Vec3::ZERO, Vec3::Y),
    //             ..default()
    //         },
    //         first_pass_layer,
    //     ));
    
    // }
}


// fn svg_load (
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut svg_load_reader: EventReader<SvgLoadEvent>,
//     holder_entity_query: Query<Entity, With<SvgHolder>>,
// ) {
    
//     for event in svg_load_reader.read() {
//         for holder_entity in holder_entity_query.iter() {
//             commands.spawn(Camera2dBundle::default());
//             println!("");
//             println!("LOADED THE SVG");
//             println!("{:?}", event.entity);
//             println!("{:?}", event.file_name);
//             println!("");

//             // let text_bundle = commands.spawn((
//             //     TextBundle::from_section(
//             //         // format!(text),
//             //         "heloo guys",
//             //         TextStyle {
//             //             font_size: 20.,
//             //             ..default()
//             //         },
//             //     ),
//             //     Label,
//             //     AccessibilityNode(NodeBuilder::new(Role::ListItem)),
//             // )).id();

//             // let svg = asset_server.load("result.svg");

//             // println!("{:?}", svg);
//             // println!("made it to load svg");
//             // let svg_bundle = commands.spawn((
//             //     Svg2dBundle {
//             //         svg,
//             //         origin: Origin::Center, // Origin::TopLeft is the default
//             //         ..Default::default()
//             //     },
//             // )).id();
//             // println!("made it to make svg bundle");
            

//             let outer = commands.spawn((
//                 NodeBundle {
//                     style: Style {
//                         width: Val::Px(500.0),
//                         height: Val::Px(125.0),
//                         margin: UiRect::top(Val::VMin(5.)),
//                         ..default()
//                     },
//                     // a `NodeBundle` is transparent by default, so to see the image we have to its color to `WHITE`
//                     background_color: Color::WHITE.into(),
//                     ..default()
//                 },
//                 UiImage::new(asset_server.load("result.png")),
//             )).id();

//             commands.entity(holder_entity).push_children(&[outer]);
//         }
//     }
    
// }

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



fn routing_system(
    mut commands: Commands,
    view_list_query: Query<(Entity, &Children), With<ViewList>>,
    mut svg_load_writer: EventWriter<SvgLoadEvent>,
    mut routing_event_reader: EventReader<RoutingEvent>,
    asset_server: Res<AssetServer>,
) {
    for event in routing_event_reader.read() {
        for (view_list, view_list_children) in view_list_query.iter() {
            // remove all current page stuff
            for &child in view_list_children.iter() {
                commands.entity(view_list).remove_children(&[child]);
                commands.entity(child).despawn_recursive();
            }

            // let mut page_item: Entity = Entity::PLACEHOLDER;
            let mut page_entities: Vec<Entity> = Vec::new();
            // Add new page stuff

            match event.chapter_number {
                0 => {
                    match event.section_number {
                        0 => { // Chapter 0, Section 0
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 0, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 0, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 0, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 0, Section 4
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                1 => {
                    match event.section_number {
                        0 => { // Chapter 1, Section 0
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        1 => { // Chapter 1, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                // 1 => {page_item = chapter1section1subsection1::get_page(&mut commands);},
                                // 2 => {page_item = chapter1section1subsection2::get_page(&mut commands);},
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 1, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {chapter1section2subsection4::get(&mut commands, &asset_server, &mut svg_load_writer, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 1, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 1, Section 4
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                2 => {
                    match event.section_number {
                        0 => { // Chapter 2, Section 0
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 2, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 2, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 2, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 2, Section 4
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                3 => {
                    match event.section_number {
                        0 => { // Chapter 3, Section 0
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 3, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 3, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 3, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 3, Section 4
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                4 => {
                    match event.section_number {
                        0 => { // Chapter 4, Section 0
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 4, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 4, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 4, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 4, Section 4
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                5 => {
                    match event.section_number {
                        0 => { // Chapter 5, Section 0
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 5, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 5, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 5, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 5, Section 4
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                6 => {
                    match event.section_number {
                        0 => { // Chapter 6, Section 0
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 6, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 6, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 6, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 6, Section 4
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                7 => { // Chapter 7, Section 0
                    match event.section_number {
                        0 => {
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 7, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 7, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 7, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 7, Section 4
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                8 => {
                    match event.section_number {
                        0 => { // Chapter 8, Section 0
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 8, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 8, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 8, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 8, Section 4 
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                9 => {
                    match event.section_number {
                        0 => { // Chapter 9, Section 0
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 9, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 9, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 9, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 9, Section 4
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                10 => {
                    match event.section_number {
                        0 => { // Chapter 10, Section 0
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 10, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 10, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 10, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 10, Section 4 
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                11 => {
                    match event.section_number {
                        0 => { // Chapter 11, Section 0
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        1 => { // Chapter 11, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 11, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 11, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 11, Section 4
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                12 => {
                    match event.section_number {
                        0 => { // Chapter 12, Section 0
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 12, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 12, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 12, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 12, Section 4
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                13 => {
                    match event.section_number {
                        0 => { // Chapter 13, Section 0
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 13, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 13, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 13, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 13, Section 4
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                14 => {
                    match event.section_number {
                        0 => { // Chapter 14, Section 0
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 14, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 14, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 14, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 14, Section 4
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                15 => {
                    match event.section_number {
                        0 => { // Chapter 15, Section 0 Bibliography
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 15, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 15, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 15, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 15, Section 4
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }

            for entity in page_entities {
                commands.entity(view_list).push_children(&[entity]);
            }
        }
    }
    // add new page
}