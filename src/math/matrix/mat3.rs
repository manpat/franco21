use std::ops::Mul;
use crate::vector::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Mat3{pub rows: [Vec3; 3]}

impl Mat3 {
	pub fn new(d: [f32; 9]) -> Mat3 {
		Mat3 {
			rows: [
				Vec3::new(d[0], d[1], d[2]),
				Vec3::new(d[3], d[4], d[5]),
				Vec3::new(d[6], d[7], d[8]),
			]
		}
	}

	pub fn from_rows(rows: [Vec3; 3]) -> Mat3 { Mat3 { rows } }
	pub fn from_columns(columns: [Vec3; 3]) -> Mat3 {
		let [a, b, c] = columns;
		Mat3::new([
			a.x, b.x, c.x,
			a.y, b.y, c.y,
			a.z, b.z, c.z,
		])
	}

	pub fn identity() -> Mat3 {
		Mat3::new([
			1.0, 0.0, 0.0,
			0.0, 1.0, 0.0,
			0.0, 0.0, 1.0,
		])
	}

	pub fn column_x(&self) -> Vec3 {
		let [a,b,c] = &self.rows;
		Vec3::new(a.x, b.x, c.x)
	}

	pub fn column_y(&self) -> Vec3 {
		let [a,b,c] = &self.rows;
		Vec3::new(a.y, b.y, c.y)
	}

	pub fn column_z(&self) -> Vec3 {
		let [a,b,c] = &self.rows;
		Vec3::new(a.z, b.z, c.z)
	}

	pub fn columns(&self) -> [Vec3; 3] {
		[self.column_x(), self.column_y(), self.column_z()]
	}

	pub fn inverse(&self) -> Mat3 {
		unimplemented!()
	}
}


impl Mul<Mat3> for Mat3 {
	type Output = Mat3;
	fn mul(self, o: Mat3) -> Mat3 {
		let [cx, cy, cz] = o.columns();

		Mat3::new([
			self.rows[0].dot(cx),
			self.rows[0].dot(cy),
			self.rows[0].dot(cz),

			self.rows[1].dot(cx),
			self.rows[1].dot(cy),
			self.rows[1].dot(cz),

			self.rows[2].dot(cx),
			self.rows[2].dot(cy),
			self.rows[2].dot(cz),
		])
	}
}

impl Mul<Vec2> for Mat3 {
	type Output = Vec3;
	fn mul(self, o: Vec2) -> Vec3 {
		let o = o.extend(1.0);
		Vec3::new(
			self.rows[0].dot(o),
			self.rows[1].dot(o),
			self.rows[2].dot(o),
		)
	}
}

impl Mul<Vec3> for Mat3 {
	type Output = Vec3;
	fn mul(self, o: Vec3) -> Vec3 {
		Vec3::new(
			self.rows[0].dot(o),
			self.rows[1].dot(o),
			self.rows[2].dot(o),
		)
	}
}
