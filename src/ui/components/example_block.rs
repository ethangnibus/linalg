use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    // winit::WinitSettings,
};

use crate::ui::util::theme;
use super::super::subsection_cameras;
use super::example_header;

pub fn spawn(
    commands: &mut Commands,
    theme: &theme::CurrentTheme,
    camera_setup_writer: &mut EventWriter<subsection_cameras::CameraSetupEvent>,
    image_path: &String,
    ratio: f32,
    height: Val,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    mut images: &mut ResMut<Assets<Image>>,
    view_list_entity: Entity,
    crew_id: u8,
) {
    example_header::spawn(commands, theme, view_list_entity, "WEE MADE IT");
    
    
}