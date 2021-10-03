use crate::prelude::*;

use model::{UiPosition, MapObjectType};
use view::BasicMesh;


pub struct UiView {
	shader: gfx::Shader,
	mesh: gfx::Mesh<gfx::ColorVertex>,
	mesh_data: gfx::MeshData<gfx::ColorVertex>,

	zoom_in_icon: BasicMesh,
	zoom_out_icon: BasicMesh,

	map_icon: BasicMesh,
	sail_icon: BasicMesh,
	anchor_icon: BasicMesh,
	steering_wheel: BasicMesh,

	text_find_friends: BasicMesh,
	text_got_friend: BasicMesh,
	text_got_all_friends: BasicMesh,

	wiggle_phase: f32,

	map_view: MapView,
}

impl UiView {
	pub fn new(gfx: &mut gfx::Context, resources: &model::Resources) -> Result<UiView> {
		let ui_scene = resources.main_project.find_scene("ui").unwrap();

		let zoom_in_icon = BasicMesh::from_entity(ui_scene.find_entity("ICON_zoom_in").unwrap());
		let zoom_out_icon = BasicMesh::from_entity(ui_scene.find_entity("ICON_zoom_out").unwrap());

		let map_icon = BasicMesh::from_entity(ui_scene.find_entity("ICON_map").unwrap());
		let sail_icon = BasicMesh::from_entity(ui_scene.find_entity("ICON_sail").unwrap());
		let anchor_icon = BasicMesh::from_entity(ui_scene.find_entity("ICON_anchor").unwrap());
		let steering_wheel = BasicMesh::from_entity(ui_scene.find_entity("SteeringWheel").unwrap());

		let text_find_friends = BasicMesh::from_entity(ui_scene.find_entity("TEXT_find_friends").unwrap());
		let text_got_friend = BasicMesh::from_entity(ui_scene.find_entity("TEXT_got_friend").unwrap());
		let text_got_all_friends = BasicMesh::from_entity(ui_scene.find_entity("TEXT_got_all_friends").unwrap());

		let shader = gfx.new_simple_shader(shaders::COLOR_3D_VERT, shaders::FLAT_COLOR_FRAG)?;

		let map_view = MapView::new(gfx, &ui_scene)?;

		Ok(UiView {
			shader,
			mesh: gfx::Mesh::new(gfx),
			mesh_data: gfx::MeshData::new(),

			zoom_in_icon,
			zoom_out_icon,

			map_icon,
			sail_icon,
			anchor_icon,
			steering_wheel,

			text_find_friends,
			text_got_friend,
			text_got_all_friends,

			wiggle_phase: 0.0,

			map_view,
		})
	}

	pub fn update(&mut self, model: &model::Model) {
		use model::GameState;

		self.mesh_data.clear();

		let buttons = [
			(&model.ui.map_button, &self.map_icon),
			(&model.ui.sail_button, &self.sail_icon),
			(&model.ui.anchor_button, &self.anchor_icon),
			(&model.ui.zoom_in_button, &self.zoom_in_icon),
			(&model.ui.zoom_out_button, &self.zoom_out_icon),
		];

		for (button, icon) in buttons {
			let pos = button.position.resolve(model.ui.aspect);
			let phase = button.state.as_phase();
			let wiggle = (self.wiggle_phase * TAU).sin() * phase * PI/16.0;
			let transform = Mat3x4::rotate_z_translate(wiggle, pos.extend(0.0))
				* Mat3x4::uniform_scale(1.0 + phase*0.2);
			icon.build_into(&mut self.mesh_data, transform);
		}

		let text_wiggle = (self.wiggle_phase * TAU).sin() * PI/16.0;
		let text_pos = model::UiPosition::Center(Vec2::zero()).resolve(model.ui.aspect);
		let transform = Mat3x4::rotate_z_translate(text_wiggle, text_pos.extend(1.0));

		match model.global.game_state {
			GameState::Starting(_) => {
				self.text_find_friends.build_into(&mut self.mesh_data, transform);
			}
			GameState::GotFriend(_) => {
				self.text_got_friend.build_into(&mut self.mesh_data, transform);
			}
			GameState::Ending(_) => {
				self.text_got_all_friends.build_into(&mut self.mesh_data, transform);
			}

			_ => {}
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





struct MapView {
	shader: gfx::Shader,
	mesh: gfx::Mesh<gfx::ColorVertex>,
	mesh_data: gfx::MeshData<gfx::ColorVertex>,

	usable_area: Vec2,

	bg_uimesh: BasicMesh,
	island_uimesh: BasicMesh,
	rocks_uimesh: BasicMesh,
	player_uimesh: BasicMesh,
	friend_uimesh: BasicMesh,
}

impl MapView {
	fn new(gfx: &mut gfx::Context, ui_scene: &toy::SceneRef<'_>) -> Result<MapView> {
		let shader = gfx.new_simple_shader(shaders::COLOR_3D_VERT, shaders::FLAT_COLOR_FRAG)?;

		let usable_area = ui_scene.find_entity("REF_usable_area").unwrap().scale.to_xy();

		let bg_uimesh = BasicMesh::from_entity(ui_scene.find_entity("MapBg").unwrap());
		let island_uimesh = BasicMesh::from_entity(ui_scene.find_entity("ICON_island").unwrap());
		let rocks_uimesh = BasicMesh::from_entity(ui_scene.find_entity("ICON_rocks").unwrap());
		let player_uimesh = BasicMesh::from_entity(ui_scene.find_entity("ICON_player").unwrap());
		let friend_uimesh = BasicMesh::from_entity(ui_scene.find_entity("ICON_friend").unwrap());

		Ok(MapView {
			shader,
			mesh: gfx::Mesh::new(gfx),
			mesh_data: gfx::MeshData::new(),

			usable_area,

			bg_uimesh,
			island_uimesh,
			rocks_uimesh,
			player_uimesh,
			friend_uimesh,
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

			let uimesh = match object.ty {
				MapObjectType::SmallIsland => &self.island_uimesh,
				MapObjectType::Rocks2 => &self.rocks_uimesh,
				_ => continue,
			};

			uimesh.build_into(&mut self.mesh_data, island_transform);
		}

		for friend in model.world.friends.iter() {
			if friend.met_player {
				continue
			}

			let pos = (friend.map_position * map_to_ui_factor).extend(0.35);
			let island_transform = base_transform * Mat3x4::translate(pos);
			self.friend_uimesh.build_into(&mut self.mesh_data, island_transform);
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