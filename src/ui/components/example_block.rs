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
use super::example_footer;

pub fn spawn(
    commands: &mut Commands,
    theme: &theme::CurrentTheme,
    height: Val,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    mut images: &mut ResMut<Assets<Image>>,
    view_list_entity: Entity,
    crew_id: u8,
) {
    example_header::spawn(
        commands,
        theme,
        view_list_entity,
        format!(" Example {}", crew_id).as_str(),
    );

    subsection_cameras::setup_camera(
        commands,
        theme,
        Val::Px(500.0),
        meshes,
        materials,
        images,
        view_list_entity,
        crew_id,
    );

    example_footer::spawn(
        commands,
        theme,
        view_list_entity,
        format!(" ...").as_str(),
    );
    
    
}