#version 450

layout(location = 0) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

layout(set = 1, binding = 0) uniform uint uniform uint seed;
layout(set = 1, binding = 1) uniform uint uniform uint size;
layout(set = 1, binding = 2) uniform uint uniform float scale;
layout(set = 1, binding = 3) uniform uint uniform int octaves;
layout(set = 1, binding = 4) uniform uint uniform float persistance;
layout(set = 1, binding = 5) uniform uint uniform int lacunarity;

// Based on Morgan McGuire @morgan3d
// https://www.shadertoy.com/view/4dS3Wd
float noise (in vec2 st) {
    vec2 i = floor(st);
    vec2 f = fract(st);

    // Four corners in 2D of a tile
    float a = random(i);
    float b = random(i + vec2(1.0, 0.0));
    float c = random(i + vec2(0.0, 1.0));
    float d = random(i + vec2(1.0, 1.0));

    vec2 u = f * f * (3.0 - 2.0 * f);

    return mix(a, b, u.x) +
            (c - a)* u.y * (1.0 - u.x) +
            (d - b) * u.x * u.y;
}

void main() {
	if (scale <= 0.) scale = 0.0001;

	float sample_x, sample_y, simplex_val;
	float amplitude, frequency, noise_height;

	for(int y = 0; y < size; y++) {
		for(int x = 0; x < size; x++) {
			amplitude = 1.;
			frequency = 1.;
			noise_height = 0.;

			for(int i = 0; i < octaves; i++) {
				sample_x = x / scale * frequency;
				sample_y = y / scale * frequency;

				simplex_val = noise(vec2(sample_x, sample_y));
				noise_height += simplex_val * amplitude;

				amplitude *= persistance;
				frequency *= lacunarity;
			}
		}
	}

	o_Target = vec4(1., 1., 1., noise_height);
}

