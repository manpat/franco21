use crate::prelude::*;
use std::collections::HashMap;

pub struct IslandView {
	island_kinds: HashMap<model::MapObjectType, IslandKind>,
	shader: gfx::Shader,
}


impl IslandView {
	pub fn new(gfx: &mut gfx::Context, resources: &model::Resources) -> Result<Self> {
		let island_scenes = resources.main_project.scenes()
			.filter(|scene| scene.name.starts_with("ISLAND_"));

		let island_kinds = island_scenes
			.map(|scene| {
				let ty = model::MapObjectType::from_name(&scene.name);
				(ty, IslandKind::new(gfx, scene))
			})
			.collect();

		let shader = gfx.new_simple_shader(shaders::COLOR_3D_INSTANCED_VERT, shaders::FLAT_COLOR_FOG_FRAG)?;

		Ok(IslandView {
			island_kinds,
			shader,
		})
	}

	pub fn update(&mut self, model: &model::Model) {
		let player_pos_map = model.player.map_position;

		for (ty, kind) in self.island_kinds.iter_mut() {
			let instance_transforms: Vec<_> = model.world.map.objects.iter()
				.filter(move |object| object.ty == *ty)
				.map(move |object| {
					let diff_map = object.map_position - player_pos_map;
					Mat3x4::rotate_y_translate(object.rotation, model::map_to_world(diff_map).to_x0z())
				})
				.collect();

			kind.instance_buffer.upload(&instance_transforms);
		}
	}

	pub fn draw(&self, ctx: &mut view::ViewContext) {
		ctx.gfx.bind_shader(self.shader);

		for kind in self.island_kinds.values() {
			kind.draw(ctx);
		}
	}
}




struct IslandKind {
	mesh: gfx::Mesh<gfx::ColorVertex>,
	instance_buffer: gfx::Buffer<Mat3x4>,
}


impl IslandKind {
	fn new(gfx: &mut gfx::Context, scene: toy::SceneRef<'_>) -> IslandKind {
		let mut mesh_data = gfx::MeshData::new();

		for entity in scene.entities() {
			let raw_mesh = entity.mesh_data().unwrap();
			let color_data = raw_mesh.color_data(None).unwrap();
			let txform = entity.transform();

			let vertices = raw_mesh.positions.iter()
				.zip(&color_data.data)
				.map(move |(&pos, color)| gfx::ColorVertex::new(txform * pos, color));

			mesh_data.extend(vertices, raw_mesh.indices.iter().cloned());
		}

		let mut instance_buffer = gfx.new_buffer(gfx::BufferUsage::Stream);
		instance_buffer.upload(&[
			Mat3x4::translate(Vec3::new(30.0, 0.0, 0.0))
		]);

		IslandKind {
			mesh: gfx::Mesh::from_mesh_data(gfx, &mesh_data),
			instance_buffer,
		}
	}

	fn draw(&self, ctx: &mut view::ViewContext) {
		ctx.gfx.bind_shader_storage_buffer(0, self.instance_buffer);
		self.mesh.draw_instanced(&mut ctx.gfx, gfx::DrawMode::Triangles, self.instance_buffer.len());
	}
}