use bevy::prelude::*;
use super::util::subsection::SubsectionGameEntity;
use super::util::theme;
use super::view;
use super::subsection_cameras;
use crate::pages;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RoutingEvent>()
            .add_systems(Update, routing_system.after(subsection_cameras::resize_camera_system))
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




pub fn routing_system(
    mut commands: Commands,
    mut view_list_query: Query<(Entity, &Children, &mut view::ViewList, &mut Style), With<view::ViewList>>,
    subsection_game_entity_query: Query<Entity, With<SubsectionGameEntity>>,
    mut camera_query: Query<Entity, With<subsection_cameras::MiniCamera>>, // replaced with film crew query
    mut film_crew_query: Query<Entity, With<subsection_cameras::FilmCrew>>,
    mut camera_setup_writer: EventWriter<subsection_cameras::CameraSetupEvent>,
    mut routing_event_reader: EventReader<RoutingEvent>,
    mut ui_resize_writer: EventWriter<view::UiResizeEvent>,
    asset_server: Res<AssetServer>,
    mut current_route: ResMut<CurrentRoute>,
    theme: Res<theme::CurrentTheme>,
    view_query: Query<&Node, With<view::View>>,


    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    for event in routing_event_reader.read() {
        

        for (view_list, view_list_children, mut scrolling_list, mut style) in view_list_query.iter_mut() {
            // Scroll back to the top of the view
            scrolling_list.position = 0.0;
            style.top = Val::Px(scrolling_list.position);


            current_route.chapter_number = event.chapter_number;
            current_route.section_number = event.section_number;
            current_route.subsection_number = event.subsection_number;

            for view_node in view_query.iter() {
                let size = view_node.size();
                // println!("\nView size before: {:?}\n", size);
            }

            // for camera3d in camera_query.iter_mut() {
            //     commands.entity(camera3d).despawn_recursive();
            // }
            for film_crew_entity in film_crew_query.iter_mut() {
                // println!("Despawning film crew here!");
                commands.entity(film_crew_entity).despawn_recursive();
            }
            
            // println!("entities removed:");
            // remove all current page ui
            // for &child in view_list_children.iter() {
                
            //     println!("{:?}", child);
            //     // commands.entity(child).despawn_descendants();
            //     commands.entity(child).despawn_recursive();
                
            //     // commands.entity(child).despawn_descendants();
            //     commands.entity(view_list).remove_children(&[child]);
                
            // }
            commands.entity(view_list).despawn_descendants();
            
            // for film_crew in film_crew_query.iter() {
            //     commands.entity(film_crew).despawn_recursive();
            // }

            // // remove all current objects from this subsection
            // // lights, cubes, etc.
            // for game_entity in subsection_game_entity_query.iter() {
            //     commands.entity(game_entity).despawn();
            // }

            // Fixme: remember to also remove meshes!!!

            // let mut page_item: Entity = Entity::PLACEHOLDER;
            // let mut page_entities: Vec<Entity> = Vec::new();
            // Add new page stuff
            let film_crew_entity = commands.spawn((
                SpatialBundle {
                    visibility: Visibility::Inherited,
                    ..default()
                },
                subsection_cameras::FilmCrew
            )).id();


            match event.chapter_number {
                0 => {
                    match event.section_number {
                        0 => { // Chapter 0, Section 0
                            match event.subsection_number {
                                0 => {pages::chapter_0::section_0::subsection_0::partial::get(&mut commands, &theme, film_crew_entity, &mut camera_setup_writer, &mut meshes, &mut materials, &mut images, view_list)}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 0, Section 1
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        2 => { // Chapter 0, Section 2
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        3 => { // Chapter 0, Section 3
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        4 => { // Chapter 0, Section 4
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                1 => {
                    match event.section_number {
                        0 => { // Chapter 1, Section 0
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        1 => { // Chapter 1, Section 1
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                // 1 => {page_item = chapter1section1subsection1::get_page(&mut commands);},
                                // 2 => {page_item = chapter1section1subsection2::get_page(&mut commands);},
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        2 => { // Chapter 1, Section 2
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::chapter_1::section_2::subsection_4::partial::get(&mut commands, &theme, film_crew_entity, &asset_server, &mut camera_setup_writer, &mut meshes, &mut materials, &mut images, view_list)}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        3 => { // Chapter 1, Section 3
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        4 => { // Chapter 1, Section 4
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                2 => {
                    match event.section_number {
                        0 => { // Chapter 2, Section 0
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 2, Section 1
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        2 => { // Chapter 2, Section 2
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        3 => { // Chapter 2, Section 3
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        4 => { // Chapter 2, Section 4
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                3 => {
                    match event.section_number {
                        0 => { // Chapter 3, Section 0
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 3, Section 1
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        2 => { // Chapter 3, Section 2
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        3 => { // Chapter 3, Section 3
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        4 => { // Chapter 3, Section 4
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                4 => {
                    match event.section_number {
                        0 => { // Chapter 4, Section 0
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 4, Section 1
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        2 => { // Chapter 4, Section 2
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        3 => { // Chapter 4, Section 3
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        4 => { // Chapter 4, Section 4
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                5 => {
                    match event.section_number {
                        0 => { // Chapter 5, Section 0
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 5, Section 1
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        2 => { // Chapter 5, Section 2
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        3 => { // Chapter 5, Section 3
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        4 => { // Chapter 5, Section 4
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                6 => {
                    match event.section_number {
                        0 => { // Chapter 6, Section 0
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 6, Section 1
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        2 => { // Chapter 6, Section 2
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        3 => { // Chapter 6, Section 3
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        4 => { // Chapter 6, Section 4
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                7 => { // Chapter 7, Section 0
                    match event.section_number {
                        0 => {
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 7, Section 1
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        2 => { // Chapter 7, Section 2
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        3 => { // Chapter 7, Section 3
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        4 => { // Chapter 7, Section 4
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                8 => {
                    match event.section_number {
                        0 => { // Chapter 8, Section 0
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 8, Section 1
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        2 => { // Chapter 8, Section 2
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        3 => { // Chapter 8, Section 3
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        4 => { // Chapter 8, Section 4 
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                9 => {
                    match event.section_number {
                        0 => { // Chapter 9, Section 0
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 9, Section 1
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        2 => { // Chapter 9, Section 2
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        3 => { // Chapter 9, Section 3
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        4 => { // Chapter 9, Section 4
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                10 => {
                    match event.section_number {
                        0 => { // Chapter 10, Section 0
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 10, Section 1
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        2 => { // Chapter 10, Section 2
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        3 => { // Chapter 10, Section 3
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        4 => { // Chapter 10, Section 4 
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                11 => {
                    match event.section_number {
                        0 => { // Chapter 11, Section 0
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        1 => { // Chapter 11, Section 1
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        2 => { // Chapter 11, Section 2
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        3 => { // Chapter 11, Section 3
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        4 => { // Chapter 11, Section 4
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                12 => {
                    match event.section_number {
                        0 => { // Chapter 12, Section 0
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 12, Section 1
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        2 => { // Chapter 12, Section 2
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        3 => { // Chapter 12, Section 3
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        4 => { // Chapter 12, Section 4
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                13 => {
                    match event.section_number {
                        0 => { // Chapter 13, Section 0
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 13, Section 1
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        2 => { // Chapter 13, Section 2
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        3 => { // Chapter 13, Section 3
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        4 => { // Chapter 13, Section 4
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                14 => {
                    match event.section_number {
                        0 => { // Chapter 14, Section 0
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 14, Section 1
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        2 => { // Chapter 14, Section 2
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        3 => { // Chapter 14, Section 3
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        4 => { // Chapter 14, Section 4
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                15 => {
                    match event.section_number {
                        0 => { // Chapter 15, Section 0 Bibliography
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                    
                        },
                        1 => { // Chapter 15, Section 1
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        2 => { // Chapter 15, Section 2
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        3 => { // Chapter 15, Section 3
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        4 => { // Chapter 15, Section 4
                            match event.subsection_number {
                                0 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                1 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                2 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                3 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                4 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                5 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                6 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                7 => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                                _ => {pages::page_not_found::get(&mut commands, view_list )}, // FIXME
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }

            // for entity in page_entities {
            //     commands.entity(view_list).push_children(&[entity]);
            // }
            
        }
        for view_node in view_query.iter() {
            let size = view_node.size();
            // println!("\nView size after: {:?}\n", size);
        }
        ui_resize_writer.send(view::UiResizeEvent);
        // camera_setup_writer.send(subsection_cameras::CameraSetupEvent);
    }
    // add new page
}