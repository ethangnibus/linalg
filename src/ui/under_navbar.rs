use super::chapter_container::HeaderButton;
use super::navbar;
use super::sidebar;
use super::sidebar_frame;
use super::util::theme;
use super::view;
use super::view::UiResizeEvent;
use bevy::winit::WinitWindows;
use bevy::{prelude::*, ui::FocusPolicy};

// ending today
// remember to do colors

const SIDEBAR_WIDTH: f32 = 38.23; // in percentage golden ratio
const SWIPERS_WIDTH: Val = Val::Px(10.0);
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
            .insert_resource(ShowingSidebar(true))
            .add_event::<SidebarCollapseInteractionEvent>()
            .add_event::<SidebarVisibilityEvent>()
            .add_systems(
                Update,
                (
                    sidebar_swiper_interactions,
                    sidebar_swiper_color_change_system,
                    sidebar_visibility_system,
                ),
            );
    }
}

// Returns root node
pub fn setup(commands: &mut Commands, width: f32, height: f32) -> Entity {
    // Make ECS for root and navbar
    // return entities
    let under_navbar = sidebar_frame::setup(commands, width, height);
    let sidebar = sidebar::setup(commands, SIDEBAR_WIDTH);

    let sidebar_swiper = sidebar_swiper(commands);

    let right_border = right_swiper();
    let right_border = commands.spawn(right_border).id();

    let view = view::setup(commands);

    // make under_navbar parent of sidebar and scrollable_page
    commands.entity(under_navbar).push_children(&[
        sidebar,
        sidebar_swiper,
        view,
        right_border,
    ]);

    return under_navbar;
}

pub fn sidebar_swiper(commands: &mut Commands) -> Entity {
    let sidebar_swiper = (
        SidebarSwiper,
        ButtonBundle {
            style: Style {
                // width: Val::Percent(1.0),
                width: SWIPERS_WIDTH,
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
            background_color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..default()
        },
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

pub fn right_swiper() -> (NodeBundle) {
    return (NodeBundle {
        style: Style {
            // width: Val::Percent(1.0),
            width: SWIPERS_WIDTH,
            // flex_grow: 1.0,
            height: Val::Percent(100.0),
            border: UiRect::all(Val::Px(0.0)),
            ..default()
        },
        background_color: SWIPERS_COLOR_DEFAULT,
        ..default()
    });
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
    mut sidebar_swiper_query: Query<&mut BorderColor, With<SidebarSwiper>>,
    // mut sidebar_button_query: Query<&mut BorderColor, With<navbar::SidebarButton>>,
    mut sidebar_swiper_color_event_reader: EventReader<SidebarCollapseInteractionEvent>,
) {
    for event in sidebar_swiper_color_event_reader.read() {
        for mut sidebar_swiper_border_color in &mut sidebar_swiper_query.iter_mut() {
            let color = event.0;

            if color != theme::NOT_A_COLOR {
                *sidebar_swiper_border_color = event.0.into();
            }
        }
        // for mut sidebar_button_border_color in &mut sidebar_button_query.iter_mut() {
        //     *sidebar_button_border_color = event.0.into();
        // }
    }
}

fn sidebar_visibility_system(
    mut sidebar_query: Query<(&mut Visibility, &mut Style), With<sidebar::Sidebar>>,
    mut sidebar_visibility_event: EventReader<SidebarVisibilityEvent>,
    mut ui_resize_writer: EventWriter<UiResizeEvent>,
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
                    sidebar_style.width = Val::Percent(SIDEBAR_WIDTH);
                }
                Visibility::Inherited => {
                    *sidebar_visibility = Visibility::Inherited;
                    sidebar_style.width = bevy::prelude::Val::Vw(SIDEBAR_WIDTH);
                }
            }
        }
        ui_resize_writer.send(UiResizeEvent);
    }
}
