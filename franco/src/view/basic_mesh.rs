use crate::prelude::*;


pub struct BasicMesh {
	vertices: Vec<(Vec3, Color)>,
	indices: Vec<u16>,
}


impl BasicMesh {
	pub fn from_entity(entity: toy::EntityRef<'_>) -> BasicMesh {
		let raw_mesh = entity.mesh_data().unwrap();
		let colors = &raw_mesh.color_data(None).unwrap().data;
		BasicMesh {
			vertices: raw_mesh.positions.iter().cloned()
				.zip(colors.iter().cloned().map(Color::from))
				.collect(),

			indices: raw_mesh.indices.clone(),
		}
	}

	pub fn build_into(&self, mesh_data: &mut gfx::MeshData<gfx::ColorVertex>, transform: Mat3x4) {
		let vertices = self.vertices.iter()
			.map(move |&(pos, color)| gfx::ColorVertex::new(transform * pos, color));

		mesh_data.extend(
			vertices,
			self.indices.iter().cloned()
		);
	}
}