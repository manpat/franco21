#version 450
#import 3d

in vec4 v_color;
in vec3 v_pos;

layout(location=0) out vec4 out_color;

void main() {
	const float dist = length(v_pos.xz);
	const vec3 color = apply_fog(v_color.rgb, dist);
	out_color = vec4(color, v_color.a * (1.0 - calc_fog(dist)));
}

