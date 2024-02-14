

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
        app.add_systems(Update, rotator_system);
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

    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 4.0 }));
    let cube_material_handle = materials.add(StandardMaterial {
        base_color: theme::cube_base_color(theme).into(),
        // emissive: theme::cube_emissive_color(theme).into(),
        // base_color: theme::sidebar_color(theme).into(),
        metallic: 1.0,
        reflectance: 0.1,
        perceptual_roughness: 1.0,
        ..default()
    });

    // The cube that will be rendered to the texture.
    let cube = commands.spawn((
        theme::ColorFunction {
            background: theme::cube_base_color,
            border: theme::cube_base_color,
        },
        PbrBundle {
            mesh: cube_handle,
            material: cube_material_handle,
            // transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        SpinnyCube,
        crew_render_layer,
        subsection::SubsectionGameEntity,
    )).id();
    
    commands.entity(film_crew_entity).push_children(&[cube]);
}


/// Rotates the inner cube (first pass)
fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<SpinnyCube>>) {
    for mut transform in &mut query {
        transform.rotate_x(0.65 * time.delta_seconds());
        transform.rotate_z(0.55 * time.delta_seconds());
        transform.rotate_y(-0.5 * time.delta_seconds());
    }
}