use crate::ui::util::style;
use crate::ui::util::theme;

use super::super::subsection_cameras::CameraBackgroundBanner;
use super::super::subsection_cameras;
use super::super::view::SvgHolder;
use super::super::util::subsection;
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
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    mut images: &mut ResMut<Assets<Image>>,
    view_list_entity: Entity,
) {
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
        )
    ).id();
    

    println!("Background_banner id: {:?}", background_banner);

    // camera_setup_writer.send(subsection_cameras::CameraSetupEvent);

    // let size = node.size();
    // let size = Extent3d {
    //     width: size.x.ceil() as u32,
    //     height: size.y.ceil() as u32,
    //     ..default()
    // };
    commands.entity(view_list_entity).push_children(&[background_banner]);
    let size = Extent3d {
        width: 1000,
        height: 1000,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: size.clone(),
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
        
    // fill image.data with zeroes
    image.resize(size.clone());
    let image_handle = images.add(image);
    

    let ui_image = UiImage { texture: image_handle.clone(), flip_x: false, flip_y: false };
    
    commands.entity(background_banner).insert(ui_image); // FIXME: this gets added multiple times.. bad

    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 4.0 }));
    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 0.75, 0.90),
        metallic: 20.0,
        reflectance: 0.02,
        unlit: false,
        ..default()
    });

    

    let first_pass_layer = RenderLayers::layer(1);

    let camera = commands.spawn(
        (
        Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(theme::background_color(&theme)),
                ..default()
            },
            camera: Camera {
                viewport: Some(Viewport {
                    physical_position: UVec2::new(0, 0),
                    physical_size: UVec2::new(
                        size.width.clone(),
                        size.height.clone(),
                    ),
                    ..default()
                }),
                // render before the "main pass" camera
                order: 1,
                target: RenderTarget::Image(image_handle),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            // projection: Projection::Perspective(
            //     PerspectiveProjection {
            //         aspect_ratio: 0.003,
            //         ..default()
            //     }
            // ),
            // projection: Projection::Orthographic(
            //     OrthographicProjection {
            //         scale: 0.1,
            //         scaling_mode: ScalingMode::AutoMax {max_width: 100.0, max_height: 100.0},
            //         ..default()
            //     }
            // ),
            ..default()
        },
        // UI config is a separate component
        UiCameraConfig {
            show_ui: false,
        },
        first_pass_layer,
        subsection_cameras::MiniCamera{number: 0},
    )).id();

    // commands.entity(camera_banner_entity).push_children(&[camera]);

    // The cube that will be rendered to the texture.
    let cube = commands.spawn((
        PbrBundle {
            mesh: cube_handle,
            material: cube_material_handle,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..default()
        },
        subsection_cameras::FirstPassCube,
        first_pass_layer,
        subsection::SubsectionGameEntity,
    )).id();

    // Light
    // NOTE: Currently lights are shared between passes - see https://github.com/bevyengine/bevy/issues/3462
    let light = commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                intensity: 100.0,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
            ..default()
        },
        subsection::SubsectionGameEntity,
    )).id();

    let film_crew = commands.spawn((
        subsection_cameras::FilmCrew {
            camera_entity: camera,
            mesh_entity: cube,
            light_entity: light,
        },
        SpatialBundle{
            ..default()
        },
    )).id();
    commands.entity(film_crew).push_children(&[camera, cube, light]);
}
