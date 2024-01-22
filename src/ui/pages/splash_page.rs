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
use super::super::subsection_cameras::SvgLoadEvent;

pub fn get(commands: &mut Commands, svg_load_writer: &mut EventWriter<SvgLoadEvent>, page_entities: &mut Vec<Entity>) {
    page_entities.push(
        text_section::camera(commands, svg_load_writer, &"3.png".into(), 5.5)
    );
}