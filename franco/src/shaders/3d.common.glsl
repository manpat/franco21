

layout(std140, row_major, binding = 0) uniform CameraData {
	mat4 projection_view;
} u_camera;

layout(std140, row_major, binding = 1) uniform WorldData {
	vec2 player_position;
} u_world;



