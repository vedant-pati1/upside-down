use bevy::prelude::*;
use std::collections::HashMap;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        let mat = LevelMatrix::generate_from_numeric_mat(vec![
            vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        ]);

        app.add_systems(Startup, setup).insert_resource(mat);
    }
}

impl LevelMatrix {
    fn generate_from_numeric_mat(input: Vec<Vec<u32>>) -> Self {
        let map: HashMap<u32, Tiles> = [(0, Tiles::Nothing), (1, Tiles::Ground)]
            .into_iter()
            .collect();

        let rows = input.len();
        let columns = input[0].len();

        let matrix = input
            .into_iter()
            .map(|row| row.into_iter().map(|val| map[&val]).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self {
            rows,
            columns,
            matrix,
        }
    }
}

#[derive(Clone, Copy)]
enum Tiles {
    Nothing,
    Ground,
}
#[derive(Resource)]
struct LevelMatrix {
    rows: usize,
    columns: usize,
    matrix: Vec<Vec<Tiles>>,
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
            let world_x: i32 = c as i32 - (matrix.columns as i32) / 2;
            let world_y: i32 = (matrix.rows as i32) / 2 - (r as i32);
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
                        x: (crate::WIDTH as f32) * (world_x as f32) / (matrix.columns as f32),
                        y: (crate::HEIGHT as f32) * (world_y as f32) / (matrix.rows as f32),
                        z: 0.0,
                    })
                    .with_scale(Vec3::splat(2.5)),
                ));
            };
        }
    }
}
