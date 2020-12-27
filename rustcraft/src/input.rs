//! Module handling the player's key and mouse input

use crate::camera::PerspectiveCamera;
use crate::timestep::TimeStep;
use glfw::{Key, Action, Window};
use cgmath::num_traits::FromPrimitive;

/// The default mouse speed
const MOVE_SPEED: f32 = 4.0;

/// The default mouse sensitivity
const MOUSE_SENSITIVITY: f32 = 0.25;

/// The default zoom sensitivity
const _ZOOM_SENSITIVITY: f32 = -3.0;


pub fn handle_key_input(timestep: TimeStep, window: &Window, camera: &mut PerspectiveCamera) {

    // Camera Movement
    let look = camera.look();
    let right = camera.right();
    let up = camera.up();

    // Forward / Backward
    if window.get_key(Key::W) == Action::Press {
        camera.set_offset(MOVE_SPEED * timestep.seconds() * look);
    } else if window.get_key(Key::S) == Action::Press {
        camera.set_offset(MOVE_SPEED * timestep.seconds() * -look);
    }

    // Left / Right
    if window.get_key(Key::A) == Action::Press {
        camera.set_offset(MOVE_SPEED * timestep.seconds() * -right);
    } else if window.get_key(Key::D) == Action::Press {
        camera.set_offset(MOVE_SPEED * timestep.seconds() * right);
    }

    // Up / Down
    if window.get_key(Key::Z) == Action::Press {
        camera.set_offset(MOVE_SPEED * timestep.seconds() * up);
    } else if window.get_key(Key::Y) == Action::Press {
        camera.set_offset(MOVE_SPEED * timestep.seconds() * -up);
    }
}

pub fn handle_mouse_input(window: &mut Window, camera: &mut PerspectiveCamera) {
    let (width, height) = window.get_size();
    let (mouse_x, mouse_y) = window.get_cursor_pos();
    camera.rotate(
        (f32::from(width as i16) / 2.0 - f32::from_f64(mouse_x).unwrap()) * MOUSE_SENSITIVITY,
        (f32::from(height as i16) / 2.0 - f32::from_f64(mouse_y).unwrap()) * MOUSE_SENSITIVITY,
        0.0
    );
    window.set_cursor_pos( width as f64 / 2.0, height as f64 / 2.0);
}
