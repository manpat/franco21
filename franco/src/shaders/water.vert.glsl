#version 450
#import 3d
#import water

layout(location=0) in vec3 a_pos;
layout(location=1) in vec4 a_color;

out float v_height;

void main() {
	RippleInstance instance = s_instances[gl_InstanceID];
	const vec2 ripple_pos = a_pos.xz + sin(a_pos.xz/2.0 + vec2(instance.ripple_phase));

	const float wave = sin(ripple_pos.x * 0.3 + instance.ripple_phase)
		+ cos(ripple_pos.y * 0.4 + instance.ripple_phase*1.1);

	const float ripple = instance.ripple_factor * (1.0 + wave/2.0) * u_wave.peak_height;

	gl_Position = u_camera.projection_view * vec4(a_pos + vec3(instance.offset, ripple).xzy, 1.0);

	v_height = a_pos.y + ripple;
}

