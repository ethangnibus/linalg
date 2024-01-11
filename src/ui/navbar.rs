use bevy::prelude::*;

use super::{sidebar, under_navbar, util::theme};

// Marker for UI node
#[derive(Component)]
pub struct Navbar;

// Marker
#[derive(Component)]
pub struct SidebarButton;

// Marker
#[derive(Component)]
pub struct SidebarButtonText;


pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            sidebar_button_interactions,
            sidebar_button_color_change_system,
            sidebar_button_text_color_change_system

            
        ));
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
    let navbar_banner = navbar_banner(commands, theme, height);
    let hamburger_button = hamburger_button(commands, theme, height);

    commands.entity(navbar).push_children(&[sidebar_button, navbar_banner, hamburger_button]);

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
            flex_direction: FlexDirection::Row,
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
        .spawn((TextBundle::from_section(
            "<",
            TextStyle {
                font_size: 50.0,
                color: theme::navbar_text_color(theme).into(),
                ..default()
            },
        ),
        SidebarButtonText,
        ))
        .id();

    commands.entity(sidebar_button).push_children(&[arrow_text]);

    return sidebar_button;
}

pub fn navbar_banner(commands: &mut Commands, theme: &theme::CurrentTheme, height: f32) -> Entity {
    let background_banner = commands
        .spawn((
            ButtonBundle {
                style: Style {
                    // height: Val::Percent(height),
                    height: Val::Percent(100.0),
                    flex_grow: 2.0,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border: UiRect {
                        left: Val::Px(0.0),
                        right: Val::Px(0.0),
                        top: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                    },
                    overflow: Overflow::clip(),
                    ..default()
                },
                
                focus_policy: bevy::ui::FocusPolicy::Block,
                background_color: theme::navbar_background_color(theme).into(),
                border_color: theme::navbar_text_color(theme).into(),
                ..default()
            },
        ))
        .id();
    
    let navbar_text = commands
        .spawn((TextBundle::from_section(
            "Math 56",
            TextStyle {
                font_size: 50.0,
                color: theme::navbar_text_color(theme).into(),
                ..default()
            },
        ),
        ))
        .id();

    commands.entity(background_banner).push_children(&[navbar_text]);

    return background_banner;
}

pub fn hamburger_button(commands: &mut Commands, theme: &theme::CurrentTheme, height: f32) -> Entity {
    let background_banner = commands
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
                border_color: theme::sidebar_collapsed_color(theme).into(),
                ..default()
            },
        ))
        .id();
    
    let text = commands
        .spawn((TextBundle::from_section(
            "=",
            TextStyle {
                font_size: 50.0,
                color: theme::sidebar_collapsed_color(theme).into(),
                ..default()
            },
        ),
        ))
        .id();

    commands.entity(background_banner).push_children(&[text]);

    return background_banner;
}

fn sidebar_button_interactions(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<SidebarButton>)>,
    mut showing_sidebar: ResMut<under_navbar::ShowingSidebar>,
    theme: Res<theme::CurrentTheme>,
    // mut sidebar_query: Query<&mut BackgroundColor, With<sidebar::Sidebar>>,
    // mut header_button_query: Query<&BackgroundColor, With<HeaderButton>>,
    mut sidebar_swiper_color_writer: EventWriter<under_navbar::SidebarCollapseInteractionEvent>,
    mut sidebar_visibility_writer: EventWriter<under_navbar::SidebarVisibilityEvent>,
) {
    let theme = theme.as_ref();
    for interaction in interaction_query.iter() {
        match *interaction {
            Interaction::Pressed => {
                match showing_sidebar.0 {
                    true => {
                        sidebar_visibility_writer.send(under_navbar::SidebarVisibilityEvent(Visibility::Hidden));
                    }
                    false => {
                        sidebar_visibility_writer
                            .send(under_navbar::SidebarVisibilityEvent(Visibility::Inherited));
                    }
                }
                sidebar_swiper_color_writer.send(under_navbar::SidebarCollapseInteractionEvent(
                    theme::NOT_A_COLOR
                ));
                showing_sidebar.0 = !showing_sidebar.0;
            }
            Interaction::Hovered => match showing_sidebar.0 {
                true => {
                    sidebar_swiper_color_writer.send(under_navbar::SidebarCollapseInteractionEvent(
                        theme::sidebar_collapsed_color(theme),
                    ));
                }
                false => {
                    sidebar_swiper_color_writer
                        .send(under_navbar::SidebarCollapseInteractionEvent(theme::sidebar_color(theme)));
                }
            },
            Interaction::None => match showing_sidebar.0 {
                true => {
                    sidebar_swiper_color_writer
                        .send(under_navbar::SidebarCollapseInteractionEvent(theme::sidebar_color(theme)));
                }
                false => {
                    sidebar_swiper_color_writer.send(under_navbar::SidebarCollapseInteractionEvent(
                        theme::sidebar_collapsed_color(theme),
                    ));
                }
            },
        }
    }
}


fn sidebar_button_color_change_system(
    mut sidebar_swiper_query: Query<&mut BorderColor, With<SidebarButton>>,
    // mut sidebar_button_query: Query<&mut BorderColor, With<navbar::SidebarButton>>,
    mut sidebar_swiper_color_event_reader: EventReader<under_navbar::SidebarCollapseInteractionEvent>,
) {
    for event in sidebar_swiper_color_event_reader.read() {
        for mut sidebar_swiper_border_color in &mut sidebar_swiper_query.iter_mut() {
            let color = event.0;
            if color != theme::NOT_A_COLOR {
                *sidebar_swiper_border_color = color.into();
            }
        }
        // for mut sidebar_button_border_color in &mut sidebar_button_query.iter_mut() {
        //     *sidebar_button_border_color = event.0.into();
        // }

    }
}

fn sidebar_button_text_color_change_system(
    mut sidebar_swiper_query: Query<&mut Text, With<SidebarButtonText>>,
    theme: Res<theme::CurrentTheme>,
    // mut sidebar_button_query: Query<&mut BorderColor, With<navbar::SidebarButton>>,
    mut sidebar_swiper_color_event_reader: EventReader<under_navbar::SidebarCollapseInteractionEvent>,
) {
    let theme = theme.as_ref();
    for event in sidebar_swiper_color_event_reader.read() {
        for mut sidebar_button_text in &mut sidebar_swiper_query.iter_mut() {

            let color = event.0;

            if color == theme::NOT_A_COLOR {
                if sidebar_button_text.sections[0].value == String::from(">") {
                    sidebar_button_text.sections[0].value = String::from("<");
                } else if sidebar_button_text.sections[0].value == String::from("<") {
                    sidebar_button_text.sections[0].value = String::from(">");
                }
            } else {
                sidebar_button_text.sections[0].style.color = event.0.into();
            }
        }
    }
}