

layout(std140, row_major, binding = 0) uniform CameraData {
	mat4 projection_view;
} u_camera;

layout(std140, row_major, binding = 1) uniform WorldData {
	vec4 sky_color;
	vec4 water_obscure_color;
	vec2 player_position;
	float fog_start;
	float fog_distance;
} u_world;


float calc_fog(in float world_distance) {
	return clamp((world_distance - u_world.fog_start) / u_world.fog_distance, 0.0, 1.0);
}


vec3 apply_fog(in vec3 source_color, in float world_distance) {
	const float fade = calc_fog(world_distance);
	return mix(source_color, u_world.sky_color.rgb, fade);
}
