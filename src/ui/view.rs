use super::routes;
use super::scrollable_page;
use super::subsection_cameras;
use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    render::{
        camera::{ComputedCameraValues, RenderTarget, Viewport},
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },

    ui,
    // winit::WinitSettings,
};
// use bevy_prototype_lyon::prelude::*;
// use bevy_svg::prelude::*;
use super::theme;
use crate::pages::splash_page;
use bevy_mod_picking::prelude::*;


// Marker for UI node
#[derive(Component)]
pub struct View;

#[derive(Component, Default)]
pub struct ViewList {
    pub position: f32,
}

#[derive(Event)]
pub struct UiResizeEvent;

#[derive(Component)]
pub struct SvgHolder;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(routes::SystemsPlugin)
            .add_plugins(subsection_cameras::SystemsPlugin)
            .add_event::<UiResizeEvent>()
            // .add_systems(Startup, spawn_svg)
            // .add_plugins(ShapePlugin)
            .add_systems(Update, (mouse_scroll));
    }
}

pub fn setup(commands: &mut Commands, theme: &theme::CurrentTheme, view_entity: Entity) -> Entity {
    let view = view_entity;

    // let mut page_entities: Vec<Entity> = Vec::new();
    let page_entities = page_items(commands);
    // splash_page::get(commands, svg_load_writer, &mut page_entities);

    let view_list = scrollable_page::get_page(theme);
    let view_list = commands.spawn((
        ViewList::default(),
        view_list,
        Pickable::IGNORE,
    )).id();

    for entity in page_entities {
        commands.entity(view_list).push_children(&[entity]);
    }
    commands.entity(view).push_children(&[view_list]);

    return view;
}

pub fn new(commands: &mut Commands, theme: &theme::CurrentTheme) -> Entity {
    return commands.spawn((
        View,
        theme::ColorFunction {
            background: theme::background_color,
            border: theme::navbar_swiper_color,
        },
        ButtonBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                // align_self: AlignSelf::Stretch,
                // flex_grow: 1.0,
                width: Val::Percent(100.0),
                flex_shrink: 2.0,
                height: Val::Percent(100.0),
                border: UiRect {
                    top: Val::Px(1.0),
                    bottom: Val::Px(0.0),
                    left: Val::Px(1.0),
                    right: Val::Px(1.0),
                },
                // flex_grow: 1.0,
                overflow: Overflow::clip(),
                ..default()
            },
            background_color: theme::background_color(theme).into(),
            border_color: theme::navbar_swiper_color(theme).into(),
            ..default()
        },
        Pickable::IGNORE,
    )).id();
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
                    left: Val::Px(0.0),
                    right: Val::Px(0.0),
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
    // let svg_window = get_svg(commands);
    // page_items.push(svg_window);

    return page_items;
}

use std::f32::consts::PI;

fn mouse_scroll(
    mut interaction_query: Query<&Interaction, With<View>>,
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

                        // println!("items_height {:?}", items_height);
                        // println!("container_height {:?}", container_height);
                        // println!("max_scroll {:?}\n", max_scroll);

                        let dy = match mouse_wheel_event.unit {
                            MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                            MouseScrollUnit::Pixel => mouse_wheel_event.y,
                        };

                        scrolling_list.position += dy * 0.5;
                        scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
                        style.top = Val::Px(scrolling_list.position);
                    }
                }
            },
            Interaction::Pressed => {
                // println!("interacted");
            },
            _ => {}
        }
    }
    mouse_wheel_events.clear();
}
