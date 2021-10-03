use crate::prelude::*;

#[derive(Debug)]
pub struct Player {
	pub map_position: Vec2,
	pub heading: f32,
	pub speed: f32,

	pub sail_state: SailState,
}

impl Player {
	pub fn new() -> Player {
		Player {
			map_position: Vec2::zero(),
			heading: 0.0,
			speed: 0.0,

			sail_state: SailState::Anchored,
		}
	}
}


pub const MAX_SAIL_SPEED: i32 = 5;

#[derive(Copy, Clone, Debug)]
pub enum SailState {
	Anchored,

	Sailing {
		speed: i32
	},
}


