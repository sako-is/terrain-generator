use bevy::prelude::*;
use crate::terrain::*;
use bevy_egui::{egui, EguiContexts};

pub fn customize_terrain_menu(
    mut contexts: EguiContexts,
    map_info: ResMut<MapInfo>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut query: Query<&mut Handle<StandardMaterial>, With<TerrainMarker>>
) {
    let mi = map_info.into_inner();

    egui::Window::new("Customize Terrain").show(contexts.ctx_mut(), |ui| {
        ui.add(egui::Slider::new(&mut mi.seed, 0..=4294967295).text("seed"));
        ui.add(egui::Slider::new(&mut mi.width, 128..=1024).text("width"));
        ui.add(egui::Slider::new(&mut mi.height, 128..=1024).text("height"));
        ui.add(egui::Slider::new(&mut mi.scale, 0.0..=100.).text("scale"));
        ui.add(egui::Slider::new(&mut mi.octaves, 1..=10).text("octaves"));
        ui.add(egui::Slider::new(&mut mi.persistance, 0.0..=1.).text("persistance"));
        ui.add(egui::Slider::new(&mut mi.lacunarity, 1..=10).text("lacunarity"));
        if ui.button("Update").clicked() {
            let map = noise::noise_map(
                mi.seed,
                mi.width, 
                mi.height, 
                mi.scale,
                mi.octaves,
                mi.persistance,
                mi.lacunarity
            );
            let image = map_image(&map, mi);

            let image_handle = images.add(image);

            let mut material = query.get_single_mut().unwrap();
            *material = materials.add(image_handle);
        }
    });
}
