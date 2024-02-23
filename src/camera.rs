use bevy::prelude::*;
use bevy_panorbit_camera::*;

pub mod floating;
pub mod egui;

pub struct CameraPlugin {
    pub sensitivity: f32,
    pub speed: f32
}

#[derive(Resource)]
pub struct CameraInfo {
    sensitivity: f32,
    speed: f32
}

impl Plugin for CameraPlugin {
    fn build(&self, app_: &mut App) {
        app_.add_systems(Startup, setup_camera)
            .insert_resource(CameraInfo { 
                sensitivity: self.sensitivity, speed: self.speed })
            .insert_resource(egui::EguiWantsFocus {
                prev: false, curr: false })
            .add_systems(Update, egui::check_egui_wants_focus)
            .add_systems(Update, floating::input_handler);
    }
}

#[derive(Component)]
pub struct CameraController {
    pitch: f32,
    yaw: f32,
    sensitivity: f32,
    speed: f32,
    velocity: Vec3,
    friction: f32
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            pitch: 0.,
            yaw: 0.,
            sensitivity: 1.5,
            speed: 20.,
            velocity: Vec3::ZERO,
            friction: 0.5
        }
    }
}

pub fn setup_camera(mut commands: Commands, cam_info: Res<CameraInfo>) {
    commands.spawn((
        Camera3dBundle {
            ..default()
        },
        CameraController {
            sensitivity: cam_info.sensitivity,
            speed: cam_info.speed,
            ..default()
        }
    ));
}

pub fn pan_orbit(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle { 
            .. default() 
        },
        PanOrbitCamera::default()
    ));
}
