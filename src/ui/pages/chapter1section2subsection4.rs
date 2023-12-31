use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    // winit::WinitSettings,
};

use super::super::components::{
    page_header,
    text_section,
};

pub fn get(commands: &mut Commands, page_entities: &mut Vec<Entity>) {
    page_entities.push(
        page_header::spawn(commands, "1.2.4 Spans")
    );

    page_entities.push(
        text_section::spawn(commands, 
"
With the notion of linear combinations in hand, we now arrive at a natural question: Given a collection of real n-vectors, what does the set of all linear combinations of the collection look like?
"
 
 
    )
    )
    
    
}