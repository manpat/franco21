

layout(std140, row_major, binding = 0) uniform CameraData {
	mat4 projection_view;
} u_camera;

layout(std140, row_major, binding = 1) uniform WorldData {
	mat4 global_transform;
} u_world;

