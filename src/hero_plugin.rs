use bevy::prelude::*;
use std::time::Duration;

pub struct HeroPlugin;

impl Plugin for HeroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, exit_game)
            .add_systems(Update, animate_hero);
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
struct MainHero;

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
        self.timer = Timer::new(
            Duration::from_secs_f32(1.0 / self.fps as f32),
            TimerMode::Once,
        );
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    let texture: Handle<Image> =
        asset_server.load("male_hero_free/individual_sheets/male_hero-idle.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(128), 10, 1, None, None);
    let atlas_layout = texture_atlas_layouts.add(layout);
    let animation_config = AnimationConfig::new(0, 9, 60);

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
    ));
}

fn exit_game(keys: Res<ButtonInput<KeyCode>>, mut exit: MessageWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

fn animate_hero(
    time: Res<Time>,
    query: Query<(&mut Sprite, &mut AnimationConfig, &mut Transform), With<MainHero>>,
    keys: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
) {
    for (mut sprite, mut animation_config, mut transform) in query {
        let image: Handle<Image> = match &animation_config.animation_state {
            HeroAnimationState::Running => {
                asset_server.load("male_hero_free/individual_sheets/male_hero-run.png")
            }
            HeroAnimationState::Falling => {
                asset_server.load("male_hero_free/individual_sheets/male_hero-fall.png")
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
        if keys.pressed(KeyCode::ArrowLeft) {
            sprite.flip_x = true;
            transform.translation.x -= 3.0;
            animation_config.direction = Direction::Left;
            animation_config.animation_state = HeroAnimationState::Walking;
        } else if keys.pressed(KeyCode::ArrowRight) {
            sprite.flip_x = false;
            transform.translation.x += 3.0;
            animation_config.direction = Direction::Right;
            animation_config.animation_state = HeroAnimationState::Walking;
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
