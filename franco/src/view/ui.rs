use crate::prelude::*;

pub struct UiView {
	shader: gfx::Shader,
	mesh: gfx::Mesh<gfx::ColorVertex>,
	mesh_data: gfx::MeshData<gfx::ColorVertex>,

	map_icon: UiMesh,
	steering_wheel: UiMesh,
}

impl UiView {
	pub fn new(gfx: &mut gfx::Context, resources: &model::Resources) -> Result<UiView> {
		let ui_scene = resources.main_project.find_scene("ui").unwrap();

		let map_icon = UiMesh::from_entity(ui_scene.find_entity("ICON_map").unwrap());
		let steering_wheel = UiMesh::from_entity(ui_scene.find_entity("SteeringWheel").unwrap());

		let shader = gfx.new_simple_shader(shaders::COLOR_3D_VERT, shaders::FLAT_COLOR_FRAG)?;

		Ok(UiView {
			shader,
			mesh: gfx::Mesh::new(gfx),
			mesh_data: gfx::MeshData::new(),

			map_icon,
			steering_wheel,
		})
	}

	pub fn update(&mut self, model: &model::Model) {
		self.mesh_data.clear();

		for button in model.ui.buttons.iter() {
			let pos = button.position.resolve(model.ui.aspect);
			let transform = Mat3x4::translate(pos.extend(0.0));
			self.map_icon.build_into(&mut self.mesh_data, transform);
		}

		let pos = model::UiPosition::Bottom(0.0).resolve(model.ui.aspect);
		let wheel_transform = Mat3x4::rotate_x_translate(-PI/8.0, pos.extend(-0.5))
			* Mat3x4::rotate_z(model.ui.wheel.angle)
			* Mat3x4::scale(Vec3::splat(1.0));

		self.steering_wheel.build_into(&mut self.mesh_data, wheel_transform);

		self.mesh.upload(&self.mesh_data);
	}

	pub fn draw(&self, ctx: &mut view::ViewContext<'_>) {
		ctx.gfx.bind_shader(self.shader);
		self.mesh.draw(&mut ctx.gfx, gfx::DrawMode::Triangles);
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