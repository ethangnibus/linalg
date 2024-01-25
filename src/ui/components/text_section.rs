use crate::ui::util::style;
use crate::ui::util::theme;

use super::super::subsection_cameras::CameraBackgroundBanner;
use super::super::subsection_cameras;
use super::super::view::SvgHolder;
use bevy::render::camera::Viewport;
use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    core_pipeline::clear_color::ClearColorConfig,
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    // winit::WinitSettings,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    window::WindowRef,
};
// use bevy_svg::prelude::*;

pub fn spawn(commands: &mut Commands, text: &str) -> Entity {
    // make banner behind the text
    let background_banner = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0), // FIXME: fix so navbar isnt weird when going to 1.2.4
                height: Val::Auto,
                padding: UiRect {
                    left: Val::Px(4.0),
                    right: Val::Px(0.0),
                    top: Val::Px(14.0),
                    bottom: Val::Px(14.0),
                },
                border: UiRect {
                    left: Val::Px(0.0),
                    right: Val::Px(0.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(4.0),
                },
                justify_items: JustifyItems::Start,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgb(0.1, 0.1, 0.1).into(),
            border_color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..default()
        })
        .id();

    // make the text that appears on the banner
    let text_bundle = commands
        .spawn((
            TextBundle::from_section(
                // format!(text),
                text,
                TextStyle {
                    font_size: 20.,
                    ..default()
                },
            ),
            Label,
            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
            SvgHolder,
        ))
        .id();

    commands
        .entity(background_banner)
        .push_children(&[text_bundle]);

    return background_banner;
}

pub fn image(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    svg_load_writer: &mut EventWriter<subsection_cameras::CameraSetupEvent>,
    image_path: String,
    ratio: f32,
) -> Entity {
    let img = UiImage::new(asset_server.load(image_path));
    // make banner behind the text
    let background_banner = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Vw(ratio),
                    padding: UiRect {
                        left: Val::Px(0.0),
                        right: Val::Px(0.0),
                        top: Val::Px(0.0),
                        bottom: Val::Px(4.0),
                    },
                    border: UiRect {
                        left: Val::Px(0.0),
                        right: Val::Px(0.0),
                        top: Val::Px(0.0),
                        bottom: Val::Px(4.0),
                    },
                    justify_items: JustifyItems::Start,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(1.0, 1.0, 1.0).into(),
                border_color: Color::rgb(0.1, 0.1, 0.1).into(),
                ..default()
            },
            img,
        ))
        .id();
    return background_banner;
}

pub fn camera(
    commands: &mut Commands,
    theme: &theme::CurrentTheme,
    camera_setup_writer: &mut EventWriter<subsection_cameras::CameraSetupEvent>,
    image_path: &String,
    ratio: f32,
    height: Val,
) -> Entity {
    // make banner behind the text
    let background_banner = commands
        .spawn((
            // theme::ColorFunction {
            //     background: theme::background_color,
            //     border: theme::background_color,
            // },
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: height,
                    padding: style::NO_BORDER,
                    border: style::NO_BORDER,
                    justify_items: JustifyItems::Start,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::WHITE.into(), // FIXME: Change to background color and change to white when camera loads
                ..default()
            },
            CameraBackgroundBanner,
            // image,
        ))
        .id();

    println!("Background_banner id: {:?}", background_banner);

    camera_setup_writer.send(subsection_cameras::CameraSetupEvent {
        entity: background_banner,
        file_name: image_path.to_string(),
    });

    return background_banner;
}
