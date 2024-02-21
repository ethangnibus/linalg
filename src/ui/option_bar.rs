use super::chapter_container;
use super::subsection_cameras;
use super::under_navbar;
use super::util::{
    style,
    theme::{self, ColorFunction},
};
use super::view;
use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    render::color,
    // winit::WinitSettings,
    ui::FocusPolicy,
};

const OPTION_BAR_WIDTH: f32 = 500.0;

// Marker for UI node
#[derive(Component)]
pub struct OptionBar;

// Marker
#[derive(Component)]
pub struct ThemesHeader;

// Marker
#[derive(Component)]
pub struct OptionBarSwiper;

#[derive(Event)]
pub struct OptionBarCollapseEvent(pub Color);

#[derive(Event)]
pub struct OptionBarVisibilityEvent(pub Visibility);

#[derive(Event)]
pub struct ThemeButtonColorEvent {
    pub theme: theme::CurrentTheme,
    pub color: Color,
}

#[derive(Resource)]
pub struct ShowingOptionBar(pub bool);

#[derive(Component)]
pub struct DarkThemeButtonText;
#[derive(Component)]
pub struct DarkThemeButtonLine;
#[derive(Component)]
pub struct LightThemeButtonText;
#[derive(Component)]
pub struct LightThemeButtonLine;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ShowingOptionBar(false))
            .add_event::<OptionBarCollapseEvent>()
            .add_event::<OptionBarVisibilityEvent>()
            .add_event::<ThemeButtonColorEvent>()
            .add_event::<theme::ThemeChangeEvent>()
            // .add_plugins(chapter_container::SystemsPlugin);
            // .add_event::<SidebarScrollEvent>()
            .add_systems(
                Update,
                (
                    option_bar_swiper_interacitons,
                    option_bar_swiper_color_change_system,
                    option_bar_visibility_system.after(subsection_cameras::resize_camera_system),
                    themes_header_color_change_system,
                    theme_button_interaction,
                    theme_change_node_color_change_system,
                    theme_change_text_color_change_system,
                    theme_button_text_color_change_system,
                    theme_button_line_color_change_system,
                    theme_change_mesh_color_change_system,
                ),
            );
    }
}

pub fn setup(
    commands: &mut Commands,
    theme: &theme::CurrentTheme,
    width: f32,
    option_bar_entity: Entity,
) -> Entity {
    // let option_bar_holder = option_bar_holder(commands);
    let option_bar = option_bar_entity;

    let themes_header = themes_header(commands, theme);
    let light_theme_button = theme_button(commands, theme, theme::CurrentTheme::Light, "Light");
    let dark_theme_button = theme_button(commands, theme, theme::CurrentTheme::Dark, "Dark");
    let matrix_theme_button = theme_button(commands, theme, theme::CurrentTheme::Matrix, "Matrix");
    let cyberpunk_theme_button =
        theme_button(commands, theme, theme::CurrentTheme::Cyberpunk, "CyberPunk");

    commands.entity(option_bar).push_children(&[
        themes_header,
        light_theme_button,
        dark_theme_button,
        matrix_theme_button,
        cyberpunk_theme_button,
    ]);

    return option_bar;
}

pub fn option_bar(commands: &mut Commands, theme: &theme::CurrentTheme, width: f32) -> Entity {
    return commands
        .spawn((
            OptionBar,
            theme::ColorFunction {
                background: theme::background_color,
                border: theme::background_color,
            },
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_self: AlignSelf::Stretch,
                    height: Val::Percent(100.0),
                    width: Val::Percent(0.0),
                    max_width: Val::Vw(100.0),
                    overflow: Overflow::clip(),
                    ..default()
                },
                visibility: Visibility::Hidden,
                background_color: theme::background_color(theme).into(),
                ..default()
            },
        ))
        .id();
}

pub fn themes_header(commands: &mut Commands, theme: &theme::CurrentTheme) -> Entity {
    let header_button = commands
        .spawn((
            ThemesHeader,
            theme::ColorFunction {
                background: theme::option_bar_header_color,
                border: theme::sidebar_collapsed_color,
            },
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    min_height: chapter_container::HEADER_BUTTON_HEIGHT,
                    // height: Val::Percent(100.0),
                    border: chapter_container::HEADER_BUTTON_BORDER,
                    padding: chapter_container::HEADER_BUTTON_BORDER,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: theme::option_bar_header_color(&theme).into(),
                border_color: theme::sidebar_collapsed_color(&theme).into(),
                ..default()
            },
        ))
        .id();

    let text_item = commands
        .spawn((
            theme::ColorFunction {
                background: theme::option_bar_header_text_color,
                border: theme::option_bar_header_text_color,
            },
            TextBundle::from_section(
                "Themes",
                TextStyle {
                    font_size: chapter_container::CHAPTER_BUTTON_FONT_SIZE,
                    color: theme::option_bar_header_text_color(&theme),
                    ..default()
                },
            ),
            Label,
            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
        ))
        .id();

    commands.entity(header_button).push_children(&[text_item]);
    return header_button;
}

pub fn theme_button(
    commands: &mut Commands,
    theme: &theme::CurrentTheme,
    next_theme: theme::CurrentTheme,
    text: &str,
) -> Entity {
    let theme_button_entity = commands
        .spawn((
            theme::ThemeButton {
                next_theme: next_theme,
            },
            theme::ColorFunction {
                background: theme::background_color,
                border: theme::background_color,
            },
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    min_height: chapter_container::SIDEBAR_BUTTON_HEIGHT,
                    border: chapter_container::CHAPTER_BUTTON_BORDER,
                    padding: chapter_container::CHAPTER_BUTTON_BORDER,
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::Start,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                focus_policy: FocusPolicy::Block,
                background_color: theme::background_color(theme).into(),
                ..default()
            },
        ))
        .id();

    let text_item = commands
        .spawn((
            theme::ThemeButtonText {
                next_theme: next_theme,
            },
            theme::ColorFunction {
                background: theme::sidebar_color,
                border: theme::sidebar_color,
            },
            TextBundle::from_section(
                text,
                TextStyle {
                    font_size: chapter_container::CHAPTER_BUTTON_FONT_SIZE,
                    color: theme::navbar_text_color(theme),
                    ..default()
                },
            ),
        ))
        .id();

    let bottom_line = commands
        .spawn((
            theme::ThemeButtonLine {
                next_theme: next_theme,
            },
            theme::ColorFunction {
                background: theme::sidebar_color,
                border: theme::sidebar_color,
            },
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(1.0),
                    border: chapter_container::HIDDEN_BUTTON_BORDER,
                    padding: chapter_container::HIDDEN_BUTTON_BORDER,
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::Start,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: theme::navbar_text_color(theme).into(),
                // border_color: Color::rgb(0.1, 0.1, 0.1).into(),
                ..default()
            },
        ))
        .id();

    commands
        .entity(theme_button_entity)
        .push_children(&[text_item, bottom_line]);

    return theme_button_entity;
}

pub fn option_bar_swiper(commands: &mut Commands, theme: &theme::CurrentTheme) -> Entity {
    return commands
        .spawn((
            OptionBarSwiper,
            theme::ColorFunction {
                background: theme::swiper_background_color,
                border: theme::sidebar_collapsed_color,
            },
            ButtonBundle {
                style: Style {
                    // width: Val::Percent(1.0),
                    width: style::SWIPERS_WIDTH,
                    // flex_grow: 1.0,
                    height: Val::Percent(100.0),
                    border: UiRect {
                        left: Val::Px(0.0),
                        right: Val::Px(2.0),
                        top: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                    },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                focus_policy: FocusPolicy::Block,
                background_color: theme::swiper_background_color(theme).into(),
                border_color: theme::sidebar_collapsed_color(theme).into(),
                ..default()
            },
        ))
        .id();
}

// ========================= Theme button events =========================
fn theme_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &theme::ThemeButton),
        (Changed<Interaction>, With<theme::ThemeButton>),
    >,
    mut theme_button_color_writer: EventWriter<ThemeButtonColorEvent>,
    mut theme_change_writer: EventWriter<theme::ThemeChangeEvent>,
    mut theme: ResMut<theme::CurrentTheme>,
) {
    for (mut interaction, mut theme_button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *theme = theme_button.next_theme;
                theme_change_writer.send(theme::ThemeChangeEvent);
            }
            Interaction::Hovered => theme_button_color_writer.send(ThemeButtonColorEvent {
                theme: theme_button.next_theme,
                color: theme::sidebar_collapsed_color(&theme),
            }),
            Interaction::None => theme_button_color_writer.send(ThemeButtonColorEvent {
                theme: theme_button.next_theme,
                color: theme::sidebar_color(&theme),
            }),
        }
    }
}

fn theme_change_node_color_change_system(
    mut theme_change_reader: EventReader<theme::ThemeChangeEvent>,
    mut node_query: Query<
        (&mut BackgroundColor, &mut BorderColor, &ColorFunction),
        With<ColorFunction>,
    >,
    theme: Res<theme::CurrentTheme>,
) {
    for event in theme_change_reader.read() {
        for (mut background_color, mut border_color, ColorFunction) in node_query.iter_mut() {
            *background_color = (ColorFunction.background)(&theme).into();
            *border_color = (ColorFunction.border)(&theme).into();
        }
    }
}

fn theme_change_text_color_change_system(
    mut theme_change_reader: EventReader<theme::ThemeChangeEvent>,
    mut text_query: Query<(&mut Text, &ColorFunction), With<ColorFunction>>,
    theme: Res<theme::CurrentTheme>,
) {
    for event in theme_change_reader.read() {
        for (mut text, color_function) in text_query.iter_mut() {
            text.sections[0].style.color = (color_function.background)(&theme).into();
        }
    }
}

fn theme_change_mesh_color_change_system(
    mut theme_change_reader: EventReader<theme::ThemeChangeEvent>,
    mut mesh_query: Query<(&Handle<StandardMaterial>, &ColorFunction), With<ColorFunction>>,

    mut materials: ResMut<Assets<StandardMaterial>>,
    theme: Res<theme::CurrentTheme>,
) {
    for event in theme_change_reader.read() {
        for (material_handle, color_function) in mesh_query.iter_mut() {
            let material = materials.get_mut(material_handle).unwrap(); // Fixme: Change in case of bugs
            
            material.base_color = (color_function.background)(&theme).into();
            // material.emissive = (color_function.border)(&theme).into();
        }
    }
}

fn theme_button_text_color_change_system(
    mut theme_button_color_reader: EventReader<ThemeButtonColorEvent>,
    mut theme_change_reader: EventReader<theme::ThemeChangeEvent>,
    mut text_query: Query<
        (&mut Text, &ColorFunction, &theme::ThemeButtonText),
        With<theme::ThemeButtonText>,
    >,
    theme: Res<theme::CurrentTheme>,
) {
    for event in theme_change_reader.read() {
        for (mut text, color_function, _theme_button_text) in text_query.iter_mut() {
            text.sections[0].style.color = (color_function.background)(&theme).into();
        }
    }
    let theme = theme.as_ref();
    for event in theme_button_color_reader.read() {
        for (mut text, _color_function, theme_button_text) in text_query.iter_mut() {
            if event.theme == theme_button_text.next_theme {
                text.sections[0].style.color = event.color;
            }
        }
    }
}

fn theme_button_line_color_change_system(
    mut theme_button_color_reader: EventReader<ThemeButtonColorEvent>,
    mut theme_change_reader: EventReader<theme::ThemeChangeEvent>,
    mut line_query: Query<
        (
            &mut BackgroundColor,
            &ColorFunction,
            &theme::ThemeButtonLine,
        ),
        With<theme::ThemeButtonLine>,
    >,
    theme: Res<theme::CurrentTheme>,
) {
    for event in theme_button_color_reader.read() {
        for (mut background_color, _color_function, theme_button_line) in line_query.iter_mut() {
            if event.theme == theme_button_line.next_theme {
                *background_color = event.color.into();
            }
        }
    }
}

// ========================= Option bar collapse systems =========================
fn option_bar_swiper_interacitons(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<OptionBarSwiper>)>,
    mut showing_sidebar: ResMut<ShowingOptionBar>,
    theme: Res<theme::CurrentTheme>,
    // mut sidebar_query: Query<&mut BackgroundColor, With<sidebar::Sidebar>>,
    // mut header_button_query: Query<&BackgroundColor, With<HeaderButton>>,
    mut sidebar_swiper_color_writer: EventWriter<OptionBarCollapseEvent>,
    mut sidebar_visibility_writer: EventWriter<OptionBarVisibilityEvent>,
) {
    let theme = theme.as_ref();
    for interaction in interaction_query.iter() {
        match *interaction {
            Interaction::Pressed => {
                match showing_sidebar.0 {
                    true => {
                        sidebar_visibility_writer
                            .send(OptionBarVisibilityEvent(Visibility::Hidden));
                    }
                    false => {
                        sidebar_visibility_writer
                            .send(OptionBarVisibilityEvent(Visibility::Inherited));
                    }
                }
                sidebar_swiper_color_writer.send(OptionBarCollapseEvent(theme::NOT_A_COLOR));
                showing_sidebar.0 = !showing_sidebar.0;
            }
            Interaction::Hovered => match showing_sidebar.0 {
                true => {
                    sidebar_swiper_color_writer.send(OptionBarCollapseEvent(
                        theme::sidebar_collapsed_color(theme),
                    ));
                }
                false => {
                    sidebar_swiper_color_writer
                        .send(OptionBarCollapseEvent(theme::navbar_swiper_color(theme)));
                }
            },
            Interaction::None => match showing_sidebar.0 {
                true => {
                    sidebar_swiper_color_writer
                        .send(OptionBarCollapseEvent(theme::navbar_swiper_color(theme)));
                }
                false => {
                    sidebar_swiper_color_writer.send(OptionBarCollapseEvent(
                        theme::sidebar_collapsed_color(theme),
                    ));
                }
            },
        }
    }
}

fn option_bar_swiper_color_change_system(
    mut option_bar_swiper_query: Query<(&mut BorderColor, &mut ColorFunction), With<OptionBarSwiper>>,
    mut option_bar_swiper_color_reader: EventReader<OptionBarCollapseEvent>,
) {
    for event in option_bar_swiper_color_reader.read() {
        for (mut option_bar_swiper_border_color, mut color_function) in &mut option_bar_swiper_query.iter_mut() {
            let color = event.0;

            if color != theme::NOT_A_COLOR {
                *option_bar_swiper_border_color = event.0.into();
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

fn option_bar_visibility_system(
    mut option_bar_query: Query<(&mut Visibility, &mut Style), With<OptionBar>>,
    mut option_bar_visibility_event: EventReader<OptionBarVisibilityEvent>,
    mut ui_resize_writer: EventWriter<view::UiResizeEvent>,
) {
    for event in option_bar_visibility_event.read() {
        for (mut sidebar_visibility, mut sidebar_style) in &mut option_bar_query.iter_mut() {
            let visibility_kind: Visibility = event.0.into();
            match visibility_kind {
                Visibility::Hidden => {
                    *sidebar_visibility = Visibility::Hidden;
                    sidebar_style.width = Val::Percent(0.0);
                }
                Visibility::Visible => {
                    *sidebar_visibility = Visibility::Visible;
                    sidebar_style.width = Val::Px(OPTION_BAR_WIDTH);
                }
                Visibility::Inherited => {
                    *sidebar_visibility = Visibility::Inherited;
                    sidebar_style.width = Val::Px(OPTION_BAR_WIDTH);
                }
            }
        }
        ui_resize_writer.send(view::UiResizeEvent);
    }
}

fn themes_header_color_change_system(
    mut themes_header_query: Query<(&mut BorderColor, &mut theme::ColorFunction), With<ThemesHeader>>,
    // mut sidebar_button_query: Query<&mut BorderColor, With<navbar::SidebarButton>>,
    mut option_bar_collape_reader: EventReader<OptionBarCollapseEvent>,
) {
    for event in option_bar_collape_reader.read() {
        for (mut themes_header_color, mut color_function) in &mut themes_header_query.iter_mut() {
            let color = event.0;
            if color != theme::NOT_A_COLOR {
                *themes_header_color = color.into();
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
