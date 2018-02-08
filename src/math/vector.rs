use std::ops::{Add, Sub, Mul, Div, Neg};
use easing::*;
use rand::{Rand, Rng};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vec2{pub x: f32, pub y: f32}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vec3{pub x: f32, pub y: f32, pub z: f32}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vec4{pub x: f32, pub y: f32, pub z: f32, pub w: f32}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2i{pub x: i32, pub y: i32}


impl Vec2 {
	pub const fn new(x: f32, y: f32) -> Vec2 { Vec2{x, y} }
	pub const fn splat(x: f32) -> Vec2 { Vec2::new(x, x) }
	pub const fn zero() -> Vec2 { Vec2::splat(0.0) }
	pub fn from_angle(th: f32) -> Vec2 { Vec2::new(th.cos(), th.sin()) }

	pub fn to_x0z(self) -> Vec3 { Vec3::new(self.x, 0.0, self.y) }
	pub fn to_vec2i(self) -> Vec2i { Vec2i::new(self.x as i32, self.y as i32) }
	pub fn to_tuple(self) -> (f32,f32) { (self.x, self.y) }
	pub fn to_angle(self) -> f32 { self.y.atan2(self.x) }
	pub fn extend(self, z: f32) -> Vec3 { Vec3::new(self.x, self.y, z) }

	pub fn length(self) -> f32 { self.dot(self).sqrt() }
	pub fn perp(self) -> Vec2 { Vec2::new(-self.y, self.x) }
	
	pub fn normalize(self) -> Vec2 { self * (1.0/self.length()) }
	pub fn dot(self, o: Vec2) -> f32 { self.x*o.x + self.y*o.y }
}

impl Vec3 {
	pub const fn new(x: f32, y: f32, z: f32) -> Vec3 { Vec3{x, y, z} }
	pub const fn splat(x: f32) -> Vec3 { Vec3::new(x, x, x) }
	pub const fn zero() -> Vec3 { Vec3::splat(0.0) }
	pub fn from_x_angle(th: f32) -> Vec3 { Vec3::new(0.0, th.sin(), th.cos()) }
	pub fn from_y_angle(th: f32) -> Vec3 { Vec3::new(th.cos(), 0.0, th.sin()) }

	pub fn to_tuple(&self) -> (f32,f32,f32) { (self.x, self.y, self.z) }
	pub fn to_xy(self) -> Vec2 { Vec2::new(self.x, self.y) }
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

impl Vec4 {
	pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 { Vec4{x, y, z, w} }
	pub const fn splat(x: f32) -> Vec4 { Vec4::new(x, x, x, x) }
	pub const fn zero() -> Vec4 { Vec4::splat(0.0) }
	pub fn from_slice(o: &[f32]) -> Vec4 {
		assert!(o.len() >= 4);
		Vec4::new(o[0], o[1], o[2], o[3])
	}

	pub fn to_tuple(&self) -> (f32,f32,f32,f32) { (self.x, self.y, self.z, self.w) }
	pub fn to_vec3(&self) -> Vec3 { Vec3::new(self.x, self.y, self.z) }

	pub fn length(&self) -> f32 { self.dot(*self).sqrt() }

	pub fn dot(&self, o: Vec4) -> f32 { self.x*o.x + self.y*o.y + self.z*o.z + self.w*o.w }
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


impl Add for Vec2 {
	type Output = Vec2;
	fn add(self, o: Vec2) -> Vec2 {
		Vec2::new(self.x + o.x, self.y + o.y)
	}
}
impl Add<f32> for Vec2 {
	type Output = Vec2;
	fn add(self, o: f32) -> Vec2 {
		Vec2::new(self.x + o, self.y + o)
	}
}

impl Sub for Vec2 {
	type Output = Vec2;
	fn sub(self, o: Vec2) -> Vec2 {
		Vec2::new(self.x - o.x, self.y - o.y)
	}
}
impl Sub<f32> for Vec2 {
	type Output = Vec2;
	fn sub(self, o: f32) -> Vec2 {
		Vec2::new(self.x - o, self.y - o)
	}
}

impl Neg for Vec2 {
	type Output = Vec2;
	fn neg(self) -> Vec2 {
		Vec2::new(-self.x, -self.y)
	}
}

impl Mul<Vec2> for Vec2 {
	type Output = Vec2;
	fn mul(self, o: Vec2) -> Vec2 {
		Vec2::new(self.x * o.x, self.y * o.y)
	}
}

impl Div<Vec2> for Vec2 {
	type Output = Vec2;
	fn div(self, o: Vec2) -> Vec2 {
		Vec2::new(self.x / o.x, self.y / o.y)
	}
}

impl Mul<f32> for Vec2 {
	type Output = Vec2;
	fn mul(self, o: f32) -> Vec2 {
		Vec2::new(self.x * o, self.y * o)
	}
}

impl Div<f32> for Vec2 {
	type Output = Vec2;
	fn div(self, o: f32) -> Vec2 {
		Vec2::new(self.x / o, self.y / o)
	}
}


impl Add for Vec3 {
	type Output = Vec3;
	fn add(self, o: Vec3) -> Vec3 {
		Vec3::new(self.x + o.x, self.y + o.y, self.z + o.z)
	}
}
impl Add<f32> for Vec3 {
	type Output = Vec3;
	fn add(self, o: f32) -> Vec3 {
		Vec3::new(self.x + o, self.y + o, self.z + o)
	}
}

impl Sub for Vec3 {
	type Output = Vec3;
	fn sub(self, o: Vec3) -> Vec3 {
		Vec3::new(self.x - o.x, self.y - o.y, self.z - o.z)
	}
}
impl Sub<f32> for Vec3 {
	type Output = Vec3;
	fn sub(self, o: f32) -> Vec3 {
		Vec3::new(self.x - o, self.y - o, self.z - o)
	}
}

impl Neg for Vec3 {
	type Output = Vec3;
	fn neg(self) -> Vec3 {
		Vec3::new(-self.x, -self.y, -self.z)
	}
}

impl Mul<Vec3> for Vec3 {
	type Output = Vec3;
	fn mul(self, o: Vec3) -> Vec3 {
		Vec3::new(self.x * o.x, self.y * o.y, self.z * o.z)
	}
}

impl Div<Vec3> for Vec3 {
	type Output = Vec3;
	fn div(self, o: Vec3) -> Vec3 {
		Vec3::new(self.x / o.x, self.y / o.y, self.z / o.z)
	}
}

impl Mul<f32> for Vec3 {
	type Output = Vec3;
	fn mul(self, o: f32) -> Vec3 {
		Vec3::new(self.x * o, self.y * o, self.z * o)
	}
}

impl Div<f32> for Vec3 {
	type Output = Vec3;
	fn div(self, o: f32) -> Vec3 {
		Vec3::new(self.x / o, self.y / o, self.z / o)
	}
}


impl Mul<Vec4> for Vec4 {
	type Output = Vec4;
	fn mul(self, o: Vec4) -> Vec4 {
		Vec4::new(self.x * o.x, self.y * o.y, self.z * o.z, self.w * o.w)
	}
}

impl Mul<f32> for Vec4 {
	type Output = Vec4;
	fn mul(self, o: f32) -> Vec4 {
		Vec4::new(self.x * o, self.y * o, self.z * o, self.w * o)
	}
}


impl Add for Vec2i {
	type Output = Vec2i;
	fn add(self, o: Vec2i) -> Vec2i {
		Vec2i::new(self.x + o.x, self.y + o.y)
	}
}

impl Sub for Vec2i {
	type Output = Vec2i;
	fn sub(self, o: Vec2i) -> Vec2i {
		Vec2i::new(self.x - o.x, self.y - o.y)
	}
}

impl Neg for Vec2i {
	type Output = Vec2i;
	fn neg(self) -> Vec2i {
		Vec2i::new(-self.x, -self.y)
	}
}


macro_rules! impl_ease_for_vec2 {
	($func: ident) => (
		fn $func(&self, start: Vec2, end: Vec2) -> Vec2 {
			Vec2 {
				x: self.$func(start.x, end.x),
				y: self.$func(start.y, end.y),
			}
		}
	)
}

macro_rules! impl_ease_for_vec3 {
	($func: ident) => (
		fn $func(&self, start: Vec3, end: Vec3) -> Vec3 {
			Vec3 {
				x: self.$func(start.x, end.x),
				y: self.$func(start.y, end.y),
				z: self.$func(start.z, end.z),
			}
		}
	)
}

impl Ease<Vec2> for f32 {
	impl_ease_for_vec2!(ease_linear);

	impl_ease_for_vec2!(ease_quad_in);
	impl_ease_for_vec2!(ease_quad_out);
	impl_ease_for_vec2!(ease_quad_inout);

	impl_ease_for_vec2!(ease_exp_in);
	impl_ease_for_vec2!(ease_exp_out);
	impl_ease_for_vec2!(ease_exp_inout);

	impl_ease_for_vec2!(ease_elastic_in);
	impl_ease_for_vec2!(ease_elastic_out);
	impl_ease_for_vec2!(ease_elastic_inout);

	impl_ease_for_vec2!(ease_back_in);
	impl_ease_for_vec2!(ease_back_out);
	impl_ease_for_vec2!(ease_back_inout);

	impl_ease_for_vec2!(ease_bounce_in);
	impl_ease_for_vec2!(ease_bounce_out);
	impl_ease_for_vec2!(ease_bounce_inout);
}

impl Ease<Vec3> for f32 {
	impl_ease_for_vec3!(ease_linear);

	impl_ease_for_vec3!(ease_quad_in);
	impl_ease_for_vec3!(ease_quad_out);
	impl_ease_for_vec3!(ease_quad_inout);

	impl_ease_for_vec3!(ease_exp_in);
	impl_ease_for_vec3!(ease_exp_out);
	impl_ease_for_vec3!(ease_exp_inout);

	impl_ease_for_vec3!(ease_elastic_in);
	impl_ease_for_vec3!(ease_elastic_out);
	impl_ease_for_vec3!(ease_elastic_inout);

	impl_ease_for_vec3!(ease_back_in);
	impl_ease_for_vec3!(ease_back_out);
	impl_ease_for_vec3!(ease_back_inout);

	impl_ease_for_vec3!(ease_bounce_in);
	impl_ease_for_vec3!(ease_bounce_out);
	impl_ease_for_vec3!(ease_bounce_inout);
}


impl Rand for Vec2 {
	fn rand<R: Rng>(rng: &mut R) -> Self {
		Vec2::new(rng.gen(), rng.gen())
	}
}

impl Rand for Vec3 {
	fn rand<R: Rng>(rng: &mut R) -> Self {
		Vec3::new(rng.gen(), rng.gen(), rng.gen())
	}
}

impl Rand for Vec4 {
	fn rand<R: Rng>(rng: &mut R) -> Self {
		Vec4::new(rng.gen(), rng.gen(), rng.gen(), rng.gen())
	}
}

impl Rand for Vec2i {
	fn rand<R: Rng>(rng: &mut R) -> Self {
		Vec2i::new(rng.gen(), rng.gen())
	}
}