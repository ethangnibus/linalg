
use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;
use bevy::ui::FocusPolicy;
use bevy_mod_picking::prelude::*;

use bevy::{
    render::view::RenderLayers,
    prelude::*,
};

use crate::ui::util::examples_3d;
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

    let cube_handle = meshes.add(Mesh::from(
        shape::UVSphere {
            radius: 1.0,
            ..default()
        }
    ));
    let cube_material_handle = materials.add(StandardMaterial {
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
    let cube = commands
        .spawn((
            theme::ColorFunction {
                background: theme::cube_base_color,
                border: theme::cube_emissive_color,
            },
            PbrBundle {
                mesh: cube_handle,
                material: cube_material_handle,
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..default()
            },
            theme::HIGHLIGHT_TINT,  
            // FocusPolicy::Block,
            // SpinnyCube,
            crew_render_layer,
            subsection::SubsectionGameEntity,

            //pickable stuff
            PickableBundle::default(), // <- Makes the mesh pickable.
            On::<Pointer<DragStart>>::target_insert(Pickable::IGNORE), // Disable picking
            On::<Pointer<DragEnd>>::target_insert(Pickable::default()), // Re-enable picking
            On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
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