
use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;
use bevy::pbr::ViewLightEntities;
use bevy::render::mesh::shape::Cylinder;
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
        app.add_event::<MeshSelectionEvent>()
            .add_event::<PanOrbitToggleEvent>()
            .add_systems(Update, (
                mesh_selection_system,
                disable_pan_orbit_system,
            ));
    }
}

#[derive(Event)]
pub struct MeshSelectionEvent;

impl From<ListenerInput<Pointer<Click>>> for MeshSelectionEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        return MeshSelectionEvent;
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
pub struct StandardBasisVector;



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
            radius: 1.0,
            ..default()
        }
    ));
    let sphere_material_handle = materials.add(StandardMaterial {
        // base_color: Color::rgb(1.0, 0.75, 0.90),
        base_color: theme::cube_base_color(theme).into(),
        metallic: 1.0,
        reflectance: 0.1,
        perceptual_roughness: 1.0,
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
                background: theme::cube_base_color,
                border: theme::cube_emissive_color,
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
            On::<Pointer<Click>>::send_event::<MeshSelectionEvent>(), 
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
    let standard_basis_vector_x = commands.spawn((
        StandardBasisVector,
        PbrBundle {
            mesh: cylinder_handle.clone(),
            material: sphere_material_handle.clone(),
            transform: Transform::from_translation(Vec3::new(3.0, 0.0, 0.0)),
            ..default()
        },
        crew_render_layer,
        subsection::SubsectionGameEntity,

        //pickable stuff
        PickableBundle {
            ..default()
        }, // <- Makes the mesh pickable.
    )).id();
    let standard_basis_vector_y = commands.spawn((
        StandardBasisVector,
        PbrBundle {
            mesh: cylinder_handle.clone(),
            material: sphere_material_handle.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 3.0, 0.0)),
            ..default()
        },
        crew_render_layer,
        subsection::SubsectionGameEntity,

        //pickable stuff
        PickableBundle {
            ..default()
        }, // <- Makes the mesh pickable.
    )).id();
    let standard_basis_vector_z = commands.spawn((
        StandardBasisVector,
        PbrBundle {
            mesh: cylinder_handle.clone(),
            material: sphere_material_handle.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 3.0)),
            ..default()
        },
        crew_render_layer,
        subsection::SubsectionGameEntity,

        //pickable stuff
        PickableBundle {
            ..default()
        }, // <- Makes the mesh pickable.
    )).id();
    
    commands.entity(sphere).push_children(&[
        standard_basis_vector_x,
        standard_basis_vector_y,
        standard_basis_vector_z,
    ]);
    
    commands.entity(film_crew_entity).push_children(&[sphere]);
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


pub fn mesh_selection_system(
    mut mesh_selection_reader: EventReader<MeshSelectionEvent>,
    q_parent: Query<&Children, With<VectorSphere>>,
    mut q_child: Query<&mut Visibility, With<StandardBasisVector>>,
) {
    for mesh_selection_event in mesh_selection_reader.read() {
        

        for children in q_parent.iter() {
            for &child in children.iter() {
                let mut visibility = q_child.get_mut(child).unwrap();
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