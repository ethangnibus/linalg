
use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;
use bevy_mod_picking::prelude::*;

use bevy::{
    render::view::RenderLayers,
    prelude::*,
};

use crate::ui::{
    util::theme,
    util::subsection,
};

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // Add the setup_ui system as a startup system
        app.add_systems(Update, (
            // rotator_system,
            spin,
        ));
    }
}


#[derive(Component)]
pub struct SpinnyCube;



pub fn setup_scene(
    commands: &mut Commands,
    theme: &theme::CurrentTheme,
    film_crew_entity: Entity,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    crew_id: u8,
) {
    
    let crew_render_layer = RenderLayers::layer(crew_id);

    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let cube_material_handle = if crew_id == 1 {
        materials.add(StandardMaterial {
            // base_color: Color::rgb(1.0, 0.75, 0.90),
            base_color: theme::sidebar_color(theme).into(),
            metallic: 20.0,
            reflectance: 0.02,
            unlit: false,
            ..default()
        })
    } else {
        materials.add(StandardMaterial {
            // base_color: Color::rgb(0.0, 0.1, 0.90),
            base_color: theme::sidebar_collapsed_color(theme).into(),
            metallic: 20.0,
            reflectance: 0.02,
            unlit: false,
            ..default()
        })
    };


    spawn_axis(commands, theme, film_crew_entity, meshes, materials, crew_render_layer);
    spawn_grid(commands, theme, film_crew_entity, meshes, materials, crew_render_layer);
    

    // The cube that will be rendered to the texture.
    let cube = commands
        .spawn((
            PbrBundle {
                mesh: cube_handle,
                material: cube_material_handle,
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..default()
            },
            SpinnyCube,
            crew_render_layer,
            subsection::SubsectionGameEntity,

            //pickable stuff
            PickableBundle::default(), // <- Makes the mesh pickable.
            On::<Pointer<DragStart>>::target_insert(Pickable::IGNORE), // Disable picking
            On::<Pointer<DragEnd>>::target_insert(Pickable::default()), // Re-enable picking
            On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
                println!("CUBE SELECTED");
                transform.translation.x += drag.delta.x; // Make the square follow the mouse
                transform.translation.y -= drag.delta.y;
            }),
            On::<Pointer<Drop>>::commands_mut(|event, commands| {
                commands.entity(event.dropped).insert(Spin(FRAC_PI_2)); // Spin dropped entity
                commands.entity(event.target).insert(Spin(-FRAC_PI_2)); // Spin dropped-on entity
            }),
        ))
        .id();
    
    commands.entity(film_crew_entity).push_children(&[cube]);
}


fn spawn_axis(
    commands: &mut Commands,
    theme: &theme::CurrentTheme,
    film_crew_entity: Entity,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    crew_render_layer: RenderLayers,
) {
    // Materials
    let x_axis_material_handle = materials.add(StandardMaterial {
        // base_color: Color::rgba(1.0, 1.0, 1.0, 0.2).into(),
        base_color: theme::axis_base_color(theme),
        metallic: 20.0,
        reflectance: 0.02,
        unlit: false,
        emissive: theme::axis_emissive_color(theme),
        // alpha_mode: AlphaMode::Blend,
        
        ..default()
    });
    let y_axis_material_handle = materials.add(StandardMaterial {
        // base_color: Color::rgba(1.0, 1.0, 1.0, 0.2).into(),
        base_color: theme::axis_base_color(theme),
        metallic: 20.0,
        reflectance: 0.02,
        unlit: false,
        emissive: theme::axis_emissive_color(theme),
        // alpha_mode: AlphaMode::Blend,
        ..default()
    });
    let z_axis_material_handle = materials.add(StandardMaterial {
        // base_color: Color::rgba(1.0, 1.0, 1.0, 0.2).into(),
        base_color: theme::axis_base_color(theme),
        metallic: 20.0,
        reflectance: 0.02,
        unlit: false,
        emissive: theme::axis_emissive_color(theme),
        // alpha_mode: AlphaMode::Blend,
        ..default()
    });

    // Meshes
    let axis_handle = meshes.add(
        shape::Capsule {
            radius: 0.03,
            depth: 20.0,
            latitudes: 32,
            longitudes: 64,
            ..default()
        }.into()
    );

    // Spawn into this world
    let y_axis = commands
        .spawn((
            theme::ColorFunction {
                background: theme::axis_base_color,
                border: theme::axis_emissive_color,
            },
            PbrBundle {
                mesh: axis_handle.clone(),
                material: y_axis_material_handle.clone(),
                ..default()
            },
            // SpinnyCube,
            crew_render_layer,
            subsection::SubsectionGameEntity,
        ))
        .id();
    let z_axis = commands
        .spawn((
            theme::ColorFunction {
                background: theme::axis_base_color,
                border: theme::axis_emissive_color,
            },
            PbrBundle {
                mesh: axis_handle.clone(),
                material: z_axis_material_handle.clone(),
                transform: Transform::from_rotation(
                    Quat::from_rotation_x(0.5 * PI)
                ),
                ..default()
            },
            // SpinnyCube,
            crew_render_layer,
            subsection::SubsectionGameEntity,
        ))
        .id();
    let x_axis = commands
        .spawn((
            theme::ColorFunction {
                background: theme::axis_base_color,
                border: theme::axis_emissive_color,
            },
            PbrBundle {
                mesh: axis_handle,
                material: x_axis_material_handle,
                transform: Transform::from_rotation(
                    Quat::from_rotation_z(0.5 * PI)
                ),
                ..default()
            },
            // SpinnyCube,
            crew_render_layer,
            subsection::SubsectionGameEntity,
        ))
        .id();

        commands.entity(film_crew_entity).push_children(&[
            x_axis,
            y_axis,
            z_axis,
        ]);
}

fn spawn_grid(
    commands: &mut Commands,
    theme: &theme::CurrentTheme,
    film_crew_entity: Entity,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    crew_render_layer: RenderLayers,
) {
    let mut tcolor = theme::text_color(theme);

    // Materials
    let grid_material_handle = materials.add(StandardMaterial {
        base_color: theme::grid_base_color(theme),
        metallic: 20.0,
        reflectance: 0.01,
        unlit: false,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    // Meshes
    let grid_line_handle = meshes.add(
        shape::Capsule {
            radius: 0.01,
            depth: 20.0,
            ..default()
        }.into()
    );

    for x in (-10..=10).filter(|&x| x != 0) {
        let offset: f32 = x as f32;

        // Spawn into this world
        let grid_z_line = commands
        .spawn((
            PbrBundle {
                mesh: grid_line_handle.clone(),
                material: grid_material_handle.clone(),
                transform: Transform::from_rotation(
                    Quat::from_rotation_x(0.5 * PI)
                ).with_translation(
                    Vec3 { x: offset, y: 0.0, z: 0.0 }
                ),
                ..default()
            },
            // SpinnyCube,
            crew_render_layer,
            subsection::SubsectionGameEntity,
        ))
        .id();

        let grid_x_line = commands
            .spawn((
                theme::ColorFunction {
                    background: theme::grid_base_color,
                    border: theme::grid_emissive_color,
                },
                PbrBundle {
                    mesh: grid_line_handle.clone(),
                    material: grid_material_handle.clone(),
                    transform: Transform::from_rotation(
                        Quat::from_rotation_z(0.5 * PI)
                    ).with_translation(
                        Vec3 { x: 0.0, y: 0.0, z: offset }
                    ),
                    ..default()
                },
                // SpinnyCube,
                crew_render_layer,
                subsection::SubsectionGameEntity,
            ))
            .id();

        commands.entity(film_crew_entity).push_children(&[
            grid_z_line,
            grid_x_line,
        ]);
    }
    
    

    
}

/// Rotates the inner cube (first pass)
fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<SpinnyCube>>) {
    for mut transform in &mut query {
        transform.rotate_x(1.5 * time.delta_seconds());
        transform.rotate_z(1.3 * time.delta_seconds());
    }
}


#[derive(Component)]
struct Spin(f32);

fn spin(mut square: Query<(&mut Spin, &mut Transform)>) {
    for (mut spin, mut transform) in square.iter_mut() {
        transform.rotation = Quat::from_rotation_z(spin.0);
        let delta = -spin.0.clamp(-1.0, 1.0) * 0.05;
        spin.0 += delta;
    }
}