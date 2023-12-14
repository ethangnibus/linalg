use super::scrollable_page;
use super::sidebar;
use super::sidebar_frame;
use bevy::prelude::*;

// Marker for UI node
#[derive(Component)]
pub struct UnderNavbar;

// Marker for swiper node
#[derive(Component)]
pub struct SidebarSwiper;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(sidebar_frame::SystemsPlugin)
            .add_plugins(sidebar::SystemsPlugin)
            .add_plugins(scrollable_page::SystemsPlugin)
            .add_event::<ChangeSidebarColorEvent>()
            .add_systems(Update, (horizontal_swiper_resize_system, sidebar_color_change_system));
    }
}

// Returns root node
pub fn setup(commands: &mut Commands, width: f32, height: f32) -> Entity {
    // Make ECS for root and navbar
    // return entities
    let under_navbar = sidebar_frame::setup(commands, width, height);
    let sidebar_width: f32 = 20.0; // in percentage
    let sidebar = sidebar::setup(commands, sidebar_width);

    let horizontal_swiper = (horizontal_swiper(), SidebarSwiper);
    let horizontal_swiper = commands.spawn(horizontal_swiper).id();

    let scrollable_page = scrollable_page::setup(commands);

    // make under_navbar parent of sidebar and scrollable_page
    commands
        .entity(under_navbar)
        .push_children(&[sidebar, horizontal_swiper, scrollable_page]);

    return under_navbar;
}

pub fn horizontal_swiper() -> ButtonBundle {
    return ButtonBundle {
        style: Style {
            width: Val::Percent(1.0),
            height: Val::Percent(100.0),
            border: UiRect::all(Val::Px(0.0)),
            ..default()
        },
        background_color: Color::rgb(1.0, 1.0, 1.0).into(),
        ..default()
    }
}


// In your main.rs or lib.rs
#[derive(Event, Debug)]
pub struct ChangeSidebarColorEvent(pub Color);

// In your horizontal_swiper_resize_system function
fn horizontal_swiper_resize_system(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<SidebarSwiper>)>,
    // mut sidebar_query: Query<&mut BackgroundColor, With<sidebar::Sidebar>>,
    mut events: EventWriter<ChangeSidebarColorEvent>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::RED.into();
                events.send(ChangeSidebarColorEvent(Color::RED));
            }
            Interaction::Hovered => {
                *color = Color::BLUE.into();
                println!("Sending Event");
                events.send(ChangeSidebarColorEvent(Color::BLUE));
            }
            Interaction::None => {
                *color = Color::GREEN.into();
                // events.send(ChangeSidebarColorEvent(Color::GREEN));
            }
        }
    }
}

// In another system that handles the event
fn sidebar_color_change_system(
    mut sidebar_query: Query<&mut BackgroundColor, With<sidebar::Sidebar>>,
    mut color_event_reader: EventReader<ChangeSidebarColorEvent>,
) {
    println!("printing sidebar_query");
    println!("{:?}", sidebar_query);
    println!("ending printing sidebar_query");
    for event in color_event_reader.read() {
        println!("Receiving event: {:?}", event);

        for mut sidebar_color in &mut sidebar_query.iter_mut() {
            println!("Modifying sidebar_color: {:?}", sidebar_color);
            *sidebar_color = event.0.into();
        }
    }
}
