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
use super::super::theme;

pub fn get(commands: &mut Commands, theme: &theme::CurrentTheme, svg_load_writer: &mut EventWriter<SvgLoadEvent>, page_entities: &mut Vec<Entity>) {
    page_entities.push(
        text_section::camera(commands, theme, svg_load_writer, &"3.png".into(), 5.5, Val::Vh(100.0))
    );
}