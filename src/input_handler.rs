use bevy::prelude::*;
use crate::rotation::RotationResource;
use crate::Player;

const MOVE_SPEED: f32 = 0.1;
const ROT_SPEED: f32 = 0.05;

pub fn handle_input (
    mut transforms: Query<&mut Transform, With<Player>>,
    rotation_resource: Res<RotationResource>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for mut transform in transforms.iter_mut() {
        let mut direction = Vec3::ZERO;
        let mut rot = Quat::IDENTITY;

        // Translation movement
        if keys.pressed(KeyCode::KeyW) {
            direction.x -= 1.0
        };
        if keys.pressed(KeyCode::KeyA) {
            direction.z += 1.0
        };
        if keys.pressed(KeyCode::KeyS) {
            direction.x += 1.0
        };
        if keys.pressed(KeyCode::KeyD) {
            direction.z -= 1.0
        };
        if keys.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0
        };
        if keys.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0
        };

        // Rotation using numpad
        if keys.pressed(KeyCode::Numpad7) {
            rot.x += 1.0;
        };
        if keys.pressed(KeyCode::Numpad4) {
            rot.x -= 1.0;
        };
        if keys.pressed(KeyCode::Numpad8) {
            rot.y += 1.0;
        };
        if keys.pressed(KeyCode::Numpad5) {
            rot.y -= 1.0;
        };
        if keys.pressed(KeyCode::Numpad9) {
            rot.z += 1.0;
        };
        if keys.pressed(KeyCode::Numpad6) {
            rot.z -= 1.0;
        };

        if 0.0 < direction.length() {
            transform.translation += MOVE_SPEED * direction.normalize();
        }

        if rot != Quat::IDENTITY {
            println!("{:?}", transform.rotation);
            transform.rotate_x(rot.x * ROT_SPEED);
            transform.rotate_y(rot.y * ROT_SPEED);
            transform.rotate_z(rot.z * ROT_SPEED);
        }

        // Apply the latest rotation
        transform.rotation = rotation_resource.0.load();
    }
}
