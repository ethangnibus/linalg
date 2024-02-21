use bevy::prelude::*;

pub mod chapter_0;
pub mod chapter_1;
pub mod chapter_2;
pub mod chapter_3;
pub mod chapter_4;
pub mod chapter_5;
pub mod chapter_6;
pub mod chapter_7;
pub mod chapter_8;
pub mod chapter_9;
pub mod chapter_10;
pub mod chapter_11;
pub mod chapter_12;
pub mod chapter_13;
pub mod chapter_14;
pub mod chapter_15;
pub mod page_not_found;
pub mod splash_page;


pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // Add the setup_ui system as a startup system
        app.add_plugins(chapter_0::SystemsPlugin)
        .add_plugins(chapter_1::SystemsPlugin)
        .add_plugins(chapter_2::SystemsPlugin)
        .add_plugins(chapter_3::SystemsPlugin)
        .add_plugins(chapter_4::SystemsPlugin)
        .add_plugins(chapter_5::SystemsPlugin)
        .add_plugins(chapter_6::SystemsPlugin)
        .add_plugins(chapter_7::SystemsPlugin)
        .add_plugins(chapter_8::SystemsPlugin)
        .add_plugins(chapter_9::SystemsPlugin)
        .add_plugins(chapter_10::SystemsPlugin)
        .add_plugins(chapter_11::SystemsPlugin)
        .add_plugins(chapter_12::SystemsPlugin)
        .add_plugins(chapter_13::SystemsPlugin)
        .add_plugins(chapter_14::SystemsPlugin)
        .add_plugins(chapter_15::SystemsPlugin);
    }
}