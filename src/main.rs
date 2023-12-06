use bevy::prelude::*;

#[derive(Component)]
struct MyCameraMarker;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(10.0, 12.0, 16.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MyCameraMarker,
    ));
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
fn main() {
    App::new()
        //set the global default
        .insert_resource(ClearColor(Color::rgb(0.9, 0.3, 0.6)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, setup_camera))
        .run();
}
