use bevy::{prelude::*, render::{view::visibility, color}, ui::FocusPolicy};

use super::{option_bar::{self, ShowingOptionBar}, sidebar, under_navbar, util::style, util::theme};

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                sidebar_button_interactions,
                sidebar_button_color_change_system,
                sidebar_button_text_color_change_system,
                option_bar_button_interactions,
                option_bar_button_color_change_system,
                option_bar_button_text_color_change_system,
                navbar_swiper_interactions,
                navbar_swiper_color_change_system,
                navbar_visibility_system,
                navbar_banner_text_color_change_system,
            ),
        )
        .insert_resource(ShowingNavbar(true))
        .add_event::<NavbarCollapseEvent>()
        .add_event::<NavbarVisibilityEvent>();
    }
}

pub const NAVBAR_HEIGHT: Val = Val::Px(50.0);
pub const NAVBAR_PADDING: UiRect = UiRect {
    left: Val::Px(4.0),
    right: Val::Px(4.0),
    top: Val::Px(4.0),
    bottom: Val::Px(6.0),
};

// Marker for UI node
#[derive(Component)]
pub struct Navbar;

#[derive(Component)]
pub struct NavbarHolder;

// Marker
#[derive(Component)]
pub struct SidebarButton;

// Marker
#[derive(Component)]
pub struct SidebarButtonText;

// Marker
#[derive(Component)]
pub struct OptionBarButton;

// Marker
#[derive(Component)]
pub struct OptionBarButtonText;

// Marker
#[derive(Component)]
pub struct NavbarSwiper;

// Marker
#[derive(Component)]
pub struct NavbarBanner;

// Marker
#[derive(Component)]
pub struct NavbarBannerText;

// pub fn setup(mut commands: Commands) {
//     //
//     println!("navbar.rs");
// }

pub fn setup(commands: &mut Commands, theme: &theme::CurrentTheme, height: f32, navbar_holder_entity: Entity) {
    let navbar_holder = navbar_holder_entity;
    let navbar = new(commands, theme, height);
    let navbar_swiper = navbar_swiper(commands, theme);
    commands
        .entity(navbar_holder)
        .push_children(&[navbar, navbar_swiper]);

    let sidebar_button = sidebar_button(commands, theme, height);
    let navbar_banner = navbar_banner(commands, theme, height);
    let option_bar_button = option_bar_button(commands, theme, height);

    commands
        .entity(navbar)
        .push_children(&[sidebar_button, navbar_banner, option_bar_button]);
}

pub fn navbar_holder(commands: &mut Commands, theme: &theme::CurrentTheme, height: f32) -> Entity {
    return commands
        .spawn((
            NavbarHolder,
            NodeBundle {
            style: Style {
                height: Val::Px(300.0),
                flex_direction: FlexDirection::Column,
                overflow: Overflow::clip(),
                ..default()
            },
            ..default()
        })).id();
}
pub fn new(commands: &mut Commands, theme: &theme::CurrentTheme, height: f32) -> Entity {
    return commands
        .spawn((
            theme::ColorFunction {
                background: theme::navbar_background_color,
                border: theme::navbar_background_color,
            },
            NodeBundle {
                style: Style {
                    // height: Val::Percent(height),
                    height: NAVBAR_HEIGHT,
                    width: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_items: JustifyItems::Center,
                    flex_direction: FlexDirection::Row,
                    border: UiRect {
                        left: Val::Px(0.0),
                        right: Val::Px(0.0),
                        top: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                    },
                    padding: NAVBAR_PADDING,
                    overflow: Overflow::clip(),
                    ..default()
                },
                background_color: theme::navbar_background_color(theme).into(),
                ..default()
            },
            Navbar,
        ))
        .id();
}

pub fn sidebar_button(commands: &mut Commands, theme: &theme::CurrentTheme, height: f32) -> Entity {
    let sidebar_button = commands
        .spawn((
            theme::ColorFunction {
                background: theme::sidebar_header_color,
                border: theme::navbar_swiper_color,
            },
            ButtonBundle {
                style: Style {
                    // height: Val::Percent(height),
                    height: style::BUTTON_HEIGHT,
                    aspect_ratio: Some(1.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border: UiRect {
                        top: Val::Px(1.0),
                        bottom: Val::Px(2.0),
                        left: Val::Px(1.0),
                        right: Val::Px(2.0),
                    },
                    overflow: Overflow::clip(),
                    ..default()
                },
                visibility: Visibility::Inherited,
                focus_policy: bevy::ui::FocusPolicy::Block,
                background_color: theme::sidebar_header_color(theme).into(),
                border_color: theme::navbar_swiper_color(theme).into(),
                ..default()
            },
            SidebarButton,
        ))
        .id();

    let arrow_text = commands
        .spawn((
            theme::ColorFunction {
                background: theme::sidebar_button_text_color,
                border: theme::sidebar_button_text_color,
            },
            TextBundle::from_section(
                "<",
                TextStyle {
                    font_size: 50.0,
                    color: theme::sidebar_button_text_color(theme).into(),
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
            theme::ColorFunction {
                background: theme::navbar_background_color,
                border: theme::navbar_background_color,
            },
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
                visibility: Visibility::Inherited,
                focus_policy: bevy::ui::FocusPolicy::Block,
                background_color: theme::navbar_background_color(theme).into(),
                ..default()
            },
            NavbarBanner,
        ))
        .id();

    let navbar_text = commands
        .spawn((
            theme::ColorFunction {
                background: theme::navbar_swiper_color,
                border: theme::navbar_swiper_color,
            },
            TextBundle::from_section(
                "Projection Matrix",
                TextStyle {
                    font_size: 40.0,
                    color: theme::navbar_swiper_color(theme).into(),
                    ..default()
                },
            ),
            NavbarBannerText,
        ))
        .id();

    commands
        .entity(background_banner)
        .push_children(&[navbar_text]);

    return background_banner;
}

pub fn option_bar_button(
    commands: &mut Commands,
    theme: &theme::CurrentTheme,
    height: f32,
) -> Entity {
    let background_banner = commands
        .spawn((
            theme::ColorFunction {
                background: theme::option_bar_header_color,
                border: theme::sidebar_collapsed_color,
            },
            ButtonBundle {
                style: Style {
                    // height: Val::Percent(height),
                    height: style::BUTTON_HEIGHT,
                    aspect_ratio: Some(1.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border: UiRect {
                        left: Val::Px(1.0),
                        right: Val::Px(2.0),
                        top: Val::Px(1.0),
                        bottom: Val::Px(2.0),
                    },
                    overflow: Overflow::clip(),
                    ..default()
                },
                visibility: Visibility::Inherited,
                focus_policy: bevy::ui::FocusPolicy::Block,
                background_color: theme::option_bar_header_color(theme).into(),
                border_color: theme::sidebar_collapsed_color(theme).into(),
                ..default()
            },
            OptionBarButton,
        ))
        .id();

    let text = commands
        .spawn((
            theme::ColorFunction {
                background: theme::option_button_text_color,
                border: theme::option_button_text_color,
            },
            TextBundle::from_section(
                "+",
                TextStyle {
                    font_size: 50.0,
                    color: theme::option_button_text_color(theme).into(),
                    ..default()
                },
            ),
            OptionBarButtonText,
        ))
        .id();

    commands.entity(background_banner).push_children(&[text]);

    return background_banner;
}

// ==================== sidebar button ====================
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
                        sidebar_visibility_writer
                            .send(under_navbar::SidebarVisibilityEvent(Visibility::Hidden));
                    }
                    false => {
                        sidebar_visibility_writer
                            .send(under_navbar::SidebarVisibilityEvent(Visibility::Inherited));
                    }
                }
                sidebar_swiper_color_writer.send(under_navbar::SidebarCollapseInteractionEvent(
                    theme::NOT_A_COLOR,
                ));
                showing_sidebar.0 = !showing_sidebar.0;
            }
            Interaction::Hovered => match showing_sidebar.0 {
                true => {
                    sidebar_swiper_color_writer.send(
                        under_navbar::SidebarCollapseInteractionEvent(
                            theme::sidebar_collapsed_color(theme),
                        ),
                    );
                }
                false => {
                    sidebar_swiper_color_writer.send(
                        under_navbar::SidebarCollapseInteractionEvent(theme::navbar_swiper_color(theme)),
                    );
                }
            },
            Interaction::None => match showing_sidebar.0 {
                true => {
                    sidebar_swiper_color_writer.send(
                        under_navbar::SidebarCollapseInteractionEvent(theme::navbar_swiper_color(theme)),
                    );
                }
                false => {
                    sidebar_swiper_color_writer.send(
                        under_navbar::SidebarCollapseInteractionEvent(
                            theme::sidebar_collapsed_color(theme),
                        ),
                    );
                }
            },
        }
    }
}

fn sidebar_button_color_change_system(
    mut sidebar_button_query: Query<(&mut BorderColor, &mut theme::ColorFunction), With<SidebarButton>>,
    // mut sidebar_button_query: Query<&mut BorderColor, With<navbar::SidebarButton>>,
    mut sidebar_collapse_reader: EventReader<under_navbar::SidebarCollapseInteractionEvent>,
) {
    for event in sidebar_collapse_reader.read() {
        for (mut sidebar_button_border_color, mut color_function) in &mut sidebar_button_query.iter_mut() {
            let color = event.0;
            if color != theme::NOT_A_COLOR {
                *sidebar_button_border_color = color.into();
            } else {
                if color_function.border == theme::navbar_swiper_color {
                    color_function.border = theme::sidebar_collapsed_color;
                } else if color_function.border == theme::sidebar_collapsed_color {
                    color_function.border = theme::navbar_swiper_color;
                }
            }
        }
    }
}

fn sidebar_button_text_color_change_system(
    mut sidebar_swiper_query: Query<(&mut Text, &mut theme::ColorFunction), With<SidebarButtonText>>,
    theme: Res<theme::CurrentTheme>,
    // mut sidebar_button_query: Query<&mut BorderColor, With<navbar::SidebarButton>>,
    mut sidebar_swiper_color_event_reader: EventReader<
        under_navbar::SidebarCollapseInteractionEvent,
    >,
) {
    let theme = theme.as_ref();
    for event in sidebar_swiper_color_event_reader.read() {
        for (mut sidebar_button_text, mut color_function) in &mut sidebar_swiper_query.iter_mut() {
            let color = event.0;

            if color == theme::NOT_A_COLOR {
                if sidebar_button_text.sections[0].value == String::from(">") {
                    sidebar_button_text.sections[0].value = String::from("<");
                    // color_function.background = theme::navbar_swiper_color;
                    // color_function.border = theme::navbar_swiper_color;
                } else if sidebar_button_text.sections[0].value == String::from("<") {
                    sidebar_button_text.sections[0].value = String::from(">");
                    // color_function.background = theme::sidebar_collapsed_color;
                    // color_function.border = theme::sidebar_collapsed_color;
                }
            } else {
                // sidebar_button_text.sections[0].style.color = event.0.into();
            }
        }
    }
}

// ==================== navbar swiper ====================
pub fn navbar_swiper(commands: &mut Commands, theme: &theme::CurrentTheme) -> Entity {
    let navbar_swiper = commands
        .spawn((
            NavbarSwiper,
            theme::ColorFunction {
                background: theme::swiper_background_color,
                border: theme::navbar_swiper_color,
            },
            ButtonBundle {
                style: Style {
                    // width: Val::Percent(1.0),
                    width: Val::Percent(100.0),
                    height: Val::Px(10.0),
                    border: UiRect {
                        left: Val::Px(0.0),
                        right: Val::Px(0.0),
                        top: Val::Px(2.0),
                        bottom: Val::Px(0.0),
                    },
                    ..default()
                },
                focus_policy: FocusPolicy::Block,
                background_color: theme::swiper_background_color(theme).into(), // FIXME
                border_color: theme::navbar_swiper_color(theme).into(),
                ..default()
            },
        ))
        .id();

    return navbar_swiper;
}

#[derive(Event)]
pub struct NavbarCollapseEvent {
    color: Color,
}

#[derive(Resource)]
pub struct ShowingNavbar(bool);

#[derive(Event)]
pub struct NavbarVisibilityEvent(pub Visibility);

fn navbar_swiper_interactions(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<NavbarSwiper>)>,
    theme: Res<theme::CurrentTheme>,
    mut showing_navbar: ResMut<ShowingNavbar>,
    mut navbar_collapse_writer: EventWriter<NavbarCollapseEvent>,
    mut navbar_visibility_writer: EventWriter<NavbarVisibilityEvent>,
) {
    let theme = theme.as_ref();
    for interaction in interaction_query.iter() {
        match interaction {
            Interaction::Pressed => {
                match showing_navbar.0 {
                    true => {
                        navbar_visibility_writer.send(NavbarVisibilityEvent(Visibility::Hidden));
                    }
                    false => {
                        navbar_visibility_writer.send(NavbarVisibilityEvent(Visibility::Inherited));
                    }
                }
                navbar_collapse_writer.send(NavbarCollapseEvent {
                    color: theme::NOT_A_COLOR,
                });
                showing_navbar.0 = !showing_navbar.0;
            }
            Interaction::Hovered => {
                match showing_navbar.0 {
                    true => navbar_collapse_writer.send(NavbarCollapseEvent {
                        color: theme::sidebar_collapsed_color(theme),
                    }),
                    false => navbar_collapse_writer.send(NavbarCollapseEvent {
                        color: theme::navbar_swiper_color(theme),
                    }),
                }
            }
            Interaction::None => {
                match showing_navbar.0 {
                    true => navbar_collapse_writer.send(NavbarCollapseEvent {
                        color: theme::navbar_swiper_color(theme),
                    }),
                    false => navbar_collapse_writer.send(NavbarCollapseEvent {
                        color: theme::sidebar_collapsed_color(theme),
                    }),
                }
            }
        }
    }
}

fn navbar_swiper_color_change_system(
    mut navbar_collapse_event_reader: EventReader<NavbarCollapseEvent>,
    mut navbar_swiper_query: Query<(&mut BorderColor, &mut theme::ColorFunction), With<NavbarSwiper>>,
) {
    for event in navbar_collapse_event_reader.read() {
        for (mut navbar_swiper_border_color, mut color_function) in navbar_swiper_query.iter_mut() {
            if event.color != theme::NOT_A_COLOR {
                *navbar_swiper_border_color = event.color.into();
            } else {
                if color_function.border == theme::sidebar_collapsed_color {
                    color_function.border = theme::navbar_swiper_color;
                } else if color_function.border == theme::navbar_swiper_color {
                    color_function.border = theme::sidebar_collapsed_color;
                }
            }
        }
    }
}

fn navbar_visibility_system(
    mut navbar_query: Query<(&mut Visibility, &mut Style), With<Navbar>>,
    mut navbar_visibility_reader: EventReader<NavbarVisibilityEvent>,
) {
    for event_visibility in navbar_visibility_reader.read() {
        for (mut navbar_visibility, mut navbar_style) in navbar_query.iter_mut() {
            match event_visibility.0 {
                Visibility::Hidden => {
                    *navbar_visibility = Visibility::Hidden;
                    navbar_style.height = Val::Px(0.0);
                    navbar_style.padding = style::NO_PADDING;
                }
                Visibility::Visible => {
                    *navbar_visibility = Visibility::Visible;
                    navbar_style.height = NAVBAR_HEIGHT;
                    navbar_style.padding = NAVBAR_PADDING;
                }
                Visibility::Inherited => {
                    *navbar_visibility = Visibility::Inherited;
                    navbar_style.height = NAVBAR_HEIGHT;
                    navbar_style.padding = NAVBAR_PADDING;
                }
            }
        }
    }
}

fn navbar_banner_text_color_change_system(
    mut text_query: Query<&mut Text, With<NavbarBannerText>>,
    // mut sidebar_button_query: Query<&mut BorderColor, With<navbar::SidebarButton>>,
    mut navbar_collapse_reader: EventReader<NavbarCollapseEvent>,
) {
    for event in navbar_collapse_reader.read() {
        for mut text in &mut text_query.iter_mut() {
            let color = event.color;

            if color != theme::NOT_A_COLOR {
                text.sections[0].style.color = color.into();
            }
        }
    }
}

// ==================== option bar button ====================
fn option_bar_button_interactions(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<OptionBarButton>)>,
    mut showing_option_bar: ResMut<option_bar::ShowingOptionBar>,
    theme: Res<theme::CurrentTheme>,
    // mut sidebar_query: Query<&mut BackgroundColor, With<sidebar::Sidebar>>,
    // mut header_button_query: Query<&BackgroundColor, With<HeaderButton>>,
    mut option_bar_swiper_color_writer: EventWriter<option_bar::OptionBarCollapseEvent>,
    mut option_bar_visibility_writer: EventWriter<option_bar::OptionBarVisibilityEvent>,
) {
    let theme = theme.as_ref();
    for interaction in interaction_query.iter() {
        match *interaction {
            Interaction::Pressed => {
                match showing_option_bar.0 {
                    true => {
                        option_bar_visibility_writer
                            .send(option_bar::OptionBarVisibilityEvent(Visibility::Hidden));
                    }
                    false => {
                        option_bar_visibility_writer
                            .send(option_bar::OptionBarVisibilityEvent(Visibility::Inherited));
                    }
                }
                option_bar_swiper_color_writer
                    .send(option_bar::OptionBarCollapseEvent(theme::NOT_A_COLOR));
                showing_option_bar.0 = !showing_option_bar.0;
            }
            Interaction::Hovered => match showing_option_bar.0 {
                true => {
                    option_bar_swiper_color_writer.send(option_bar::OptionBarCollapseEvent(
                        theme::sidebar_collapsed_color(theme),
                    ));
                }
                false => {
                    option_bar_swiper_color_writer.send(option_bar::OptionBarCollapseEvent(
                        theme::navbar_swiper_color(theme),
                    ));
                }
            },
            Interaction::None => match showing_option_bar.0 {
                true => {
                    option_bar_swiper_color_writer.send(option_bar::OptionBarCollapseEvent(
                        theme::navbar_swiper_color(theme),
                    ));
                }
                false => {
                    option_bar_swiper_color_writer.send(option_bar::OptionBarCollapseEvent(
                        theme::sidebar_collapsed_color(theme),
                    ));
                }
            },
        }
    }
}

fn option_bar_button_color_change_system(
    mut option_bar_button_query: Query<(&mut BorderColor, &mut theme::ColorFunction), With<OptionBarButton>>,
    // mut sidebar_button_query: Query<&mut BorderColor, With<navbar::SidebarButton>>,
    mut option_bar_color_reader: EventReader<option_bar::OptionBarCollapseEvent>,
    showing_option_bar: Res<ShowingOptionBar>,

    theme: Res<theme::CurrentTheme>,
) {
    let theme = theme.as_ref();
    for event in option_bar_color_reader.read() {
        for (mut option_bar_border_color, mut color_function) in &mut option_bar_button_query.iter_mut() {
            let color = event.0;
            if color != theme::NOT_A_COLOR {
                *option_bar_border_color = event.0.into();
            } else {
                if color_function.border == theme::navbar_swiper_color {
                    color_function.border = theme::sidebar_collapsed_color;
                } else if color_function.border == theme::sidebar_collapsed_color {
                    color_function.border = theme::navbar_swiper_color;
                }
            }
        }
    }
}

fn option_bar_button_text_color_change_system(
    mut option_bar_text_query: Query<(&mut Text, &mut theme::ColorFunction), With<OptionBarButtonText>>,
    theme: Res<theme::CurrentTheme>,
    // mut sidebar_button_query: Query<&mut BorderColor, With<navbar::SidebarButton>>,
    mut option_bar_color_reader: EventReader<option_bar::OptionBarCollapseEvent>,
) {
    let theme = theme.as_ref();
    for event in option_bar_color_reader.read() {
        for (mut option_bar_button_text, mut color_funciton) in &mut option_bar_text_query.iter_mut() {
            let color = event.0;

            if color == theme::NOT_A_COLOR {
                if option_bar_button_text.sections[0].value == String::from("+") {
                    option_bar_button_text.sections[0].value = String::from("-");
                    // color_funciton.border = theme::navbar_swiper_color;
                    // color_funciton.background = theme::navbar_swiper_color;
                } else if option_bar_button_text.sections[0].value == String::from("-") {
                    option_bar_button_text.sections[0].value = String::from("+");
                    // color_funciton.border = theme::sidebar_collapsed_color;
                    // color_funciton.background = theme::sidebar_collapsed_color;
                }
            } else {
                // option_bar_button_text.sections[0].style.color = event.0.into();
            }
        }
    }
}
