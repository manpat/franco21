#version 450
#import 3d

in vec4 v_color;
in vec3 v_pos;

layout(location=0) out vec4 out_color;

void main() {
	const vec3 color = apply_fog(v_color.rgb, length(v_pos.xz));
	out_color = vec4(color, v_color.a);
}

