use crate::math::vector::Vec2;
use rand::{Rand, Rng};


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2i {
	pub x: i32,
	pub y: i32,
}

impl Vec2i {
	pub const fn new(x: i32, y: i32) -> Vec2i { Vec2i{x, y} }
	pub const fn splat(x: i32) -> Vec2i { Vec2i::new(x, x) }
	pub const fn zero() -> Vec2i { Vec2i::splat(0) }

	pub fn from_tuple(t: (i32,i32)) -> Vec2i { Vec2i::new(t.0, t.1) }
	pub fn to_tuple(self) -> (i32,i32) { (self.x, self.y) }
	pub fn to_vec2(self) -> Vec2 { Vec2::new(self.x as f32, self.y as f32) }

	pub fn length(self) -> f32 {
		((self.x*self.x + self.y*self.y) as f32).sqrt()
	}
}


impl Rand for Vec2i {
	fn rand<R: Rng>(rng: &mut R) -> Self {
		Vec2i::new(rng.gen(), rng.gen())
	}
}
