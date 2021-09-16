use crate::prelude::*;

pub mod traits;
pub mod geom_2d;
pub mod geom_3d;
pub mod color_mesh_builder;
pub mod plane_mesh_builder_adaptor;

pub use traits::{PolyBuilder2D, PolyBuilder3D, ColoredPolyBuilder};
pub use color_mesh_builder::ColorMeshBuilder;
pub use plane_mesh_builder_adaptor::PlaneMeshBuilderAdaptor;

pub mod geom {
	pub use super::geom_2d::*;
	pub use super::geom_3d::*;
}


pub struct Mesh<V: gfx::Vertex> {
	pub vao: gfx::Vao,
	pub vertex_buffer: gfx::Buffer<V>,
	pub index_buffer: gfx::Buffer<u16>,
}


impl<V: gfx::Vertex> Mesh<V> {
	pub fn with_buffer_usage(gfx: &mut gfx::Context, buffer_usage: gfx::BufferUsage) -> Self {
		let mut vao = gfx.new_vao();

		let vertex_buffer = gfx.new_buffer(buffer_usage);
		let index_buffer = gfx.new_buffer(buffer_usage);

		vao.bind_vertex_buffer(0, vertex_buffer);
		vao.bind_index_buffer(index_buffer);

		Mesh {
			vao,
			vertex_buffer,
			index_buffer,
		}
	}

	pub fn new(gfx: &mut gfx::Context) -> Self {
		Mesh::with_buffer_usage(gfx, gfx::BufferUsage::Stream)
	}

	pub fn from_mesh_data(gfx: &mut gfx::Context, mesh_data: &MeshData<V>) -> Self {
		let mut mesh = Mesh::with_buffer_usage(gfx, gfx::BufferUsage::Static);
		mesh.upload(mesh_data);
		mesh
	}

	pub fn draw(&self, gfx: &mut gfx::RenderState<'_>, draw_mode: gfx::DrawMode) {
		gfx.bind_vao(self.vao);
		gfx.draw_indexed(draw_mode, self.index_buffer.len());
	}

	pub fn draw_instanced(&self, gfx: &mut gfx::RenderState<'_>, draw_mode: gfx::DrawMode, num_instances: u32) {
		gfx.bind_vao(self.vao);
		gfx.draw_instances_indexed(draw_mode, self.index_buffer.len(), num_instances);
	}

	pub fn upload(&mut self, mesh_data: &MeshData<V>) {
		self.vertex_buffer.upload(&mesh_data.vertices);
		self.index_buffer.upload(&mesh_data.indices);
	}

	pub fn upload_separate(&mut self, vertices: &[V], indices: &[u16]) {
		self.vertex_buffer.upload(vertices);
		self.index_buffer.upload(indices);
	}
}



pub struct MeshData<V: gfx::Vertex> {
	pub vertices: Vec<V>,
	pub indices: Vec<u16>,
}


impl<V: gfx::Vertex> MeshData<V> {
	pub fn new() -> Self {
		MeshData {
			vertices: Vec::new(),
			indices: Vec::new(),
		}
	}

	pub fn clear(&mut self) {
		self.vertices.clear();
		self.indices.clear();
	}

	pub fn extend(&mut self, vs: impl IntoIterator<Item=V>, is: impl IntoIterator<Item=u16>) {
		let index_start = self.vertices.len() as u16;
		self.vertices.extend(vs);
		self.indices.extend(is.into_iter().map(|idx| index_start + idx));
	}
}
