


layout(std140, binding = 4) uniform WaveUniforms {
	vec3 base_color;
	float peak_start;
	vec3 peak_color;
	float peak_height;
} u_wave;



vec3 water_color_at_height(in float height) {
	const float alpha = clamp((height-u_wave.peak_start) / u_wave.peak_height, 0.0, 1.0);
	return mix(u_wave.base_color, u_wave.peak_color, alpha);
}




struct RippleInstance {
	vec2 offset;
	float ripple_factor;
	float ripple_phase;
};

layout(std430, binding = 0) readonly buffer RippleData {
	RippleInstance s_instances[];
};