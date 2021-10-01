use crate::prelude::*;


pub struct WaterView {
	water_mesh: gfx::Mesh<gfx::ColorVertex>,
	shader: gfx::Shader,
}


impl WaterView {
	pub fn new(gfx: &mut gfx::Context, resources: &model::Resources) -> Result<Self> {
		let boat_ent = resources.main_project.find_entity("WaterPlane").unwrap();

		let raw_mesh = boat_ent.mesh_data().unwrap();
		let color_data = raw_mesh.color_data(None).unwrap();

		let mut mesh_data = gfx::MeshData::new();

		let vertices = raw_mesh.positions.iter()
			.zip(&color_data.data)
			.map(|(pos, color)| gfx::ColorVertex::new(*pos, color));

		mesh_data.extend(vertices, raw_mesh.indices.iter().cloned());

		let shader = gfx.new_simple_shader(
			include_str!("../shaders/water.vert.glsl"),
			shaders::FLAT_COLOR_FRAG
		)?;

		Ok(WaterView {
			water_mesh: gfx::Mesh::from_mesh_data(gfx, &mesh_data),
			shader,
		})
	}

	pub fn update(&mut self) {}

	pub fn draw(&self, ctx: &mut view::ViewContext) {
		ctx.gfx.bind_shader(self.shader);
		self.water_mesh.draw(&mut ctx.gfx, gfx::DrawMode::Triangles);
	}
}
