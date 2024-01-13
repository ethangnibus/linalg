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

// Marker for UI node
#[derive(Component)]
pub struct OptionBar;

// Marker
#[derive(Component)]
pub struct OptionBarSwiper;

#[derive(Resource)]
pub struct ShowingOptionBar(pub bool);

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(ShowingOptionBar(true))
        // .add_plugins(chapter_container::SystemsPlugin);
        // .add_event::<SidebarScrollEvent>()
        .add_systems(Update, (temp));
    }
}

pub fn setup(commands: &mut Commands, width: f32) -> Entity {
    // let option_bar_holder = option_bar_holder(commands);

    let option_bar = option_bar(commands, width);
    return option_bar;
}

pub fn option_bar(commands: &mut Commands, width: f32) -> Entity {
    return commands.spawn((
        OptionBar,
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                height: Val::Percent(100.0),
                width: Val::Percent(width),
                overflow: Overflow::clip(),
                ..default()
            },
            background_color: Color::rgb(1.0, 0.0, 1.0).into(),
            ..default()
        }
    )).id();
}

pub fn option_bar_holder(commands: &mut Commands) -> Entity {
    return commands.spawn((
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                // flex_grow: 1.0,
                width: Val::Auto,
                flex_grow: 1.0,
                overflow: Overflow::clip(),
                ..default()
            },
            ..default()
        }
    )).id();
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

fn temp() {}