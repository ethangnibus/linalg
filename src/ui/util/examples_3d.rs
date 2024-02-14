use std::f32::consts::PI;

use bevy::{
    render::view::RenderLayers,
    prelude::*,
};

use super::{subsection, theme};

pub fn spawn_axis(
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
        // metallic: 20.0,
        // reflectance: 0.02,
        unlit: true,
        // emissive: theme::axis_emissive_color(theme),
        // alpha_mode: AlphaMode::Blend,
        
        ..default()
    });
    let y_axis_material_handle = materials.add(StandardMaterial {
        // base_color: Color::rgba(1.0, 1.0, 1.0, 0.2).into(),
        base_color: theme::axis_base_color(theme),
        // metallic: 20.0,
        // reflectance: 0.02,
        unlit: true,
        // emissive: theme::axis_emissive_color(theme),
        // alpha_mode: AlphaMode::Blend,
        ..default()
    });
    let z_axis_material_handle = materials.add(StandardMaterial {
        // base_color: Color::rgba(1.0, 1.0, 1.0, 0.2).into(),
        base_color: theme::axis_base_color(theme),
        // metallic: 20.0,
        // reflectance: 0.02,
        unlit: true,
        // emissive: theme::axis_emissive_color(theme),
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


pub fn spawn_grid(
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
        // metallic: 20.0,
        // reflectance: 0.01,
        unlit: true,
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
