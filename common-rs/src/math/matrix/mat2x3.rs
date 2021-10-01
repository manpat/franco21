use std::ops::Mul;
use crate::vector::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Mat2x3{pub rows: [Vec3; 2]}

impl Mat2x3 {
	pub fn new(d: [f32; 6]) -> Mat2x3 {
		Mat2x3 {
			rows: [
				Vec3::new(d[0], d[1], d[2]),
				Vec3::new(d[3], d[4], d[5]),
			]
		}
	}

	pub fn from_rows(rows: [Vec3; 2]) -> Mat2x3 { Mat2x3 { rows } }
	pub fn from_columns(columns: [Vec2; 3]) -> Mat2x3 {
		let [a, b, c] = columns;
		Mat2x3::new([
			a.x, b.x, c.x,
			a.y, b.y, c.y,
		])
	}

	pub fn identity() -> Mat2x3 { Mat2x3::uniform_scale(1.0) }
	pub fn uniform_scale(s: f32) -> Mat2x3 { Mat2x3::scale(Vec2::splat(s)) }

	pub fn translate(t: Vec2) -> Mat2x3 {
		Mat2x3::new([
			1.0, 0.0, t.x,
			0.0, 1.0, t.y,
		])
	}

	pub fn scale(s: Vec2) -> Mat2x3 { Mat2x3::scale_translate(s, Vec2::zero()) }
	pub fn rotate(ph: f32) -> Mat2x3 { Mat2x3::rotate_translate(ph, Vec2::zero()) }

	pub fn scale_translate(s: Vec2, t: Vec2) -> Mat2x3 {
		Mat2x3::new([
			s.x, 0.0, t.x,
			0.0, s.y, t.y,
		])
	}

	pub fn rotate_translate(ph: f32, t: Vec2) -> Mat2x3 {
		let (rx, ry) = (ph.cos(), ph.sin());
		Mat2x3::new([
			rx, -ry, t.x,
			ry,  rx, t.y,
		])
	}

	pub fn column_x(&self) -> Vec2 {
		let [a,b] = &self.rows;
		Vec2::new(a.x, b.x)
	}

	pub fn column_y(&self) -> Vec2 {
		let [a,b] = &self.rows;
		Vec2::new(a.y, b.y)
	}

	pub fn column_z(&self) -> Vec2 {
		let [a,b] = &self.rows;
		Vec2::new(a.z, b.z)
	}

	pub fn columns(&self) -> [Vec2; 3] {
		[self.column_x(), self.column_y(), self.column_z()]
	}

	pub fn inverse(&self) -> Mat2x3 {
		// [a  b  c]
		// [d  e  f]
		// [0  0  1]

		let [Vec3{x: a, y: b, z: c}, Vec3{x: d, y: e, z: f}] = self.rows;

		let cofactor_20 = b*f - c*e;
		let cofactor_21 = a*f - c*d;

		let inv_determinant = 1.0 / (a*e - b*d);
		let adjugate = Mat2x3::new([
			 e,-b, cofactor_20,
			-d, a,-cofactor_21,
		]);

		adjugate * inv_determinant
	}
}


impl Mul<Mat2x3> for Mat2x3 {
	type Output = Mat2x3;
	fn mul(self, o: Mat2x3) -> Mat2x3 {
		let cx = o.column_x().extend(0.0);
		let cy = o.column_y().extend(0.0);
		let cz = o.column_z().extend(1.0);

		Mat2x3::new([
			self.rows[0].dot(cx),
			self.rows[0].dot(cy),
			self.rows[0].dot(cz),

			self.rows[1].dot(cx),
			self.rows[1].dot(cy),
			self.rows[1].dot(cz),
		])
	}
}

impl Mul<Vec2> for Mat2x3 {
	type Output = Vec2;
	fn mul(self, o: Vec2) -> Vec2 {
		let o = o.extend(1.0);
		Vec2::new(
			self.rows[0].dot(o),
			self.rows[1].dot(o),
		)
	}
}

impl Mul<f32> for Mat2x3 {
	type Output = Mat2x3;
	fn mul(self, o: f32) -> Mat2x3 {
		Mat2x3::from_rows([
			self.rows[0] * o,
			self.rows[1] * o
		])
	}
}
