use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    winit::WinitSettings,
};

// Marker for UI node
#[derive(Component)]
pub struct ScrollablePage;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_scroll);
    }
}


pub fn setup(commands: &mut Commands) -> Entity {
    // let scrollable_page = new();
    // let scrollable_page = testing(&commands);
    // return commands.spawn(scrollable_page).id();
    let scrollable_page = new();
    let moving_panel = (
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgb(0.0, 0.5, 0.5).into(),
            ..default()
        },
        ScrollingList::default(),
        AccessibilityNode(NodeBuilder::new(Role::List)),
    );

    let scrollable_page = commands.spawn(scrollable_page).id();
    let moving_panel = commands.spawn(moving_panel).id();



    for i in 0..1000 {
        let list_item = (
            TextBundle::from_section(
                format!("Item {i}"),
                TextStyle {
                    font_size: 20.,
                    ..default()
                },
            ),
            Label,
            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
        );
        let list_item = commands.spawn(list_item).id();
        commands.entity(moving_panel).push_children(&[list_item]);
    }

    commands.entity(scrollable_page).push_children(&[moving_panel]);
    // commands.entity(container).push_children(&[button]);

    return scrollable_page;
}

pub fn new() -> (ScrollablePage, NodeBundle) {
    return (ScrollablePage, NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_self: AlignSelf::Stretch,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            overflow: Overflow::clip_y(),
            ..default()
        },
        background_color: Color::rgb(0.0, 1.0, 0.0).into(),
        ..default()
    });
}

fn temp() {}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const PRESSED_BUTTON: Color = Color::rgb(0.0, 0.0, 0.0);

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}



#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
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