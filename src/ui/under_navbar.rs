use super::scrollable_page;
use super::sidebar;
use super::sidebar_frame;
use bevy::prelude::*;

// Marker for Node
#[derive(Component)]
pub struct UnderNavbar;

// Marker for swiper node
#[derive(Component)]
pub struct SidebarSwiper;

// true iff the sidebar is shown and the swiper is to the right
#[derive(Component)]
pub struct ShowingSidebar(bool);

// Events
#[derive(Event, Debug)]
pub struct SidebarSwiperColorEvent(pub Color);

#[derive(Event)]
pub struct SidebarVisibilityEvent(pub Visibility);


pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(sidebar_frame::SystemsPlugin)
            .add_plugins(sidebar::SystemsPlugin)
            .add_plugins(scrollable_page::SystemsPlugin)
            .add_event::<SidebarSwiperColorEvent>()
            .add_event::<SidebarVisibilityEvent>()
            .add_systems(Update, (sidebar_swiper_interactions, sidebar_color_change_system, sidebar_visibility_system));
    }
}

// Returns root node
pub fn setup(commands: &mut Commands, width: f32, height: f32) -> Entity {
    // Make ECS for root and navbar
    // return entities
    let under_navbar = sidebar_frame::setup(commands, width, height);
    let sidebar_width: f32 = 20.0; // in percentage
    let sidebar = sidebar::setup(commands, sidebar_width);

    let sidebar_swiper = sidebar_swiper();
    let sidebar_swiper = commands.spawn(sidebar_swiper).id();

    let scrollable_page = scrollable_page::setup(commands);

    // make under_navbar parent of sidebar and scrollable_page
    commands
        .entity(under_navbar)
        .push_children(&[sidebar, sidebar_swiper, scrollable_page]);

    return under_navbar;
}

pub fn sidebar_swiper() -> (SidebarSwiper, ButtonBundle, ShowingSidebar) {
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

// In your sidebar_swiper_interactions function
fn sidebar_swiper_interactions(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &mut ShowingSidebar), (Changed<Interaction>, With<SidebarSwiper>)>,
    // mut sidebar_query: Query<&mut BackgroundColor, With<sidebar::Sidebar>>,
    mut sidebar_swiper_color_writer: EventWriter<SidebarSwiperColorEvent>,
    mut sidebar_visibility_writer: EventWriter<SidebarVisibilityEvent>,
) {
    for (interaction, mut color, mut showing_sidebar) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // sidebar_swiper_color_writer.send(SidebarSwiperColorEvent(Color::RED));
                
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
                match showing_sidebar.0 {
                    true => {
                        sidebar_swiper_color_writer.send(SidebarSwiperColorEvent(Color::rgb(0.5, 0.5, 0.5)));
                    }
                    false => {
                        sidebar_swiper_color_writer.send(SidebarSwiperColorEvent(Color::rgb(1.0, 1.0, 1.0)));
                    }
                }
            }
            Interaction::None => {
                match showing_sidebar.0 {
                    true => {
                        sidebar_swiper_color_writer.send(SidebarSwiperColorEvent(Color::rgb(1.0, 1.0, 1.0)));
                    }
                    false => {
                        sidebar_swiper_color_writer.send(SidebarSwiperColorEvent(Color::rgb(0.5, 0.5, 0.5)));
                    }
                }
            }
        }
    }
}

// In another system that handles the event
fn sidebar_color_change_system(
    mut sidebar_swiper_query: Query<&mut BackgroundColor, With<SidebarSwiper>>,
    mut color_event_reader: EventReader<SidebarSwiperColorEvent>,
) {
    // println!("printing sidebar_query");
    // println!("{:?}", sidebar_query);
    // println!("ending printing sidebar_query");
    for event in color_event_reader.read() {
        // println!("Receiving event: {:?}", event);

        for mut sidebar_swiper_color in &mut sidebar_swiper_query.iter_mut() {
            // println!("Modifying sidebar_color: {:?}", sidebar_color);
            *sidebar_swiper_color = event.0.into();
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