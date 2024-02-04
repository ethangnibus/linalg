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

#[derive(Component)]
pub struct ExampleSkeletonCorner {
    pub crew_id: u8,
}

pub fn spawn(
    commands: &mut Commands,
    theme: &theme::CurrentTheme,
    film_crew_entity: Entity,
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
        crew_id,
        format!(" Example {}", crew_id).as_str(),
    );

    subsection_cameras::setup_camera(
        commands,
        theme,
        film_crew_entity,
        Val::Px(400.0),
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
        crew_id,
        format!(" ...").as_str(),
    );
    
    
}


pub fn example_skeleton_color_system(
    mut camera_selection_reader: EventReader<subsection_cameras::CameraSelectionEvent>,
    mut example_skeleton_query: Query<(&mut BorderColor, &mut theme::ColorFunction, &ExampleSkeletonCorner), With<ExampleSkeletonCorner>>,
    theme: Res<theme::CurrentTheme>,
) {
    let theme = theme.as_ref();
    for camera_selection_event in camera_selection_reader.read() {
        for (mut border_color, mut color_function, mut skeleton_corner) in example_skeleton_query.iter_mut() {
            if skeleton_corner.crew_id == camera_selection_event.crew_id {
                *border_color = theme::sidebar_color(theme).into();
                color_function.border = theme::sidebar_color;
            } else {
                *border_color = theme::swiper_background_color(theme).into();
                color_function.border = theme::swiper_background_color;
            }
        }
    }
}
