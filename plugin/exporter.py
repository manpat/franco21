
import bpy

from bpy_extras.io_utils import ExportHelper
from bpy.props import StringProperty, BoolProperty
from . import mesh, entity, serializer
from .util import swap_coords

# bpy.ops.export.toy_scene(filepath="/home/patrick/Development/wasm-toys/src/bin/fish/main.toy")
# bpy.ops.export.toy_scene(filepath="/home/patrick/Development/wasm-toys/untitled.toy", debug_run=True)

# blender --background main.blend --python-expr "import bpy; bpy.ops.export.toy_scene(filepath='/home/patrick/Development/skelet/assets/main', debug_run=False)"

# https://github.com/lsalzman/iqm/blob/777d946f6ba65fa93874d43c2fd728aac6c70b2d/blender-2.80/iqm_export.py

VERSION = 3



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
			ser = serializer.Serializer(out, debug_run)
			ser.write_magic_number(VERSION)

			for s in bpy.data.scenes:
				for m in self.collect_meshes(s):
					mesh.write_mesh(ser, m)

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


	def write_entity(self, ser, entity):
		ser.start_section("ENTY")
		ser.write_string(entity.name)

		ser.write_v3(*entity.position)
		ser.write_v4(*entity.rotation)
		ser.write_v3(*entity.scale)
		ser.write_u16(entity.mesh_id)
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

			self.mesh_count += 1
			self.mesh_ids[odata] = self.mesh_count # ids start at 1

			yield mesh.collect_mesh(depsgraph, obj)


	def collect_entities(self, scene, ent_list):
		for obj in scene.objects:
			# Armature encoded with mesh
			if obj.type == 'ARMATURE':
				continue

			entity_id = self.entity_ids.get(obj, None)
			if entity_id is not None:
				ent_list.append(entity_id)
				continue

			# entity ids are 1-indexed
			entity_id = len(self.entity_ids) + 1
			self.entity_ids[obj] = entity_id

			ent_list.append(entity_id)

			mesh_id = 0
			if obj.type == 'MESH' and obj.data:
				mesh_id = self.mesh_ids.get(obj.data, 0)

			# TODO: object type
			# TODO: collections
			# TODO: tags

			scale = obj.scale
			scale = [scale.x, scale.z, scale.y]

			# TODO: handle parent transforms

			yield entity.Entity(
				obj.name, entity_id, mesh_id,

				swap_coords(obj.location.xyz),
				swap_coords(obj.rotation_euler.to_quaternion()), # This okay so long as handedness stays the same
				scale,
			)




def menu_func(self, context):
	self.layout.operator_context = 'INVOKE_DEFAULT'
	self.layout.operator(ExportToyScene.bl_idname, text="Toy Scene (.toy)")