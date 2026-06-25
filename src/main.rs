mod hero_plugin;
mod level_designer;

use bevy::{prelude::*, window::WindowResolution};

const HEIGHT: u32 = 900;
const WIDTH: u32 = 1000;

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
        .run();
}
