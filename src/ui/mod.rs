// ui_plugin.rs
use bevy::prelude::*;
pub mod navbar;
pub mod navbar_frame;
mod root;
pub mod scrollable_page;
pub mod sidebar;
pub mod sidebar_frame;
pub mod under_navbar;
pub mod view;
pub mod chapter_container;

pub struct SetupUiPlugin;

impl Plugin for SetupUiPlugin {
    fn build(&self, app: &mut App) {
        // Add the setup_ui system as a startup system
        app.add_plugins(root::SystemsPlugin)
            .add_plugins(scrollable_page::SystemsPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, temp);
    }
}

// #[derive(Component)]
// struct SidebarUI;

// fn sidebar_already_exists(q: Query<&SidebarUI>) -> bool {
//     println!("Running sidebar_already exists");
//     return q.get_single().is_ok();
// }

// fn spawn_button() {
//     println!("Spawning button");
// }

// fn when_colony_exists(q: Query<&Colony>) -> bool {
//     // q.get_single().is_ok()
//     return true;
// }

// Define your setup_ui and toggle resolution systems here
fn setup(mut commands: Commands) {
    root::setup(&mut commands);
}

fn temp() {}
