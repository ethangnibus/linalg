// ui_plugin.rs
use bevy::prelude::*;
pub mod horizontal_split;
pub mod navbar;
mod root;
pub mod scrollable_page;
pub mod sidebar;
pub mod under_navbar;
pub mod vertical_split;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        // Add the setup_ui system as a startup system
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, (button_system, temporary));
    }
}

// Define your setup_ui and toggle resolution systems here
fn setup_ui(mut commands: Commands) {
    let root = root::setup(&mut commands);
}

fn temporary() {
    // temporary system added
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const PRESSED_BUTTON: Color = Color::rgb(0.0, 0.0, 0.0);

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
