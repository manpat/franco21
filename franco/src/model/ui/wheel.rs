use crate::prelude::*;

#[derive(Debug)]
pub struct Wheel {
	pub angle: f32,
}

impl Wheel {
	pub fn new() -> Wheel {
		Wheel {
			angle: 0.0,
		}
	}
}