import collections
import bmesh

from .util import swap_coords
from . import anim

Mesh = collections.namedtuple(
	"Mesh",
	"vertices triangles color_data animation_data"
)

Vertex = collections.namedtuple("Vertex", "position layers weights")
Bone = collections.namedtuple("Bone", "name head tail")



def collect_mesh(scene, depsgraph, obj):
	# We need to set armature to the rest pose otherwise we get 
	# mesh data for the current pose instead of the unposed mesh data
	armature = None
	original_armature_pose_position = None
	for modifier in obj.modifiers:
		if modifier.type == 'ARMATURE':
			armature = modifier.object
			original_armature_pose_position = armature.data.pose_position
			armature.data.pose_position = 'REST'
			depsgraph.update()
			break

	# Triangulate and bake deformations
	bm = bmesh.new()
	bm.from_object(obj, depsgraph, deform=True)
	bmesh.ops.triangulate(bm, faces=bm.faces)

	bm.verts.ensure_lookup_table()
	bm.faces.ensure_lookup_table()

	# Once we have our triangulated mesh data, we no longer need the original mesh
	# so can restore the armature pose
	if armature is not None:
		armature.data.pose_position = original_armature_pose_position

	for f in bm.faces:
		assert len(f.loops) == 3

	layer_deform = bm.verts.layers.deform.active

	color_layers = bm.loops.layers.color.items()
	verts = []
	tris = []

	# TODO: find a faster way
	def cache_vertex(vpos, vlayers, vweights):
		for i, vb in enumerate(verts):
			if vpos != vb.position:
				continue

			for la, lb in zip(vlayers, vb.layers):
				if la != lb:
					break
			else:
				return i

		# Order by weight so if weights need to be dropped, the most important ones stay
		if vweights is not None:
			vweights = sorted(vweights, key=lambda x: x[1], reverse=True)

		# NOTE: not comparing weights
		verts.append(Vertex(vpos, vlayers, vweights))
		return len(verts)-1

	# Extract vertices and indices from bmesh
	for face in bm.faces:
		for loop in face.loops:
			vlayers = [loop[layer_id] for _, layer_id in color_layers]
			vweights = layer_deform and loop.vert[layer_deform].items() or []
			tris.append(cache_vertex(loop.vert.co, vlayers, vweights))

	vert_positions = [swap_coords(v.position) for v in verts]
	color_data = [
		(name, [v.layers[i].copy() for v in verts])
		for i, (name, _) in enumerate(color_layers)
	]

	# Optionally extract vertex weights and armature info if it's available
	animation_data = None
	if layer_deform is not None and armature is not None:
		used_group_ids = set(group_id for v in verts for (group_id, weight) in v.weights)
		group_names = [g.name for g in obj.vertex_groups]
		index_map = {
			group_id: (index, group_names[group_id])
			for (index, group_id) in enumerate(sorted(used_group_ids))
		}

		def remap_weights(weights):
			return [(index_map[index][0], weight) for (index, weight) in weights]

		bones = []
		for (index, name) in sorted(index_map.values()):
			# TODO: hierarchy information
			# TODO: orientation information
			bone = armature.data.bones.get(name)
			if bone is None:
				print("couldn't find bone for vertex group '%s'. skipping..." % (name,))
				continue
				
			bones.append(Bone(name, swap_coords(bone.head_local), swap_coords(bone.tail_local)))

		animation_data = {
			'bones': bones,
			'data': [remap_weights(v.weights) for v in verts],
			'animations': anim.collect_animations(scene, armature)
		}

	# TODO: normal data
	# TODO: uv data

	bm.free()

	return Mesh(vert_positions, tris, color_data, animation_data)


def write_mesh(ser, mesh):
	num_vertices = len(mesh.vertices)
	num_triangles = len(mesh.triangles) // 3

	# WebGL 1 only supports 16b element arrays
	assert num_vertices < 65536
	assert len(mesh.triangles) % 3 == 0

	ser.start_section("MESH")

	ser.write_u16(num_vertices)
	for v in mesh.vertices:
		ser.write_v3(*v)
	
	ser.write_u16(num_triangles)
	if num_vertices < 256:
		for t in mesh.triangles:
			ser.write_u8(t)
	else:
		for t in mesh.triangles:
			ser.write_u16(t)

	ser.write_u8(len(mesh.color_data))
	for name, data in mesh.color_data:
		ser.write_tag("MDTA")
		ser.write_string(name)
		ser.write_u16(len(data))
		for el in data:
			ser.write_v4(*el)

	if mesh.animation_data is not None:
		bones = mesh.animation_data['bones']
		vert_weights = mesh.animation_data['data']

		ser.start_section("WEIG")
		ser.write_u8(len(bones))
		for bone in bones:
			ser.write_string(bone.name)
			ser.write_v3(*bone.head)
			ser.write_v3(*bone.tail)

		ser.write_u16(num_vertices)
			
		def chunks(lst, n):
			for i in range(0, len(lst), n):
				yield lst[i:i + n]

		# TODO: are weights always normalised? should they be?
		# if normalised, can I omit the last one?

		# limit of 3 weights per vert
		bits_per_weight_count = 2
		verts_per_chunk = 8 // bits_per_weight_count
		weights_per_vert = (1<<bits_per_weight_count)-1

		# each vert has a variable number of associated bones and weights
		# this monstrosity is just to cut down on the cost of basically 
		# encoding a list per vert
		for chunked_weights in chunks(vert_weights, verts_per_chunk):
			packed_weight_counts = 0
			for weights in chunked_weights:
				packed_weight_counts <<= bits_per_weight_count
				packed_weight_counts |= len(weights[:weights_per_vert]) # truncate least significant verts

			# pad packed_weight_counts with zero 
			padding_weight_pairs = verts_per_chunk - len(chunked_weights)
			packed_weight_counts <<= bits_per_weight_count * padding_weight_pairs

			ser.write_u8(packed_weight_counts)
			for weights in chunked_weights:
				for (index, weight) in weights[:weights_per_vert]:
					# NOTE: making an executive decision to limit to 256 bones
					ser.write_u8(index)
					ser.write_uf16(weight)

		ser.end_section()

		anim.write_animations(ser, mesh.animation_data['animations'])

	ser.end_section()