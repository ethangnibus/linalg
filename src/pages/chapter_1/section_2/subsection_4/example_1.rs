
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
use bevy::ui::FocusPolicy;
use bevy_mod_picking::prelude::*;

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
                move_span_cube_vertices,
            ));
    }
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
) {
    
    let mesh = Mesh::new(PrimitiveTopology::TriangleList)
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        // Each array is an [x, y, z] coordinate in local space.
        // Meshes always rotate around their local [0, 0, 0] when a rotation is applied to their Transform.
        // By centering our mesh around the origin, rotating the mesh preserves its center of mass.
        vec![
            // top (facing towards +y) 
            [-v1.x + v2.x - v3.x, -v1.y + v2.y - v3.y, -v1.z + v2.z - v3.z], // [-0.5, 0.5, -0.5], // vertex with index 0 //// -v1 + v2 - v3
            [v1.x + v2.x - v3.x, v1.y + v2.y - v3.y, v1.z + v2.z - v3.z], // [0.5, 0.5, -0.5], // vertex with index 1 //// v1 + v2 - v3
            [v1.x + v2.x + v3.x, v1.y + v2.y + v3.y, v1.z + v2.z + v3.z], // [0.5, 0.5, 0.5], // etc. until 23 //// v1 + v2 + v3
            [-v1.x + v2.x + v3.x, -v1.y + v2.y + v3.y, -v1.z + v2.z + v3.z], // [-0.5, 0.5, 0.5], //// -v1 + v2 + v3
            // bottom   (-y)
            [-v1.x - v2.x - v3.x, -v1.y - v2.y - v3.y, -v1.z - v2.z - v3.z], //[-0.5, -0.5, -0.5], //// -v1 - v2 - v3
            [v1.x - v2.x - v3.x, v1.y - v2.y - v3.y, v1.z - v2.z - v3.z], //[0.5, -0.5, -0.5], //// v1 - v2 - v3
            [v1.x - v2.x + v3.x, v1.y - v2.y + v3.y, v1.z - v2.z + v3.z], //[0.5, -0.5, 0.5], //// v1 - v2 + v3
            [-v1.x - v2.x + v3.x, -v1.y - v2.y + v3.y, -v1.z - v2.z + v3.z], //[-0.5, -0.5, 0.5], //// -v1 - v2 + v3
            // right    (+x)
            [v1.x - v2.x - v3.x, v1.y - v2.y - v3.y, v1.z - v2.z - v3.z], // [0.5, -0.5, -0.5], //// v1 - v2 - v3
            [v1.x - v2.x + v3.x, v1.y - v2.y + v3.y, v1.z - v2.z + v3.z], // [0.5, -0.5, 0.5], //// v1 - v2 + v3
            [v1.x + v2.x + v3.x, v1.y + v2.y + v3.y, v1.z + v2.z + v3.z], //[0.5, 0.5, 0.5], //// v1 + v2 + v3           // This vertex is at the same position as vertex with index 2, but they'll have different UV and normal
            [v1.x + v2.x - v3.x, v1.y + v2.y - v3.y, v1.z + v2.z - v3.z], //[0.5, 0.5, -0.5], //// v1 + v2 - v3
            // left     (-x)
            [-v1.x - v2.x - v3.x, -v1.y - v2.y - v3.y, -v1.z - v2.z - v3.z], //[-0.5, -0.5, -0.5], //// -v1 - v2 - v3
            [-v1.x - v2.x + v3.x, -v1.y - v2.y + v3.y, -v1.z -v2.z + v3.z], // [-0.5, -0.5, 0.5], //// -v1 - v2 + v3
            [-v1.x + v2.x + v3.x, -v1.y + v2.y + v3.y, -v1.z + v2.z + v3.z], // [-0.5, 0.5, 0.5], ////  -v1 + v2 + v3
            [-v1.x + v2.x - v3.x, -v1.y + v2.y - v3.y, -v1.z + v2.z - v3.z], // [-0.5, 0.5, -0.5], ////  -v1 + v2 - v3
            // back     (+z)
            [-v1.x - v2.x + v3.x, -v1.y - v2.y + v3.y, -v1.z - v2.z + v3.z], // [-0.5, -0.5, 0.5], ////  -v1 - v2 + v3
            [-v1.x + v2.x + v3.x, -v1.y + v2.y + v3.y, -v1.z + v2.z + v3.z], // [-0.5, 0.5, 0.5], //// -v1 + v2 + v3
            [v1.x + v2.x + v3.x, v1.y + v2.y + v3.y, v1.z + v2.z + v3.z], // [0.5, 0.5, 0.5], //// v1 + v2 + v3
            [v1.x - v2.x + v3.x, v1.y - v2.y + v3.y, v1.z - v2.z + v3.z], // [0.5, -0.5, 0.5], //// v1 - v2 + v3
            // forward  (-z)
            [-v1.x - v2.x - v3.x, -v1.y - v2.y - v3.y, -v1.z - v2.z - v3.z], // [-0.5, -0.5, -0.5], //// -v1 - v2 - v3
            [-v1.x + v2.x - v3.x, -v1.y + v2.y - v3.y, -v1.z + v2.z - v3.z], // [-0.5, 0.5, -0.5], //// -v1 + v2 - v3
            [v1.x + v2.x - v3.x, v1.y + v2.y - v3.y, v1.z + v2.z - v3.z], // [0.5, 0.5, -0.5], //// v1 + v2 - v3
            [v1.x - v2.x - v3.x, v1.y - v2.y - v3.y, v1.z - v2.z - v3.z], // [0.5, -0.5, -0.5], //// v1 - v2 - v3
        ],
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
    commands.spawn((
        SpanCube {
            quad_handle: mesh_handle.clone(),
        },
        PbrBundle {
            mesh: mesh_handle.clone(),
            material: materials.add(StandardMaterial {
                base_color: theme::line_color_transparent(theme, 0.4).into(),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            }),
            ..default()
        },
        Pickable::IGNORE,
        crew_render_layer,
    ));
    // commands.insert_resource(SpanCubeResource{
    //     quad_handle: mesh_handle,
    // })
    
}

#[derive(Component)]
pub struct SpanCubeResource{
    quad_handle: Handle<Mesh>,
}

fn move_span_cube_vertices(
    mut movement_reader: EventReader<VectorSphereMovementEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut span_cube_query: Query<&SpanCube, With<SpanCube>>,
) {
    for movement in movement_reader.read() {
        for span_cube in span_cube_query.iter() {
            println!("quad_handle: {:?}", span_cube.quad_handle);
        
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
                println!("primitive_topology: {:?}", primitive_topology);
                if let Some(vertex_buffer) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
                // Update vertices based on your logic
                    // For example, let's say you have new vertices stored in a Vec<Vec3> called new_vertices
                    println!("vertex_buffer: {:?}", vertex_buffer);

                    let VertexAttributeValues::Float32x3(vertex_buffer) = vertex_buffer else {
                        panic!("Unexpected vertex format, expected Float32x2.");
                    };
                    println!("vertex attribute_values: {:?}", vertex_buffer);
                
                    
                    // for mut x in vertex_buffer.iter() {

                    // }
                    // vertex_buffer[0] = vertex_buffer[0] + 1;
                    vertex_buffer[0][0] = -10.0;
                    vertex_buffer[0][1] = -10.0;
                    vertex_buffer[0][2] = -10.0;
                    // println!("vertex_buffer: {:?}", vertex_buffer);

                }
                // println!("got the mesh");
            }
        }
    }
}


#[derive(Component)]
pub struct VectorSphere;

#[derive(Component)]
pub struct VectorSphereBasisVector;

#[derive(Event)]
pub struct VectorSphereMovementEvent {
    pub delta_vec3: Vec3,
}


fn make_vector(
    commands: &mut Commands,
    vec: Vec3,
    crew_render_layer: RenderLayers,
    mesh_handle: Handle<Mesh>,
    material_handle: Handle<StandardMaterial>
) -> Entity {
    return commands.spawn((
        VectorSphereBasisVector,
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
            send_movement_vector(event.delta.x, vec, vector_sphere_movement_writer);
        }),
    )).id();
}

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


pub fn setup_scene(
    commands: &mut Commands,
    theme: &theme::CurrentTheme,
    film_crew_entity: Entity,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
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
        // metallic: 1.0,
        // reflectance: 0.1,
        // perceptual_roughness: 1.0,
        ..default()
    });

    let basis_vector_2 = materials.add(StandardMaterial {
        base_color: theme::line_alternate_color_2(theme).into(),
        // metallic: 1.0,
        // reflectance: 0.1,
        // perceptual_roughness: 1.0,
        ..default()
    });
    let basis_vector_3 = materials.add(StandardMaterial {
        base_color: theme::line_alternate_color_3(theme).into(),
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
    let sphere = commands
        .spawn((
            VectorSphere,
            theme::ColorFunction {
                background: theme::vector_color_3d,
                border: theme::vector_color_3d,
            },
            PbrBundle {
                mesh: sphere_handle,
                material: sphere_material_handle.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
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
        ))
        .id();
    let cylinder_handle = meshes.add(Mesh::from(
        shape::Cube {
            size: 0.5,
            ..default()
        }
    ));

    
    let v1 = Vec3 { x: 1.0, y: 1.0, z: 0.0 }.normalize();
    let v2 = Vec3 { x: 0.0, y: 0.3, z: 0.4 }.normalize();
    let v3 = Vec3 { x: 0.7, y: 0.1, z: 0.6 }.normalize();

    
    let standard_basis_vector_x = make_vector(
        commands,
        v1,
        crew_render_layer,
        cylinder_handle.clone(),
        basis_vector_1
    );
    let standard_basis_vector_y = make_vector(
        commands,
        v2,
        crew_render_layer,
        cylinder_handle.clone(),
        basis_vector_2
    );
    let standard_basis_vector_z = make_vector(
        commands,
        v3,
        crew_render_layer,
        cylinder_handle.clone(),
        basis_vector_3
    );

    
    // commands.spawn((
    //     PbrBundle {
    //     mesh: quad_handle,
    //     // This is the default color, but note that vertex colors are
    //     // multiplied by the base color, so you'll likely want this to be
    //     // white if using vertex colors.
    //     material: span_material_handle,
    //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //     ..default()
    //     }, 
    //     crew_render_layer,
    // ));

    create_custom_cube_mesh(commands, theme, meshes, materials, crew_render_layer, v1, v2, v3);
    
    commands.entity(film_crew_entity).push_children(&[sphere, standard_basis_vector_x, standard_basis_vector_y, standard_basis_vector_z]);
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

fn send_movement_vector(delta: f32, unit_vector: Vec3, mut writer: EventWriter<VectorSphereMovementEvent>) {
    let delta = delta * 0.02;
    let delta_vec = unit_vector.mul_add(Vec3 {x: delta, y: delta, z: delta }, Vec3::ZERO);
    writer.send( 
        VectorSphereMovementEvent {
            delta_vec3: delta_vec,
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
        let mut can_move = false;
        
        for mut vector_sphere_transform in vector_sphere_query.iter_mut() {
            let outcome = vector_sphere_transform.translation + movement_event.delta_vec3;

            if in_bounds(&outcome, -10.0, 10.0) {
                vector_sphere_transform.translation = outcome;
                can_move = true;
            }
        }

        for mut basis_vector_transform in basis_vector_query.iter_mut() {
            if can_move {
                basis_vector_transform.translation =
                    basis_vector_transform.translation + movement_event.delta_vec3;
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