#version 450
#import 3d

layout(location=0) in vec3 a_pos;
layout(location=1) in vec4 a_color;

layout(std140, row_major, binding = 4) uniform BoatData {
	mat4 transform;
} u_boat;


out vec4 v_color;

void main() {
	gl_Position = u_camera.projection_view * u_boat.transform * vec4(a_pos, 1.0);
	v_color = a_color;
}

