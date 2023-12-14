use super::scrollable_page;
use super::sidebar;
use super::sidebar_frame;
use bevy::prelude::*;
use bevy::render::view::visibility;

// Marker for UI node
#[derive(Component)]
pub struct UnderNavbar;

// Marker for swiper node
#[derive(Component)]
pub struct SidebarSwiper;

#[derive(Component)]
pub struct ShowingSidebar(bool);

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(sidebar_frame::SystemsPlugin)
            .add_plugins(sidebar::SystemsPlugin)
            .add_plugins(scrollable_page::SystemsPlugin)
            .add_event::<ChangeSidebarColorEvent>()
            .add_event::<SidebarVisibilityEvent>()
            .add_systems(Update, (horizontal_swiper_resize_system, sidebar_color_change_system, sidebar_visibility_system));
    }
}

// Returns root node
pub fn setup(commands: &mut Commands, width: f32, height: f32) -> Entity {
    // Make ECS for root and navbar
    // return entities
    let under_navbar = sidebar_frame::setup(commands, width, height);
    let sidebar_width: f32 = 20.0; // in percentage
    let sidebar = sidebar::setup(commands, sidebar_width);

    let horizontal_swiper = horizontal_swiper();
    let horizontal_swiper = commands.spawn(horizontal_swiper).id();

    let scrollable_page = scrollable_page::setup(commands);

    // make under_navbar parent of sidebar and scrollable_page
    commands
        .entity(under_navbar)
        .push_children(&[sidebar, horizontal_swiper, scrollable_page]);

    return under_navbar;
}

pub fn horizontal_swiper() -> (SidebarSwiper, ButtonBundle, ShowingSidebar) {
    return (
        SidebarSwiper,
        ButtonBundle {
        style: Style {
            width: Val::Percent(1.0),
            height: Val::Percent(100.0),
            border: UiRect::all(Val::Px(0.0)),
            ..default()
        },
        background_color: Color::rgb(1.0, 1.0, 1.0).into(),
        ..default()
    },
    ShowingSidebar(true)
);
}


// In your main.rs or lib.rs
#[derive(Event, Debug)]
pub struct ChangeSidebarColorEvent(pub Color);

#[derive(Event)]
pub struct SidebarVisibilityEvent(pub Visibility);

// #[derive(Event, Debug)]
// pub struct ResizeSidebarEvent(pub bevy::prelude::Val);

// In your horizontal_swiper_resize_system function
fn horizontal_swiper_resize_system(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &mut ShowingSidebar), (Changed<Interaction>, With<SidebarSwiper>)>,
    // mut sidebar_query: Query<&mut BackgroundColor, With<sidebar::Sidebar>>,
    mut sidebar_color_writer: EventWriter<ChangeSidebarColorEvent>,
    mut sidebar_visibility_writer: EventWriter<SidebarVisibilityEvent>,
) {
    for (interaction, mut color, mut showing_sidebar) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::RED.into();
                sidebar_color_writer.send(ChangeSidebarColorEvent(Color::RED));
                
                match showing_sidebar.0 {
                    true => {
                        println!("***** Making the sidebar hidden");
                        sidebar_visibility_writer.send(SidebarVisibilityEvent(Visibility::Hidden));
                    }
                    false => {
                        println!("***** Making the sidebar visible (inherited)");
                        sidebar_visibility_writer.send(SidebarVisibilityEvent(Visibility::Inherited));
                    }
                }
                showing_sidebar.0 = !showing_sidebar.0;
            }
            Interaction::Hovered => {
                *color = Color::BLUE.into();
                // println!("Sending Event");
                sidebar_color_writer.send(ChangeSidebarColorEvent(Color::BLUE));
            }
            Interaction::None => {
                *color = Color::GREEN.into();
                sidebar_color_writer.send(ChangeSidebarColorEvent(Color::GREEN));
            }
        }
    }
}

// In another system that handles the event
fn sidebar_color_change_system(
    mut sidebar_query: Query<&mut BackgroundColor, With<sidebar::Sidebar>>,
    mut color_event_reader: EventReader<ChangeSidebarColorEvent>,
) {
    // println!("printing sidebar_query");
    // println!("{:?}", sidebar_query);
    // println!("ending printing sidebar_query");
    for event in color_event_reader.read() {
        // println!("Receiving event: {:?}", event);

        for mut sidebar_color in &mut sidebar_query.iter_mut() {
            // println!("Modifying sidebar_color: {:?}", sidebar_color);
            *sidebar_color = event.0.into();
        }
    }
}


fn sidebar_visibility_system(
    mut sidebar_query: Query<(&mut Visibility, &mut Style), With<sidebar::Sidebar>>,
    mut sidebar_visibility_event: EventReader<SidebarVisibilityEvent>,
) {
    // println!("printing sidebar visibility query");
    // println!("{:?}", sidebar_query);
    // println!("ending printing sidebar_query");
    for event in sidebar_visibility_event.read() {
        // println!("Receiving event: {:?}", event);

        for (mut sidebar_visibility, mut sidebar_style) in &mut sidebar_query.iter_mut() {
            let visibility_kind: Visibility = event.0.into();
            match visibility_kind {
                Visibility::Hidden => {
                    println!("setting width to 0 in Hidden");
                    *sidebar_visibility = Visibility::Hidden;
                    sidebar_style.width = bevy::prelude::Val::Vw(0.0);
                }
                Visibility::Visible => {
                    println!("setting width to 20 in Visible");
                    *sidebar_visibility = Visibility::Visible;
                    sidebar_style.width = bevy::prelude::Val::Vw(20.0);
                }
                Visibility::Inherited => {
                    println!("setting width to 20 in Inherited");
                    *sidebar_visibility = Visibility::Inherited;
                    sidebar_style.width = bevy::prelude::Val::Vw(20.0);
                }
            }
            println!("Visiblity is now {:?}", *sidebar_visibility);
            println!();
        }
    }
}