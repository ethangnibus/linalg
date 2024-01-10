use super::chapter_container;
use super::scrollable_page;
use super::sidebar_routes;
use super::view;
use super::util;
use super::sidebar;
use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    // winit::WinitSettings,
    ui::FocusPolicy,
};

// Marker for UI node
#[derive(Component)]
pub struct Sidebar;

// Marker for the navigation content to the left of the sidebar swiper
#[derive(Component)]
pub struct SidebarContent;

// Marker for swiper node
#[derive(Component)]
pub struct SidebarSwiper;

// true iff the sidebar is shown and the swiper is to the right
#[derive(Component)]
pub struct ShowingSidebar(bool);

#[derive(Component, Default)]
pub struct SidebarList {
    pub position: f32,
}

// Events
#[derive(Event, Debug)]
pub struct SidebarSwiperColorEvent(pub Color);

#[derive(Event)]
pub struct SidebarVisibilityEvent(pub Visibility);

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SidebarSwiperColorEvent>()
            .add_event::<SidebarVisibilityEvent>()
            .add_plugins(chapter_container::SystemsPlugin)
            .add_systems(
                Update,
                (
                    sidebar_swiper_interactions,
                    sidebar_color_change_system,
                    sidebar_visibility_system,
                ),
            );
        // .add_event::<SidebarScrollEvent>()
        // .add_systems(Update, (sidebar_scroll_reciever));
    }
}

pub fn setup(commands: &mut Commands, width: f32) -> Entity {
    let sidebar = new(width);
    let sidebar = commands.spawn(sidebar).id();

    let sidebar_swiper = sidebar_swiper(commands);

    let page_items = sidebar_routes::page_items(commands);
    let scrollable_page = scrollable_page::get_page(100.0);

    let scrollable_page = commands
        .spawn((SidebarList::default(), scrollable_page))
        .id();

    commands.entity(scrollable_page).push_children(&page_items);
    commands.entity(scrollable_page).insert(SidebarContent);
    commands
        .entity(sidebar)
        .push_children(&[scrollable_page, sidebar_swiper]);

    return sidebar;
}

pub fn new(width: f32) -> (Sidebar, ButtonBundle) {
    return (
        Sidebar,
        ButtonBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_self: AlignSelf::Stretch,
                height: Val::Percent(100.0),
                // width: Val::Percent(width),
                width: Val::Auto,
                // width: Val::Px(500.0),
                ..default()
            },
            background_color: Color::rgb(0.0, 0.0, 1.0).into(),
            ..default()
        },
    );
}

pub fn sidebar_swiper(commands: &mut Commands) -> Entity {
    let sidebar_swiper = (
        SidebarSwiper,
        ButtonBundle {
            style: Style {
                // width: Val::Percent(1.0),
                width: util::SWIPERS_WIDTH,
                height: Val::Percent(100.0),
                border: UiRect {
                    left: Val::Px(2.0),
                    right: Val::Px(0.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            focus_policy: FocusPolicy::Block,
            background_color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..default()
        },
        ShowingSidebar(true),
    );
    let sidebar_swiper = commands.spawn(sidebar_swiper).id();

    // let right_line =
    //     NodeBundle {
    //         style: Style {
    //             width: Val::Px(2.0),
    //             height: Val::Percent(100.0),
    //             ..default()
    //         },
    //         // background_color: Color::rgb(1.0, 0.7, 0.1).into(),
    //         background_color: Color::rgb(1.0, 1.0, 1.0).into(),
    //         // border_color: Color::rgb(0.1, 0.1, 0.1).into(),
    //         ..default()
    //     };
    // let right_line = commands.spawn(right_line).id();

    // commands.entity(sidebar_swiper).push_children(&[right_line]);
    return sidebar_swiper;
}

// In your sidebar_swiper_interactions function
fn sidebar_swiper_interactions(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut ShowingSidebar,
        ),
        (Changed<Interaction>, With<SidebarSwiper>),
    >,
    // mut sidebar_query: Query<&mut BackgroundColor, With<sidebar::Sidebar>>,
    // mut header_button_query: Query<&BackgroundColor, With<HeaderButton>>,
    mut sidebar_swiper_color_writer: EventWriter<SidebarSwiperColorEvent>,
    mut sidebar_visibility_writer: EventWriter<SidebarVisibilityEvent>,
) {
    for (interaction, mut color, mut border_color, mut showing_sidebar) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // sidebar_swiper_color_writer.send(SidebarSwiperColorEvent(Color::RED));

                match showing_sidebar.0 {
                    true => {
                        sidebar_visibility_writer.send(SidebarVisibilityEvent(Visibility::Hidden));
                    }
                    false => {
                        sidebar_visibility_writer
                            .send(SidebarVisibilityEvent(Visibility::Inherited));
                    }
                }
                showing_sidebar.0 = !showing_sidebar.0;
            }
            Interaction::Hovered => {
                match showing_sidebar.0 {
                    true => {
                        // sidebar_swiper_color_writer.send(SidebarSwiperColorEvent(Color::rgb(0.7, 0.45, 0.45)));
                        // *color = Color::rgb(0.7, 0.45, 0.45).into();
                        // *color = Color::rgb(0.1, 0.1, 0.1).into();
                        *border_color = Color::rgb(0.5, 0.5, 0.5).into();
                    }
                    false => {
                        // sidebar_swiper_color_writer.send(SidebarSwiperColorEvent(Color::rgb(0.45, 0.45, 0.7)));
                        // *color = Color::rgb(1.0, 0.7, 0.1).into();
                        *border_color = Color::rgb(1.0, 0.7, 0.1).into();
                    }
                }
            }
            Interaction::None => {
                match showing_sidebar.0 {
                    true => {
                        // sidebar_swiper_color_writer.send(SidebarSwiperColorEvent(Color::rgb(0.3, 0.3, 0.3)));
                        // *color = SWIPERS_COLOR_DEFAULT;
                        // *color = Color::rgb(1.0, 0.7, 0.1).into();
                        *border_color = Color::rgb(1.0, 0.7, 0.1).into();
                    }
                    false => {
                        // sidebar_swiper_color_writer.send(SidebarSwiperColorEvent(Color::rgb(0.3, 0.3, 0.3)));
                        // *color = SWIPERS_COLOR_DEFAULT;
                        // *color = Color::rgb(0.1, 0.1, 0.1).into();
                        *border_color = Color::rgb(0.5, 0.5, 0.5).into();
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
    for event in color_event_reader.read() {
        for mut sidebar_swiper_color in &mut sidebar_swiper_query.iter_mut() {
            *sidebar_swiper_color = event.0.into();
        }
    }
}

fn sidebar_visibility_system(
    mut sidebar_content_query: Query<(&mut Visibility, &mut Style), With<SidebarContent>>,
    mut sidebar_visibility_event: EventReader<SidebarVisibilityEvent>,
    mut ui_resize_writer: EventWriter<view::UiResizeEvent>,
    // mut windows: NonSend<WinitWindows>,
    // mut windows: NonSend<World>,
) {
    // println!("printing sidebar visibility query");
    // println!("{:?}", sidebar_query);
    // println!("ending printing sidebar_query");
    for event in sidebar_visibility_event.read() {
        // println!("Receiving event: {:?}", event);

        for (mut sidebar_content_visibility, mut sidebar_content_style) in &mut sidebar_content_query.iter_mut() {
            
                let visibility_kind: Visibility = event.0.into();
                match visibility_kind {
                    Visibility::Hidden => {
                        sidebar_content_style.width = Val::Percent(0.0);
                        // *sidebar_content_visibility = Visibility::Hidden;
                        // sidebar_style.width = Val::Auto;
                    }
                    Visibility::Visible => {
                        sidebar_content_style.width = Val::Percent(100.0);
                        *sidebar_content_visibility = Visibility::Visible;
                        // sidebar_style.width = Val::Percent(util::SIDEBAR_WIDTH);
                    }
                    Visibility::Inherited => {
                        sidebar_content_style.width = Val::Percent(100.0);
                        *sidebar_content_visibility = Visibility::Inherited;
                        // sidebar_style.width =  Val::Percent(util::SIDEBAR_WIDTH);
                    }
                }
        
        }
        ui_resize_writer.send(view::UiResizeEvent);
    }
}
