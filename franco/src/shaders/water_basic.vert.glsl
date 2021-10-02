#version 450
#import 3d
#import water

layout(location=0) in vec3 a_pos;

out float v_height;

void main() {
	gl_Position = u_camera.projection_view * vec4(a_pos, 1.0);
	v_height = 0.0;
}

