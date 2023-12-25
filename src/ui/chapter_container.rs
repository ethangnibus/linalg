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
pub struct SectionContainer();

#[derive(Component)]
pub struct SidebarItem();

#[derive(Event, Debug)]
pub struct SidebarSwiperColorEvent(pub Color);

#[derive(Event)]
pub struct SectionsContainerVisibilityEvent(pub Visibility);

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SectionsContainerVisibilityEvent>()
            .add_systems(Update, (
                section_container_visibility_system,
                sidebar_item_mouse_scroll
            ));
    }
}

pub fn chapter_container(commands: &mut Commands, chapter_name: &String, chapter_number: u32) -> Entity {
    let chapter_number = ChapterNumber(chapter_number);
    let chapter_container = (
        ChapterContainer(),
        SidebarItem(),
        chapter_number,
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                border: UiRect {
                    left: Val::Px(4.0),
                    right: Val::Px(4.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(4.0),
                },
                padding: UiRect {
                    left: Val::Px(4.0),
                    right: Val::Px(4.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(4.0),
                },
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::rgb(0.5, 0.5, 0.5).into(),
            ..default()
        }
    );

    let chapter_container = commands.spawn(chapter_container).id();
    // let chapter_button = chapter_button(commands, chapter_name, chapter_number);

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
    let text_item = commands.spawn(text_item).id();
    commands.entity(chapter_container).push_children(&[text_item]);

    // commands.entity(chapter_container).push_children(&[chapter_button]);
    return chapter_container;
}

pub fn section_container(commands: &mut Commands, chapter_name: &String, chapter_number: u32) -> Entity {
    let chapter_number = ChapterNumber(chapter_number);
    let chapter_container = (
        SectionContainer(),
        SidebarItem(),
        chapter_number,
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                border: UiRect {
                    left: Val::Px(8.0),
                    right: Val::Px(4.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(4.0),
                },
                padding: UiRect {
                    left: Val::Px(8.0),
                    right: Val::Px(4.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(4.0),
                },
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..default()
        }
    );

    let chapter_container = commands.spawn(chapter_container).id();
    // let chapter_button = chapter_button(commands, chapter_name, chapter_number);

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
    let text_item = commands.spawn(text_item).id();
    commands.entity(chapter_container).push_children(&[text_item]);

    // commands.entity(chapter_container).push_children(&[chapter_button]);
    return chapter_container;
}

fn section_container_visibility_system (
    mut section_container_query: Query<(&mut Visibility, &mut Style), With<SectionContainer>>,
    // mut section_button_query: Query<(&mut Visibility, &mut Style), With<SectionContainer>>,
    mut section_container_visibility_event: EventReader<SectionsContainerVisibilityEvent>,
    // mut chapter_container_query: Query<(&mut Style), With<ChapterContainer>>,
) {
    for event in section_container_visibility_event.read() {
        for (mut section_container_visibility, mut style) in &mut section_container_query.iter_mut() {
            let event_visibility: Visibility = event.0.into();

            match event_visibility {
                Visibility::Hidden => {
                    *section_container_visibility = Visibility::Hidden;
                    style.height = Val::Px(0.0);
                    style.border = UiRect {
                        left: Val::Px(0.0),
                        right: Val::Px(0.0),
                        top: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                    };
                    style.padding = UiRect {
                        left: Val::Px(0.0),
                        right: Val::Px(0.0),
                        top: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                    };
                }
                Visibility::Inherited => {
                    
                }
                Visibility::Visible => {
                    
                }
            }
            // for mut style in &mut chapter_container_query.iter_mut() {
            //     style.height = Val::Px(100.0);
            // }
        }
    }
}

fn sidebar_item_mouse_scroll(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        With<SidebarItem>
    >,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut sidebar::SidebarList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
    mut section_container_visibility_writer: EventWriter<SectionsContainerVisibilityEvent>,
    // mut section_container_visibility_writer: EventWriter<SectionsContainerVisibilityEvent>,
) {
    for (interaction, mut chapter_container_background_color, mut chapter_container_border_color ) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *chapter_container_background_color = Color::rgb(0.45, 0.45, 0.7).into();
                *chapter_container_border_color = Color::rgb(0.1, 0.1, 0.1).into();
                section_container_visibility_writer.send(SectionsContainerVisibilityEvent(Visibility::Hidden));
            }
            Interaction::Hovered => {
                *chapter_container_background_color = Color::rgb(0.6, 0.6, 0.9).into();
                *chapter_container_border_color = Color::rgb(0.1, 0.1, 0.1).into();

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
                *chapter_container_background_color = Color::rgb(0.5, 0.5, 0.5).into();
                *chapter_container_border_color = Color::rgb(0.1, 0.1, 0.1).into();
            }
        }
    }
}
