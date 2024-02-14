
use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;
use bevy::pbr::ViewLightEntities;
use bevy::render::mesh::shape::Cylinder;
use bevy::render::render_resource::encase::vector;
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

#[derive(Component)]
pub struct VectorSphere;

#[derive(Component)]
pub struct VectorSphereBasisVector;

#[derive(Event)]
pub struct VectorSphereMovementEvent {
    pub delta_vec3: Vec3,
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

            // On::<Pointer<Click>>::run(|event: Listener<Pointer<Click>>, time: Res<Time>| {
            //     info!(
            //         "[{:?}]: The pointer left entity {:?}",
            //         time.elapsed_seconds(),
            //         event.target
            //     );
            // }),
            // On::<Pointer<DragStart>>::send_event::(),
            // On::<Pointer<DragStart>>::target_insert(Pickable::IGNORE), // Disable picking
            // On::<Pointer<DragEnd>>::target_insert(Pickable::default()), // Re-enable picking
            // On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
            //     transform.translation.x += drag.delta.x; // Make the square follow the mouse
            //     transform.translation.y -= drag.delta.y;
            // }),
            // On::<Pointer<Drop>>::commands_mut(|event, commands| {
            //     commands.entity(event.dropped).insert(Spin(FRAC_PI_2)); // Spin dropped entity
            //     commands.entity(event.target).insert(Spin(-FRAC_PI_2)); // Spin dropped-on entity
            // }),
        ))
        .id();
    

    let cylinder_handle = meshes.add(Mesh::from(
        shape::Cube {
            size: 0.5,
            ..default()
        }
    ));

    let v1 = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
    let v2 = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
    let v3 = Vec3 { x: 0.0, y: 0.0, z: 1.0 };

    let standard_basis_vector_x = commands.spawn((
        VectorSphereBasisVector,
        PbrBundle {
            mesh: cylinder_handle.clone(),
            material: basis_vector_1.clone(),
            transform: Transform::from_translation(v1),
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
        // On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
        //     let delta = drag.delta.x * 0.01;
        //     if delta > 0.0 && transform.translation.x < 10.0 {
        //         transform.translation.x += drag.delta.x * 0.01;
        //     } else if delta < 0.0 && transform.translation.x > -10.0 {
        //         transform.translation.x += drag.delta.x * 0.01;
        //     }
        // }),
        On::<Pointer<Drag>>::run(move |
            event: Listener<Pointer<Drag>>,
            mut vector_sphere_movement_writer: EventWriter<VectorSphereMovementEvent> | {
            send_movement_vector(event.delta.x, v1, vector_sphere_movement_writer);
        }),
    )).id();
    let standard_basis_vector_y = commands.spawn((
        VectorSphereBasisVector,
        PbrBundle {
            mesh: cylinder_handle.clone(),
            material: basis_vector_2.clone(),
            transform: Transform::from_translation(v2),
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
            send_movement_vector(event.delta.x, v2, vector_sphere_movement_writer);
        }),
    )).id();
    let standard_basis_vector_z = commands.spawn((
        VectorSphereBasisVector,
        PbrBundle {
            mesh: cylinder_handle.clone(),
            material: basis_vector_3.clone(),
            transform: Transform::from_translation(v3),
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
            send_movement_vector(-event.delta.x, v3, vector_sphere_movement_writer);
        }),
    )).id();
    
    // commands.entity(sphere).push_children(&[
    //     standard_basis_vector_x,
    //     standard_basis_vector_y,
    //     standard_basis_vector_z,
    // ]);
    
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