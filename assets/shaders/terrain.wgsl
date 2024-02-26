#import "shaders/noise.wgsl"::simplexNoise2

@group(2) @binding(0) var<uniform> seed: u32;
@group(2) @binding(1) var<uniform> size: u32;
@group(2) @binding(2) var<uniform> scale: f32;
@group(2) @binding(3) var<uniform> octaves: i32;
@group(2) @binding(4) var<uniform> persistance: f32;
@group(2) @binding(5) var<uniform> lacunarity: i32;

@fragment
fn fragment() -> @location(0) vec4<f32> {
	var nscale: f32 = scale;
	if (scale <= 0.) { nscale  = 0.0001; }

    var sample_x: f32;
	var sample_y: f32;
	var simplex_val: f32;

    var amplitude: f32;
	var frequency: f32;
	var noise_height: f32; 

    for (var y: u32 = 0; y < size; y++) {
    	for (var x: u32 = 0; x < size; x++) {
            amplitude = 1.;
            frequency = 1.;
            noise_height = 0.;

            for (var i: i32 = 0; i < octaves; i++) {
                sample_x = f32(x) / nscale * frequency;
                sample_y = f32(y) / nscale * frequency;

                simplex_val = simplexNoise2(vec2<f32>(sample_x, sample_y));
                noise_height += simplex_val * amplitude;

                amplitude *= persistance;
                frequency *= f32(lacunarity);
            }
        }
    }

	return vec4<f32>(1., 1., 1., noise_height);
}
