use bevy::prelude::*;

// Marker for UI node
#[derive(Component)]
pub struct Navbar;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, temp);
    }
}

// pub fn setup(mut commands: Commands) {
//     //
//     println!("navbar.rs");
// }

pub fn setup(commands: &mut Commands, height: f32) -> Entity {
    let navbar = new(height);

    let navbar = commands.spawn(navbar).id();

    return navbar;
}

pub fn new(height: f32) -> NodeBundle {
    return NodeBundle {
        style: Style {
            height: Val::Percent(height),
            width: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            ..default()
        },
        background_color: Color::rgb(0.1, 0.1, 0.1).into(),
        ..default()
    };
}


fn temp() {}
