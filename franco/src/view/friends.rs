use crate::prelude::*;
use std::collections::HashMap;

use view::BasicMesh;
use model::FriendState;


pub struct FriendView {
	shader: gfx::Shader,
	mesh: gfx::Mesh<gfx::ColorVertex>,
	mesh_data: gfx::MeshData<gfx::ColorVertex>,

	friend_meshes: HashMap<model::FriendName, BasicMesh>,
	anim_phase: f32,
}


impl FriendView {
	pub fn new(gfx: &mut gfx::Context, resources: &model::Resources) -> Result<Self> {
		let friend_scene = resources.main_project.find_scene("friends").unwrap();

		let friend_meshes = friend_scene.entities_with_prefix("FRIEND_")
			.map(|entity| {
				let name = model::FriendName::from_name(&entity.name);
				(name, BasicMesh::from_entity(entity))
			})
			.collect();

		let shader = gfx.new_simple_shader(shaders::COLOR_3D_VERT, shaders::FLAT_COLOR_FOG_FRAG)?;

		Ok(FriendView {
			shader,
			mesh: gfx::Mesh::new(gfx),
			mesh_data: gfx::MeshData::new(),

			friend_meshes,
			anim_phase: 0.0,
		})
	}

	pub fn update(&mut self, model: &model::Model) {
		self.mesh_data.clear();

		let player_speed = model.player.speed;

		for friend in model.world.friends.iter() {
			let player_diff_map = friend.map_position - model.player.map_position;
			let world_pos = model::map_to_world(player_diff_map).to_x0z();
			let base_transform = Mat3x4::rotate_y_translate(friend.heading, world_pos);

			let state_transform = if friend.name.is_fish() {
				calc_fish_transform(friend, self.anim_phase)
			} else {
				calc_boat_transform(friend, self.anim_phase)
			};

			self.friend_meshes[&friend.name].build_into(&mut self.mesh_data, base_transform * state_transform);
		}

		self.mesh.upload(&self.mesh_data);
		self.anim_phase += 1.0/60.0;
	}

	pub fn draw(&self, ctx: &mut view::ViewContext) {
		ctx.gfx.bind_shader(self.shader);
		self.mesh.draw(&mut ctx.gfx, gfx::DrawMode::Triangles);
	}
}



fn calc_fish_transform(friend: &model::Friend, anim_phase: f32) -> Mat3x4 {
	match friend.state {
		FriendState::HangingOut => {
			let bob = (anim_phase * PI).sin() * 0.2;
			Mat3x4::rotate_z_translate(PI/8.0, Vec3::from_y(bob))
		}

		FriendState::Following => {
			let bob = (friend.bob_phase.sin() - 0.5) * friend.speed;
			let yaw_wiggle = friend.bob_phase.sin() * PI/16.0;

			Mat3x4::rotate_y_translate(yaw_wiggle, Vec3::from_y(bob))
				* Mat3x4::rotate_z(friend.bob_phase.cos() * PI/8.0 * friend.speed.min(2.0))
		}

		FriendState::DoingTricks(phase) => {
			let height = (phase * PI).sin() * 6.0 - 2.0;
			let spin = -phase * 2.0 * TAU;

			Mat3x4::rotate_z_translate(spin, Vec3::from_y(height))
		}
	}
}



fn calc_boat_transform(friend: &model::Friend, anim_phase: f32) -> Mat3x4 {
	let bob_factor = friend.speed.clamp(0.2, 1.0) * 0.3;

	match friend.state {
		FriendState::HangingOut => {
			let bob = anim_phase.sin() * bob_factor + 0.7;
			Mat3x4::translate(Vec3::from_y(bob))
		}

		FriendState::Following => {
			let bob = friend.bob_phase.sin() * bob_factor + 0.7;
			let pitch_wobble = friend.bob_phase.cos() * PI/24.0 * friend.speed.min(2.0);
			Mat3x4::rotate_z_translate(pitch_wobble, Vec3::from_y(bob))
		}

		FriendState::DoingTricks(phase) => {
			let height = (phase * PI).sin() * 6.0 - 2.0;
			let spin = -phase * 2.0 * TAU;

			Mat3x4::rotate_z_translate(spin, Vec3::from_y(height))
		}
	}
}
