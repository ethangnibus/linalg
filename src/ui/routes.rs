use bevy::prelude::*;
use super::util::SubsectionGameEntity;
use super::view::ViewList;
use super::subsection_cameras::SvgLoadEvent;
use super::pages::*;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RoutingEvent>()
            .add_systems(Update, routing_system)
            .insert_resource(CurrentRoute {
                chapter_number: 0,
                section_number: 0,
                subsection_number: 0,
            });
    }
}

#[derive(Resource)]
pub struct CurrentRoute {
    pub chapter_number: u32,
    pub section_number: u32,
    pub subsection_number: u32,
}

#[derive(Event)]
pub struct RoutingEvent {
    pub chapter_number: u32,
    pub section_number: u32,
    pub subsection_number: u32,
}




fn routing_system(
    mut commands: Commands,
    view_list_query: Query<(Entity, &Children), With<ViewList>>,
    subsection_game_entity_query: Query<Entity, With<SubsectionGameEntity>>,
    mut svg_load_writer: EventWriter<SvgLoadEvent>,
    mut routing_event_reader: EventReader<RoutingEvent>,
    asset_server: Res<AssetServer>,
    mut current_route: ResMut<CurrentRoute>,
) {
    for event in routing_event_reader.read() {
        for (view_list, view_list_children) in view_list_query.iter() {
            current_route.chapter_number = event.chapter_number;
            current_route.section_number = event.section_number;
            current_route.subsection_number = event.subsection_number;

            // remove all current page ui
            for &child in view_list_children.iter() {
                commands.entity(child).despawn_recursive();
                commands.entity(view_list).remove_children(&[child]);
                
            }

            // remove all current objects from this subsection
            // lights, cubes, etc.
            for game_entity in subsection_game_entity_query.iter() {
                commands.entity(game_entity).despawn();
            }

            // Fixme: remember to also remove meshes!!!


            // let mut page_item: Entity = Entity::PLACEHOLDER;
            let mut page_entities: Vec<Entity> = Vec::new();
            // Add new page stuff


            match event.chapter_number {
                0 => {
                    match event.section_number {
                        0 => { // Chapter 0, Section 0
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 0, Section 1
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 0, Section 2
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 0, Section 3
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 0, Section 4
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                1 => {
                    match event.section_number {
                        0 => { // Chapter 1, Section 0
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        1 => { // Chapter 1, Section 1
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                // 1 => {page_item = chapter1section1subsection1::get_page(&mut commands);},
                                // 2 => {page_item = chapter1section1subsection2::get_page(&mut commands);},
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 1, Section 2
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {chapter1section2subsection4::get(&mut commands, &asset_server, &mut svg_load_writer, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 1, Section 3
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 1, Section 4
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                2 => {
                    match event.section_number {
                        0 => { // Chapter 2, Section 0
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 2, Section 1
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 2, Section 2
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 2, Section 3
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 2, Section 4
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                3 => {
                    match event.section_number {
                        0 => { // Chapter 3, Section 0
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 3, Section 1
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 3, Section 2
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 3, Section 3
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 3, Section 4
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                4 => {
                    match event.section_number {
                        0 => { // Chapter 4, Section 0
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 4, Section 1
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 4, Section 2
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 4, Section 3
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 4, Section 4
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                5 => {
                    match event.section_number {
                        0 => { // Chapter 5, Section 0
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 5, Section 1
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 5, Section 2
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 5, Section 3
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 5, Section 4
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                6 => {
                    match event.section_number {
                        0 => { // Chapter 6, Section 0
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 6, Section 1
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 6, Section 2
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 6, Section 3
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 6, Section 4
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                7 => { // Chapter 7, Section 0
                    match event.section_number {
                        0 => {
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 7, Section 1
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 7, Section 2
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 7, Section 3
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 7, Section 4
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                8 => {
                    match event.section_number {
                        0 => { // Chapter 8, Section 0
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 8, Section 1
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 8, Section 2
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 8, Section 3
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 8, Section 4 
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                9 => {
                    match event.section_number {
                        0 => { // Chapter 9, Section 0
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 9, Section 1
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 9, Section 2
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 9, Section 3
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 9, Section 4
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                10 => {
                    match event.section_number {
                        0 => { // Chapter 10, Section 0
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 10, Section 1
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 10, Section 2
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 10, Section 3
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 10, Section 4 
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                11 => {
                    match event.section_number {
                        0 => { // Chapter 11, Section 0
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        1 => { // Chapter 11, Section 1
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 11, Section 2
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 11, Section 3
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 11, Section 4
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                12 => {
                    match event.section_number {
                        0 => { // Chapter 12, Section 0
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 12, Section 1
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 12, Section 2
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 12, Section 3
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 12, Section 4
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                13 => {
                    match event.section_number {
                        0 => { // Chapter 13, Section 0
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 13, Section 1
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 13, Section 2
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 13, Section 3
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 13, Section 4
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                14 => {
                    match event.section_number {
                        0 => { // Chapter 14, Section 0
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 14, Section 1
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 14, Section 2
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 14, Section 3
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 14, Section 4
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                15 => {
                    match event.section_number {
                        0 => { // Chapter 15, Section 0 Bibliography
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 15, Section 1
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        2 => { // Chapter 15, Section 2
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        3 => { // Chapter 15, Section 3
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                            }
                        },
                        4 => { // Chapter 15, Section 4
                            match event.subsection_number {
                                0 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                1 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                2 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                3 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                4 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                5 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                6 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                7 => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
                                _ => {page_not_found::get(&mut commands, &mut page_entities)}, // FIXME
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