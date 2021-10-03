use crate::prelude::*;
use model::SailState;

pub struct PlayerController {
}

impl PlayerController {
	pub fn new(_engine: &mut toybox::Engine) -> PlayerController {
		PlayerController {}
	}

	pub fn update(&mut self, model: &mut model::Model) {

		let heading_factor = (1.0 / (1.0 + model.player.speed*0.5)).clamp(0.1, 1.0);

		let (target_speed, acceleration) = match model.player.sail_state {
			SailState::Anchored => (0.0, 2.0),
			SailState::Sailing{speed} => (speed as f32 * 0.5, 1.0),
		};

		model.player.speed += (target_speed - model.player.speed).min(0.005) * acceleration;

		model.player.heading += model.ui.wheel.angle/2.0 * heading_factor / 60.0;

		let map_velocity = Vec2::from_angle(model.player.heading) * model.player.speed / 60.0;
		model.player.map_position += map_velocity;


		// Wrap player position to within the map with a margin
		let map_size = model.world.map.size + Vec2::splat(50.0);
		let player_pos = &mut model.player.map_position;
		let prev_player_pos = *player_pos;

		player_pos.x = (player_pos.x + map_size.x/2.0).rem_euclid(map_size.x) - map_size.x/2.0;
		player_pos.y = (player_pos.y + map_size.y/2.0).rem_euclid(map_size.y) - map_size.y/2.0;


		// If we've warped, teleport friends
		let diff = (*player_pos - prev_player_pos) / map_size;
		let tp_dist = Vec2::new(diff.x.trunc() * map_size.x, diff.y.trunc() * map_size.y);

		if tp_dist.length() > 0.0 {
			for friend in model.world.friends.iter_mut() {
				friend.map_position += tp_dist;
			}
		}
	}
}