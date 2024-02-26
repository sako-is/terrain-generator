use bevy::prelude::*;

use bevy::{
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};

pub mod mesh;
pub mod noise;
pub mod customize;
pub mod material;

pub struct TerrainPlugin {
    pub seed: u32,
    pub size: u32,
    pub scale: f32,
    pub octaves: i32,
    pub persistance: f32,
    pub lacunarity: i32,
    pub shader: bool
}

#[derive(Resource)]
pub struct MapInfo {
    seed: u32,
    size: u32,
    scale: f32,
    octaves: i32,
    persistance: f32,
    lacunarity: i32,
}

#[derive(Component)]
pub struct TerrainMarker;

impl Plugin for TerrainPlugin {
    fn build(&self, app_: &mut App) {
        app_.insert_resource(MapInfo {
                        seed: self.seed,
                        size: self.size, 
                        scale: self.scale,
                        octaves: self.octaves,
                        persistance: self.persistance,
                        lacunarity: self.lacunarity
            })
            .add_plugins(MaterialPlugin::<material::TerrainMaterial>::default());
        if self.shader {
            app_.add_systems(Startup, spawn_terrain_gpu)
                .add_systems(Update, customize::customize_terrain_menu_gpu);
        } else {
            app_.add_systems(Startup, spawn_terrain_cpu)
                .add_systems(Update, customize::customize_terrain_menu_cpu);
        }
    }
}

pub fn spawn_terrain_cpu(mut commands: Commands, 
                     mut meshes: ResMut<Assets<Mesh>>, 
                     mut materials: ResMut<Assets<StandardMaterial>>,
                     mut images: ResMut<Assets<Image>>,
                     map_info: Res<MapInfo>
                    ) {
    let map = noise::noise_map(
                map_info.seed,
                map_info.size, 
                map_info.scale,
                map_info.octaves,
                map_info.persistance,
                map_info.lacunarity
            );
    let image = map_image(&map, map_info.into_inner());

    let image_handle = images.add(image);

    commands.spawn((PbrBundle {
        // mesh: meshes.add(mesh::create_terrain_mesh(2)),
        mesh: meshes.add(Plane3d::default().mesh().size(20., 20.)),
        material: materials.add(image_handle),
        ..default()
    }, 
    TerrainMarker));

}

pub fn map_image(map: &Vec<f32>, map_info: &MapInfo) -> Image {
    let mut image_data = Vec::<u8>::new();

    for pixel in &*map {
        image_data.push(((pixel + 1.) * 127.5) as u8);
    }

    let image = Image::new( 
        Extent3d { width: map_info.size, height: map_info.size, depth_or_array_layers: 1},
        TextureDimension::D2,
        image_data,
        TextureFormat::R8Unorm,
        RenderAssetUsages::RENDER_WORLD
    );
    image
}

pub fn spawn_terrain_gpu(mut commands: Commands, 
                     mut meshes: ResMut<Assets<Mesh>>, 
                     mut materials: ResMut<Assets<material::TerrainMaterial>>,
                     map_info: Res<MapInfo>,
                    ) 
{
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(20., 20.)),
        material: materials.add(material::TerrainMaterial {
            seed: map_info.seed,
            size: map_info.size,
            scale: map_info.scale,
            octaves: map_info.octaves,
            persistance: map_info.persistance,
            lacunarity: map_info.lacunarity,
            alpha_mode: AlphaMode::Premultiplied
        }),
        ..default()
    });
}
