use bevy::{log::Level, prelude::*};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        let mat = LevelMatrix {
            matrix: [
                [
                    Tiles::Ground,
                    Tiles::Ground,
                    Tiles::Ground,
                    Tiles::Nothing,
                    Tiles::Nothing,
                ],
                [
                    Tiles::Nothing,
                    Tiles::Nothing,
                    Tiles::Nothing,
                    Tiles::Nothing,
                    Tiles::Nothing,
                ],
                [
                    Tiles::Nothing,
                    Tiles::Nothing,
                    Tiles::Nothing,
                    Tiles::Nothing,
                    Tiles::Nothing,
                ],
                [
                    Tiles::Nothing,
                    Tiles::Nothing,
                    Tiles::Nothing,
                    Tiles::Nothing,
                    Tiles::Nothing,
                ],
                [
                    Tiles::Nothing,
                    Tiles::Nothing,
                    Tiles::Nothing,
                    Tiles::Nothing,
                    Tiles::Nothing,
                ],
            ],
        };
        app.add_systems(Startup, setup).insert_resource(mat);
    }
}

enum Tiles {
    Nothing,
    Ground,
}
#[derive(Resource)]
struct LevelMatrix {
    matrix: [[Tiles; 5]; 5],
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    matrix: Res<LevelMatrix>,
) {
    let texture: Handle<Image> = asset_server.load("1_Industrial_Tileset_1.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 3, 1, None, None);

    let altas_layout = layouts.add(layout);

    for (r, row) in matrix.matrix.iter().enumerate() {
        for (c, tile) in row.iter().enumerate() {
            if let Tiles::Ground = tile {
                commands.spawn((
                    Sprite {
                        image: texture.clone(),
                        texture_atlas: Some(TextureAtlas {
                            index: 0,
                            layout: altas_layout.clone(),
                        }),
                        ..default()
                    },
                    Transform::from_translation(Vec3 {
                        x: (crate::WIDTH as f32) * (c as f32) / 5.0,
                        y: (crate::HEIGHT as f32) * (r as f32) / 5.0,
                        z: 0.0,
                    })
                    .with_scale(Vec3::splat(2.0)),
                ));
            }
        }
    }
}
