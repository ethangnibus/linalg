use bevy::{
    prelude::*,
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
};
use super::super::components::{
    text_section,
};
use super::super::subsection_cameras;
use super::super::theme;

pub fn get(
    commands: &mut Commands,
    theme: &theme::CurrentTheme,
    camera_setup_writer: &mut EventWriter<subsection_cameras::CameraSetupEvent>,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    mut images: &mut ResMut<Assets<Image>>,
    view_list_entity: Entity,
) {
    subsection_cameras::setup_camera(commands, theme, Val::Vh(100.0), meshes, materials, images, view_list_entity, 0)
}