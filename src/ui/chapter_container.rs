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

#[derive(Component)]
pub struct ChapterButton;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, chapter_button_mouse_scroll);
    }
}

pub fn setup(commands: &mut Commands, chapter_name: String) -> Entity {
    let chapter_container = NodeBundle {
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
    };
    let sections_container = NodeBundle {
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
    };

    let section_button  = (
        ChapterButton,
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
    let chapter_button = chapter_button(commands, chapter_name);
    let section_button = commands.spawn(section_button).id();

    commands.entity(sections_container).push_children(&[section_button]);
    commands.entity(chapter_container).push_children(&[chapter_button, sections_container]);
    return chapter_container;
}



pub fn chapter_button(commands: &mut Commands, chapter_name: String) -> Entity {
    println!("spawning chapter button");
    let chapter_button  = (
        ChapterButton,
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
        (&Interaction, &mut BackgroundColor, &Parent, &Node),
        With<ChapterButton>
    >,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut sidebar::SidebarList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for (interaction, mut chapter_button_background_color, chapter_container, container_node) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *chapter_button_background_color = Color::rgb(0.6, 0.6, 0.9).into();
                // let parent = parent.get();
                // let children = Children(parent);
                // let mut text = text_query.get_mut(children[0]).unwrap();
                let chapter_container = query_node.get(chapter_container.get()).unwrap().size.y = 10;
            }
            Interaction::Hovered => {
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