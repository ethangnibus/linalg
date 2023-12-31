use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    // winit::WinitSettings,
};
use super::scrollable_page;
use super::pages::*;

// Marker for UI node
#[derive(Component)]
pub struct View;

#[derive(Component, Default)]
struct ViewList {
    position: f32,
}

#[derive(Event)]
pub struct RoutingEvent {
    pub chapter_number: u32,
    pub section_number: u32,
    pub subsection_number: u32,
}

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<RoutingEvent>()
        .add_systems(Update, (mouse_scroll, routing_system));
    }
}

pub fn setup(commands: &mut Commands) -> Entity {
    let view = new();
    let view = commands.spawn(view).id();
    
    let page_items = page_items(commands);
    let view_list = scrollable_page::get_page();
    let view_list = commands.spawn((ViewList::default(), view_list)).id();
    
    commands.entity(view_list).push_children(&page_items);
    commands.entity(view).push_children(&[view_list]);

    return view;
}

pub fn new() -> (View, ButtonBundle) {
    return (View, ButtonBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_self: AlignSelf::Stretch,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            overflow: Overflow::clip_y(),
            ..default()
        },
        background_color: Color::rgb(0.0, 1.0, 0.0).into(),
        ..default()
    });
}

fn routing_system(
    mut commands: Commands,
    view_list_query: Query<(Entity, &Children), With<ViewList>>,
    mut routing_event_reader: EventReader<RoutingEvent>,
) {
    for event in routing_event_reader.read() {
        for (view_list, view_list_children) in view_list_query.iter() {
            // remove all current page stuff
            for &child in view_list_children.iter() {
                commands.entity(view_list).remove_children(&[child]);
                commands.entity(child).despawn_recursive();
            }

            // let mut page_item: Entity = Entity::PLACEHOLDER;
            let mut page_entities: Vec<Entity> = Vec::new();
            // Add new page stuff

            match event.chapter_number {
                0 => {
                    match event.section_number {
                        0 => { // Chapter 0, Section 0
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 0, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 0, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 0, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 0, Section 4
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                1 => {
                    match event.section_number {
                        0 => { // Chapter 1, Section 0
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        1 => { // Chapter 1, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                // 1 => {page_item = chapter1section1subsection1::get_page(&mut commands);},
                                // 2 => {page_item = chapter1section1subsection2::get_page(&mut commands);},
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 1, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {chapter1section2subsection4::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 1, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 1, Section 4
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                2 => {
                    match event.section_number {
                        0 => { // Chapter 2, Section 0
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 2, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 2, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 2, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 2, Section 4
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                3 => {
                    match event.section_number {
                        0 => { // Chapter 3, Section 0
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 3, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 3, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 3, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 3, Section 4
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                4 => {
                    match event.section_number {
                        0 => { // Chapter 4, Section 0
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 4, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 4, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 4, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 4, Section 4
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                5 => {
                    match event.section_number {
                        0 => { // Chapter 5, Section 0
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 5, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 5, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 5, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 5, Section 4
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                6 => {
                    match event.section_number {
                        0 => { // Chapter 6, Section 0
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 6, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 6, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 6, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 6, Section 4
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                7 => { // Chapter 7, Section 0
                    match event.section_number {
                        0 => {
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 7, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 7, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 7, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 7, Section 4
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                8 => {
                    match event.section_number {
                        0 => { // Chapter 8, Section 0
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 8, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 8, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 8, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 8, Section 4 
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                9 => {
                    match event.section_number {
                        0 => { // Chapter 9, Section 0
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 9, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 9, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 9, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 9, Section 4
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                10 => {
                    match event.section_number {
                        0 => { // Chapter 10, Section 0
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 10, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 10, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 10, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 10, Section 4 
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                11 => {
                    match event.section_number {
                        0 => { // Chapter 11, Section 0
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        1 => { // Chapter 11, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 11, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 11, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 11, Section 4
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                12 => {
                    match event.section_number {
                        0 => { // Chapter 12, Section 0
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 12, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 12, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 12, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 12, Section 4
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                13 => {
                    match event.section_number {
                        0 => { // Chapter 13, Section 0
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 13, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 13, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 13, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 13, Section 4
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                14 => {
                    match event.section_number {
                        0 => { // Chapter 14, Section 0
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 14, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 14, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 14, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 14, Section 4
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                15 => {
                    match event.section_number {
                        0 => { // Chapter 15, Section 0 Bibliography
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 15, Section 1
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 15, Section 2
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 15, Section 3
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 15, Section 4
                            match event.subsection_number {
                                0 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }

            for entity in page_entities {
                commands.entity(view_list).push_children(&[entity]);
            }
        }
    }
    // add new page
}

pub fn page_items(commands: &mut Commands) -> Vec<Entity> {
    let mut page_items = Vec::new();
    for i in 0..3 {
        let text_item = (
            TextBundle::from_section(
                format!("Page Item: {i}"),
                TextStyle {
                    font_size: 20.,
                    ..default()
                },
            ),
            Label,
            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
        );
        let page_item = NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(200.0),
                padding: UiRect {
                    left: Val::Px(2.0),
                    right: Val::Px(4.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(4.0),
                },
                ..default()
            },
            background_color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..default()
        };

        let inner_item = NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                // justify_items: JustifyItems::Center,
                ..default()
            },
            background_color: Color::rgb(0.4, 0.4, 0.4).into(),
            ..default()
        };

        let text_item = commands.spawn(text_item).id();
        let inner_item = commands.spawn(inner_item).id();
        let page_item = commands.spawn(page_item).id();

        commands.entity(inner_item).push_children(&[text_item]);
        commands.entity(page_item).push_children(&[inner_item]);

        page_items.push(page_item);
    }
    return page_items;
}

fn page_not_found(commands: &mut Commands, page_entities: &mut Vec<Entity>) {
    let text_item = (
        TextBundle::from_section(
            format!("TODO: Remember to implement this page!"),
            TextStyle {
                font_size: 20.,
                ..default()
            },
        ),
        Label,
        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
    );
    let page_item = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Px(200.0),
            padding: UiRect {
                left: Val::Px(2.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(4.0),
            },
            border: UiRect {
                left: Val::Px(2.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(4.0),
            },
            ..default()
        },
        background_color: Color::rgb(0.3, 0.1, 0.1).into(),
        border_color: Color::rgb(0.1, 0.1, 0.1).into(),

        ..default()
    };

    let text_item = commands.spawn(text_item).id();
    let page_item = commands.spawn(page_item).id();

    commands.entity(page_item).push_children(&[text_item]);

    page_entities.push(page_item);
}

fn mouse_scroll(
    mut interaction_query: Query<
        &Interaction,
        With<View>
    >,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ViewList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                for mouse_wheel_event in mouse_wheel_events.read() {
                    for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
                        let items_height = list_node.size().y;
                        let container_height = query_node.get(parent.get()).unwrap().size().y;
            
                        let max_scroll = (items_height - container_height).max(0.);
            
                        let dy = match mouse_wheel_event.unit {
                            MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                            MouseScrollUnit::Pixel => mouse_wheel_event.y,
                        };
            
                        scrolling_list.position += dy;
                        scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
                        style.top = Val::Px(scrolling_list.position);
                    }
                }
            }
            _ => {}
        }
    }
}
