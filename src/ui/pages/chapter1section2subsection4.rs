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
    definition_block,
    definition_text_section,
    span_of_vectors_renderer,
    example_header,
    solution_header,
};
use super::super::subsection_cameras::CameraSetupEvent;
use super::super::theme;



pub fn get(commands: &mut Commands, theme: &theme::CurrentTheme, asset_server: & Res<AssetServer>, camera_setup_event: &mut EventWriter<CameraSetupEvent>, page_entities: &mut Vec<Entity>) {
    page_entities.push(
        page_header::spawn(commands, "1.2.4 Spans")
    );

    page_entities.push(
        text_section::image(commands, asset_server, camera_setup_event, "1_black.png".into(), 8.0)
    );

    let span_of_vectors_left = definition_text_section::spawn(commands, "Given a collection of vectors v1, . . . , vk ∈ Rn, their span\nSpan {v1,...,vk} ⊂ Rn\nis the set of all their linear combinations. In other words, Span {v1,...,vk} consists of all v ∈ Rn that can be expressed in the form\nv=a1v1 +···+akvk\nfor some weights a1,...,ak ∈ R.\nGeometrically, the span of a collection of vectors is the set of all vectors that can be reached by trav- eling along scales of each of the individual vectors in turn.");
    let span_of_vectors_right = span_of_vectors_renderer::spawn(commands);
    page_entities.push(
        definition_block::spawn(commands, "Span of vectors", span_of_vectors_left, span_of_vectors_right)
    );

    page_entities.push(
        text_section::camera(commands, theme, camera_setup_event, &"3.png".into(), 5.5, Val::Px(500.0))
    );

    page_entities.push(
        example_header::spawn(commands, "Example 16")
    );

    page_entities.push(
        text_section::spawn(commands, "Consider the vectors
        1 −1 v1=1,v2= 2
        Give a geometric description of their span Span {v1,v2}.")
    );

    page_entities.push(
        solution_header::spawn(commands, "SOLUTION")
    );

    page_entities.push(
        text_section::spawn(commands, "Let’s think about the set inside R2 of all linear
        combinations of our vectors 1 −1
        a1v1+a2v2=a1 1 +a2 2 foranya1,a2∈R
        To understand what this set looks like, we will build up the linear combinations
        in several steps.
        • Setting a2 = 0, we get all scales of the first vector 1
        a1v1 =a1 1 foranya1 ∈R
        This gives Span {v1}, the straight line through the origin containing v1.
        • Similarly, setting a1 = 0, we get all scales of the second vector −1
        a2v2 =a2 2 foranya2 ∈R
        This gives Span {v2}, the straight line through the origin containing v2.
        • Finally observe that Span {v1,v2} consists of all vectors formed by adding
        vectors along these
        two straight lines. Since any vector in the plane can be reached in this way,
        we conclude that Span {v1, v2} = R2")
    );

    // page_entities.push(
    //     definition_body::spawn(commands, "Span of vectors", content)
    // );

    page_entities.push(
        page_header::spawn(commands, "1.2.4 Spans")
    );
    page_entities.push(
        page_header::spawn(commands, "1.2.4 Spans")
    );
    page_entities.push(
        page_header::spawn(commands, "1.2.4 Spans")
    );
    page_entities.push(
        page_header::spawn(commands, "1.2.4 Spans")
    );
    page_entities.push(
        page_header::spawn(commands, "1.2.4 Spans")
    );
    page_entities.push(
        page_header::spawn(commands, "1.2.4 Spans")
    );
    page_entities.push(
        page_header::spawn(commands, "1.2.4 Spans")
    );
    page_entities.push(
        page_header::spawn(commands, "1.2.4 Spans")
    );
    page_entities.push(
        page_header::spawn(commands, "1.2.4 Spans")
    );
    page_entities.push(
        page_header::spawn(commands, "1.2.4 Spans")
    );
    page_entities.push(
        page_header::spawn(commands, "1.2.4 Spans")
    );
    page_entities.push(
        page_header::spawn(commands, "1.2.4 Spans")
    );
    page_entities.push(
        page_header::spawn(commands, "1.2.4 Spans")
    );
    page_entities.push(
        page_header::spawn(commands, "1.2.4 Spans")
    );
    page_entities.push(
        page_header::spawn(commands, "1.2.4 Spans")
    );
    page_entities.push(
        page_header::spawn(commands, "1.2.4 Spans")
    );
    page_entities.push(
        page_header::spawn(commands, "1.2.4 Spans")
    );
    page_entities.push(
        page_header::spawn(commands, "1.2.4 Spans")
    );page_entities.push(
        page_header::spawn(commands, "1.2.4 Spans")
    );
    
    
}

// add a system to look at all svg_text blocks above
// make a thing called an svg_text block that will hold the math text svg's we use for
// everything
// in the update system, when this page is loaded, send an event to load the svgs and put them in
// the svg blocks.
// we know the page is loaded when the title block appears. Query the title block and get
// the resources based on it's name (or make a struct that tells us which chapter this is)
// use that name to get the svgs.
