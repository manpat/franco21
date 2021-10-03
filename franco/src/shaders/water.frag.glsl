#version 450

#import 3d
#import water

in float v_height;
in vec2 v_world_pos;

layout(location=0) out vec4 out_color;

void main() {
	const float dist = length(v_world_pos);
	const vec3 water_color = water_color_at_height(v_height);
	const vec3 final_color = apply_fog(water_color, dist);

	out_color = vec4(final_color, 0.0);
}

