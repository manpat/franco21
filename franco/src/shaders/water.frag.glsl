#version 450

#import water

in float v_height;

layout(location=0) out vec4 out_color;

void main() {
	out_color = vec4(water_color_at_height(v_height), 1.0);
}

