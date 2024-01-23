// ui_plugin.rs
use bevy::prelude::*;

use self::util::theme;
pub mod navbar;
pub mod navbar_frame;
mod root;
pub mod scrollable_page;
pub mod sidebar;
pub mod sidebar_frame;
pub mod under_navbar;
pub mod view;
pub mod chapter_container;
pub mod option_bar;

pub mod routes;
pub mod sidebar_routes;
pub mod subsection_cameras;
mod util {
    pub mod theme;
    pub mod subsection;
    pub mod style;
}
mod pages {
    pub mod chapter0section0subsection0;
    pub mod chapter0section1subsection0;
    pub mod chapter0section2subsection0;
    pub mod chapter0section3subsection0;
    pub mod chapter0section4subsection0;
    pub mod chapter1section0subsection0;
    pub mod chapter1section1subsection0;
    pub mod chapter1section1subsection1;
    pub mod chapter1section1subsection2;
    pub mod chapter1section1subsection3;
    pub mod chapter1section1subsection4;
    pub mod chapter1section2subsection0;
    pub mod chapter1section2subsection1;
    pub mod chapter1section2subsection2;
    pub mod chapter1section2subsection3;
    pub mod chapter1section2subsection4;
    pub mod chapter1section2subsection5;
    pub mod chapter1section3subsection0;
    pub mod chapter1section3subsection1;
    pub mod chapter1section3subsection2;
    pub mod chapter1section3subsection3;
    pub mod chapter1section3subsection4;
    pub mod chapter1section3subsection5;
    pub mod chapter1section4subsection0;
    pub mod chapter2section0subsection0;
    pub mod chapter2section1subsection0;
    pub mod chapter2section1subsection1;
    pub mod chapter2section1subsection2;
    pub mod chapter2section1subsection3;
    pub mod chapter2section2subsection0;
    pub mod chapter2section2subsection1;
    pub mod chapter2section2subsection2;
    pub mod chapter2section2subsection3;
    pub mod chapter2section2subsection4;
    pub mod chapter2section3subsection0;
    pub mod chapter2section3subsection1;
    pub mod chapter2section3subsection2;
    pub mod chapter2section3subsection3;
    pub mod chapter2section3subsection4;
    pub mod chapter2section3subsection5;
    pub mod chapter2section3subsection6;
    pub mod chapter2section4subsection0;
    pub mod chapter3section0subsection0;
    pub mod chapter3section1subsection0;
    pub mod chapter3section1subsection1;
    pub mod chapter3section1subsection2;
    pub mod chapter3section1subsection3;
    pub mod chapter3section1subsection4;
    pub mod chapter3section1subsection5;
    pub mod chapter3section2subsection0;
    pub mod chapter3section2subsection1;
    pub mod chapter3section2subsection2;
    pub mod chapter3section2subsection3;
    pub mod chapter3section2subsection4;
    pub mod chapter3section2subsection5;
    pub mod chapter3section2subsection6;
    pub mod chapter3section3subsection0;
    pub mod chapter3section3subsection1;
    pub mod chapter3section3subsection2;
    pub mod chapter3section3subsection3;
    pub mod chapter3section3subsection4;
    pub mod chapter3section3subsection5;
    pub mod chapter3section3subsection6;
    pub mod chapter3section4subsection0;
    pub mod chapter4section0subsection0;
    pub mod chapter4section1subsection0;
    pub mod chapter4section1subsection1;
    pub mod chapter4section1subsection2;
    pub mod chapter4section1subsection3;
    pub mod chapter4section2subsection0;
    pub mod chapter4section2subsection1;
    pub mod chapter4section2subsection2;
    pub mod chapter4section2subsection3;
    pub mod chapter4section3subsection0;
    pub mod chapter4section3subsection1;
    pub mod chapter4section3subsection2;
    pub mod chapter4section3subsection3;
    pub mod chapter4section3subsection4;
    pub mod chapter4section3subsection5;
    pub mod chapter4section4subsection0;
    pub mod chapter5section0subsection0;
    pub mod chapter5section1subsection0;
    pub mod chapter5section1subsection1;
    pub mod chapter5section1subsection2;
    pub mod chapter5section1subsection3;
    pub mod chapter5section1subsection4;
    pub mod chapter5section1subsection5;
    pub mod chapter5section2subsection0;
    pub mod chapter5section2subsection1;
    pub mod chapter5section2subsection2;
    pub mod chapter5section2subsection3;
    pub mod chapter5section2subsection4;
    pub mod chapter5section2subsection5;
    pub mod chapter5section3subsection0;
    pub mod chapter6section0subsection0;
    pub mod chapter6section1subsection0;
    pub mod chapter6section1subsection1;
    pub mod chapter6section1subsection2;
    pub mod chapter6section1subsection3;
    pub mod chapter6section1subsection4;
    pub mod chapter6section1subsection5;
    pub mod chapter6section1subsection6;
    pub mod chapter6section1subsection7;
    pub mod chapter6section2subsection0;
    pub mod chapter6section2subsection1;
    pub mod chapter6section2subsection2;
    pub mod chapter6section2subsection3;
    pub mod chapter6section2subsection4;
    pub mod chapter6section3subsection0;
    pub mod chapter7section0subsection0;
    pub mod chapter7section1subsection0;
    pub mod chapter7section1subsection1;
    pub mod chapter7section1subsection2;
    pub mod chapter7section1subsection3;
    pub mod chapter7section1subsection4;
    pub mod chapter7section1subsection5;
    pub mod chapter7section2subsection0;
    pub mod chapter7section2subsection1;
    pub mod chapter7section2subsection2;
    pub mod chapter7section2subsection3;
    pub mod chapter7section3subsection0;
    pub mod chapter7section3subsection1;
    pub mod chapter7section3subsection2;
    pub mod chapter7section3subsection3;
    pub mod chapter7section4subsection0;
    pub mod chapter8section0subsection0;
    pub mod chapter8section1subsection0;
    pub mod chapter8section1subsection1;
    pub mod chapter8section1subsection2;
    pub mod chapter8section1subsection3;
    pub mod chapter8section2subsection0;
    pub mod chapter8section2subsection1;
    pub mod chapter8section2subsection2;
    pub mod chapter8section2subsection3;
    pub mod chapter8section2subsection4;
    pub mod chapter8section2subsection5;
    pub mod chapter8section2subsection6;
    pub mod chapter8section3subsection0;
    pub mod chapter9section0subsection0;
    pub mod chapter9section1subsection0;
    pub mod chapter9section1subsection1;
    pub mod chapter9section1subsection2;
    pub mod chapter9section1subsection3;
    pub mod chapter9section2subsection0;
    pub mod chapter9section2subsection1;
    pub mod chapter9section2subsection2;
    pub mod chapter9section2subsection3;
    pub mod chapter9section2subsection4;
    pub mod chapter10section0subsection0;
    pub mod chapter10section1subsection0;
    pub mod chapter10section1subsection1;
    pub mod chapter10section1subsection2;
    pub mod chapter10section1subsection3;
    pub mod chapter10section2subsection0;
    pub mod chapter10section2subsection1;
    pub mod chapter10section2subsection2;
    pub mod chapter10section2subsection3;
    pub mod chapter10section3subsection0;
    pub mod chapter11section0subsection0;
    pub mod chapter11section1subsection0;
    pub mod chapter11section1subsection1;
    pub mod chapter11section1subsection2;
    pub mod chapter11section1subsection3;
    pub mod chapter11section1subsection4;
    pub mod chapter11section2subsection0;
    pub mod chapter11section2subsection1;
    pub mod chapter11section2subsection2;
    pub mod chapter11section2subsection3;
    pub mod chapter11section2subsection4;
    pub mod chapter11section2subsection5;
    pub mod chapter11section2subsection6;
    pub mod chapter11section3subsection0;
    pub mod chapter12section0subsection0;
    pub mod chapter12section1subsection0;
    pub mod chapter12section1subsection1;
    pub mod chapter12section1subsection2;
    pub mod chapter12section2subsection0;
    pub mod chapter12section2subsection1;
    pub mod chapter12section2subsection2;
    pub mod chapter12section2subsection3;
    pub mod chapter12section2subsection4;
    pub mod chapter12section3subsection0;
    pub mod chapter13section0subsection0;
    pub mod chapter13section1subsection0;
    pub mod chapter13section1subsection1;
    pub mod chapter13section1subsection2;
    pub mod chapter13section1subsection3;
    pub mod chapter13section1subsection4;
    pub mod chapter13section1subsection5;
    pub mod chapter13section1subsection6;
    pub mod chapter13section2subsection0;
    pub mod chapter13section2subsection1;
    pub mod chapter13section2subsection2;
    pub mod chapter13section2subsection3;
    pub mod chapter13section2subsection4;
    pub mod chapter13section3subsection0;
    pub mod chapter14section0subsection0;
    pub mod chapter14section1subsection0;
    pub mod chapter14section1subsection1;
    pub mod chapter14section1subsection2;
    pub mod chapter14section1subsection3;
    pub mod chapter14section1subsection4;
    pub mod chapter14section2subsection0;
    pub mod chapter14section2subsection1;
    pub mod chapter14section2subsection2;
    pub mod chapter14section2subsection3;
    pub mod chapter14section2subsection4;
    pub mod chapter14section2subsection5;
    pub mod chapter14section2subsection6;
    pub mod chapter14section3subsection0;
    pub mod chapter15section0subsection0;
    pub mod page_not_found;
    pub mod splash_page;
}

mod components{
    pub mod page_header;
    pub mod text_section;
    pub mod definition_block;
    pub mod definition_text_section;
    pub mod span_of_vectors_renderer;
    pub mod example_header;
    pub mod solution_header;
}

// FPS STUFF
use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

/// Marker to find the container entity so we can show/hide the FPS counter
#[derive(Component)]
struct FpsRoot;

/// Marker to find the text entity so we can update it
#[derive(Component)]
struct FpsText;

fn setup_fps_counter(
    mut commands: Commands,
) {
    // create our UI root node
    // this is the wrapper/container for the text
    let root = commands.spawn((
        FpsRoot,
        NodeBundle {
            // give it a dark background for readability
            background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
            // make it "always on top" by setting the Z index to maximum
            // we want it to be displayed over all other UI
            z_index: ZIndex::Global(i32::MAX),
            style: Style {
                position_type: PositionType::Absolute,
                // position it at the top-right corner
                // 1% away from the top window edge
                right: Val::Percent(1.),
                top: Val::Auto,
                // set bottom/left to Auto, so it can be
                // automatically sized depending on the text
                bottom: Val::Percent(1.),
                left: Val::Auto,
                // give it some padding for readability
                padding: UiRect::all(Val::Px(4.0)),
                ..Default::default()
            },
            ..Default::default()
        },
    )).id();
    // create our text
    let text_fps = commands.spawn((
        FpsText,
        TextBundle {
            // use two sections, so it is easy to update just the number
            text: Text::from_sections([
                TextSection {
                    value: "FPS: ".into(),
                    style: TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        // if you want to use your game's font asset,
                        // uncomment this and provide the handle:
                        // font: my_font_handle
                        ..default()
                    }
                },
                TextSection {
                    value: " N/A".into(),
                    style: TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        // if you want to use your game's font asset,
                        // uncomment this and provide the handle:
                        // font: my_font_handle
                        ..default()
                    }
                },
            ]),
            ..Default::default()
        },
    )).id();
    commands.entity(root).push_children(&[text_fps]);
}

fn fps_text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        // try to get a "smoothed" FPS value from Bevy
        if let Some(value) = diagnostics
            .get(FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            // Format the number as to leave space for 4 digits, just in case,
            // right-aligned and rounded. This helps readability when the
            // number changes rapidly.
            text.sections[1].value = format!("{value:>4.0}");

            // Let's make it extra fancy by changing the color of the
            // text according to the FPS value:
            text.sections[1].style.color = if value >= 120.0 {
                // Above 120 FPS, use green color
                Color::rgb(0.0, 1.0, 0.0)
            } else if value >= 60.0 {
                // Between 60-120 FPS, gradually transition from yellow to green
                Color::rgb(
                    (1.0 - (value - 60.0) / (120.0 - 60.0)) as f32,
                    1.0,
                    0.0,
                )
            } else if value >= 30.0 {
                // Between 30-60 FPS, gradually transition from red to yellow
                Color::rgb(
                    1.0,
                    ((value - 30.0) / (60.0 - 30.0)) as f32,
                    0.0,
                )
            } else {
                // Below 30 FPS, use red color
                Color::rgb(1.0, 0.0, 0.0)
            }
        } else {
            // display "N/A" if we can't get a FPS measurement
            // add an extra space to preserve alignment
            text.sections[1].value = " N/A".into();
            text.sections[1].style.color = Color::WHITE;
        }
    }
}

/// Toggle the FPS counter when pressing F12
fn fps_counter_showhide(
    mut q: Query<&mut Visibility, With<FpsRoot>>,
    kbd: Res<Input<KeyCode>>,
) {
    if kbd.just_pressed(KeyCode::F12) {
        let mut vis = q.single_mut();
        *vis = match *vis {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
}
// FPS STUFF END

pub struct SetupUiPlugin;

impl Plugin for SetupUiPlugin {
    fn build(&self, app: &mut App) {
        // Add the setup_ui system as a startup system
        app.add_plugins(root::SystemsPlugin)
            .add_plugins(scrollable_page::SystemsPlugin)
            .insert_resource(theme::CurrentTheme::Dark)

            //FPS STUFF
            .add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Startup, setup_fps_counter) 
            .add_systems(Update, (
                fps_text_update_system,
                fps_counter_showhide,
            ))
            //FPS STUFF END


            .add_systems(Startup, setup)
            .add_systems(Update, temp);
    }
}



// Define your setup_ui and toggle resolution systems here
fn setup(
    mut commands: Commands,
    mut routing_event_writer: EventWriter<routes::RoutingEvent>,
    theme: Res<theme::CurrentTheme>,
) {
    let theme = theme.as_ref();
    root::setup(&mut commands, theme);
    routing_event_writer.send(routes::RoutingEvent {
        chapter_number: 0,
        section_number: 0,
        subsection_number: 0,
    });
}


fn temp() {}
