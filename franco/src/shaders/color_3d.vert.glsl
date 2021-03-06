#version 450
#import 3d

layout(location=0) in vec3 a_pos;
layout(location=1) in vec4 a_color;

out vec4 v_color;
out vec3 v_pos;

void main() {
	gl_Position = u_camera.projection_view * vec4(a_pos, 1.0);
	v_color = a_color;
	v_pos = a_pos;
}
