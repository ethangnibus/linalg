use super::super::super::super::super::components::example_header;
use super::super::super::super::super::option_bar;
use super::super::super::super::super::routes;
use super::super::super::super::super::theme;
use super::super::super::super::super::under_navbar;
use super::super::super::super::super::util::style;
use super::super::super::super::super::util::subsection;
use super::super::super::super::super::subsection_cameras;
use super::super::super::super::super::util::subsection::SubsectionGameEntity;
use super::super::super::super::super::{routes::RoutingEvent, view::UiResizeEvent};

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    ecs::event::ManualEventReader,
    prelude::*,
    render::{
        camera::{
            self, CameraProjection, ComputedCameraValues, RenderTarget, ScalingMode, Viewport,
        },
        primitives::Frustum,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::{RenderLayers, VisibleEntities},
    },
    ui,
};

pub fn setup_scene(
    commands: &mut Commands,
    theme: &theme::CurrentTheme,
    height: Val,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    mut images: &mut ResMut<Assets<Image>>,
    view_list_entity: Entity,
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
            subsection_cameras::FirstPassCube,
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

    // let film_crew = commands
    //     .spawn((
    //         FilmCrew {
    //             crew_id: crew_id,
    //             mesh_entity: cube,
    //             light_entity: light,
    //         },
    //         SpatialBundle { ..default() },
    //     ))
    //     .id();
    
    // commands
    //     .entity(film_crew)
    //     .push_children(&[cube, light]);
}


/// Rotates the inner cube (first pass)
fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<FirstPassCube>>) {
    for mut transform in &mut query {
        transform.rotate_x(1.5 * time.delta_seconds());
        transform.rotate_z(1.3 * time.delta_seconds());
    }
}