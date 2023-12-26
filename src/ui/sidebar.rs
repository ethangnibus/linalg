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
        .add_plugins(chapter_container::SystemsPlugin);
        // .add_event::<SidebarScrollEvent>()
        // .add_systems(Update, (sidebar_scroll_reciever));
    }
}

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
        let chapter_container = chapter_container::chapter_button(commands, &chapter_name, chapter_number);
        
        let section_name1 = format!("Section {}", 1);
        let section_name2 = format!("Section {}", 2);
        let section_button = chapter_container::section_button(commands, &section_name1, chapter_number);
        let section_button2 = chapter_container::section_button(commands, &section_name2, chapter_number);
        page_items.push(chapter_container);
        page_items.push(section_button);
        page_items.push(section_button2);
    }
    return page_items;
}
