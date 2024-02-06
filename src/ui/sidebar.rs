use super::chapter_container;
use super::scrollable_page;
use super::sidebar_routes;
use super::theme;
use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    // winit::WinitSettings,
};

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
        app.add_plugins(chapter_container::SystemsPlugin);
        // .add_event::<SidebarScrollEvent>()
        // .add_systems(Update, (sidebar_scroll_reciever));
    }
}

pub fn setup(commands: &mut Commands, theme: &theme::CurrentTheme, width: f32, sidebar_entity: Entity) -> Entity {
    let sidebar = sidebar_entity;

    let page_items = sidebar_routes::page_items(commands, theme);
    let scrollable_page = commands
        .spawn((
            SidebarList::default(),
            scrollable_page::get_page(theme),
        ))
        .id();

    commands.entity(scrollable_page).push_children(&page_items);
    commands.entity(sidebar).push_children(&[scrollable_page]);

    return sidebar;
}

pub fn new(commands: &mut Commands, theme: &theme::CurrentTheme, width: f32) -> Entity {
    return commands.spawn((
        Sidebar,
        ButtonBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                height: Val::Percent(100.0),
                border: UiRect {
                    top: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    left: Val::Px(0.0),
                    right: Val::Px(0.0),
                },
                // flex_grow: 1.0,
                width: Val::Px(width),
                max_width: Val::Vw(100.0),
                overflow: Overflow::clip(),
                ..default()
            },
            ..default()
        },
    )).id();
}
