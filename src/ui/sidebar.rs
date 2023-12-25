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
use super::chapter_container;

// Marker for UI node
#[derive(Component)]
pub struct Sidebar;


#[derive(Component, Default)]
pub struct SidebarList {
    pub position: f32,
}

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(chapter_container::SystemsPlugin)
        .add_event::<SidebarScrollEvent>()
        .add_systems(Update, (sidebar_mouse_scroll, sidebar_scroll_reciever));
    }
}

// pub fn setup(mut commands: Commands) {
//     //
//     println!("sidebar.rs");
// }

pub fn setup(commands: &mut Commands, width: f32) -> Entity {
    let sidebar = new(width);
    let sidebar = commands.spawn(sidebar).id();

    let page_items = page_items(commands);
    let scrollable_page = scrollable_page::get_page();

    let scrollable_page = commands.spawn((SidebarList::default(), scrollable_page)).id();

    commands.entity(scrollable_page).push_children(&page_items);
    commands.entity(sidebar).push_children(&[scrollable_page]);

    return sidebar;
}

pub fn new(width: f32) -> (Sidebar, ButtonBundle) {
    return (
        Sidebar,
        ButtonBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                height: Val::Percent(100.0),
                width: Val::Percent(width),
                // width: Val::Px(100.0),
                overflow: Overflow::clip_y(),
                ..default()
            },
            background_color: Color::rgb(1.0, 0.0, 1.0).into(),
            ..default()
        }
    );
}


pub fn page_items(commands: &mut Commands) -> Vec<Entity> {
    let mut page_items = Vec::new();
    for chapter_number in 0..1000u32 {
        let chapter_name = format!("Chapter {}", chapter_number);
        let chapter_container = chapter_container::setup(commands, chapter_name, chapter_number);
        page_items.push(chapter_container);
    }
    return page_items;
}




fn sidebar_mouse_scroll(
    mut interaction_query: Query<
        &Interaction,
        With<Sidebar>
    >,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut SidebarList, &mut Style, &Parent, &Node)>,
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

#[derive(Event)]
pub struct SidebarScrollEvent();
fn sidebar_scroll_reciever(
    mut sidebar_visibility_event: EventReader<SidebarScrollEvent>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut SidebarList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    
}