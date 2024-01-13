use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    ui::FocusPolicy,
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    // winit::WinitSettings,
};
use super::util::{
    style,
    theme,
};
use super::view;
use super::under_navbar;
use super::chapter_container;

const OPTION_BAR_WIDTH: f32 = 500.0;

// Marker for UI node
#[derive(Component)]
pub struct OptionBar;

// Marker
#[derive(Component)]
pub struct ThemesHeader;

// Marker
#[derive(Component)]
pub struct LightThemeButton;

// Marker
#[derive(Component)]
pub struct DarkThemeButton;

// Marker
#[derive(Component)]
pub struct OptionBarSwiper;

#[derive(Event)]
pub struct OptionBarCollapseEvent(
    pub Color
);

#[derive(Event)]
pub struct OptionBarVisibilityEvent(pub Visibility);

#[derive(Resource)]
pub struct ShowingOptionBar(pub bool);

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(ShowingOptionBar(false))
        .add_event::<OptionBarCollapseEvent>()
        .add_event::<OptionBarVisibilityEvent>()
        // .add_plugins(chapter_container::SystemsPlugin);
        // .add_event::<SidebarScrollEvent>()
        .add_systems(
            Update,
            (
                option_bar_swiper_interacitons,
                option_bar_swiper_color_change_system,
                option_bar_visibility_system,
            ));
    }
}

pub fn setup(commands: &mut Commands, theme: &theme::CurrentTheme, width: f32) -> Entity {
    // let option_bar_holder = option_bar_holder(commands);
    let option_bar = option_bar(commands, theme, width);

    let themes_header = themes_header(commands, theme);
    let light_theme_button = light_theme_button(commands, theme);
    let dark_theme_button = dark_theme_button(commands, theme);

    commands.entity(option_bar).push_children(&[
        themes_header,
        light_theme_button,
        dark_theme_button,
    ]);

    return option_bar;
}

pub fn option_bar(commands: &mut Commands, theme: &theme::CurrentTheme, width: f32) -> Entity {
    return commands.spawn((
        OptionBar,
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                height: Val::Percent(100.0),
                width: Val::Percent(0.0),
                overflow: Overflow::clip(),
                ..default()
            },
            visibility: Visibility::Hidden,
            background_color: theme::background_color(theme).into(),
            ..default()
        }
    )).id();
}

pub fn themes_header(commands: &mut Commands, theme: &theme::CurrentTheme) -> Entity {
    let header_button = commands.spawn((
        ThemesHeader,
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
            background_color: Color::rgb(1.0, 0.7, 0.1).into(),
            border_color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..default()
        },
    )).id();


    let text_item = commands.spawn((
        TextBundle::from_section(
            "Themes",
            TextStyle {
                font_size: chapter_container::CHAPTER_BUTTON_FONT_SIZE,
                color: Color::rgb(0.0, 0.0, 0.0).into(),
                ..default()
            },
        ),
        Label,
        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
    )).id();


    commands.entity(header_button).push_children(&[text_item]);
    return header_button;
}

pub fn light_theme_button(commands: &mut Commands, theme: &theme::CurrentTheme) -> Entity {

    let light_theme_button = commands.spawn((
        LightThemeButton,
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
            background_color: theme::background_color(theme).into(),
            focus_policy: FocusPolicy::Block,
            ..default()
        },
    )).id();

    let text_item = commands.spawn((
        TextBundle::from_section(
            "Light",
            TextStyle {
                font_size: chapter_container::CHAPTER_BUTTON_FONT_SIZE,
                color: theme::navbar_text_color(theme),
                ..default()
            },
        )
    )).id();

    let bottom_line = commands.spawn((
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
            ..default()
        },
    )).id();

    commands
        .entity(light_theme_button)
        .push_children(&[text_item, bottom_line]);

    return light_theme_button;
}

pub fn dark_theme_button(commands: &mut Commands, theme: &theme::CurrentTheme) -> Entity {
    let dark_theme_button = commands.spawn((
        DarkThemeButton,
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
    )).id();

    let text_item = commands.spawn((
        TextBundle::from_section(
            "Dark",
            TextStyle {
                font_size: chapter_container::CHAPTER_BUTTON_FONT_SIZE,
                color: theme::navbar_text_color(theme),
                ..default()
            },
        )
    )).id();

    let bottom_line = commands.spawn((
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
    )).id();

    commands
        .entity(dark_theme_button)
        .push_children(&[text_item, bottom_line]);

    return dark_theme_button;
}



pub fn option_bar_swiper (commands: &mut Commands, theme: &theme::CurrentTheme) -> Entity {
    return commands.spawn((
        OptionBarSwiper,
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
            background_color: theme::background_color(theme).into(),
            border_color: theme::sidebar_color(theme).into(),
            ..default()
        },
    )).id();
}










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
                        sidebar_visibility_writer.send(OptionBarVisibilityEvent(Visibility::Hidden));
                    }
                    false => {
                        sidebar_visibility_writer
                            .send(OptionBarVisibilityEvent(Visibility::Inherited));
                    }
                }
                sidebar_swiper_color_writer.send(OptionBarCollapseEvent(
                    theme::NOT_A_COLOR,
                ));
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
                        .send(OptionBarCollapseEvent(theme::sidebar_color(theme)));
                }
            },
            Interaction::None => match showing_sidebar.0 {
                true => {
                    sidebar_swiper_color_writer
                        .send(OptionBarCollapseEvent(theme::sidebar_color(theme)));
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
    mut option_bar_swiper_query: Query<&mut BorderColor, With<OptionBarSwiper>>,
    mut option_bar_swiper_color_reader: EventReader<OptionBarCollapseEvent>,
) {
    for event in option_bar_swiper_color_reader.read() {
        for mut option_bar_swiper_border_color in &mut option_bar_swiper_query.iter_mut() {
            let color = event.0;

            if color != theme::NOT_A_COLOR {
                *option_bar_swiper_border_color = event.0.into();
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
