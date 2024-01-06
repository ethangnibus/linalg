use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    // winit::WinitSettings,
};
use bevy_svg::prelude::*;
use super::super::view::SvgLoadEvent;
use super::super::view::SvgHolder;


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


pub fn svg(commands: &mut Commands, asset_server: & Res<AssetServer>, svg_load_writer: &mut EventWriter<SvgLoadEvent>, image_path: String, ratio: f32) -> Entity {

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

    // make the text that appears on the banner
    // println!("made it to before svg");
    // let svg = asset_server.load("result.svg");
    // println!("{:?}", svg);
    // println!("made it to load svg");
    // let svg_bundle = commands.spawn((
    //     Svg2dBundle {
    //         svg,
    //         transform: Transform::from_scale(Vec3{x: 0.1, y: 0.1, z: 0.1}),
    //         origin: Origin::Center, // Origin::TopLeft is the default
    //         ..Default::default()
    //     },
    //     Label,
    //     AccessibilityNode(NodeBuilder::new(Role::ListItem)),
    // )).id();
    // println!("made it to make svg bundle");

    // commands.entity(background_banner).push_children(&[svg_bundle]);

    // svg_load_writer.send(
    //     SvgLoadEvent{
    //         entity: background_banner,
    //         file_name: "hello".into()


    //     }
    // );

    // put load event here

    
    // img.texture_descriptor.size.width

    // ********** Dude you gotta make svg's into objects then put those objects into a world
    // and look at it with a camera. It's the only way....

    
    println!("made it to push svg bundle");
    return background_banner;
}