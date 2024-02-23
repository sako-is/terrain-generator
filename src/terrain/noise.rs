use std::vec::Vec;
use noise::*;

pub fn noise_map(seed: u32,
                 width: u32, 
                 height: u32, 
                 mut scale: f32, 
                 octaves: i32, 
                 persistance: f32, 
                 lacunarity: i32) -> Vec<f32> {
    let mut map: Vec<f32> = Vec::new();
    let simplex = SuperSimplex::new(seed);
    
    if scale <= 0. { scale  = 0.0001; }

    let (mut sample_x, mut sample_y, mut simplex_val): (f32, f32, f64);
    let (mut amplitude, mut frequency, mut noise_height): (f32, f32, f32); 

    for y in 0..height {
        for x in 0..width {
            amplitude = 1.;
            frequency = 1.;
            noise_height = 0.;

            for _ in 0..octaves {
                sample_x = x as f32 / scale * frequency;
                sample_y = y as f32 / scale * frequency;

                simplex_val = simplex.get([sample_x as f64, sample_y as f64]);
                noise_height += simplex_val as f32 * amplitude;

                amplitude *= persistance;
                frequency *= lacunarity as f32;
            }
            
           map.push(noise_height);
        }
    }

    map
}
