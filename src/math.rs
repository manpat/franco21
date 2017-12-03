#![allow(dead_code)]

use easing::*;

use std::ops::{Add, Sub, Mul, Div, Neg};

pub use std::f32::consts::PI;

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

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Mat4{pub rows: [Vec4; 4]}

#[derive(Copy, Clone, Debug)]
pub struct Quat{pub x: f32, pub y: f32, pub z: f32, pub w: f32}

impl Vec2 {
	pub const fn new(x: f32, y: f32) -> Vec2 { Vec2{x, y} }
	pub const fn splat(x: f32) -> Vec2 { Vec2::new(x, x) }
	pub const fn zero() -> Vec2 { Vec2::splat(0.0) }
	pub fn from_angle(th: f32) -> Vec2 { Vec2::new(th.cos(), th.sin()) }

	pub fn to_x0z(self) -> Vec3 { Vec3::new(self.x, 0.0, self.y) }
	pub fn to_vec2i(self) -> Vec2i { Vec2i::new(self.x as i32, self.y as i32) }
	pub fn to_tuple(self) -> (f32,f32) { (self.x, self.y) }
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

impl Mat4 {
	pub fn new(d: &[f32; 16]) -> Mat4 {
		Mat4 {
			rows: [
				Vec4::from_slice(&d[0..4]),
				Vec4::from_slice(&d[4..8]),
				Vec4::from_slice(&d[8..12]),
				Vec4::from_slice(&d[12..16]),
			]
		}
	}

	pub fn from_rows(rows: [Vec4; 4]) -> Mat4 { Mat4 { rows } }

	pub fn ident() -> Mat4 { Mat4::uniform_scale(1.0) }
	pub fn uniform_scale(s: f32) -> Mat4 { Mat4::scale(Vec3::new(s,s,s)) }

	pub fn scale(s: Vec3) -> Mat4 {
		Mat4::new(&[
			s.x, 0.0, 0.0, 0.0,
			0.0, s.y, 0.0, 0.0, 
			0.0, 0.0, s.z, 0.0,
			0.0, 0.0, 0.0, 1.0,
		])
	}

	pub fn translate(t: Vec3) -> Mat4 {
		Mat4::new(&[
			1.0, 0.0, 0.0, t.x,
			0.0, 1.0, 0.0, t.y, 
			0.0, 0.0, 1.0, t.z,
			0.0, 0.0, 0.0, 1.0,
		])
	}

	pub fn xrot(ph: f32) -> Mat4 {
		let (rx, ry) = (ph.cos(), ph.sin());

		Mat4::new(&[
			1.0, 0.0, 0.0, 0.0, 
			0.0,  rx, -ry, 0.0,
			0.0,  ry,  rx, 0.0,
			0.0, 0.0, 0.0, 1.0,
		])
	}
	pub fn yrot(ph: f32) -> Mat4 {
		let (rx, ry) = (ph.cos(), ph.sin());

		Mat4::new(&[
			 rx, 0.0, -ry, 0.0,
			0.0, 1.0, 0.0, 0.0, 
			 ry, 0.0,  rx, 0.0,
			0.0, 0.0, 0.0, 1.0,
		])
	}

	pub fn transpose(&self) -> Mat4 {
		let [a,b,c,d] = self.rows;

		Mat4::new(&[
			a.x, b.x, c.x, d.x,
			a.y, b.y, c.y, d.y,
			a.z, b.z, c.z, d.z,
			a.w, b.w, c.w, d.w,
		])
	}

	pub fn frustum(l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) -> Mat4 {
		let xco = 2.0 * n / (r - l);
		let yco = 2.0 * n / (t - b);

		let x2z = (r + l) / (r - l);
		let y2z = (t + b) / (t - b);
		let zco =-(f + n) / (f - n);

		let ztr =-2.0 * f * n / (f - n);

		Mat4::new(&[
			xco, 0.0, x2z, 0.0,
			0.0, yco, y2z, 0.0, 
			0.0, 0.0, zco, ztr,
			0.0, 0.0,-1.0, 1.0,
		])
	}

	pub fn perspective(fov: f32, aspect: f32, n: f32, f: f32) -> Mat4 {
		let scale = (fov / 2.0).tan() * n;
		let r = aspect * scale;
		let t = scale;
		Mat4::frustum(-r, r,-t, t, n, f)
	}

	pub fn determinant(&self) -> f32 {
		let [a,b,c,d] = self.rows;

		  a.x * b.y * c.z * d.w
		+ a.x * b.z * c.w * d.y
		+ a.x * b.w * c.y * d.z

		+ a.y * b.x * c.w * d.z
		+ a.y * b.z * c.x * d.w
		+ a.y * b.w * c.z * d.x

		+ a.z * b.x * c.y * d.w
		+ a.z * b.y * c.w * d.x
		+ a.z * b.w * c.x * d.y

		+ a.w * b.x * c.z * d.y
		+ a.w * b.y * c.x * d.z
		+ a.w * b.z * c.y * d.x

		- a.x * b.y * c.w * d.z
		- a.x * b.z * c.y * d.w
		- a.x * b.w * c.z * d.y

		- a.y * b.x * c.z * d.w
		- a.y * b.z * c.w * d.x
		- a.y * b.w * c.x * d.z

		- a.z * b.x * c.w * d.y
		- a.z * b.y * c.x * d.w
		- a.z * b.w * c.y * d.x

		- a.w * b.x * c.y * d.z
		- a.w * b.y * c.z * d.x
		- a.w * b.z * c.x * d.y
	}

	pub fn inverse(&self) -> Mat4 {
		let [a,b,c,d] = self.rows;
		let inv_det = 1.0 / self.determinant();

		Mat4::from_rows([
			Vec4::new(
				b.y * c.z * d.w
				+ b.z * c.w * d.y
				+ b.w * c.y * d.z
				- b.y * c.w * d.z
				- b.z * c.y * d.w
				- b.w * c.z * d.y,

				a.y * c.w * d.z
				+ a.z * c.y * d.w
				+ a.w * c.z * d.y
				- a.y * c.z * d.w
				- a.z * c.w * d.y
				- a.w * c.y * d.z,

				a.y * b.z * d.w
				+ a.z * b.w * d.y
				+ a.w * b.y * d.z
				- a.y * b.w * d.z
				- a.z * b.y * d.w
				- a.w * b.z * d.y,

				a.y * b.w * c.z
				+ a.z * b.y * c.w
				+ a.w * b.z * c.y
				- a.y * b.z * c.w
				- a.z * b.w * c.y
				- a.w * b.y * c.z
			) * inv_det,

			Vec4::new(
				b.x * c.w * d.z
				+ b.z * c.x * d.w
				+ b.w * c.z * d.x
				- b.x * c.z * d.w
				- b.z * c.w * d.x
				- b.w * c.x * d.z,

				a.x * c.z * d.w
				+ a.z * c.w * d.x
				+ a.w * c.x * d.z
				- a.x * c.w * d.z
				- a.z * c.x * d.w
				- a.w * c.z * d.x,

				a.x * b.w * d.z
				+ a.z * b.x * d.w
				+ a.w * b.z * d.x
				- a.x * b.z * d.w
				- a.z * b.w * d.x
				- a.w * b.x * d.z,

				a.x * b.z * c.w
				+ a.z * b.w * c.x
				+ a.w * b.x * c.z
				- a.x * b.w * c.z
				- a.z * b.x * c.w
				- a.w * b.z * c.x
			) * inv_det,

			Vec4::new(
				b.x * c.y * d.w
				+ b.y * c.w * d.x
				+ b.w * c.x * d.y
				- b.x * c.w * d.y
				- b.y * c.x * d.w
				- b.w * c.y * d.x,

				a.x * c.w * d.y
				+ a.y * c.x * d.w
				+ a.w * c.y * d.x
				- a.x * c.y * d.w
				- a.y * c.w * d.x
				- a.w * c.x * d.y,

				a.x * b.y * d.w
				+ a.y * b.w * d.x
				+ a.w * b.x * d.y
				- a.x * b.w * d.y
				- a.y * b.x * d.w
				- a.w * b.y * d.x,

				a.x * b.w * c.y
				+ a.y * b.x * c.w
				+ a.w * b.y * c.x
				- a.x * b.y * c.w
				- a.y * b.w * c.x
				- a.w * b.x * c.y
			) * inv_det,

			Vec4::new(
				b.x * c.z * d.y
				+ b.y * c.x * d.z
				+ b.z * c.y * d.x
				- b.x * c.y * d.z
				- b.y * c.z * d.x
				- b.z * c.x * d.y,

				a.x * c.y * d.z
				+ a.y * c.z * d.x
				+ a.z * c.x * d.y
				- a.x * c.z * d.y
				- a.y * c.x * d.z
				- a.z * c.y * d.x,

				a.x * b.z * d.y
				+ a.y * b.x * d.z
				+ a.z * b.y * d.x
				- a.x * b.y * d.z
				- a.y * b.z * d.x
				- a.z * b.x * d.y,

				a.x * b.y * c.z
				+ a.y * b.z * c.x
				+ a.z * b.x * c.y
				- a.x * b.z * c.y
				- a.y * b.x * c.z
				- a.z * b.y * c.x
			) * inv_det
		])
	}
}

impl Quat {
	pub fn from_raw(x: f32, y: f32, z: f32, w: f32) -> Quat {
		Quat{x,y,z,w}
	}

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

	pub fn to_mat4(&self) -> Mat4 {
		Mat4::from_rows([
			(*self * Vec3::new(1.0, 0.0, 0.0)).extend(0.0),
			(*self * Vec3::new(0.0, 1.0, 0.0)).extend(0.0),
			(*self * Vec3::new(0.0, 0.0, 1.0)).extend(0.0),
			Vec4::new(0.0, 0.0, 0.0, 1.0)
		])
	}
}

impl Add for Vec2 {
	type Output = Vec2;
	fn add(self, o: Vec2) -> Vec2 {
		Vec2::new(self.x + o.x, self.y + o.y)
	}
}

impl Sub for Vec2 {
	type Output = Vec2;
	fn sub(self, o: Vec2) -> Vec2 {
		Vec2::new(self.x - o.x, self.y - o.y)
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

impl Mul<f32> for Vec2 {
	type Output = Vec2;
	fn mul(self, o: f32) -> Vec2 {
		Vec2::new(self.x * o, self.y * o)
	}
}

impl Div<Vec2> for Vec2 {
	type Output = Vec2;
	fn div(self, o: Vec2) -> Vec2 {
		Vec2::new(self.x / o.x, self.y / o.y)
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

impl Sub for Vec3 {
	type Output = Vec3;
	fn sub(self, o: Vec3) -> Vec3 {
		Vec3::new(self.x - o.x, self.y - o.y, self.z - o.z)
	}
}

impl Neg for Vec3 {
	type Output = Vec3;
	fn neg(self) -> Vec3 {
		Vec3::new(-self.x, -self.y, -self.z)
	}
}

impl Mul<f32> for Vec3 {
	type Output = Vec3;
	fn mul(self, o: f32) -> Vec3 {
		Vec3::new(self.x * o, self.y * o, self.z * o)
	}
}

impl Mul<Vec3> for Vec3 {
	type Output = Vec3;
	fn mul(self, o: Vec3) -> Vec3 {
		Vec3::new(self.x * o.x, self.y * o.y, self.z * o.z)
	}
}

impl Div<f32> for Vec3 {
	type Output = Vec3;
	fn div(self, o: f32) -> Vec3 {
		Vec3::new(self.x / o, self.y / o, self.z / o)
	}
}

impl Div<Vec3> for Vec3 {
	type Output = Vec3;
	fn div(self, o: Vec3) -> Vec3 {
		Vec3::new(self.x / o.x, self.y / o.y, self.z / o.z)
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

impl Mul<Mat4> for Mat4 {
	type Output = Mat4;
	fn mul(self, o: Mat4) -> Mat4 {
		let mut d = [0.0f32; 16];
		let ot = o.transpose();

		for j in 0..4 {
			for i in 0..4 {
				d[j*4 + i] = self.rows[j].dot(ot.rows[i]);
			}
		}

		Mat4::new(&d)
	}
}

impl Mul<Vec4> for Mat4 {
	type Output = Vec4;
	fn mul(self, o: Vec4) -> Vec4 {
		Vec4::new(
			self.rows[0].dot(o),
			self.rows[1].dot(o),
			self.rows[2].dot(o),
			self.rows[3].dot(o),
		)
	}
}
impl Mul<Vec3> for Mat4 {
	type Output = Vec3;
	fn mul(self, o: Vec3) -> Vec3 {
		let o4 = o.extend(1.0);

		Vec3::new(
			self.rows[0].dot(o4),
			self.rows[1].dot(o4),
			self.rows[2].dot(o4),
		)
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
