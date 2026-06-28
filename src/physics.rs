use bevy::prelude::*;

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
    pub center: Vec2,
}

pub enum CollisionDirection {
    Left(f32),
    Right(f32),
    Up(f32),
    Down(f32),
}

impl Collider {
    pub fn collision_detection(&self, other: &Collider) -> Option<CollisionDirection> {
        let dx = self.center.x - other.center.x;
        let dy = self.center.y - other.center.y;

        let overlap_x = (self.width + other.width) / 2.0 - dx.abs();
        let overlap_y = (self.height + other.height) / 2.0 - dy.abs();

        //nothing is overlapping
        if overlap_x < 0.0 || overlap_y < 0.0 {
            return None;
        }
        //x is overlapping
        if overlap_x < overlap_y {
            if dx < 0.0 {
                return Some(CollisionDirection::Left(overlap_x));
            } else {
                return Some(CollisionDirection::Right(overlap_x));
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
