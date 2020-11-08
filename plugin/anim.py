import collections
import bpy

from .util import swap_coords, swap_coords_scale


Animation = collections.namedtuple(
	"Animation",
	"name fps channels"
)

AnimationChannel = collections.namedtuple(
	"AnimationChannel",
	"bone frames"
)

Frame = collections.namedtuple("Frame", "position rotation scale")


# https://github.com/sobotka/blender-addons/blob/5614916cf6ca0f261617384240acc0909f8adb9e/io_scene_fbx/export_fbx_bin.py#L2118
# I'm sorry but also thank you
def action_fits_armature(action, armature):
	for curve in action.fcurves:
		data_path = curve.data_path
		if curve.array_index:
			data_path = data_path + "[%d]" % curve.array_index
		try:
			armature.path_resolve(data_path)
		except ValueError:
			return False
	return True



def collect_animations(scene, armature, bones):
	animations = []

	original_frame = scene.frame_current
	original_action = armature.animation_data.action

	for action in bpy.data.actions:
		if not action_fits_armature(action, armature):
			continue

		armature.animation_data.action = action
		frame_start, frame_end = action.frame_range

		channels = {pose_bone: [] for pose_bone in armature.pose.bones}

		for frame in range(int(frame_start), int(frame_end)+1):
			scene.frame_set(frame)

			for pose_bone in armature.pose.bones:
				m = pose_bone.matrix
				m_c = pose_bone.matrix_channel.to_3x3()

				position = m.to_translation()
				rotation = pose_bone.matrix_channel.to_3x3().to_quaternion().normalized()
				scale = m_c.to_scale()

				channels[pose_bone].append(Frame(
					swap_coords(position),
					swap_coords(rotation),
					swap_coords_scale(scale)
				))

		bone_index = {bone.name: index for index, bone in enumerate(bones)}
		# print(bone_index)

		channels = (AnimationChannel(bone.name, frames) for (bone, frames) in channels.items())
		channels = filter(lambda ch: ch.bone in bone_index, channels)
		channels = sorted(channels, key=lambda ch: bone_index[ch.bone])
		fps = bpy.context.scene.render.fps
		animations.append(Animation(action.name, fps, list(channels)))

	armature.animation_data.action = original_action
	scene.frame_set(original_frame)

	return animations



def write_animations(ser, animations):
	ser.start_section("ANMS")

	for anim in animations:
		ser.start_section("ANIM")
		ser.write_string(anim.name)
		ser.write_f32(anim.fps)

		frame_count = set(len(channel.frames) for channel in anim.channels)
		assert len(frame_count) == 1
		frame_count = frame_count.pop()

		ser.write_u16(frame_count)
		ser.write_u8(len(anim.channels))
		for channel in anim.channels:
			ser.write_string(channel.bone)

			# TODO: COMPRESS
			for frame in channel.frames:
				ser.write_v3(*frame.position)
				ser.write_v4(*frame.rotation)
				ser.write_v3(*frame.scale)

		ser.end_section()

	ser.end_section()