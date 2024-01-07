use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    window::WindowRef,
    prelude::*,
    // winit::WinitSettings,
};
use bevy_svg::prelude::*;
use super::super::view::SvgLoadEvent;
use super::super::view::SvgHolder;
use super::super::view::MyMinimapCamera;
use bevy::render::camera::Viewport;



pub fn spawn(commands: &mut Commands, text: &str) -> Entity {

    // make banner behind the text
    let background_banner = commands.spawn(
        NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
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
                bottom: Val::Px(0.0),
            },
            justify_items: JustifyItems::Start,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgb(0.1, 0.1, 0.1).into(),
        border_color: Color::rgb(0.1, 0.1, 0.1).into(),
        ..default()
    }).id();

    // make the text that appears on the banner
    let text_bundle = commands.spawn((
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
    )).id();

    commands.entity(background_banner).push_children(&[text_bundle]);
    
    return background_banner;
}


pub fn image(commands: &mut Commands, asset_server: & Res<AssetServer>, svg_load_writer: &mut EventWriter<SvgLoadEvent>, image_path: String, ratio: f32) -> Entity {

    let img = UiImage::new(asset_server.load(image_path));
    // make banner behind the text
    let background_banner = commands.spawn((
        NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Vw(ratio),
            padding: UiRect {
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
            },
            border: UiRect {
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
            },
            justify_items: JustifyItems::Start,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgb(1.0, 1.0, 1.0).into(),
        border_color: Color::rgb(0.1, 0.5, 0.1).into(),
        ..default()
    },
    img
    )).id();
    return background_banner;
}


pub fn camera(commands: &mut Commands, asset_server: & Res<AssetServer>, svg_load_writer: &mut EventWriter<SvgLoadEvent>, image_path: &String, ratio: f32) -> Entity {

    let img = UiImage::new(asset_server.load(image_path));
    // make banner behind the text
    let background_banner = commands.spawn((
        NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Px(500.0),
            padding: UiRect {
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
            },
            border: UiRect {
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
            },
            justify_items: JustifyItems::Start,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgb(0.1, 0.3, 0.1).into(),
        border_color: Color::rgb(0.1, 0.5, 0.1).into(),
        ..default()
    },
    MyMinimapCamera,
    // img
    )).id();

    // svg_load_writer.send(
    //     SvgLoadEvent{
    //         entity: background_banner,
    //         file_name: image_path.to_string(),
    //     }
    // );


    return background_banner;
}