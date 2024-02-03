use bevy::{
    prelude::*,
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
};
use crate::ui::components::text_section;
use crate::ui::subsection_cameras;
use crate::ui::util::theme;
use super::example_1;

pub fn get(
    commands: &mut Commands,
    theme: &theme::CurrentTheme,
    film_crew_entity: Entity,
    camera_setup_writer: &mut EventWriter<subsection_cameras::CameraSetupEvent>,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    mut images: &mut ResMut<Assets<Image>>,
    view_list_entity: Entity,
) {


    subsection_cameras::setup_camera(commands, theme, film_crew_entity, Val::Vh(100.0), meshes, materials, images, view_list_entity, 1);
    example_1::setup_scene(
        commands,
        film_crew_entity,
        meshes,
        materials,
        1,
    );
}