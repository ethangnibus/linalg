use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    // winit::WinitSettings,
};
use super::sidebar;

#[derive(Component, Copy, Clone)]
pub struct ChapterNumber(pub u32);

#[derive(Component)]
pub struct SectionNumber(pub u32);

#[derive(Component)]
pub struct ChapterContainer();

#[derive(Component)]
pub struct ChapterButton();

#[derive(Component)]
pub struct SectionsContainer;

#[derive(Event, Debug)]
pub struct SidebarSwiperColorEvent(pub Color);

#[derive(Event)]
pub struct SectionsContainerVisibilityEvent(pub Visibility);

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SectionsContainerVisibilityEvent>()
            .add_systems(Update, (
                chapter_button_mouse_scroll,
                sections_container_visibility_system,
                chapter_container_mouse_scroll,
                sections_container_mouse_scroll
            ));
    }
}

pub fn setup(commands: &mut Commands, chapter_name: String, chapter_number: u32) -> Entity {
    let chapter_number = ChapterNumber(chapter_number);
    let chapter_container = (
        ChapterContainer(),
        chapter_number,
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect {
                    left: Val::Px(4.0),
                    right: Val::Px(4.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                },
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..default()
        }
    );
    let sections_container = (
        SectionsContainer,
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect {
                    left: Val::Px(4.0),
                    right: Val::Px(0.0),
                    top: Val::Px(4.0),
                    bottom: Val::Px(0.0),
                },
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..default()
        }
    );

    let section_button  = (
        SectionNumber(0),
        ChapterButton(),
        chapter_number,
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(300.0),
                // flex_direction: FlexDirection::Column,
                // align_items: AlignItems::Center,
                // justify_items: JustifyItems::Center,
                ..default()
            },
            background_color: Color::rgb(0.5, 0.5, 0.5).into(),
            ..default()
        }
    );

    let chapter_container = commands.spawn(chapter_container).id();
    let sections_container = commands.spawn(sections_container).id();
    let chapter_button = chapter_button(commands, chapter_name, chapter_number);
    let section_button = commands.spawn(section_button).id();

    commands.entity(sections_container).push_children(&[section_button]);
    commands.entity(chapter_container).push_children(&[chapter_button, sections_container]);
    return chapter_container;
}



pub fn chapter_button(commands: &mut Commands, chapter_name: String, chapter_number: ChapterNumber) -> Entity {
    println!("spawning chapter button");
    let chapter_button  = (
        ChapterButton(),
        chapter_number,
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                align_items: AlignItems::Center,
                justify_items: JustifyItems::Center,
                ..default()
            },
            background_color: Color::rgb(0.5, 0.5, 0.5).into(),
            ..default()
        }
    );
    let text_item = (
        TextBundle::from_section(
            chapter_name,
            TextStyle {
                font_size: 20.,
                ..default()
            },
        ),
        Label,
        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
    );

    let chapter_button = commands.spawn(chapter_button).id();
    let text_item = commands.spawn(text_item).id();

    commands.entity(chapter_button).push_children(&[text_item]);
    return chapter_button;
}

fn chapter_button_mouse_scroll(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        With<ChapterButton>
    >,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut sidebar::SidebarList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
    mut sections_container_visibility_writer: EventWriter<SectionsContainerVisibilityEvent>,
) {
    for (interaction, mut chapter_button_background_color) in &mut interaction_query {
        
        match *interaction {
            Interaction::Pressed => {
                println!("  pressed");
                *chapter_button_background_color = Color::rgb(0.6, 0.6, 0.9).into();
                sections_container_visibility_writer.send(SectionsContainerVisibilityEvent(Visibility::Hidden));
            }
            Interaction::Hovered => {
                println!("  hovered");
                *chapter_button_background_color = Color::rgb(0.45, 0.45, 0.7).into();
                
                for mouse_wheel_event in mouse_wheel_events.read() {
                    for (mut scrolling_list, mut style, sidebar_list_parent, list_node) in &mut query_list {
                        let items_height = list_node.size().y;
                        let container_height = query_node.get(sidebar_list_parent.get()).unwrap().size().y;
            
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
            Interaction::None => {
                *chapter_button_background_color = Color::rgb(0.5, 0.5, 0.5).into();
            }
        }
    }
}

fn sections_container_visibility_system (
    mut sections_container_query: Query<(&mut Visibility, &mut Style, &Parent), With<SectionsContainer>>,
    mut sections_container_visibility_event: EventReader<SectionsContainerVisibilityEvent>,
    // mut chapter_container_query: Query<(&mut Style), With<ChapterContainer>>,
) {
    // for event in sections_container_visibility_event.read() {
    //     for (mut sections_container_visibility, mut style, parent) in &mut sections_container_query.iter_mut() {
    //         let other_parent: Entity = event.1.into();
    //         if other_parent == parent.get() {
    //             *sections_container_visibility = Visibility::Hidden;
    //             style.height = Val::Percent(0.0);
    //         }

    //         // for mut style in &mut chapter_container_query.iter_mut() {
    //         //     style.height = Val::Px(100.0);
    //         // }
    //     }
    // }
}



fn chapter_container_mouse_scroll(
    mut interaction_query: Query<
        (&Interaction),
        With<ChapterContainer>
    >,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut sidebar::SidebarList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
    // mut sections_container_visibility_writer: EventWriter<SectionsContainerVisibilityEvent>,
) {
    for (interaction) in &mut interaction_query {
        
        match *interaction {
            Interaction::Pressed => {
            }
            Interaction::Hovered => {
                for mouse_wheel_event in mouse_wheel_events.read() {
                    for (mut scrolling_list, mut style, sidebar_list_parent, list_node) in &mut query_list {
                        let items_height = list_node.size().y;
                        let container_height = query_node.get(sidebar_list_parent.get()).unwrap().size().y;
            
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
            Interaction::None => {
            }
        }
    }
}

fn sections_container_mouse_scroll(
    mut interaction_query: Query<
        (&Interaction),
        With<SectionsContainer>
    >,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut sidebar::SidebarList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
    // mut sections_container_visibility_writer: EventWriter<SectionsContainerVisibilityEvent>,
) {
    for (interaction) in &mut interaction_query {
        
        match *interaction {
            Interaction::Pressed => {
            }
            Interaction::Hovered => {
                for mouse_wheel_event in mouse_wheel_events.read() {
                    for (mut scrolling_list, mut style, sidebar_list_parent, list_node) in &mut query_list {
                        let items_height = list_node.size().y;
                        let container_height = query_node.get(sidebar_list_parent.get()).unwrap().size().y;
            
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
            Interaction::None => {
            }
        }
    }
}