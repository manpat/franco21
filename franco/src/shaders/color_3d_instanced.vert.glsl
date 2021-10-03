#version 450
#import 3d

layout(location=0) in vec3 a_pos;
layout(location=1) in vec4 a_color;

out vec4 v_color;
out vec3 v_pos;


layout(std140, row_major, binding = 0) readonly buffer BasicInstances {
	mat4x3 s_instance_transforms[];
};


void main() {
	const mat4x3 transform = s_instance_transforms[gl_InstanceID];

	const vec3 worldspace = transform * vec4(a_pos, 1.0);
	gl_Position = u_camera.projection_view * vec4(worldspace, 1.0);
	v_color = a_color;
	v_pos = worldspace;
}
