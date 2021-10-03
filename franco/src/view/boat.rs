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
		let speed = model.player.speed;
		let factor = (speed*40.0).clamp(0.2, 1.0);

		let uniforms = BoatUniforms {
			transform: Mat4::translate(Vec3::from_y((0.7 + self.time.sin()) * factor * 0.3))
				* Mat4::rotate_y(model.player.heading)
				* Mat4::rotate_x((self.time).sin() * PI/48.0 * factor)
				* Mat4::rotate_z((0.5 + self.time.cos()) * PI/16.0 * factor),
		};

		self.boat_ubo.upload(&[uniforms]);

		self.time += speed.max(1.0 / 60.0);
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