use crate::prelude::*;


pub struct WaterView {
	plane_mesh: gfx::Mesh<gfx::ColorVertex>,
	ripple_mesh: gfx::Mesh<gfx::ColorVertex>,
	basic_shader: gfx::Shader,
	ripple_shader: gfx::Shader,

	ripple_instance_buffer: gfx::Buffer<RippleInstanceData>,
	water_ubo: gfx::Buffer<WaterUniforms>,
	wave_phase: f32,
}


impl WaterView {
	pub fn new(gfx: &mut gfx::Context, resources: &model::Resources) -> Result<Self> {
		let water_plane_ent = resources.main_project.find_entity("WATER_plane").unwrap();
		let ripple_ent = resources.main_project.find_entity("WATER_ripple").unwrap();

		let plane_mesh = {
			let raw_mesh = water_plane_ent.mesh_data().unwrap();

			let mut mesh_data = gfx::MeshData::new();

			let vertices = raw_mesh.positions.iter()
				.map(|pos| gfx::ColorVertex::new(*pos, Vec3::zero()));

			mesh_data.extend(vertices, raw_mesh.indices.iter().cloned());

			gfx::Mesh::from_mesh_data(gfx, &mesh_data)
		};

		let ripple_mesh = {
			let raw_mesh = ripple_ent.mesh_data().unwrap();

			let mut mesh_data = gfx::MeshData::new();

			let vertices = raw_mesh.positions.iter()
				.map(|pos| gfx::ColorVertex::new(*pos, Vec3::zero()));

			mesh_data.extend(vertices, raw_mesh.indices.iter().cloned());

			gfx::Mesh::from_mesh_data(gfx, &mesh_data)
		};

		let basic_shader = gfx.new_simple_shader(
			include_str!("../shaders/water_basic.vert.glsl"),
			shaders::WATER_FRAG
		)?;

		let ripple_shader = gfx.new_simple_shader(
			include_str!("../shaders/water.vert.glsl"),
			shaders::WATER_FRAG
		)?;

		Ok(WaterView {
			plane_mesh,
			ripple_mesh,

			basic_shader,
			ripple_shader,

			ripple_instance_buffer: gfx.new_buffer(gfx::BufferUsage::Stream),
			water_ubo: gfx.new_buffer(gfx::BufferUsage::Stream),
			wave_phase: 0.0,
		})
	}

	pub fn update(&mut self, model: &model::Model) {
		let mut instances = [
			Vec2::new(-28.0,-28.0),
			Vec2::new(-22.0, 0.0),
			Vec2::new(-26.0, 22.0),

			Vec2::new(-6.0,-20.0),
			Vec2::new( 1.0, 0.0),
			Vec2::new( 2.0, 29.0),

			Vec2::new(25.0,-21.0),
			Vec2::new(27.0, 4.0),
			Vec2::new(29.0, 27.0),
		];

		let player_pos_world = model::map_to_world(model.player.map_position);

		let max_diff = 47.0;

		for instance in instances.iter_mut() {
			let diff = *instance - player_pos_world;
			instance.x = (diff.x + max_diff).rem_euclid(max_diff*2.0) - max_diff;
			instance.y = (diff.y + max_diff).rem_euclid(max_diff*2.0) - max_diff;
		}

		let instance_data: Vec<_> = instances.iter().enumerate()
			.map(|(idx, inst)| {
				let dist = inst.length();
				RippleInstanceData {
					worldspace_offset: *inst,
					ripple_factor: (1.0 - (dist/max_diff).powf(1.8)).clamp(0.0, 1.0),
					ripple_phase: idx as f32 / 2.3 + self.wave_phase,
				}
			})
			.collect();

		self.ripple_instance_buffer.upload(&instance_data);


		let uniforms = WaterUniforms {
			base_color: Color::hsv(200.0, 0.6, 0.89).into(),
			peak_color: Color::hsv(200.0, 0.2, 1.0).into(),
			peak_start: 0.3,
			peak_height: 0.5,
		};

		self.water_ubo.upload(&[uniforms]);


		self.wave_phase += 1.0/60.0;
	}

	pub fn draw(&self, ctx: &mut view::ViewContext) {
		ctx.gfx.bind_uniform_buffer(4, self.water_ubo);

		ctx.gfx.bind_shader(self.basic_shader);
		self.plane_mesh.draw(&mut ctx.gfx, gfx::DrawMode::Triangles);

		ctx.gfx.bind_shader(self.ripple_shader);
		ctx.gfx.bind_shader_storage_buffer(0, self.ripple_instance_buffer);
		self.ripple_mesh.draw_instanced(&mut ctx.gfx, gfx::DrawMode::Triangles, self.ripple_instance_buffer.len());
	}
}



#[repr(C)]
#[derive(Copy, Clone)]
struct RippleInstanceData {
	worldspace_offset: Vec2,
	ripple_factor: f32,
	ripple_phase: f32,
}



#[repr(C)]
#[derive(Copy, Clone)]
struct WaterUniforms {
	base_color: Vec3,
	peak_start: f32,
	peak_color: Vec3,
	peak_height: f32,
}

