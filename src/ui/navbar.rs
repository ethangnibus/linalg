use bevy::prelude::*;

use super::{sidebar, under_navbar, util::theme};

// Marker for UI node
#[derive(Component)]
pub struct Navbar;

#[derive(Component)]
pub struct SidebarButton;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sidebar_button_color_change_system);
    }
}

// pub fn setup(mut commands: Commands) {
//     //
//     println!("navbar.rs");
// }

pub fn setup(commands: &mut Commands, theme: &theme::CurrentTheme, height: f32) -> Entity {
    let navbar = new(theme, height);

    let navbar = commands.spawn(navbar).id();

    let sidebar_button = sidebar_button(commands, theme, height);

    commands.entity(navbar).push_children(&[sidebar_button]);

    return navbar;
}

pub fn new(theme: &theme::CurrentTheme, height: f32) -> NodeBundle {
    return NodeBundle {
        style: Style {
            // height: Val::Percent(height),
            height: Val::Percent(height),
            width: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            border: UiRect {
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(4.0),
            },
            padding: UiRect {
                left: Val::Px(4.0),
                right: Val::Px(4.0),
                top: Val::Px(4.0),
                bottom: Val::Px(4.0),
            },
            overflow: Overflow::clip(),
            ..default()
        },
        background_color: theme::navbar_background_color(theme).into(),
        border_color: Color::rgb(0.1, 0.1, 0.1).into(),
        ..default()
    };
}

pub fn sidebar_button(commands: &mut Commands, theme: &theme::CurrentTheme, height: f32) -> Entity {
    let sidebar_button = commands
        .spawn((
            ButtonBundle {
                style: Style {
                    // height: Val::Percent(height),
                    height: Val::Percent(100.0),
                    aspect_ratio: Some(1.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border: UiRect {
                        left: Val::Px(2.0),
                        right: Val::Px(2.0),
                        top: Val::Px(2.0),
                        bottom: Val::Px(2.0),
                    },
                    overflow: Overflow::clip(),
                    ..default()
                },
                focus_policy: bevy::ui::FocusPolicy::Block,
                background_color: theme::navbar_background_color(theme).into(),
                border_color: theme::navbar_text_color(theme).into(),
                ..default()
            },
            SidebarButton,
        ))
        .id();

    let arrow_text = commands
        .spawn(TextBundle::from_section(
            ">",
            TextStyle {
                font_size: 50.0,
                color: theme::navbar_text_color(theme).into(),
                ..default()
            },
        ))
        .id();

    commands.entity(sidebar_button).push_children(&[arrow_text]);

    return sidebar_button;
}

// fn sidebar_button_interactions(
//     mut interaction_query: Query<
//         (&Interaction, &mut BackgroundColor, &mut BorderColor),
//         (Changed<Interaction>, With<SidebarButton>),
//     >,
//     mut showing_sidebar: ResMut<under_navbar::ShowingSidebar>,
//     // mut sidebar_query: Query<&mut BackgroundColor, With<sidebar::Sidebar>>,
//     // mut header_button_query: Query<&BackgroundColor, With<HeaderButton>>,
//     mut sidebar_swiper_color_writer: EventWriter<under_navbar::SidebarSwiperColorEvent>,
//     mut sidebar_visibility_writer: EventWriter<under_navbar::SidebarVisibilityEvent>,
// ) {
//     // for mut showing_sidebar in interaction_query.iter_mut() {
//     //     for (interaction, mut sidebar_swiper_color, mut border_sidebar_swiper_color, mut showing_sidebar) in sidebar_swiper_query.iter_mut() {
//     //         match *interaction {
//     //             Interaction::Pressed => {

//     //                 match showing_sidebar.0 {
//     //                     true => {
//     //                         sidebar_visibility_writer.send(under_navbar::SidebarVisibilityEvent(Visibility::Hidden));
//     //                     }
//     //                     false => {
//     //                         sidebar_visibility_writer
//     //                             .send(under_navbar::SidebarVisibilityEvent(Visibility::Inherited));
//     //                     }
//     //                 }
//     //                 showing_sidebar.0 = !showing_sidebar.0;
//     //             }
//     //             Interaction::Hovered => {
//     //                 match showing_sidebar.0 {
//     //                     true => {
//     //                         // *sidebar_swiper_color = sidebar_swiper_color::rgb(0.7, 0.45, 0.45).into();
//     //                         // *sidebar_swiper_color = sidebar_swiper_color::rgb(0.1, 0.1, 0.1).into();
//     //                         *border_sidebar_swiper_color = Color::rgb(0.5, 0.5, 0.5).into();
//     //                     }
//     //                     false => {
//     //                         // sidebar_swiper_color_writer.send(SidebarSwiperColorEvent(Color::rgb(0.45, 0.45, 0.7)));
//     //                         // *color = Color::rgb(1.0, 0.7, 0.1).into();
//     //                         *border_sidebar_swiper_color = Color::rgb(1.0, 0.7, 0.1).into();
//     //                     }
//     //                 }
//     //             }
//     //             Interaction::None => {
//     //                 match showing_sidebar.0 {
//     //                     true => {
//     //                         // sidebar_swiper_color_writer.send(SidebarSwiperColorEvent(Color::rgb(0.3, 0.3, 0.3)));
//     //                         // *color = SWIPERS_COLOR_DEFAULT;
//     //                         // *color = Color::rgb(1.0, 0.7, 0.1).into();
//     //                         *border_sidebar_swiper_color = Color::rgb(1.0, 0.7, 0.1).into();
//     //                     }
//     //                     false => {
//     //                         // sidebar_swiper_color_writer.send(SidebarSwiperColorEvent(Color::rgb(0.3, 0.3, 0.3)));
//     //                         // *color = SWIPERS_COLOR_DEFAULT;
//     //                         // *color = Color::rgb(0.1, 0.1, 0.1).into();
//     //                         *border_sidebar_swiper_color = Color::rgb(0.5, 0.5, 0.5).into();
//     //                     }
//     //                 }
//     //             }
//     //         }
//     //     }
//     // }
// }


fn sidebar_button_color_change_system(
    mut sidebar_swiper_query: Query<&mut BorderColor, With<SidebarButton>>,
    // mut sidebar_button_query: Query<&mut BorderColor, With<navbar::SidebarButton>>,
    mut sidebar_swiper_color_event_reader: EventReader<under_navbar::SidebarCollapseInteractionEvent>,
) {
    for event in sidebar_swiper_color_event_reader.read() {
        for mut sidebar_swiper_border_color in &mut sidebar_swiper_query.iter_mut() {
            *sidebar_swiper_border_color = event.0.into();
        }
        // for mut sidebar_button_border_color in &mut sidebar_button_query.iter_mut() {
        //     *sidebar_button_border_color = event.0.into();
        // }

    }
}