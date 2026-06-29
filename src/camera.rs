use std::f32;

use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, rotate_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::default()));
}

fn rotate_camera(query: Query<&mut Transform, With<Camera2d>>, keys: Res<ButtonInput<KeyCode>>) {
    //how to do single query
    //
    for mut transform in query {
        if keys.pressed(KeyCode::KeyJ) {
            transform.rotate(Quat::from_rotation_z(f32::consts::FRAC_PI_2));
        } else if keys.pressed(KeyCode::KeyL) {
            transform.rotate(Quat::from_rotation_z(-f32::consts::FRAC_PI_2));
        }
    }
}
