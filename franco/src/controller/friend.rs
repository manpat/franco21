use crate::prelude::*;

pub const TRICK_TIME: f32 = 1.4;
pub const FISH_PLAYER_DIST_THRESHOLD: f32 = 0.5;
pub const BOAT_PLAYER_DIST_THRESHOLD: f32 = 1.0;

pub struct FriendController {
}

impl FriendController {
	pub fn new(_engine: &mut toybox::Engine) -> FriendController {
		FriendController {}
	}

	pub fn update(&mut self, model: &mut model::Model) {
		use model::FriendState;

		for (idx, friend) in model.world.friends.iter_mut().enumerate() {
			let friend_direction = (idx % 2) as f32 * 2.0 - 1.0;

			let trick_rate = 1.0 / TRICK_TIME / 60.0;

			let dist_threshold = match friend.name.is_fish() {
				true => FISH_PLAYER_DIST_THRESHOLD,
				false => BOAT_PLAYER_DIST_THRESHOLD,
			};

			let player_diff = model.player.map_position - friend.map_position;
			let player_dist = (player_diff.length() - dist_threshold).max(0.0);
			let heading_towards_player = player_diff.to_angle();

			friend.map_position += Vec2::from_angle(friend.heading) * friend.speed / 60.0;
			friend.bob_phase += (1.0 + friend.speed / 2.0) * PI / 60.0;

			friend.heading_wander += (rand::random::<f32>()*2.0 - 1.0) * PI / 60.0;
			friend.heading_wander *= 1.0 - 1.0/60.0;

			friend.decision_timer -= 1.0/60.0;
			let decision_time = friend.decision_timer < 0.0;

			match friend.state {
				FriendState::HangingOut => {
					friend.speed += -friend.speed.min(1.0) * 4.0 / 60.0;

					// Look towards player
					friend.heading += angle_difference(heading_towards_player, friend.heading) / 60.0;

					if decision_time {
						if rand::random::<f32>() < 0.2 {
							friend.state = FriendState::DoingTricks(0.0);
						}

						friend.decision_timer = 3.0;
					}
				}

				FriendState::Following => {
					if player_dist > 0.0 {
						// Head towards player but also along player heading
						let attraction_heading_diff = angle_difference(heading_towards_player, friend.heading);
						let cohesion_heading_diff = angle_difference(model.player.heading, friend.heading);

						let heading_diff = attraction_heading_diff + cohesion_heading_diff/player_dist.max(1.0) + friend.heading_wander;

						friend.heading += heading_diff / 60.0;
					} else {
						// Head around player if too close
						friend.heading += angle_difference(heading_towards_player + PI/2.0*friend_direction, friend.heading) * 0.5 / 60.0;
					}

					if player_dist > 0.0 {
						friend.speed += (player_dist.min(4.0) - friend.speed) / 60.0;

						if decision_time {
							if rand::random::<f32>() < 0.3 {
								friend.state = FriendState::DoingTricks(0.0);
							}
						}

					} else if decision_time {
						friend.state = FriendState::HangingOut;
						friend.decision_timer = 3.0;
					}
				}

				FriendState::DoingTricks(phase) => {
					let new_phase = phase + trick_rate;
					if new_phase < 1.0 {
						friend.state = FriendState::DoingTricks(new_phase);
					} else {
						friend.decision_timer = 3.0;
						friend.state = match friend.met_player && player_dist > 0.0 {
							false => FriendState::HangingOut,
							true => FriendState::Following,
						}
					}
				}
			}
		}
	}
}




fn angle_difference(a: f32, b: f32) -> f32 {
	let mut angle_diff = (a - b) % TAU;

	if angle_diff > PI {
		angle_diff -= TAU;
	} else if angle_diff < -PI {
		angle_diff += TAU;
	}

	angle_diff
}