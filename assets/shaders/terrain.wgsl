@fragment
fn frag_main(@builtin(position) coord: vec4<f32>) -> @location(0) vec4<f32> {
	return vec4<f32>(coord.x, coord.y, 0.0, 1.0);
}
