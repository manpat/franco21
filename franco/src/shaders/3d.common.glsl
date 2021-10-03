

layout(std140, row_major, binding = 0) uniform CameraData {
	mat4 projection_view;
} u_camera;

layout(std140, row_major, binding = 1) uniform WorldData {
	vec4 sky_color;
	vec2 player_position;
	float fog_start;
	float fog_distance;
} u_world;



vec3 apply_fog(in vec3 source_color, in float world_distance) {
	const float fade = clamp((world_distance - u_world.fog_start) / u_world.fog_distance, 0.0, 1.0);
	return mix(source_color, u_world.sky_color.rgb, fade);
}
