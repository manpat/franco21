use crate::prelude::*;

use model::UiPosition;



pub struct UiView {
	shader: gfx::Shader,
	mesh: gfx::Mesh<gfx::ColorVertex>,
	mesh_data: gfx::MeshData<gfx::ColorVertex>,

	map_icon: UiMesh,
	steering_wheel: UiMesh,

	wiggle_phase: f32,

	map_view: MapView,
}

impl UiView {
	pub fn new(gfx: &mut gfx::Context, resources: &model::Resources) -> Result<UiView> {
		let ui_scene = resources.main_project.find_scene("ui").unwrap();

		let map_icon = UiMesh::from_entity(ui_scene.find_entity("ICON_map").unwrap());
		let steering_wheel = UiMesh::from_entity(ui_scene.find_entity("SteeringWheel").unwrap());

		let shader = gfx.new_simple_shader(shaders::COLOR_3D_VERT, shaders::FLAT_COLOR_FRAG)?;

		let map_view = MapView::new(gfx, &ui_scene)?;

		Ok(UiView {
			shader,
			mesh: gfx::Mesh::new(gfx),
			mesh_data: gfx::MeshData::new(),

			map_icon,
			steering_wheel,

			wiggle_phase: 0.0,

			map_view,
		})
	}

	pub fn update(&mut self, model: &model::Model) {
		self.mesh_data.clear();

		let buttons = [
			(&model.ui.map_button, &self.map_icon)
		];

		for (button, icon) in buttons {
			let pos = button.position.resolve(model.ui.aspect);
			let wiggle = (self.wiggle_phase * TAU).sin() * button.state.as_phase() * PI/16.0;
			let transform = Mat3x4::rotate_z_translate(wiggle, pos.extend(0.0));
			icon.build_into(&mut self.mesh_data, transform);
		}

		let pos = model.ui.wheel.position().resolve(model.ui.aspect);
		let wheel_phase = model.ui.wheel.state.as_phase();
		let wheel_transform = Mat3x4::rotate_x_translate(-PI/8.0, pos.extend(-0.5))
			* Mat3x4::rotate_z(model.ui.wheel.angle)
			* Mat3x4::scale(Vec3::splat(1.0 + wheel_phase*0.5));

		self.steering_wheel.build_into(&mut self.mesh_data, wheel_transform);

		self.mesh.upload(&self.mesh_data);

		self.map_view.update(model);

		self.wiggle_phase += 1.5 / 60.0;
		self.wiggle_phase %= 1.0;
	}

	pub fn draw(&self, ctx: &mut view::ViewContext<'_>) {
		ctx.gfx.bind_shader(self.shader);
		self.mesh.draw(&mut ctx.gfx, gfx::DrawMode::Triangles);

		self.map_view.draw(ctx);
	}
}



pub struct UiMesh {
	vertices: Vec<(Vec3, Color)>,
	indices: Vec<u16>,
}


impl UiMesh {
	pub fn from_entity(entity: toy::EntityRef<'_>) -> UiMesh {
		let raw_mesh = entity.mesh_data().unwrap();
		let colors = &raw_mesh.color_data(None).unwrap().data;
		UiMesh {
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





struct MapView {
	shader: gfx::Shader,
	mesh: gfx::Mesh<gfx::ColorVertex>,
	mesh_data: gfx::MeshData<gfx::ColorVertex>,

	usable_area: Vec2,

	bg_uimesh: UiMesh,
	island_uimesh: UiMesh,
	player_uimesh: UiMesh,
}

impl MapView {
	fn new(gfx: &mut gfx::Context, ui_scene: &toy::SceneRef<'_>) -> Result<MapView> {
		let shader = gfx.new_simple_shader(shaders::COLOR_3D_VERT, shaders::FLAT_COLOR_FRAG)?;

		let usable_area = ui_scene.find_entity("REF_usable_area").unwrap().scale.to_xy();

		let bg_uimesh = UiMesh::from_entity(ui_scene.find_entity("MapBg").unwrap());
		let island_uimesh = UiMesh::from_entity(ui_scene.find_entity("ICON_island").unwrap());
		let player_uimesh = UiMesh::from_entity(ui_scene.find_entity("ICON_player").unwrap());

		Ok(MapView {
			shader,
			mesh: gfx::Mesh::new(gfx),
			mesh_data: gfx::MeshData::new(),

			usable_area,

			bg_uimesh,
			island_uimesh,
			player_uimesh,
		})
	}

	fn update(&mut self, model: &model::Model) {
		let open_pos = UiPosition::Center(Vec2::zero()).resolve(model.ui.aspect);
		let close_pos = UiPosition::TopLeft(Vec2::splat(-1.0)).resolve(model.ui.aspect);

		let map_phase = model.ui.map.state.as_phase();

		let map_pos = map_phase.ease_linear(close_pos, open_pos).extend(0.1);
		let map_scale = Vec3::splat(map_phase.ease_linear(0.01, 1.0));

		let base_transform = Mat3x4::scale_translate(map_scale, map_pos);

		self.mesh_data.clear();
		self.bg_uimesh.build_into(&mut self.mesh_data, base_transform);

		let map_to_ui_factor = self.usable_area / model.world.map.size;

		for object in model.world.map.objects.iter() {
			let pos = (object.map_position * map_to_ui_factor).extend(0.3);
			let island_transform = base_transform * Mat3x4::translate(pos);

			self.island_uimesh.build_into(&mut self.mesh_data, island_transform);
		}

		let pos = (model.player.map_position * map_to_ui_factor).extend(0.4);
		let player_transform = base_transform * Mat3x4::rotate_z_translate(model.player.heading, pos);
		self.player_uimesh.build_into(&mut self.mesh_data, player_transform);

		self.mesh.upload(&self.mesh_data);
	}

	fn draw(&self, ctx: &mut view::ViewContext<'_>) {
		ctx.gfx.bind_shader(self.shader);
		self.mesh.draw(&mut ctx.gfx, gfx::DrawMode::Triangles);
	}
}