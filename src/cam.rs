use bevy::{prelude::*, utils::info};
use std::collections::HashMap;

// Define a struct for holding velocity information
#[derive(Resource, Default)] // Derive Resource to allow this struct to be a Bevy resource
pub struct CameraVelocity {
    pub velocity: Vec2,
}


// System to handle key input for camera movement
pub fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut camera_velocity: ResMut<CameraVelocity>,  // Resource holding the current velocity
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    // Define a map of key velocities
    let key_velocity_map: HashMap<KeyCode, Vec2> = HashMap::from([
        (KeyCode::KeyW, Vec2::new(0.0, 1.0)),   // Move up
        (KeyCode::KeyA, Vec2::new(-1.0, 0.0)),  // Move left
        (KeyCode::KeyS, Vec2::new(0.0, -1.0)),  // Move down
        (KeyCode::KeyD, Vec2::new(1.0, 0.0)),   // Move right
    ]);

    // Handle key press events
    for key in keys.get_just_pressed() {
        info!("key pressed: {:?}", key);
        if let Some(velocity) = key_velocity_map.get(key) {
            camera_velocity.velocity += *velocity;
        }
    }

    // Handle key release events
    for key in keys.get_just_released() {
        info!("key released: {:?}", key);
        if let Some(velocity) = key_velocity_map.get(key) {
            camera_velocity.velocity -= *velocity;
        }
    }
}