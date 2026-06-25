mod hero_plugin;

use bevy::{prelude::*, window::WindowResolution};

const HEIGHT: u32 = 720;
const WIDTH: u32 = 1080;

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
        .run();
}
