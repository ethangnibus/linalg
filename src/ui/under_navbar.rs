
use super::chapter_container::HeaderButton;
use super::navbar;
use super::sidebar;
use super::sidebar_frame;
use super::option_bar;
use super::util::theme::background_color;
use super::util::{
    theme,
    style,
};
use super::view;
use super::subsection_cameras;
use bevy::winit::WinitWindows;
use bevy::{prelude::*, ui::FocusPolicy};

// ending today
// remember to do colors

const SIDEBAR_WIDTH: f32 = 500.0; // in percentage golden ratio
const SWIPERS_COLOR_DEFAULT: BackgroundColor = BackgroundColor(Color::rgb(0.1, 0.1, 0.1));

// Marker for Node
#[derive(Component)]
pub struct UnderNavbar;

// Marker for swiper node
#[derive(Component)]
pub struct SidebarSwiper;

// true iff the sidebar is shown and the swiper is to the right
#[derive(Resource)]
pub struct ShowingSidebar(pub bool);

// Events
#[derive(Event, Debug)]
pub struct SidebarCollapseInteractionEvent(pub Color);

#[derive(Event)]
pub struct SidebarVisibilityEvent(pub Visibility);

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(sidebar_frame::SystemsPlugin)
            .add_plugins(sidebar::SystemsPlugin)
            .add_plugins(view::SystemsPlugin)
            .add_plugins(option_bar::SystemsPlugin)
            .insert_resource(ShowingSidebar(true))
            .add_event::<SidebarCollapseInteractionEvent>()
            .add_event::<SidebarVisibilityEvent>()
            .add_systems(
                Update,
                (
                    sidebar_swiper_interactions,
                    sidebar_swiper_color_change_system,
                    sidebar_visibility_system.after(subsection_cameras::resize_camera_system),
                ),
            );
    }
}

// Returns root node
pub fn setup(commands: &mut Commands, theme: &theme::CurrentTheme, under_navbar_entity: Entity) {
    // Make ECS for root and navbar
    // return entities
    let under_navbar = under_navbar_entity;

    let sidebar = sidebar::new(commands, theme, SIDEBAR_WIDTH);
    let sidebar_swiper = sidebar_swiper(commands, theme);
    let view = view::new(commands, theme);
    let option_bar_swiper = option_bar::option_bar_swiper(commands, theme);
    let option_bar = option_bar::option_bar(commands, theme, SIDEBAR_WIDTH);

    sidebar::setup(commands, theme, SIDEBAR_WIDTH, sidebar);
    view::setup(commands, theme, view);
    option_bar::setup(commands, theme, SIDEBAR_WIDTH, option_bar);

    // make under_navbar parent of sidebar and scrollable_page
    commands.entity(under_navbar).push_children(&[
        sidebar,
        sidebar_swiper,
        view,
        option_bar_swiper,
        option_bar,
    ]);




}


pub fn new(commands: &mut Commands, width: f32, height: f32) -> Entity {
    return commands.spawn((
        UnderNavbar,
        NodeBundle {
        style: Style {
            width: Val::Percent(width),
            height: Val::Percent(100.0),
            flex_grow: 1.0,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        background_color: Color::rgb(0.0, 0.0, 1.0).into(),
        ..default()
    })).id()
}

pub fn sidebar_swiper(commands: &mut Commands, theme: &theme::CurrentTheme) -> Entity {
    return commands.spawn((
        SidebarSwiper,
        theme::ColorFunction {
            background: theme::swiper_background_color,
            border: theme::sidebar_color,
        },
        ButtonBundle {
            style: Style {
                // width: Val::Percent(1.0),
                width: style::SWIPERS_WIDTH,
                // flex_grow: 1.0,
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
            background_color: theme::swiper_background_color(theme).into(),
            border_color: theme::sidebar_color(theme).into(),
            ..default()
        },
    )).id();
}


// In your sidebar_swiper_interactions function
fn sidebar_swiper_interactions(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<SidebarSwiper>)>,
    mut showing_sidebar: ResMut<ShowingSidebar>,
    theme: Res<theme::CurrentTheme>,
    // mut sidebar_query: Query<&mut BackgroundColor, With<sidebar::Sidebar>>,
    // mut header_button_query: Query<&BackgroundColor, With<HeaderButton>>,
    mut sidebar_swiper_color_writer: EventWriter<SidebarCollapseInteractionEvent>,
    mut sidebar_visibility_writer: EventWriter<SidebarVisibilityEvent>,
) {
    let theme = theme.as_ref();
    for interaction in interaction_query.iter() {
        match *interaction {
            Interaction::Pressed => {
                match showing_sidebar.0 {
                    true => {
                        sidebar_visibility_writer.send(SidebarVisibilityEvent(Visibility::Hidden));
                    }
                    false => {
                        sidebar_visibility_writer
                            .send(SidebarVisibilityEvent(Visibility::Inherited));
                    }
                }
                sidebar_swiper_color_writer.send(SidebarCollapseInteractionEvent(
                    theme::NOT_A_COLOR,
                ));
                showing_sidebar.0 = !showing_sidebar.0;
            }
            Interaction::Hovered => match showing_sidebar.0 {
                true => {
                    sidebar_swiper_color_writer.send(SidebarCollapseInteractionEvent(
                        theme::sidebar_collapsed_color(theme),
                    ));
                }
                false => {
                    sidebar_swiper_color_writer
                        .send(SidebarCollapseInteractionEvent(theme::sidebar_color(theme)));
                }
            },
            Interaction::None => match showing_sidebar.0 {
                true => {
                    sidebar_swiper_color_writer
                        .send(SidebarCollapseInteractionEvent(theme::sidebar_color(theme)));
                }
                false => {
                    sidebar_swiper_color_writer.send(SidebarCollapseInteractionEvent(
                        theme::sidebar_collapsed_color(theme),
                    ));
                }
            },
        }
    }
}

// In another system that handles the event
fn sidebar_swiper_color_change_system(
    mut sidebar_swiper_query: Query<(&mut BorderColor, &mut theme::ColorFunction), With<SidebarSwiper>>,
    // mut sidebar_button_query: Query<&mut BorderColor, With<navbar::SidebarButton>>,
    mut sidebar_swiper_color_event_reader: EventReader<SidebarCollapseInteractionEvent>,
) {
    for event in sidebar_swiper_color_event_reader.read() {
        for (mut sidebar_swiper_border_color, mut color_function) in &mut sidebar_swiper_query.iter_mut() {
            let color = event.0;

            if color != theme::NOT_A_COLOR {
                *sidebar_swiper_border_color = event.0.into();
            } else {
                if color_function.border == theme::sidebar_color {
                    color_function.border = theme::sidebar_collapsed_color;
                } else if color_function.border == theme::sidebar_collapsed_color {
                    color_function.border = theme::sidebar_color;
                }
            }
        }
    }
}

pub fn sidebar_visibility_system(
    mut sidebar_query: Query<(&mut Visibility, &mut Style), With<sidebar::Sidebar>>,
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

        for (mut sidebar_visibility, mut sidebar_style) in &mut sidebar_query.iter_mut() {
            let visibility_kind: Visibility = event.0.into();
            match visibility_kind {
                Visibility::Hidden => {
                    *sidebar_visibility = Visibility::Hidden;
                    sidebar_style.width = Val::Percent(0.0);
                }
                Visibility::Visible => {
                    *sidebar_visibility = Visibility::Visible;
                    sidebar_style.width = Val::Px(SIDEBAR_WIDTH);
                }
                Visibility::Inherited => {
                    *sidebar_visibility = Visibility::Inherited;
                    sidebar_style.width = Val::Px(SIDEBAR_WIDTH);
                }
            }
        }
        ui_resize_writer.send(view::UiResizeEvent);
    }
}
