use crate::math::vector::{Vec2, Vec4};
use rand::{Rand, Rng};


#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Vec3 {
	pub const fn new(x: f32, y: f32, z: f32) -> Vec3 { Vec3{x, y, z} }
	pub const fn splat(x: f32) -> Vec3 { Vec3::new(x, x, x) }
	pub const fn zero() -> Vec3 { Vec3::splat(0.0) }
	pub fn from_x_angle(th: f32) -> Vec3 { Vec3::new(0.0, th.sin(), th.cos()) }
	pub fn from_y_angle(th: f32) -> Vec3 { Vec3::new(th.cos(), 0.0, th.sin()) }
	pub fn from_slice(o: &[f32]) -> Vec3 {
		assert!(o.len() >= 3);
		Vec3::new(o[0], o[1], o[2])
	}

	pub const fn from_x(x: f32) -> Vec3 { Vec3::new(x, 0.0, 0.0) }
	pub const fn from_y(y: f32) -> Vec3 { Vec3::new(0.0, y, 0.0) }
	pub const fn from_z(z: f32) -> Vec3 { Vec3::new(0.0, 0.0, z) }

	pub fn to_tuple(&self) -> (f32,f32,f32) { (self.x, self.y, self.z) }
	pub fn to_xy(&self) -> Vec2 { Vec2::new(self.x, self.y) }
	pub fn to_xz(&self) -> Vec2 { Vec2::new(self.x, self.z) }
	pub fn extend(&self, w: f32) -> Vec4 { Vec4::new(self.x, self.y, self.z, w) }

	pub fn length(&self) -> f32 { self.dot(*self).sqrt() }
	pub fn normalize(&self) -> Vec3 { *self * (1.0/self.length()) }

	pub fn dot(&self, o: Vec3) -> f32 { self.x*o.x + self.y*o.y + self.z*o.z }
	pub fn cross(&self, o: Vec3) -> Vec3 {
		Vec3::new(
			self.y*o.z - self.z*o.y,
			self.z*o.x - self.x*o.z,
			self.x*o.y - self.y*o.x,
		)
	}
}


impl Rand for Vec3 {
	fn rand<R: Rng>(rng: &mut R) -> Self {
		Vec3::new(rng.gen(), rng.gen(), rng.gen())
	}
}
