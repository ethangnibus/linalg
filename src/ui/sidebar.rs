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

// Marker for UI node
#[derive(Component)]
pub struct Sidebar;


#[derive(Component, Default)]
struct SidebarList {
    position: f32,
}

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_scroll);
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
    for i in 0..1000 {
        let text_item = (
            TextBundle::from_section(
                format!("Chapter: {i}"),
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
                height: Val::Px(50.0),
                padding: UiRect::axes(Val::Px(4.0), Val::Px(2.0)),
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
            background_color: Color::rgb(0.5, 0.5, 0.5).into(),
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




fn mouse_scroll(
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