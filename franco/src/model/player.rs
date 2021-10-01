use crate::prelude::*;

#[derive(Debug)]
pub struct Player {
	pub map_position: Vec2,
	pub heading: f32,
}

impl Player {
	pub fn new() -> Player {
		Player {
			map_position: Vec2::zero(),
			heading: 0.0,
		}
	}
}




