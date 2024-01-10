use super::chapter_container::HeaderButton;
use super::sidebar;
use super::sidebar_frame;
use super::util;
use super::view;
use bevy::winit::WinitWindows;
use bevy::{prelude::*, ui::FocusPolicy};

const SWIPERS_COLOR_DEFAULT: BackgroundColor = BackgroundColor(Color::rgb(0.1, 0.1, 0.1));

// Marker for Node
#[derive(Component)]
pub struct UnderNavbar;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(sidebar_frame::SystemsPlugin)
            .add_plugins(sidebar::SystemsPlugin)
            .add_plugins(view::SystemsPlugin);
    }
}

// Returns root node
pub fn setup(commands: &mut Commands, width: f32, height: f32) -> Entity {
    // Make ECS for root and navbar
    // return entities
    let under_navbar = sidebar_frame::setup(commands, width, height);
    let sidebar = sidebar::setup(commands, util::SIDEBAR_WIDTH);

    let right_border = right_swiper();
    let right_border = commands.spawn(right_border).id();

    let view = view::setup(commands);

    // make under_navbar parent of sidebar and scrollable_page
    commands
        .entity(under_navbar)
        .push_children(&[sidebar, view, right_border]);

    return under_navbar;
}

pub fn right_swiper() -> (NodeBundle) {
// pub fn right_swiper() -> (NodeBundle, ShowingSidebar) {
    return (
        NodeBundle {
            style: Style {
                // width: Val::Percent(1.0),
                width: util::SWIPERS_WIDTH,
                height: Val::Percent(100.0),
                border: UiRect::all(Val::Px(0.0)),
                ..default()
            },
            background_color: SWIPERS_COLOR_DEFAULT,
            ..default()
        }
        // ShowingSidebar(true),
    );
}
