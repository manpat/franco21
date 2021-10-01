use crate::prelude::*;


pub struct BoatView {
	boat_mesh: gfx::Mesh<gfx::ColorVertex>,
	boat_ubo: gfx::Buffer<BoatUniforms>,
	shader: gfx::Shader,

	time: f32,
}


impl BoatView {
	pub fn new(gfx: &mut gfx::Context, resources: &model::Resources) -> Result<Self> {
		let boat_ent = resources.main_project.find_entity("Boat").unwrap();

		let raw_mesh = boat_ent.mesh_data().unwrap();
		let color_data = raw_mesh.color_data(None).unwrap();

		let mut mesh_data = gfx::MeshData::new();

		let vertices = raw_mesh.positions.iter()
			.zip(&color_data.data)
			.map(|(pos, color)| gfx::ColorVertex::new(*pos, color));

		mesh_data.extend(vertices, raw_mesh.indices.iter().cloned());

		let shader = gfx.new_simple_shader(
			include_str!("../shaders/boat.vert.glsl"),
			shaders::FLAT_COLOR_FRAG
		)?;

		Ok(BoatView {
			boat_mesh: gfx::Mesh::from_mesh_data(gfx, &mesh_data),
			boat_ubo: gfx.new_buffer(gfx::BufferUsage::Stream),
			shader,

			time: 0.0,
		})
	}

	pub fn update(&mut self, model: &model::Model) {
		let uniforms = BoatUniforms {
			transform: Mat4::translate(Vec3::from_y(self.time.cos() * 0.1))
				* Mat4::rotate_y(PI/3.0 + self.time/4.0 + model.player.heading)
				* Mat4::rotate_x(self.time.sin() * PI/32.0),
		};

		self.boat_ubo.upload(&[uniforms]);

		self.time += 1.0 / 60.0;
	}

	pub fn draw(&self, ctx: &mut view::ViewContext) {
		ctx.gfx.bind_shader(self.shader);
		ctx.gfx.bind_uniform_buffer(4, self.boat_ubo);
		self.boat_mesh.draw(&mut ctx.gfx, gfx::DrawMode::Triangles);
	}
}



#[repr(C)]
#[derive(Copy, Clone)]
struct BoatUniforms {
	transform: Mat4,
}