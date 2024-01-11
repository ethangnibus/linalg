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

pub mod routes;
pub mod sidebar_routes;
pub mod subsection_cameras;
mod util {
    pub mod theme;
    pub mod subsection;
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

pub struct SetupUiPlugin;

impl Plugin for SetupUiPlugin {
    fn build(&self, app: &mut App) {
        // Add the setup_ui system as a startup system
        app.add_plugins(root::SystemsPlugin)
            .add_plugins(scrollable_page::SystemsPlugin)
            .insert_resource(theme::CurrentTheme::Dark)
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
fn setup(
    mut commands: Commands,
    theme: Res<theme::CurrentTheme>,
) {
    let theme = theme.as_ref();
    root::setup(&mut commands, theme);
}

fn temp() {}
