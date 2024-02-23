use bevy::prelude::*;
use bevy::pbr::{light_consts, CascadeShadowConfigBuilder};
use rand::Rng;

use bevy_egui::EguiPlugin;

use bevy_panorbit_camera::*;

use std::f32::consts::PI;

mod terrain;
mod camera;

fn main() {
    let mut rng = rand::thread_rng();

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::hex("31748f").unwrap()))
        // .add_plugins(camera::CameraPlugin {
        //     sensitivity: 1.5, speed: 20. })
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, camera::pan_orbit)
        .add_plugins(terrain::TerrainPlugin {
            seed: rng.gen(), 
            width: 256, 
            height: 256, 
            scale: 50., 
            octaves: 4, 
            persistance: 0.5, 
            lacunarity: 3 
        })
        .add_plugins(EguiPlugin)
        .add_systems(Startup, spawn_lights)
        .run();
}

fn spawn_lights(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });
    
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });

}
