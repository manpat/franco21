#version 450
#import 3d

in vec2 v_uv;

layout(binding=0) uniform sampler2D u_scene_color;
layout(binding=1) uniform sampler2D u_friend_color;

layout(binding=2) uniform sampler2D u_scene_depth;
layout(binding=3) uniform sampler2D u_friend_depth;

layout(location=0) out vec4 out_color;

void main() {
	vec4 scene_color = texture(u_scene_color, v_uv);
	vec4 friend_color = texture(u_friend_color, v_uv);

	float scene_depth = texture(u_scene_depth, v_uv).r;
	float friend_depth = texture(u_friend_depth, v_uv).r;

	const float depth_fail = float(scene_depth < friend_depth);
	const float friend_presence = friend_color.a;

	const float scene_foreground = scene_color.a;

	if (scene_foreground < 0.5) {
		// Water
		friend_color.rgb = mix(friend_color.rgb, u_world.water_obscure_color.rgb, depth_fail);
		out_color = mix(scene_color, friend_color, friend_presence);
	} else {
		// Not water
		out_color = mix(scene_color, friend_color, 1.0 - depth_fail);
	}
}
