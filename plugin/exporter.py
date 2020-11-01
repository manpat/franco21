
import bpy

from bpy_extras.io_utils import ExportHelper
from bpy.props import StringProperty, BoolProperty
from .serializer import Serializer

# import mathutils
import bmesh
# from bpy import context

# bpy.ops.export.toy_scene(filepath="/home/patrick/Development/wasm-toys/src/bin/fish/main.toy")
# bpy.ops.export.toy_scene(filepath="/home/patrick/Development/wasm-toys/untitled.toy", debug_run=True)

# blender --background main.blend --python-expr "import bpy; bpy.ops.export.toy_scene(filepath='/home/patrick/Development/skelet/assets/main', debug_run=False)"

# https://github.com/lsalzman/iqm/blob/777d946f6ba65fa93874d43c2fd728aac6c70b2d/blender-2.80/iqm_export.py

VERSION = 3


def swap_coords(co):
	assert len(co) == 3 or len(co) == 4

	if len(co) == 3:
		return [co.x, co.z, -co.y]
	else:
		return [co.x, co.z, -co.y, co.w]


class ExportToyScene(bpy.types.Operator, ExportHelper):
	"""Toy scene exporter"""
	bl_idname = "export.toy_scene"
	bl_label = "Export Toy Scene"

	filename_ext = ".toy"
	filter_glob: StringProperty(
		default="*.toy",
		options={'HIDDEN'},
		maxlen=255,  # Max internal buffer length, longer would be clamped.
	)

	debug_run: BoolProperty(
		name="Debug Run"
	)

	def execute(self, context):
		bpy.context.evaluated_depsgraph_get()
		debug_run = self.debug_run

		fname = self.filepath
		if fname == "":
			self.report({'ERROR'}, "Empty filepath!")
			return {'CANCELLED'}

		if debug_run and not fname.lower().endswith(".toy.txt"):
			fname += ".toy.txt"
		elif not fname.lower().endswith(".toy"):
			fname += ".toy"
			self.filepath += ".toy"

		self.entity_ids = {}
		self.mesh_ids = {}
		self.mesh_count = 0

		for scene in bpy.data.scenes:
			scene.view_layers[0].update() # to make sure they have a depsgraph

		with open(fname, 'wb') as out:
			ser = Serializer(out, debug_run)
			ser.write_magic_number(VERSION)

			for s in bpy.data.scenes:
				for m in self.collect_meshes(s):
					self.write_mesh(ser, m)

				entities = []
				for e in self.collect_entities(s, entities):
					self.write_entity(ser, e)

				ser.start_section("SCNE")
				ser.write_string(s.name)

				ser.write_u32(len(entities))
				for e in entities:
					ser.write_u32(e)
				ser.end_section()


		return {'FINISHED'}


	def write_mesh(self, ser, mesh):
		num_vertices = len(mesh['vertices'])
		num_triangles = len(mesh['triangles']) // 3

		# WebGL 1 only supports 16b element arrays
		assert num_vertices < 65536
		assert len(mesh['triangles']) % 3 == 0

		ser.start_section("MESH")

		ser.write_u16(num_vertices)
		for v in mesh['vertices']:
			ser.write_v3(*v)
		
		ser.write_u16(num_triangles)
		if num_vertices < 256:
			for t in mesh['triangles']:
				ser.write_u8(t)
		else:
			for t in mesh['triangles']:
				ser.write_u16(t)

		ser.write_u8(len(mesh['color_data']))
		for name, data in mesh['color_data']:
			ser.write_tag("MDTA")
			ser.write_string(name)
			ser.write_u16(len(data))
			for el in data:
				ser.write_v4(*el)

		if mesh['weight_data'] is not None:
			groups = mesh['weight_data']['groups']
			vert_weights = mesh['weight_data']['data']

			ser.start_section("WEIG")
			ser.write_u8(len(groups))
			for group in groups:
				ser.write_string(group)

			ser.write_u16(num_vertices)
				
			def chunks(lst, n):
				for i in range(0, len(lst), n):
					yield lst[i:i + n]

			# TODO: are weights always normalised? should they be?
			# if normalised, can I omit the last one?

			# limit of 3 weights per vert
			bits_per_weight_count = 2
			vert_weights_per_chunk = 8 // bits_per_weight_count

			# each vert has a variable number of associated bones and weights
			# this monstrosity is just to cut down on the cost of basically 
			# encoding a list per vert
			for chunked_weights in chunks(vert_weights, vert_weights_per_chunk):
				packed_weight_counts = 0
				for weights in chunked_weights:
					assert len(weights) < (1 << bits_per_weight_count+1)
					packed_weight_counts <<= bits_per_weight_count
					packed_weight_counts |= len(weights)

				# pad packed_weight_counts with zero 
				padding_weight_pairs = vert_weights_per_chunk - len(chunked_weights)
				packed_weight_counts <<= bits_per_weight_count * padding_weight_pairs

				ser.write_u8(packed_weight_counts)
				for weights in chunked_weights:
					for (index, weight) in weights:
						# NOTE: making an executive decision to limit to 256 bones
						ser.write_u8(index)
						ser.write_uf16(weight)

			ser.end_section()

		ser.end_section()


	def write_entity(self, ser, entity):
		ser.start_section("ENTY")
		ser.write_string(entity['name'])

		ser.write_v3(*entity['position'])
		ser.write_v4(*entity['rotation'])
		ser.write_v3(*entity['scale'])
		ser.write_u16(entity['mesh_id'])
		ser.end_section()


	def collect_meshes(self, scene):
		# depsgraph = scene.view_layers[0].depsgraph
		depsgraph = bpy.context.evaluated_depsgraph_get()

		for obj in scene.objects:
			if obj.type != 'MESH':
				continue

			odata = obj.data
			if odata in self.mesh_ids:
				continue

			armature = None
			original_armature_pose_position = None
			for modifier in obj.modifiers:
				if modifier.type == 'ARMATURE':
					armature = modifier.object
					original_armature_pose_position = armature.data.pose_position
					armature.data.pose_position = 'REST'
					depsgraph.update()
					break

			bm = bmesh.new()
			bm.from_object(obj, depsgraph, deform=True)
			bmesh.ops.triangulate(bm, faces=bm.faces)

			if armature is not None:
				armature.data.pose_position = original_armature_pose_position

			bm.verts.ensure_lookup_table()
			bm.faces.ensure_lookup_table()

			for f in bm.faces:
				assert len(f.loops) == 3

			layer_deform = bm.verts.layers.deform.active

			color_layers = bm.loops.layers.color.items()
			verts = []
			tris = []

			# TODO: find a faster way
			def vert_index(vpos, vlayers, vweights):
				for i, vb in enumerate(verts):
					if vpos != vb['pos']:
						continue

					for la, lb in zip(vlayers, vb['layers']):
						if la != lb:
							break
					else:
						return i

				# Order by weight so if weights need to be dropped, the most important ones stay
				if vweights is not None:
					vweights = sorted(vweights, key=lambda x: x[1], reverse=True)

				# NOTE: not comparing weights
				verts.append({ 'pos': vpos, 'layers': vlayers, 'weights': vweights })
				return len(verts)-1


			for face in bm.faces:
				for loop in face.loops:
					vlayers = [loop[layer_id] for _, layer_id in color_layers]
					vweights = layer_deform and loop.vert[layer_deform].items() or []
					tris.append(vert_index(loop.vert.co, vlayers, vweights))


			vert_positions = [swap_coords(v['pos']) for v in verts]
			color_layer_data = [
				(name, [v['layers'][i].copy() for v in verts])
				for i, (name, _) in enumerate(color_layers)
			]

			weight_data = None
			if layer_deform is not None:
				used_groups = set(group_id for v in verts for (group_id, weight) in v['weights'])
				group_names = [g.name for g in obj.vertex_groups]
				index_map = {
					group_id: (index, group_names[group_id])
					for (index, group_id) in enumerate(sorted(used_groups))
				}
				print(used_groups)
				print(group_names)
				print(index_map)

				def remap_weights(weights):
					return [(index_map[index][0], weight) for (index, weight) in weights]

				weight_data = {
					'groups': [name for (index, name) in sorted(index_map.values())],
					'data': [remap_weights(v['weights']) for v in verts]
				}

			# TODO: normal data
			# TODO: uv data

			bm.free()

			self.mesh_count += 1
			self.mesh_ids[odata] = self.mesh_count # ids start at 1

			yield {
				'vertices': vert_positions,
				'triangles': tris,
				'color_data': color_layer_data,
				'weight_data': weight_data,
			}



	def collect_entities(self, scene, ent_list):
		for obj in scene.objects:
			entity_id = self.entity_ids.get(obj, None)
			if entity_id is not None:
				ent_list.append(entity_id)
				continue

			# entity ids are 1-indexed
			entity_id = len(self.entity_ids) + 1
			self.entity_ids[obj] = entity_id

			ent_list.append(entity_id)

			mesh_id = 0
			if obj.data:
				mesh_id = self.mesh_ids.get(obj.data, 0)

			# TODO: object type
			# TODO: collections
			# TODO: tags

			scale = obj.scale
			scale = [scale.x, scale.z, scale.y]

			# TODO: handle parent transforms

			yield {
				'name': obj.name,
				'entity_id': entity_id,
				'mesh_id': mesh_id,

				'position': swap_coords(obj.location.xyz),
				'rotation': swap_coords(obj.rotation_euler.to_quaternion()), # This okay so long as handedness stays the same
				'scale': scale,
			}




def menu_func(self, context):
	self.layout.operator_context = 'INVOKE_DEFAULT'
	self.layout.operator(ExportToyScene.bl_idname, text="Toy Scene (.toy)")