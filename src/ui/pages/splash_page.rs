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
use super::super::subsection_cameras::CameraSetupEvent;
use super::super::theme;

pub fn get(
    commands: &mut Commands,
    theme: &theme::CurrentTheme,
    camera_setup_writer: &mut EventWriter<CameraSetupEvent>,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    mut images: &mut ResMut<Assets<Image>>,
    view_list_entity: Entity,
) {
    text_section::camera(commands, theme, camera_setup_writer, &"3.png".into(), 5.5, Val::Vh(100.0), meshes, materials, images, view_list_entity)
}