mod camera;
mod hero_plugin;
mod level_designer;
mod physics;
use bevy::{prelude::*, window::WindowResolution};

const HEIGHT: u32 = 800;
const WIDTH: u32 = 800;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(WIDTH, HEIGHT),
                        title: "Upside-down".into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(hero_plugin::HeroPlugin)
        .add_plugins(level_designer::LevelPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(physics::PhysicsDebugPlugin)
        .add_systems(Update, exit_game)
        .run();
}

fn exit_game(keys: Res<ButtonInput<KeyCode>>, mut exit: MessageWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
