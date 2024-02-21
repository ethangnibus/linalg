
use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;
use bevy::pbr::ViewLightEntities;
use bevy::render::mesh;
use bevy::render::mesh::shape::Cylinder;
use bevy::render::mesh::Indices;
use bevy::render::mesh::VertexAttributeValues;
use bevy::render::render_resource::encase::vector;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::render::view::visibility;
use bevy::render::Render;
use bevy::ui::FocusPolicy;
use bevy::window::PrimaryWindow;
use bevy_mod_picking::prelude::*;
// use bevy::render::render_resource::PrimitiveTopology::LineStrip;
use bevy::render::render_resource::PrimitiveTopology::LineList;

use bevy::{
    render::view::RenderLayers,
    prelude::*,
};
use bevy_panorbit_camera::PanOrbitCamera;

use crate::ui::util::examples_3d;
use crate::ui::{
    util::theme,
    util::subsection,
};

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // Add the setup_ui system as a startup system
        app.add_event::<VectorSphereSelectionEvent>()
            .add_event::<PanOrbitToggleEvent>()
            .add_event::<VectorSphereMovementEvent>()
            .add_systems(Update, (
                vector_sphere_selection_system,
                vector_sphere_movement_system,
                basis_vectors_movement_system,
                disable_pan_orbit_system,
                move_span_cube_vertices.after(vector_sphere_movement_system),
                sphere_position_banner_interactions,
                update_sphere_position_text,
            ))
            .add_systems(Startup, spawn_spaceship);
    }
}

#[derive(Component)]
pub struct Spaceship;

pub fn spawn_spaceship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // let space_bundle = commands.spawn((
    //     SceneBundle {
    //         scene: asset_server.load("SPACESHIP.glb#Scene0"),
    //         visibility: Visibility::Visible,
    //         ..default()
    //     },
    //     RenderLayers::layer(1),
    //     Spaceship,
    // )).id();

}
#[derive(Event)]
pub struct VectorSphereSelectionEvent;

impl From<ListenerInput<Pointer<Click>>> for VectorSphereSelectionEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        return VectorSphereSelectionEvent;
    }
}

pub enum PanOrbitToggleState {
    DragStart,
    DragEnd,
}

#[derive(Component)]
pub struct SpanCube {
    quad_handle: Handle<Mesh>,
}

#[derive(Event)]
pub struct PanOrbitToggleEvent {
    state: PanOrbitToggleState,
}
impl From<ListenerInput<Pointer<DragStart>>> for PanOrbitToggleEvent {
    fn from(event: ListenerInput<Pointer<DragStart>>) -> Self {
        return PanOrbitToggleEvent {
            state: PanOrbitToggleState::DragStart
        };
    }
}
impl From<ListenerInput<Pointer<DragEnd>>> for PanOrbitToggleEvent {
    fn from(event: ListenerInput<Pointer<DragEnd>>) -> Self {
        return PanOrbitToggleEvent {
            state: PanOrbitToggleState::DragEnd
        };
    }
}










fn create_custom_cube_mesh(
    commands: &mut Commands,
    theme: &theme::CurrentTheme,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    crew_render_layer: RenderLayers,
    v1: Vec3,
    v2: Vec3,
    v3: Vec3,
) -> Entity {
    let v1 = v1 * 0.1;
    let v2 = v2 * 0.1;
    let v3 = v3 * 0.1;

    let mesh = Mesh::new(PrimitiveTopology::TriangleList)
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        // Each array is an [x, y, z] coordinate in local space.
        // Meshes always rotate around their local [0, 0, 0] when a rotation is applied to their Transform.
        // By centering our mesh around the origin, rotating the mesh preserves its center of mass.
        vec![
            // top (facing towards +y) 
            [-v1.x + v2.x - v3.x, -v1.y + v2.y - v3.y, -v1.z + v2.z - v3.z], // vertex_buffer[0] = lth // [-0.5, 0.5, -0.5], // vertex with index 0 //// -v1 + v2 - v3
            [v1.x + v2.x - v3.x, v1.y + v2.y - v3.y, v1.z + v2.z - v3.z], //    vertex_buffer[1] = rth // [0.5, 0.5, -0.5], // vertex with index 1 //// v1 + v2 - v3
            [v1.x + v2.x + v3.x, v1.y + v2.y + v3.y, v1.z + v2.z + v3.z], //    vertex_buffer[2] = rts             // [0.5, 0.5, 0.5], // etc. until 23 //// v1 + v2 + v3
            [-v1.x + v2.x + v3.x, -v1.y + v2.y + v3.y, -v1.z + v2.z + v3.z], // vertex_buffer[3] = lts                // [-0.5, 0.5, 0.5], //// -v1 + v2 + v3
            // bottom   (-y)
            [-v1.x - v2.x - v3.x, -v1.y - v2.y - v3.y, -v1.z - v2.z - v3.z], // vertex_buffer[4] = lbh                //[-0.5, -0.5, -0.5], //// -v1 - v2 - v3
            [v1.x - v2.x - v3.x, v1.y - v2.y - v3.y, v1.z - v2.z - v3.z], //    vertex_buffer[5] = rbh             //[0.5, -0.5, -0.5], //// v1 - v2 - v3
            [v1.x - v2.x + v3.x, v1.y - v2.y + v3.y, v1.z - v2.z + v3.z], //    vertex_buffer[6] = rbs             //[0.5, -0.5, 0.5], //// v1 - v2 + v3
            [-v1.x - v2.x + v3.x, -v1.y - v2.y + v3.y, -v1.z - v2.z + v3.z], // vertex_buffer[7] = lbs                //[-0.5, -0.5, 0.5], //// -v1 - v2 + v3
            // right    (+x)
            [v1.x - v2.x - v3.x, v1.y - v2.y - v3.y, v1.z - v2.z - v3.z], //    vertex_buffer[8] = rbh             // [0.5, -0.5, -0.5], //// v1 - v2 - v3
            [v1.x - v2.x + v3.x, v1.y - v2.y + v3.y, v1.z - v2.z + v3.z], //    vertex_buffer[9] = rbs             // [0.5, -0.5, 0.5], //// v1 - v2 + v3
            [v1.x + v2.x + v3.x, v1.y + v2.y + v3.y, v1.z + v2.z + v3.z], //    vertex_buffer[10] = rts             //[0.5, 0.5, 0.5], //// v1 + v2 + v3           // This vertex is at the same position as vertex with index 2, but they'll have different UV and normal
            [v1.x + v2.x - v3.x, v1.y + v2.y - v3.y, v1.z + v2.z - v3.z], //    vertex_buffer[11] = rth             //[0.5, 0.5, -0.5], //// v1 + v2 - v3
            // left     (-x)
            [-v1.x - v2.x - v3.x, -v1.y - v2.y - v3.y, -v1.z - v2.z - v3.z], // vertex_buffer[12] = lbh                //[-0.5, -0.5, -0.5], //// -v1 - v2 - v3
            [-v1.x - v2.x + v3.x, -v1.y - v2.y + v3.y, -v1.z -v2.z + v3.z], //  vertex_buffer[13] = lbs               // [-0.5, -0.5, 0.5], //// -v1 - v2 + v3
            [-v1.x + v2.x + v3.x, -v1.y + v2.y + v3.y, -v1.z + v2.z + v3.z], // vertex_buffer[14] = lts                // [-0.5, 0.5, 0.5], ////  -v1 + v2 + v3
            [-v1.x + v2.x - v3.x, -v1.y + v2.y - v3.y, -v1.z + v2.z - v3.z], // vertex_buffer[15] = lth                // [-0.5, 0.5, -0.5], ////  -v1 + v2 - v3
            // back     (+z)
            [-v1.x - v2.x + v3.x, -v1.y - v2.y + v3.y, -v1.z - v2.z + v3.z], // vertex_buffer[16] = lbs                // [-0.5, -0.5, 0.5], ////  -v1 - v2 + v3
            [-v1.x + v2.x + v3.x, -v1.y + v2.y + v3.y, -v1.z + v2.z + v3.z], // vertex_buffer[17] = lts                // [-0.5, 0.5, 0.5], //// -v1 + v2 + v3
            [v1.x + v2.x + v3.x, v1.y + v2.y + v3.y, v1.z + v2.z + v3.z], //    vertex_buffer[18] = rts             // [0.5, 0.5, 0.5], //// v1 + v2 + v3
            [v1.x - v2.x + v3.x, v1.y - v2.y + v3.y, v1.z - v2.z + v3.z], //    vertex_buffer[19] = rbs             // [0.5, -0.5, 0.5], //// v1 - v2 + v3
            // forward  (-z)
            [-v1.x - v2.x - v3.x, -v1.y - v2.y - v3.y, -v1.z - v2.z - v3.z], // vertex_buffer[20] = lbh                // [-0.5, -0.5, -0.5], //// -v1 - v2 - v3
            [-v1.x + v2.x - v3.x, -v1.y + v2.y - v3.y, -v1.z + v2.z - v3.z], // vertex_buffer[21] = lth                // [-0.5, 0.5, -0.5], //// -v1 + v2 - v3
            [v1.x + v2.x - v3.x, v1.y + v2.y - v3.y, v1.z + v2.z - v3.z], //    vertex_buffer[22] = rth             // [0.5, 0.5, -0.5], //// v1 + v2 - v3
            [v1.x - v2.x - v3.x, v1.y - v2.y - v3.y, v1.z - v2.z - v3.z], //    vertex_buffer[23] = rbh             // [0.5, -0.5, -0.5], //// v1 - v2 - v3
        ],
        // vec![
        //     // top (facing towards +y)
        //     [-0.5, 0.5, -0.5], // vertex with index 0
        //     [0.5, 0.5, -0.5], // vertex with index 1
        //     [0.5, 0.5, 0.5], // etc. until 23
        //     [-0.5, 0.5, 0.5],
        //     // bottom   (-y)
        //     [-0.5, -0.5, -0.5],
        //     [0.5, -0.5, -0.5],
        //     [0.5, -0.5, 0.5],
        //     [-0.5, -0.5, 0.5],
        //     // right    (+x)
        //     [0.5, -0.5, -0.5],
        //     [0.5, -0.5, 0.5],
        //     [0.5, 0.5, 0.5], // This vertex is at the same position as vertex with index 2, but they'll have different UV and normal
        //     [0.5, 0.5, -0.5],
        //     // left     (-x)
        //     [-0.5, -0.5, -0.5],
        //     [-0.5, -0.5, 0.5],
        //     [-0.5, 0.5, 0.5],
        //     [-0.5, 0.5, -0.5],
        //     // back     (+z)
        //     [-0.5, -0.5, 0.5],
        //     [-0.5, 0.5, 0.5],
        //     [0.5, 0.5, 0.5],
        //     [0.5, -0.5, 0.5],
        //     // forward  (-z)
        //     [-0.5, -0.5, -0.5],
        //     [-0.5, 0.5, -0.5],
        //     [0.5, 0.5, -0.5],
        //     [0.5, -0.5, -0.5],
        // ],

        // Here's which elements correspond with each point:
        // lth = [0, 15, 21];
        // rth = [1, 11, 22];
        // lts = [3, 14, 17];
        // rts = [2, 10, 18];
        // lbh = [4, 12, 20];
        // rbh = [5, 8, 23];
        // lbs = [7, 13, 16];
        // rbs = [6, 9, 19];
    )
    // Set-up UV coordinated to point to the upper (V < 0.5), "dirt+grass" part of the texture.
    // Take a look at the custom image (assets/textures/array_texture.png)
    // so the UV coords will make more sense
    // Note: (0.0, 0.0) = Top-Left in UV mapping, (1.0, 1.0) = Bottom-Right in UV mapping
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            // Assigning the UV coords for the top side.
            [0.0, 0.2], [0.0, 0.0], [1.0, 0.0], [1.0, 0.25],
            // Assigning the UV coords for the bottom side.
            [0.0, 0.45], [0.0, 0.25], [1.0, 0.25], [1.0, 0.45],
            // Assigning the UV coords for the right side.
            [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
            // Assigning the UV coords for the left side.
            [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
            // Assigning the UV coords for the back side.
            [0.0, 0.45], [0.0, 0.2], [1.0, 0.2], [1.0, 0.45],
            // Assigning the UV coords for the forward side.
            [0.0, 0.45], [0.0, 0.2], [1.0, 0.2], [1.0, 0.45],
        ],
    )
    // For meshes with flat shading, normals are orthogonal (pointing out) from the direction of
    // the surface.
    // Normals are required for correct lighting calculations.
    // Each array represents a normalized vector, which length should be equal to 1.0.
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            // Normals for the top side (towards +y)
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            // Normals for the bottom side (towards -y)
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            // Normals for the right side (towards +x)
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            // Normals for the left side (towards -x)
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            // Normals for the back side (towards +z)
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            // Normals for the forward side (towards -z)
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
        ],
    )
    // Create the triangles out of the 24 vertices we created.
    // To construct a square, we need 2 triangles, therefore 12 triangles in total.
    // To construct a triangle, we need the indices of its 3 defined vertices, adding them one
    // by one, in a counter-clockwise order (relative to the position of the viewer, the order
    // should appear counter-clockwise from the front of the triangle, in this case from outside the cube).
    // Read more about how to correctly build a mesh manually in the Bevy documentation of a Mesh,
    // further examples and the implementation of the built-in shapes.
    .with_indices(Some(Indices::U32(vec![
        0,3,1 , 1,3,2, // triangles making up the top (+y) facing side.
        4,5,7 , 5,6,7, // bottom (-y)
        8,11,9 , 9,11,10, // right (+x)
        12,13,15 , 13,14,15, // left (-x)
        16,19,17 , 17,19,18, // back (+z)
        20,21,23 , 21,22,23, // forward (-z)
    ])));

    let mesh_handle = meshes.add(mesh);
    let span_entity = commands.spawn((
        SpanCube {
            quad_handle: mesh_handle.clone(),
        },
        theme::ColorFunction {
            border: theme::line_color_transparent,
            background: theme::line_color_transparent,
        },
        PbrBundle {
            mesh: mesh_handle.clone(),
            material: materials.add(StandardMaterial {
                base_color: theme::line_color_transparent(theme).into(),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            }),
            ..default()
        },
        Pickable::IGNORE,
        crew_render_layer,
    )).id();
    return span_entity;
    // commands.insert_resource(SpanCubeResource{
    //     quad_handle: mesh_handle,
    // })
    
}

#[derive(Component)]
pub struct SpanCubeResource{
    quad_handle: Handle<Mesh>,
}

pub fn between(num: f32, start: f32, end: f32) -> bool {
    return (num >= start && num <= end)
        || (num <= start && num >= end);

}
fn move_span_cube_vertices(
    mut movement_reader: EventReader<VectorSphereMovementEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut span_cube_query: Query<&SpanCube, With<SpanCube>>,
    mut vector_sphere_query: Query<(&mut Transform), With<VectorSphere>>,
    mut commands: Commands,
) {
    // leave if there's no movement event
    // let mut sum: u8 = 0;
    // for movement_event in movement_reader.read() {
    //     sum += 1;
    // }
    // if sum == 0 { return };
    // println!("here we react to a movement event");

    for movement_event in movement_reader.read() {
        for vector_sphere_transform in vector_sphere_query.iter() {
            for span_cube in span_cube_query.iter() {
                // println!("quad_handle: {:?}", span_cube.quad_handle);
            
                let quad_handle = span_cube.quad_handle.clone();
                if let Some(mut mesh) = meshes.get_mut(&quad_handle) {
                    // Modify the vertices of the quad mesh
                    // if let Some(vertex_buffer) = mesh.attribute_mut::<Vec3>(Mesh::ATTRIBUTE_POSITION) {
                    //     // Update vertices based on your logic
                    //     // For example, let's say you have new vertices stored in a Vec<Vec3> called new_vertices
                    //     for (i, vertex) in new_vertices.iter().enumerate() {
                    //         vertex_buffer[i] = *vertex;
                    //     }
                    // }
                    let primitive_topology = mesh.primitive_topology();
                    // println!("primitive_topology: {:?}", primitive_topology);
                    if let Some(vertex_buffer) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
                    // Update vertices based on your logic
                        // For example, let's say you have new vertices stored in a Vec<Vec3> called new_vertices
                        // println!("vertex_buffer: {:?}", vertex_buffer);

                        let VertexAttributeValues::Float32x3(vertex_buffer) = vertex_buffer else {
                            panic!("Unexpected vertex format, expected Float32x2.");
                        };

                        // Here's which elements correspond with each point:
                        let lth_indices = [0, 15, 21]; // -x +y -z
                        let rth_indices = [1, 11, 22]; // +x +y -z
                        let lts_indices = [3, 14, 17]; // -x +y +z
                        let rts_indices = [2, 10, 18]; // +x +y +z
                        let lbh_indices = [4, 12, 20]; // -x -y -z
                        let rbh_indices = [5, 8, 23];  // +x -y -z
                        let lbs_indices = [7, 13, 16]; // -x -y +z
                        let rbs_indices = [6, 9, 19];  // +x -y +z
                        
                        
                        let x = vector_sphere_transform.translation;

                        match movement_event.associated_vector {
                            AssociatedVector::V1 => {
                                if x.dot(movement_event.unit_vector) > 0.0 {
                                    // +x
                                    push_plane_to_sphere_vector(
                                        x,
                                        vertex_buffer,
                                        &rth_indices,
                                        &rts_indices,
                                        &rbs_indices,
                                        &rbh_indices,
                                    );
                                    push_plane_to_axis(
                                        AssociatedVector::V1,
                                        vertex_buffer,
                                        &lth_indices,
                                        &lts_indices,
                                        &lbs_indices,
                                        &lbh_indices,
                                    );
                                } else if x.dot(movement_event.unit_vector) < 0.0 {
                                    // - x
                                    push_plane_to_sphere_vector(
                                        x,
                                        vertex_buffer,
                                        &lth_indices,
                                        &lts_indices,
                                        &lbs_indices,
                                        &lbh_indices,
                                    );
                                    push_plane_to_axis(
                                        AssociatedVector::V1,
                                        vertex_buffer,
                                        &rth_indices,
                                        &rts_indices,
                                        &rbs_indices,
                                        &rbh_indices,
                                    );
                                }
                            }
                            AssociatedVector::V2 => {
                                if x.dot(movement_event.unit_vector) > 0.0 {
                                    // +y
                                    push_plane_to_sphere_vector(
                                        x,
                                        vertex_buffer,
                                        &lth_indices,
                                        &rth_indices,
                                        &lts_indices,
                                        &rts_indices,
                                    );
                                    push_plane_to_axis(
                                        AssociatedVector::V2,
                                        vertex_buffer,
                                        &lbh_indices,
                                        &rbh_indices,
                                        &lbs_indices,
                                        &rbs_indices,
                                    );
                                } else if x.dot(movement_event.unit_vector) < 0.0 {
                                    // -y
                                    push_plane_to_sphere_vector(
                                        x,
                                        vertex_buffer,
                                        &lbh_indices,
                                        &rbh_indices,
                                        &lbs_indices,
                                        &rbs_indices,
                                    );
                                    push_plane_to_axis(
                                        AssociatedVector::V2,
                                        vertex_buffer,
                                        &lth_indices,
                                        &rth_indices,
                                        &lts_indices,
                                        &rts_indices,
                                    );
                                }

                            }
                            AssociatedVector::V3 => {
                                if x.dot(movement_event.unit_vector) > 0.0 {
                                    // +z
                                    push_plane_to_sphere_vector(
                                        x,
                                        vertex_buffer,
                                        &lts_indices,
                                        &rts_indices,
                                        &lbs_indices,
                                        &rbs_indices,
                                    );
                                    push_plane_to_axis(
                                        AssociatedVector::V3,
                                        vertex_buffer,
                                        &lth_indices,
                                        &rth_indices,
                                        &lbh_indices,
                                        &rbh_indices,
                                    );
                                } else if x.dot(movement_event.unit_vector) < 0.0 {
                                    // -z
                                    push_plane_to_sphere_vector(
                                        x,
                                        vertex_buffer,
                                        &lth_indices,
                                        &rth_indices,
                                        &lbh_indices,
                                        &rbh_indices,
                                    );
                                    push_plane_to_axis(
                                        AssociatedVector::V3,
                                        vertex_buffer,
                                        &lts_indices,
                                        &rts_indices,
                                        &lbs_indices,
                                        &rbs_indices,
                                    );
                                }

                            }
                        }
                        // if between(u.dot(x), u.dot(lbs), u.dot(lbh))
                        // && between(v.dot(x), v.dot(lbs), v.dot(rbs))
                        // && between(w.dot(x), w.dot(lbs), w.dot(lts)) {
                        //     println!("point in cube");
                        
                        
                        
                        // } else {
                        //     println!("point not in cube");
                        //     println!("unit vector: {:?}", movement_event.unit_vector);
                        //     println!("delta: {:?}", movement_event.delta);
                        //     let delta_vector = movement_event.unit_vector.mul_add(
                        //         Vec3 {x: movement_event.delta, y: movement_event.delta, z: movement_event.delta },
                        //             Vec3::ZERO
                        //     );
                        //     match movement_event.associated_vector {
                        //         AssociatedVector::V1 => {
                        //             // if movement_event.delta > 0.0 {
                        //             //     // move span towards +v1
                        //             //     for index in [1, 11, 22] {
                                            
                        //             //     }

                        //             // } else if movement_event.delta < 0.0 {
                        //             //     // move span towards -v1

                        //             // } else { // delta == 0
                        //             //     // do nothing
                                        
                        //             // }
                        //             for index in [1, 11, 22] {
                        //                 vertex_buffer[index][0] += delta_vector.x;
                        //                 vertex_buffer[index][0] += delta_vector.y;
                        //                 vertex_buffer[index][0] += delta_vector.z;
                        //             }
                                    
                        //         }
                        //         AssociatedVector::V2 => {

                        //         }
                        //         AssociatedVector::V3 => {

                        //         }
                        //     }
                        // }

                        // vertex_buffer[0][0] = -10.0;
                        // vertex_buffer[0][1] = -10.0;
                        // vertex_buffer[0][2] = -10.0;
                        // println!("vertex_buffer: {:?}", vertex_buffer);
                       
                    }
                    // println!("got the mesh");
                }
            }
        }
    }
}



fn push_plane_to_sphere_vector(
    x: Vec3,
    vertex_buffer: &mut Vec<[f32; 3]>,
    v00_indices: &[usize; 3],
    v01_indices: &[usize; 3],
    v10_indices: &[usize; 3],
    v11_indices: &[usize; 3],
) {
    let v00 = Vec3::from(vertex_buffer[v00_indices[0]]);
    let v01 = Vec3::from(vertex_buffer[v01_indices[0]]);
    let v10 = Vec3::from(vertex_buffer[v10_indices[0]]);
    let v11 = Vec3::from(vertex_buffer[v11_indices[0]]);

    let plane_to_point = shortest_vector_from_plane_to_point(
        x,
        (v00 - v01).cross(v00 - v10).normalize(),
        v11,
    );
    // println!("\nPlane to point: {:?}", plane_to_point);

    for index in v00_indices.iter() { //
        vertex_buffer[*index][0] += plane_to_point.x;
        vertex_buffer[*index][1] += plane_to_point.y;
        vertex_buffer[*index][2] += plane_to_point.z;
    }
    for index in v01_indices.iter() { //
        vertex_buffer[*index][0] += plane_to_point.x;
        vertex_buffer[*index][1] += plane_to_point.y;
        vertex_buffer[*index][2] += plane_to_point.z;
    }
    for index in v10_indices.iter() { //
        vertex_buffer[*index][0] += plane_to_point.x;
        vertex_buffer[*index][1] += plane_to_point.y;
        vertex_buffer[*index][2] += plane_to_point.z;
    }
    for index in v11_indices.iter() { //
        vertex_buffer[*index][0] += plane_to_point.x;
        vertex_buffer[*index][1] += plane_to_point.y;
        vertex_buffer[*index][2] += plane_to_point.z;
    }
}

fn push_plane_to_axis(
    associated_vector: AssociatedVector,
    vertex_buffer: &mut Vec<[f32; 3]>,
    v00_indices: &[usize; 3],
    v01_indices: &[usize; 3],
    v10_indices: &[usize; 3],
    v11_indices: &[usize; 3],
) {
    let v00 = Vec3::from(vertex_buffer[v00_indices[0]]);
    let v01 = Vec3::from(vertex_buffer[v01_indices[0]]);
    let v10 = Vec3::from(vertex_buffer[v10_indices[0]]);
    let v11 = Vec3::from(vertex_buffer[v11_indices[0]]);

    match associated_vector {
        AssociatedVector::V1 => {
            for index in v00_indices.iter() { //
                vertex_buffer[*index][0] = 0.0;
            }
            for index in v01_indices.iter() { //
                vertex_buffer[*index][0] = 0.0;
            }
            for index in v10_indices.iter() { //
                vertex_buffer[*index][0] = 0.0;
            }
            for index in v11_indices.iter() { //
                vertex_buffer[*index][0] = 0.0;
            }
        }
        AssociatedVector::V2 => {
            for index in v00_indices.iter() { //
                vertex_buffer[*index][1] = 0.0;
            }
            for index in v01_indices.iter() { //
                vertex_buffer[*index][1] = 0.0;
            }
            for index in v10_indices.iter() { //
                vertex_buffer[*index][1] = 0.0;
            }
            for index in v11_indices.iter() { //
                vertex_buffer[*index][1] = 0.0;
            }

        }
        AssociatedVector::V3 => {
            for index in v00_indices.iter() {
                vertex_buffer[*index][2] = 0.0;
            }
            for index in v01_indices.iter() {
                vertex_buffer[*index][2] = 0.0;
            }
            for index in v10_indices.iter() {
                vertex_buffer[*index][2] = 0.0;
            }
            for index in v11_indices.iter() {
                vertex_buffer[*index][2] = 0.0;
            }
        }
    }
}



fn shortest_vector_from_plane_to_point(point: Vec3, plane_normal: Vec3, plane_point: Vec3) -> Vec3 {
    // Project the point onto the plane
    let projected_point = project_point_onto_plane(point, plane_normal, plane_point);

    // Calculate the distance between the original point and the projected point
    // let distance = (projected_point - point).length();

    return point - projected_point;
}

fn project_point_onto_plane(point: Vec3, plane_normal: Vec3, plane_point: Vec3) -> Vec3 {
    let v = point - plane_point;
    let distance_to_plane = v.dot(plane_normal);
    let projected_point = point - distance_to_plane * plane_normal;
    // println!("projected point: {:?}", projected_point);
    projected_point
}


#[derive(Component)]
pub struct VectorSphere;

#[derive(Component)]
pub struct VectorSphereBasisVector;


#[derive(Event)]
pub struct VectorSphereMovementEvent {
    pub associated_vector: AssociatedVector,
    pub unit_vector: Vec3,
    pub delta: f32,
}

#[derive(Component)]
pub struct SpherePositionBanner;

#[derive(Component)]
pub struct SpaceshipPositionText;

fn make_vector(
    commands: &mut Commands,
    vec: Vec3,
    associated_vector: AssociatedVector,
    crew_render_layer: RenderLayers,
    mesh_handle: Handle<Mesh>,
    material_handle: Handle<StandardMaterial>,
    color_function: theme::ColorFunction,
) -> Entity {
    return commands.spawn((
        VectorSphereBasisVector,
        color_function,
        PbrBundle {
            mesh: mesh_handle.clone(),
            material: material_handle.clone(),
            transform: Transform::from_translation(vec),
            ..default()
        },
        crew_render_layer,
        subsection::SubsectionGameEntity,

        //pickable stuff
        PickableBundle {
            ..default()
        }, // <- Makes the mesh pickable.
        On::<Pointer<DragStart>>::send_event::<PanOrbitToggleEvent>(),
        On::<Pointer<DragEnd>>::send_event::<PanOrbitToggleEvent>(),
        On::<Pointer<Drag>>::run(move |
            event: Listener<Pointer<Drag>>,
            mut vector_sphere_movement_writer: EventWriter<VectorSphereMovementEvent> | {
            send_movement_vector(event.delta.x, vec, associated_vector.clone(), vector_sphere_movement_writer);
        }),
    )).id();
}

#[derive(Clone)]
pub enum AssociatedVector {
    V1,
    V2,
    V3,
}

#[derive(Component)]
pub struct SpanFace {
    associated_vector: AssociatedVector,
    start: bool,
}

fn make_vector_mesh(mut meshes: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {
    // meshes.add()
    let mesh = Mesh::new(PrimitiveTopology::TriangleList)
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        // Each array is an [x, y, z] coordinate in local space.
        // Meshes always rotate around their local [0, 0, 0] when a rotation is applied to their Transform.
        // By centering our mesh around the origin, rotating the mesh preserves its center of mass.
        vec![
            // top (facing towards +y)
            [-0.5, 0.5, -0.5], // vertex with index 0
            [0.5, 0.5, -0.5], // vertex with index 1
            [0.5, 0.5, 0.5], // etc. until 23
            [-0.5, 0.5, 0.5],
            // bottom   (-y)
            [-0.5, -0.5, -0.5],
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [-0.5, -0.5, 0.5],
            // right    (+x)
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5], // This vertex is at the same position as vertex with index 2, but they'll have different UV and normal
            [0.5, 0.5, -0.5],
            // left     (-x)
            [-0.5, -0.5, -0.5],
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [-0.5, 0.5, -0.5],
            // back     (+z)
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, -0.5, 0.5],
            // forward  (-z)
            [-0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [0.5, 0.5, -0.5],
            [0.5, -0.5, -0.5],
        ],
    )
    // Create the triangles out of the 24 vertices we created.
    // To construct a square, we need 2 triangles, therefore 12 triangles in total.
    // To construct a triangle, we need the indices of its 3 defined vertices, adding them one
    // by one, in a counter-clockwise order (relative to the position of the viewer, the order
    // should appear counter-clockwise from the front of the triangle, in this case from outside the cube).
    // Read more about how to correctly build a mesh manually in the Bevy documentation of a Mesh,
    // further examples and the implementation of the built-in shapes.
    .with_indices(Some(Indices::U32(vec![
        0,3,1 , 1,3,2, // triangles making up the top (+y) facing side.
        4,5,7 , 5,6,7, // bottom (-y)
        8,11,9 , 9,11,10, // right (+x)
        12,13,15 , 13,14,15, // left (-x)
        16,19,17 , 17,19,18, // back (+z)
        20,21,23 , 21,22,23, // forward (-z)
    ])));

    return meshes.add(mesh);
}

pub fn setup_scene(
    commands: &mut Commands,
    theme: &theme::CurrentTheme,
    film_crew_entity: Entity,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    mut asset_server: &mut Res<AssetServer>,
    crew_id: u8,
) {
    let crew_render_layer = RenderLayers::layer(crew_id);
    

    let sphere_handle = meshes.add(Mesh::from(
        shape::UVSphere {
            radius: 0.3,
            sectors: 36 * 2,
            stacks: 18 * 2,
            ..default()
        }
    ));

    let quad_handle = meshes.add(Mesh::from(
        shape::Quad {
            size: Vec2 { x: 1.0, y: 2.0 },
            flip: false,
            ..default()
        }
    ));

    let sphere_material_handle = materials.add(StandardMaterial {
        // base_color: Color::rgb(1.0, 0.75, 0.90),
        base_color: theme::vector_color_3d(theme).into(),
        alpha_mode: AlphaMode::Blend,
        metallic: 1.0,
        reflectance: 0.1,
        perceptual_roughness: 1.0,
        ..default()
    });
    

    let span_material_handle = materials.add(StandardMaterial {
        base_color: theme::line_color(theme).into(),
        // metallic: 1.0,
        // reflectance: 0.1,
        // perceptual_roughness: 1.0,
        ..default()
    });

    let basis_vector_1 = materials.add(StandardMaterial {
        base_color: theme::line_alternate_color_1(theme).into(),
        alpha_mode: AlphaMode::Blend,
        // metallic: 1.0,
        // reflectance: 0.1,
        // perceptual_roughness: 1.0,
        ..default()
    });
    let basis_vector_2 = materials.add(StandardMaterial {
        base_color: theme::line_alternate_color_2(theme).into(),
        alpha_mode: AlphaMode::Blend,
        // metallic: 1.0,
        // reflectance: 0.1,
        // perceptual_roughness: 1.0,
        ..default()
    });
    let basis_vector_3 = materials.add(StandardMaterial {
        base_color: theme::line_alternate_color_3(theme).into(),
        alpha_mode: AlphaMode::Blend,
        // metallic: 1.0,
        // reflectance: 0.1,
        // perceptual_roughness: 1.0,
        ..default()
    });


    examples_3d::spawn_axis(
        commands,
        theme,
        film_crew_entity,
        meshes,
        materials,
        crew_render_layer
    );
    examples_3d::spawn_grid(
        commands,
        theme,
        film_crew_entity,
        meshes,
        materials,
        crew_render_layer
    );


    // The cube that will be rendered to the texture.
    let sphere = commands.spawn((
        VectorSphere,
        theme::ColorFunction {
            background: theme::vector_color_3d,
            border: theme::vector_color_3d,
        },
        PbrBundle {
            mesh: sphere_handle.clone(),
            material: sphere_material_handle.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
        // theme::HIGHLIGHT_TINT,
        // FocusPolicy::Block,
        // SpinnyCube,
        crew_render_layer,
        subsection::SubsectionGameEntity,

        //pickable stuff
        PickableBundle {
            ..default()
        }, // <- Makes the mesh pickable.
        On::<Pointer<DragStart>>::send_event::<PanOrbitToggleEvent>(),
        On::<Pointer<DragEnd>>::send_event::<PanOrbitToggleEvent>(),
        On::<Pointer<Click>>::send_event::<VectorSphereSelectionEvent>(),
    )).id();
    

    let vector_mesh_handle = meshes.add(Mesh::from(
        shape::Cube {
            size: 0.5,
            ..default()
        }
    ));


    
    commands.entity(film_crew_entity).push_children(&[
        sphere,
    ]);
}

pub fn disable_pan_orbit_system (
    mut pan_orbit_toggle_reader: EventReader<PanOrbitToggleEvent>,
    mut pan_orbit_camera_query: Query<&mut PanOrbitCamera, With<PanOrbitCamera>>,
) {
    for pan_orbit_toggle in pan_orbit_toggle_reader.read() {
        let mut pan_orbit_camera = pan_orbit_camera_query.single_mut();

        match &pan_orbit_toggle.state {
            PanOrbitToggleState::DragStart => {
                pan_orbit_camera.enabled = false;
            }
            PanOrbitToggleState::DragEnd => {
                pan_orbit_camera.enabled = true;
            }
        }
    }
}

pub fn update_sphere_position_text(
    mut text_query: Query<&mut Text, With<SpaceshipPositionText>>,
    sphere_transform_query: Query<&Transform, With<VectorSphere>>,
    mut movement_event_reader: EventReader<VectorSphereMovementEvent>,
) {
    for movement_event in movement_event_reader.read() {
        for mut text in text_query.iter_mut() {
            for sphere_transform in sphere_transform_query.iter() {
                text.sections[0].value = format!(
                    "No vectors\n\n\n\nSPHERE:\n      {:?}\n      {:?}\n      {:?}",
                    sphere_transform.translation.x,
                    sphere_transform.translation.y,
                    sphere_transform.translation.z,
                );
            }
        }
    }
    
}
pub fn sphere_position_banner_interactions (
    mut pan_orbit_toggle_reader: EventReader<PanOrbitToggleEvent>,
    mut sphere_position_banner_query: Query<Entity, With<SpherePositionBanner>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,

    theme: Res<theme::CurrentTheme>,
    mut commands: Commands,
) {
    
    for pan_orbit_toggle in pan_orbit_toggle_reader.read() {
        
        for sphere_position_banner in sphere_position_banner_query.iter() {
            commands.entity(sphere_position_banner).despawn_recursive();
        }
        if let Some(cursor_position) = q_windows.single().cursor_position() {
            let theme = theme.as_ref();
            match pan_orbit_toggle.state {
                PanOrbitToggleState::DragStart => {
                    let background_banner = commands.spawn((
                        SpherePositionBanner,
                        NodeBundle {
                            style: Style {
                                width: Val::Px(300.0),
                                height: Val::Px(230.0),
                                position_type: PositionType::Absolute,
                                left: Val::Px(cursor_position.x - 150.0),
                                top: Val::Px(cursor_position.y + 100.0),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::SpaceBetween,
                                ..default()
                            },
                            z_index: ZIndex::Global(10),
                            background_color: theme::transparent_maker(theme, theme::background_color).into(),
                            ..default()
                        }
                    )).id();

                    let top = commands.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Auto,
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                ..default()
                            },
                            ..default()
                        }
                    )).id();

                    let bottom = commands.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Auto,
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                ..default()
                            },
                            ..default()
                        }
                    )).id();
                    commands.entity(background_banner).push_children(&[top, bottom]);

                    let tl = commands.spawn((
                        NodeBundle {
                            style: Style {
                                height: Val::Px(20.0),
                                aspect_ratio: Some(1.0),
                                border: UiRect {
                                    top: Val::Px(4.0),
                                    bottom: Val::Px(0.0),
                                    left: Val::Px(4.0),
                                    right: Val::Px(0.0),
                                },
                                ..default()
                            },
                            border_color: theme::sidebar_header_color(theme).into(),
                            ..default()
                        }
                    )).id();
                    let tr = commands.spawn((
                        NodeBundle {
                            style: Style {
                                height: Val::Px(20.0),
                                aspect_ratio: Some(1.0),
                                border: UiRect {
                                    top: Val::Px(4.0),
                                    bottom: Val::Px(0.0),
                                    left: Val::Px(0.0),
                                    right: Val::Px(4.0),
                                },
                                ..default()
                            },
                            border_color: theme::sidebar_header_color(theme).into(),
                            ..default()
                        }
                    )).id();
                    commands.entity(top).push_children(&[tl, tr]);


                    let bl = commands.spawn((
                        NodeBundle {
                            style: Style {
                                height: Val::Px(20.0),
                                aspect_ratio: Some(1.0),
                                border: UiRect {
                                    top: Val::Px(0.0),
                                    bottom: Val::Px(4.0),
                                    left: Val::Px(4.0),
                                    right: Val::Px(0.0),
                                },
                                ..default()
                            },
                            border_color: theme::sidebar_header_color(theme).into(),
                            ..default()
                        }
                    )).id();
                    let br = commands.spawn((
                        NodeBundle {
                            style: Style {
                                height: Val::Px(20.0),
                                aspect_ratio: Some(1.0),
                                border: UiRect {
                                    top: Val::Px(0.0),
                                    bottom: Val::Px(4.0),
                                    left: Val::Px(0.0),
                                    right: Val::Px(4.0),
                                },
                                ..default()
                            },
                            border_color: theme::sidebar_header_color(theme).into(),
                            ..default()
                        }
                    )).id();
                    commands.entity(bottom).push_children(&[bl, br]);

                    let text = commands.spawn((
                        // Create a TextBundle that has a Text with a single section.
                        TextBundle::from_section(
                            // Accepts a `String` or any type that converts into a `String`, such as `&str`
                            "No vectors\n\n\n\nSPHERE:\n      0.0\n      0.0\n      0.0",
                            TextStyle {
                                // This font is loaded and will be used instead of the default font.
                                font_size: 25.0,
                                color: theme::navbar_swiper_color(theme).into(),
                                ..default()
                            },
                        ) 
                        // Set the style of the TextBundle itself.
                        .with_style(Style {
                            left: Val::Px(16.0),
                            top: Val::Px(8.0),
                            position_type: PositionType::Absolute,
                            justify_self: JustifySelf::Center,
                            ..default()
                        }),
                        SpaceshipPositionText,
                    )).id();

                    commands.entity(background_banner).push_children(&[text]);


                    let vector_background = commands.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(200.0),
                                height: Val::Px(100.0),
                                top: Val::Px(135.0),
                                left: Val::Px(50.0),
                                position_type: PositionType::Absolute,
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                ..default()
                            },
                            // background_color: Color::RED.into(),
                            ..default()
                        }
                    )).id();
                    commands.entity(background_banner).push_children(&[vector_background]);

                    let vector_bracket_left = commands.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(20.0),
                                height: Val::Percent(80.0),
                                border: UiRect {
                                    top: Val::Px(4.0),
                                    bottom: Val::Px(4.0),
                                    left: Val::Px(4.0),
                                    right: Val::Px(0.0),
                                },
                                ..default()
                            },
                            border_color: theme::navbar_swiper_color(theme).into(),
                            ..default()
                        }
                    )).id();

                    let vector_bracket_right = commands.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(20.0),
                                height: Val::Percent(80.0),
                                border: UiRect {
                                    top: Val::Px(4.0),
                                    bottom: Val::Px(4.0),
                                    left: Val::Px(0.0),
                                    right: Val::Px(4.0),
                                },
                                ..default()
                            },
                            border_color: theme::navbar_swiper_color(theme).into(),
                            ..default()
                        }
                    )).id();
                    commands.entity(vector_background).push_children(&[vector_bracket_left, vector_bracket_right]);
                }
                PanOrbitToggleState::DragEnd => {
                }
    
            }
        } else {
            println!("Cursor is not in the game window.");
        }   
    }
}

fn send_movement_vector(delta: f32, unit_vector: Vec3, associated_vector: AssociatedVector, mut writer: EventWriter<VectorSphereMovementEvent>) {
    let delta = delta * 0.02;
    // let delta_vec = unit_vector.mul_add(Vec3 {x: delta, y: delta, z: delta }, Vec3::ZERO);
    writer.send( 
        VectorSphereMovementEvent {
            associated_vector: associated_vector,
            unit_vector: unit_vector,
            delta: delta,
            
        }
    );
}


pub fn vector_sphere_selection_system(
    mut mesh_selection_reader: EventReader<VectorSphereSelectionEvent>,
    vector_sphere_query: Query<&VectorSphere, With<VectorSphere>>,
    mut basis_vector_query: Query<(&VectorSphereBasisVector, &mut Visibility), With<VectorSphereBasisVector>>,
) {
    for mesh_selection_event in mesh_selection_reader.read() {
        for vector_sphere in vector_sphere_query.iter() {
            for (basis_vector, mut visibility) in basis_vector_query.iter_mut() {
                match *visibility {
                    Visibility::Hidden => {
                        *visibility = Visibility::Inherited;
                    }
                    Visibility::Inherited => {
                        *visibility = Visibility::Hidden;
                    }
                    Visibility::Visible => {
                        *visibility = Visibility::Hidden;
                    }
                }
            }
        }
    }
}



pub fn vector_sphere_movement_system(
    mut movement_reader: EventReader<VectorSphereMovementEvent>,
    mut vector_sphere_query: Query<(&mut Transform), With<VectorSphere>>,
    mut basis_vector_query: Query<(&mut Transform), (With<VectorSphereBasisVector>, Without<VectorSphere>)>,
) {
    for movement_event in movement_reader.read() {
        let delta_vector = movement_event.unit_vector.mul_add(
            Vec3 {x: movement_event.delta, y: movement_event.delta, z: movement_event.delta },
             Vec3::ZERO
        );
        let mut can_move = false;
        
        for mut vector_sphere_transform in vector_sphere_query.iter_mut() {
            let outcome = vector_sphere_transform.translation + delta_vector;
            
            if in_bounds(&outcome, -10.0, 10.0) {
                vector_sphere_transform.translation = outcome;
                can_move = true;
            }
        }

        for mut basis_vector_transform in basis_vector_query.iter_mut() {
            if can_move {
                basis_vector_transform.translation =
                    basis_vector_transform.translation + delta_vector;
            }
        }
    }
    
}

fn in_bounds(vec: &Vec3, low: f32, high: f32) -> bool {
    return vec.x < 10.0 && vec.x > -10.0
        && vec.y < 10.0 && vec.y > -10.0
        && vec.z < 10.0 && vec.z > -10.0;
}

pub fn basis_vectors_movement_system(
    mut movement_reader: EventReader<VectorSphereMovementEvent>,
    // mut vector_sphere_query: Query<(&mut Transform), With<VectorSphere>>,
    mut basis_vector_query: Query<(&mut Transform), With<VectorSphereBasisVector>>,
) {
    // for movement_event in movement_reader.read() {
    //     for mut basis_vector_transform in basis_vector_query.iter_mut() {
    //         basis_vector_transform.translation =
    //             basis_vector_transform.translation + movement_event.delta_vec3;
    //     }
    // }
    
}