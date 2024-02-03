

use bevy::{
    render::view::RenderLayers,
    prelude::*,
};

use crate::ui::{
    theme,
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
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    crew_id: u8,
) {
    let crew_render_layer = RenderLayers::layer(crew_id);

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

    // The cube that will be rendered to the texture.
    let cube = commands
        .spawn((
            PbrBundle {
                mesh: cube_handle,
                material: cube_material_handle,
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                ..default()
            },
            SpinnyCube,
            crew_render_layer,
            subsection::SubsectionGameEntity,
        ))
        .id();

    // Light
    // NOTE: Currently lights are shared between passes - see https://github.com/bevyengine/bevy/issues/3462
    let light = commands
        .spawn((
            PointLightBundle {
                point_light: PointLight {
                    intensity: 100.0,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
                ..default()
            },
            subsection::SubsectionGameEntity,
        ))
        .id();
}


/// Rotates the inner cube (first pass)
fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<SpinnyCube>>) {
    for mut transform in &mut query {
        transform.rotate_x(1.5 * time.delta_seconds());
        transform.rotate_z(1.3 * time.delta_seconds());
    }
}