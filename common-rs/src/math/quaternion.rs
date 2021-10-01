use std::ops::{Add, Mul};
use crate::matrix::*;
use crate::vector::*;
use crate::easing::*;
use std::f32::consts::PI;

#[derive(Copy, Clone, Debug)]
pub struct Quat {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32,
}

impl Quat {
	pub const fn from_raw(x: f32, y: f32, z: f32, w: f32) -> Quat {
		Quat{x,y,z,w}
	}

	pub const fn identity() -> Quat {
		Quat::from_raw(0.0, 0.0, 0.0, 1.0)
	}

	#[deprecated]
	pub const fn ident() -> Quat { Quat::identity() }

	pub fn new(axis: Vec3, angle: f32) -> Quat {
		let angle = angle / 2.0;
		let s = angle.sin();

		Quat::from_raw(
			axis.x * s,
			axis.y * s,
			axis.z * s,
			angle.cos()
		)
	}

	pub fn from_pitch(pitch: f32) -> Quat {
		Quat::new(Vec3::from_x(1.0), pitch)
	}

	pub fn from_yaw(yaw: f32) -> Quat {
		Quat::new(Vec3::from_y(1.0), yaw)
	}

	pub fn from_roll(roll: f32) -> Quat {
		Quat::new(Vec3::from_z(1.0), roll)
	}

	pub fn forward(&self) -> Vec3 { *self * Vec3::from_z(-1.0) }
	pub fn right(&self) -> Vec3 { *self * Vec3::from_x(1.0) }
	pub fn up(&self) -> Vec3 { *self * Vec3::from_y(1.0) }

	pub fn imaginary(&self) -> Vec3 {
		Vec3::new(self.x, self.y, self.z)
	}

	pub fn magnitude(&self) -> f32 {
		(self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w).sqrt()
	}

	pub fn normalize(&self) -> Quat {
		let m = self.magnitude();
		Quat::from_raw(self.x/m, self.y/m, self.z/m, self.w/m)
	}

	pub fn conjugate(&self) -> Quat {
		Quat::from_raw(-self.x, -self.y, -self.z, self.w)
	}

	pub fn scale(&self, f: f32) -> Quat {
		// TODO: improve
		(*self * f + Quat::identity() * (1.0 - f)).normalize()
	}

	pub fn to_mat4(&self) -> Mat4 {
		Mat4::from_columns([
			(*self * Vec3::new(1.0, 0.0, 0.0)).extend(0.0),
			(*self * Vec3::new(0.0, 1.0, 0.0)).extend(0.0),
			(*self * Vec3::new(0.0, 0.0, 1.0)).extend(0.0),
			Vec4::from_w(1.0)
		])
	}

	pub fn to_mat3x4(&self) -> Mat3x4 {
		Mat3x4::from_columns([
			*self * Vec3::new(1.0, 0.0, 0.0),
			*self * Vec3::new(0.0, 1.0, 0.0),
			*self * Vec3::new(0.0, 0.0, 1.0),
			Vec3::splat(0.0),
		])
	}

	// Stolen and adjusted from https://en.wikipedia.org/wiki/Conversion_between_quaternions_and_Euler_angles
	// TODO: test these!
	pub fn yaw(&self) -> f32 {
		let siny_cosp = 2.0 * (self.w * self.y + self.z * self.x);
		let cosy_cosp = 1.0 - 2.0 * (self.x * self.x + self.y * self.y);
		siny_cosp.atan2(cosy_cosp)
	}

	pub fn roll(&self) -> f32 {
		let sinr_cosp = 2.0 * (self.w * self.z + self.x * self.y);
		let cosr_cosp = 1.0 - 2.0 * (self.z * self.z + self.x * self.x);
		sinr_cosp.atan2(cosr_cosp)
	}

	pub fn pitch(&self) -> f32 {
		let sinp = 2.0 * (self.w * self.x - self.y * self.z);
		if sinp.abs() >= 1.0 {
			(PI / 2.0).copysign(sinp) // use 90 degrees if out of range
		} else {
			sinp.asin()
		}
	}
}


impl Add<Quat> for Quat {
	type Output = Quat;
	fn add(self, o: Quat) -> Quat {
		Quat::from_raw(self.x+o.x, self.y+o.y, self.z+o.z, self.w+o.w)
	}
}


impl Mul<Quat> for Quat {
	type Output = Quat;
	fn mul(self, o: Quat) -> Quat {
		Quat::from_raw(
			 self.w*o.x - self.z*o.y + self.y*o.z + self.x*o.w,
			 self.z*o.x + self.w*o.y - self.x*o.z + self.y*o.w,
			-self.y*o.x + self.x*o.y + self.w*o.z + self.z*o.w,
			-self.x*o.x - self.y*o.y - self.z*o.z + self.w*o.w
		)
	}
}

impl Mul<f32> for Quat {
	type Output = Quat;
	fn mul(self, o: f32) -> Quat {
		Quat::from_raw(self.x*o, self.y*o, self.z*o, self.w*o)
	}
}

impl Mul<Vec3> for Quat {
	type Output = Vec3;
	fn mul(self, o: Vec3) -> Vec3 {
		let q = Quat::from_raw(o.x, o.y, o.z, 0.0);
		(self * q * self.conjugate()).imaginary()
	}
}


macro_rules! impl_ease_for_quat {
	($func: ident) => (
		fn $func(&self, start: Quat, end: Quat) -> Quat {
			Quat {
				x: self.$func(start.x, end.x),
				y: self.$func(start.y, end.y),
				z: self.$func(start.z, end.z),
				w: self.$func(start.w, end.w),
			}
		}
	)
}


impl Ease<Quat> for f32 {
	impl_ease_for_quat!(ease_linear);

	impl_ease_for_quat!(ease_quad_in);
	impl_ease_for_quat!(ease_quad_out);
	impl_ease_for_quat!(ease_quad_inout);

	impl_ease_for_quat!(ease_exp_in);
	impl_ease_for_quat!(ease_exp_out);
	impl_ease_for_quat!(ease_exp_inout);

	impl_ease_for_quat!(ease_elastic_in);
	impl_ease_for_quat!(ease_elastic_out);
	impl_ease_for_quat!(ease_elastic_inout);

	impl_ease_for_quat!(ease_back_in);
	impl_ease_for_quat!(ease_back_out);
	impl_ease_for_quat!(ease_back_inout);

	impl_ease_for_quat!(ease_bounce_in);
	impl_ease_for_quat!(ease_bounce_out);
	impl_ease_for_quat!(ease_bounce_inout);
}


#[cfg(test)]
mod tests {
	use crate::*;

	#[test]
	fn test_from_pitch() {
		let ident = Quat::from_pitch(0.0);
		assert_vec_eq!(ident.forward(), Vec3::from_z(-1.0));
		assert_vec_eq!(ident.right(), Vec3::from_x(1.0));
		assert_vec_eq!(ident.up(), Vec3::from_y(1.0));
		assert_almost_eq!(ident.yaw(), 0.0);
		assert_almost_eq!(ident.pitch(), 0.0);
		assert_almost_eq!(ident.roll(), 0.0);

		let r90 = Quat::from_pitch(PI/2.0);
		assert_vec_eq!(r90.forward(), Vec3::from_y(1.0));
		assert_vec_eq!(r90.right(), Vec3::from_x(1.0));
		assert_vec_eq!(r90.up(), Vec3::from_z(1.0));
		assert_almost_eq!(r90.yaw(), 0.0);
		assert_almost_eq!(r90.pitch(), PI/2.0);
		assert_almost_eq!(r90.roll(), 0.0);

		let r180 = Quat::from_pitch(PI);
		assert_vec_eq!(r180.forward(), Vec3::from_z(1.0));
		assert_vec_eq!(r180.right(), Vec3::from_x(1.0));
		assert_vec_eq!(r180.up(), Vec3::from_y(-1.0));
		// Yay for angle stability
		// assert_almost_eq!(r180.yaw(), 0.0);
		// assert_almost_eq!(r180.pitch(), PI);
		// assert_almost_eq!(r180.roll(), 0.0);
	}

	#[test]
	fn test_from_yaw() {
		let ident = Quat::from_yaw(0.0);
		assert_vec_eq!(ident.forward(), Vec3::from_z(-1.0));
		assert_vec_eq!(ident.right(), Vec3::from_x(1.0));
		assert_vec_eq!(ident.up(), Vec3::from_y(1.0));
		assert_almost_eq!(ident.yaw(), 0.0);
		assert_almost_eq!(ident.pitch(), 0.0);
		assert_almost_eq!(ident.roll(), 0.0);

		let r45 = Quat::from_yaw(PI/4.0);
		assert_vec_eq!(r45.forward(), Vec3::new(-INV_SQRT_2, 0.0, -INV_SQRT_2));
		assert_vec_eq!(r45.right(), Vec3::new(INV_SQRT_2, 0.0, -INV_SQRT_2));
		assert_vec_eq!(r45.up(), Vec3::from_y(1.0));
		assert_almost_eq!(r45.yaw(), PI/4.0);
		assert_almost_eq!(r45.pitch(), 0.0);
		assert_almost_eq!(r45.roll(), 0.0);

		let r90 = Quat::from_yaw(PI/2.0);
		assert_vec_eq!(r90.forward(), Vec3::from_x(-1.0));
		assert_vec_eq!(r90.right(), Vec3::from_z(-1.0));
		assert_vec_eq!(r90.up(), Vec3::from_y(1.0));
		assert_almost_eq!(r90.yaw(), PI/2.0);
		assert_almost_eq!(r90.pitch(), 0.0);
		assert_almost_eq!(r90.roll(), 0.0);

		let r180 = Quat::from_yaw(PI);
		assert_vec_eq!(r180.forward(), Vec3::from_z(1.0));
		assert_vec_eq!(r180.right(), Vec3::from_x(-1.0));
		assert_vec_eq!(r180.up(), Vec3::from_y(1.0));
		// Yay for angle stability
		// assert_almost_eq!(r180.yaw(), PI);
		// assert_almost_eq!(r180.pitch(), 0.0);
		// assert_almost_eq!(r180.roll(), 0.0);
	}

	#[test]
	fn test_from_roll() {
		let ident = Quat::from_roll(0.0);
		assert_vec_eq!(ident.forward(), Vec3::from_z(-1.0));
		assert_vec_eq!(ident.right(), Vec3::from_x(1.0));
		assert_vec_eq!(ident.up(), Vec3::from_y(1.0));
		assert_almost_eq!(ident.yaw(), 0.0);
		assert_almost_eq!(ident.pitch(), 0.0);
		assert_almost_eq!(ident.roll(), 0.0);

		let r90 = Quat::from_roll(PI/2.0);
		assert_vec_eq!(r90.forward(), Vec3::from_z(-1.0));
		assert_vec_eq!(r90.right(), Vec3::from_y(1.0));
		assert_vec_eq!(r90.up(), Vec3::from_x(-1.0));
		assert_almost_eq!(r90.yaw(), 0.0);
		assert_almost_eq!(r90.pitch(), 0.0);
		assert_almost_eq!(r90.roll(), PI/2.0);

		let r180 = Quat::from_roll(PI);
		assert_vec_eq!(r180.forward(), Vec3::from_z(-1.0));
		assert_vec_eq!(r180.right(), Vec3::from_x(-1.0));
		assert_vec_eq!(r180.up(), Vec3::from_y(-1.0));
		// Yay for angle stability
		// assert_almost_eq!(r180.yaw(), 0.0);
		// assert_almost_eq!(r180.pitch(), 0.0);
		// assert_almost_eq!(r180.roll(), PI);
	}
}
