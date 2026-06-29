use bevy::prelude::*;

use crate::{hero_plugin::MainHero, level_designer::TileEntity};

pub struct PhysicsDebugPlugin;

impl Plugin for PhysicsDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                spawn_collision_boundary_debug_boxes,
                sync_collision_boundary_debug_boxes,
            )
                .chain(),
        );
    }
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
struct Gravity {
    x: f32,
    y: f32,
}

//4 sided shape
#[derive(Component)]
pub struct Collider {
    pub height: f32,
    pub width: f32,
}

#[derive(Component)]
struct CollisionBoundaryDebugSource {
    visual_entity: Entity,
}

#[derive(Component)]
struct CollisionBoundaryDebugVisual;

pub enum CollisionDirection {
    Left(f32),
    Right(f32),
    Up(f32),
    Down(f32),
}

impl Collider {
    pub fn collision_detection(
        &self,
        center: Vec2,
        other: &Collider,
        other_center: Vec2,
    ) -> Option<CollisionDirection> {
        let dx = center.x - other_center.x;
        let dy = center.y - other_center.y;

        let overlap_x = (self.width + other.width) / 2.0 - dx.abs();
        let overlap_y = (self.height + other.height) / 2.0 - dy.abs();

        //nothing is overlapping
        if overlap_x < 0.0 || overlap_y < 0.0 {
            return None;
        }
        //x is overlapping
        if overlap_x < overlap_y {
            if dx < 0.0 {
                return Some(CollisionDirection::Right(overlap_x));
            } else {
                return Some(CollisionDirection::Left(overlap_x));
            }
        } else {
            if dy < 0.0 {
                return Some(CollisionDirection::Up(overlap_y));
            } else {
                return Some(CollisionDirection::Down(overlap_y));
            }
        }
    }
}

fn spawn_collision_boundary_debug_boxes(
    mut commands: Commands,
    hero_query: Query<
        (Entity, &Transform, &Collider),
        (
            With<MainHero>,
            Without<TileEntity>,
            Without<CollisionBoundaryDebugSource>,
        ),
    >,
    tile_query: Query<
        (Entity, &Transform, &Collider),
        (
            With<TileEntity>,
            Without<MainHero>,
            Without<CollisionBoundaryDebugSource>,
        ),
    >,
) {
    for (entity, transform, collider) in &hero_query {
        spawn_collision_boundary_debug_box(
            &mut commands,
            entity,
            transform,
            collider,
            Color::srgba(1.0, 0.0, 0.0, 0.5),
        );
    }

    for (entity, transform, collider) in &tile_query {
        spawn_collision_boundary_debug_box(
            &mut commands,
            entity,
            transform,
            collider,
            Color::srgba(0.0, 0.0, 1.0, 0.5),
        );
    }
}

fn spawn_collision_boundary_debug_box(
    commands: &mut Commands,
    collider_entity: Entity,
    source_transform: &Transform,
    collider: &Collider,
    color: Color,
) {
    let mut visual_transform = Transform::from_translation(source_transform.translation);
    visual_transform.translation.z = 10.0;

    let visual_entity = commands
        .spawn((
            Sprite::from_color(color, Vec2::new(collider.width, collider.height)),
            visual_transform,
            CollisionBoundaryDebugVisual,
        ))
        .id();

    commands
        .entity(collider_entity)
        .insert(CollisionBoundaryDebugSource { visual_entity });
}

fn sync_collision_boundary_debug_boxes(
    source_query: Query<
        (&Transform, &Collider, &CollisionBoundaryDebugSource),
        Without<CollisionBoundaryDebugVisual>,
    >,
    mut visual_query: Query<
        (&mut Transform, &mut Sprite),
        (
            With<CollisionBoundaryDebugVisual>,
            Without<CollisionBoundaryDebugSource>,
        ),
    >,
) {
    for (source_transform, collider, debug_source) in &source_query {
        if let Ok((mut transform, mut sprite)) = visual_query.get_mut(debug_source.visual_entity) {
            transform.translation.x = source_transform.translation.x;
            transform.translation.y = source_transform.translation.y;
            sprite.custom_size = Some(Vec2::new(collider.width, collider.height));
        }
    }
}
