use bevy::prelude::*;
use std::f32;
use std::time::Duration;

use crate::physics::translation_using_vel;

use super::level_designer::TileEntity;
use super::physics::{Collider, Gravity, Velocity};

pub struct HeroPlugin;

impl Plugin for HeroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_hero)
            .add_systems(Update, (animate_hero, move_hero, rotate_gravity));
    }
}

#[derive(Component)]
struct AnimationConfig {
    start_index: usize,
    end_index: usize,
    fps: u8,
    timer: Timer,
    direction: Direction,
    animation_state: HeroAnimationState,
}
enum Direction {
    Left,
    Right,
}
enum HeroAnimationState {
    Running,
    Walking,
    Falling,
    Idle,
}

#[derive(Component)]
pub struct MainHero;

impl AnimationConfig {
    fn new(start: usize, end: usize, fps: u8) -> Self {
        let timer = Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once);
        AnimationConfig {
            start_index: start,
            end_index: end,
            fps,
            timer,
            direction: Direction::Left,
            animation_state: HeroAnimationState::Idle,
        }
    }
    fn reset_timer(self: &mut Self) {
        self.timer.reset();
        // self.timer = Timer::new(
        //     Duration::from_secs_f32(1.0 / self.fps as f32),
        //     TimerMode::Once,
        // );
    }
}

fn add_hero(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture: Handle<Image> =
        asset_server.load("male_hero_free/individual_sheets/male_hero-idle.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(128, 122), 10, 1, None, None);
    let atlas_layout = texture_atlas_layouts.add(layout);
    let animation_config = AnimationConfig::new(0, 9, 24);

    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: atlas_layout,
                index: animation_config.start_index,
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(2.0)),
        animation_config,
        MainHero,
        Velocity(Vec2::ZERO),
        Gravity(Vec2::new(0.0, -1.0)),
        Collider {
            height: 70.0,
            width: 40.0,
        },
    ));
}

fn animate_hero(
    time: Res<Time>,
    query: Query<(&mut Sprite, &mut AnimationConfig), With<MainHero>>,
    keys: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
) {
    for (mut sprite, mut animation_config) in query {
        let image: Handle<Image> = match &animation_config.animation_state {
            HeroAnimationState::Running => {
                asset_server.load("male_hero_free/individual_sheets/male_hero-run.png")
            }
            HeroAnimationState::Falling => {
                asset_server.load("male_hero_free/individual_sheets/male_hero-fall_loop.png")
            }
            HeroAnimationState::Walking => {
                asset_server.load("male_hero_free/individual_sheets/male_hero-walk.png")
            }
            HeroAnimationState::Idle => {
                asset_server.load("male_hero_free/individual_sheets/male_hero-idle.png")
            }
        };

        sprite.image = image;
        animation_config.timer.tick(time.delta());
        if keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA) {
            sprite.flip_x = true;
            animation_config.direction = Direction::Left;
            animation_config.animation_state = HeroAnimationState::Walking;
        } else if keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD) {
            sprite.flip_x = false;
            animation_config.direction = Direction::Right;
            animation_config.animation_state = HeroAnimationState::Walking;
        } else if keys.pressed(KeyCode::ArrowDown) {
            animation_config.end_index = 2;
            animation_config.animation_state = HeroAnimationState::Falling;
        } else {
            animation_config.animation_state = HeroAnimationState::Idle;
        }

        if let Some(atlas) = &mut sprite.texture_atlas
            && animation_config.timer.just_finished()
        {
            if atlas.index == animation_config.end_index {
                atlas.index = animation_config.start_index;
            } else {
                atlas.index += 1;
            }
            animation_config.reset_timer();
        }
    }
}

fn move_hero(
    time: Res<Time>,
    hero_query: Query<
        (
            &mut Sprite,
            &mut Transform,
            &Collider,
            &mut Velocity,
            &mut Gravity,
        ),
        (Without<TileEntity>, With<MainHero>),
    >,
    tile_query: Query<(&Transform, &Collider), (With<TileEntity>, Without<MainHero>)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (mut hero_sprite, mut hero_transform, hero_collider, mut vel, mut gravity) in hero_query {
        for (tile_transform, tile_collider) in tile_query {
            if keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA) {
                vel.0.x = -3.0;
                vel.0.y = 0.0;
            } else if keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD) {
                vel.0.x = 3.0;
                vel.0.y = 0.0;
            } else if keys.pressed(KeyCode::ArrowUp) || keys.pressed(KeyCode::KeyW) {
                vel.0.x = 0.0;
                vel.0.y = 3.0;
            } else if keys.pressed(KeyCode::ArrowDown) || keys.pressed(KeyCode::KeyW) {
                vel.0.x = 0.0;
                vel.0.y = -3.0;
            } else {
                vel.0.x = 0.0;
                vel.0.y = 0.0
            }
            let mut grav = Velocity(gravity.0);
            translation_using_vel(&mut hero_transform, &mut vel, &mut grav, 0.05);

            if let Some(diff) = hero_collider.collision_detection(
                hero_transform.translation.truncate(),
                tile_collider,
                tile_transform.translation.truncate(),
            ) {
                hero_transform.translation -= diff.extend(0.0);
            }
            // println!(
            //     "X: {}, y: {}",
            //     hero_transform.translation.x, hero_transform.translation.y
            // );
        }
    }
}

fn rotate_gravity(
    mut query: Query<(&mut Transform, &mut Gravity), With<MainHero>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let (mut transform, mut gravity) = query.single_mut().unwrap();

    if keys.pressed(KeyCode::KeyJ) {
        let angle = f32::consts::FRAC_PI_2;
        gravity.0 = gravity.0.rotate(Vec2::from_angle(angle));
        let tmp = transform.translation.truncate();
        transform.translation = tmp.rotate(Vec2::from_angle(angle)).extend(0.0);
    } else if keys.pressed(KeyCode::KeyL) {
        let angle = -f32::consts::FRAC_PI_2;
        gravity.0 = gravity.0.rotate(Vec2::from_angle(angle));
        let tmp = transform.translation.truncate();
        transform.translation = tmp.rotate(Vec2::from_angle(angle)).extend(0.0);
    }

    // println!("X: {}, y: {}", gravity.0.x, gravity.0.y);
}
