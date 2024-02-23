use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use std::f32::consts::*;
use bevy::window::CursorGrabMode;
use crate::camera::egui::*;

use crate::camera::CameraController;

pub const RADIANS_PER_DOT: f32 = 1.0 / 180.0;

pub fn input_handler(
    mut windows: Query<&mut Window>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    egui_focus: Res<EguiWantsFocus>,
    mut mouse_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
    time: Res<Time>,
    mut toggle_cursor_grab: Local<bool>,
    mut mouse_cursor_grab: Local<bool>
) {
    let Ok((mut transform, mut controller)) = query.get_single_mut() else { 
                            panic!("Can't create transform or controller") };
        
    let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
    controller.yaw = yaw;
    controller.pitch = pitch;

    let mut axis = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::Space) {
        axis.y += 1.;
    }
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        axis.y -= 1.;
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        axis.z += 1.;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        axis.z -= 1.;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        axis.x -= 1.;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        axis.x += 1.;
    }
   
    let mut cursor_grab_change = false;
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        *toggle_cursor_grab = false;
        cursor_grab_change = true;
    } 
    if mouse_button_input.pressed(MouseButton::Left) {
        *mouse_cursor_grab = true;
        cursor_grab_change = true;
    }
    if mouse_button_input.just_released(MouseButton::Left) {
        *mouse_cursor_grab = false;
        cursor_grab_change = true;
    }

    if egui_focus.prev && egui_focus.curr {
        *mouse_cursor_grab = false;
        cursor_grab_change = true;
    }
    
    let cursor_grab = *mouse_cursor_grab || *toggle_cursor_grab;


    if axis != Vec3::ZERO {
        controller.velocity = axis.normalize() * controller.speed;
    } else {
        let friction = controller.friction.clamp(0.0, 1.0);
        controller.velocity *= 1.0 - friction;
        if controller.velocity.length_squared() < 1e-6 {
            controller.velocity = Vec3::ZERO;
        }
    }
    
    if cursor_grab_change {
        if cursor_grab {
            for mut window in &mut windows {
                if !window.focused {
                    continue;
                }

                window.cursor.grab_mode = CursorGrabMode::Confined;
                window.cursor.visible = false;
            }
        } else {
            for mut window in &mut windows {
                window.cursor.grab_mode = CursorGrabMode::None;
                window.cursor.visible = true;
            }
        }
    }

    let forward = *transform.forward();
    let right = *transform.right();

    transform.translation += controller.velocity.x * time.delta_seconds() * right
            + controller.velocity.y * time.delta_seconds() * Vec3::Y
            + controller.velocity.z * time.delta_seconds() * forward;

    let mut mouse_delta = Vec2::ZERO;
    if cursor_grab {
        for mouse_event in mouse_events.read() {
            mouse_delta += mouse_event.delta;
        }
    } else {
        mouse_events.clear();
    }

    if mouse_delta != Vec2::ZERO {
        // Apply look update
        controller.pitch = (controller.pitch
            - mouse_delta.y * RADIANS_PER_DOT * controller.sensitivity)
            .clamp(-PI / 2., PI / 2.);
        controller.yaw -= mouse_delta.x * RADIANS_PER_DOT * controller.sensitivity;
        transform.rotation =
            Quat::from_euler(EulerRot::ZYX, 0.0, controller.yaw, controller.pitch);
    }
}
