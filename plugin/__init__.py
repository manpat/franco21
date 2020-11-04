bl_info = {
	"name": "Toy Scene format",
	"author": "Patrick Monaghan",
	"description": "Exports scenes in a format that wasm-toys can eat",
	"category": "Import-Export",
	"version": (0, 0, 1),
	"blender": (2, 80, 0),
}

# ugh
if "bpy" in locals():
	import imp
	imp.reload(exporter)
	imp.reload(serializer)
	imp.reload(entity)
	imp.reload(mesh)
	imp.reload(anim)
	imp.reload(util)
else:
	import bpy
	from . import exporter, serializer, entity, mesh, anim, util


# Register and add to the file selector
def register():
	bpy.utils.register_class(exporter.ExportToyScene)
	bpy.types.TOPBAR_MT_file_export.append(exporter.menu_func)

def unregister():
	bpy.utils.unregister_class(exporter.ExportToyScene)
	bpy.types.TOPBAR_MT_file_export.remove(exporter.menu_func)

if __name__ == '__main__':
	register()
