use crate::prelude::*;


toybox::declare_input_context! {
	struct PlayerActions "Player Control" {
		state forward { "Forward" [Scancode::W] }
		state back { "Back" [Scancode::S] }
		// mouse mouse { "Mouse" [1.0] }
	}
}

pub struct PlayerController {
	actions: PlayerActions,
}

impl PlayerController {
	pub fn new(engine: &mut toybox::Engine) -> PlayerController {
		PlayerController {
			actions: PlayerActions::new_active(&mut engine.input),
		}
	}

	pub fn update(&mut self, engine: &mut toybox::Engine, model: &mut model::Model) {
		let input = engine.input.frame_state();

		if input.active(self.actions.forward) {
			model.player.speed += 0.01/60.0;
		}

		if input.active(self.actions.back) {
			model.player.speed -= 0.01/60.0;
		}

		model.player.speed = model.player.speed.clamp(0.0, 3.0);

		let heading_factor = (1.0 - model.player.speed*10.0).clamp(0.1, 1.0);
		let turn_rate = 0.5*PI/60.0 * heading_factor;

		// if input.active(self.actions.left) {
		// 	model.player.heading += turn_rate;
		// }

		// if input.active(self.actions.right) {
		// 	model.player.heading -= turn_rate;
		// }

		model.player.heading += model.ui.wheel.angle/3.0 * heading_factor / 60.0;

		let map_velocity = Vec2::from_angle(model.player.heading) * model.player.speed;
		model.player.map_position += map_velocity;


		let map_size = model.world.map.size;
		let player_pos = &mut model.player.map_position;

		// Wrap player position to within the map
		player_pos.x = (player_pos.x + map_size.x/2.0).rem_euclid(map_size.x) - map_size.x/2.0;
		player_pos.y = (player_pos.y + map_size.y/2.0).rem_euclid(map_size.y) - map_size.y/2.0;
	}
}