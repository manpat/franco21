use std::ops::Mul;
use crate::matrix::Mat4;
use crate::vector::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Mat3x4 {
	pub rows: [Vec4; 3]
}

impl Mat3x4 {
	pub fn new(d: &[f32; 12]) -> Mat3x4 {
		Mat3x4 {
			rows: [
				Vec4::from_slice(&d[0..4]),
				Vec4::from_slice(&d[4..8]),
				Vec4::from_slice(&d[8..12]),
			]
		}
	}

	pub fn from_rows(rows: [Vec4; 3]) -> Mat3x4 { Mat3x4 { rows } }
	pub fn from_columns(columns: [Vec3; 4]) -> Mat3x4 {
		let [a, b, c, d] = columns;

		Mat3x4::new(&[
			a.x, b.x, c.x, d.x,
			a.y, b.y, c.y, d.y,
			a.z, b.z, c.z, d.z,
		])
	}

	pub fn identity() -> Mat3x4 { Mat3x4::uniform_scale(1.0) }
	pub fn uniform_scale(s: f32) -> Mat3x4 { Mat3x4::scale(Vec3::new(s,s,s)) }

	pub fn translate(t: Vec3) -> Mat3x4 {
		Mat3x4::new(&[
			1.0, 0.0, 0.0, t.x,
			0.0, 1.0, 0.0, t.y, 
			0.0, 0.0, 1.0, t.z,
		])
	}

	pub fn scale_translate(s: Vec3, t: Vec3) -> Mat3x4 {
		Mat3x4::new(&[
			s.x, 0.0, 0.0, t.x,
			0.0, s.y, 0.0, t.y, 
			0.0, 0.0, s.z, t.z,
		])
	}

	pub fn rotate_x_translate(ph: f32, t: Vec3) -> Mat3x4 {
		let (rx, ry) = (ph.cos(), ph.sin());

		Mat3x4::new(&[
			1.0, 0.0, 0.0, t.x,
			0.0,  rx, -ry, t.y,
			0.0,  ry,  rx, t.z,
		])
	}
	pub fn rotate_y_translate(ph: f32, t: Vec3) -> Mat3x4 {
		let (rx, ry) = (ph.cos(), ph.sin());

		Mat3x4::new(&[
			 rx, 0.0,  ry, t.x,
			0.0, 1.0, 0.0, t.y, 
			-ry, 0.0,  rx, t.z,
		])
	}
	pub fn rotate_z_translate(ph: f32, t: Vec3) -> Mat3x4 {
		let (rx, ry) = (ph.cos(), ph.sin());

		Mat3x4::new(&[
			 rx, -ry, 0.0, t.x,
			 ry,  rx, 0.0, t.y,
			0.0, 0.0, 1.0, t.z,
		])
	}

	pub fn scale(s: Vec3) -> Mat3x4 {
		Mat3x4::scale_translate(s, Vec3::zero())
	}

	pub fn rotate_x(ph: f32) -> Mat3x4 {
		Mat3x4::rotate_x_translate(ph, Vec3::zero())
	}

	pub fn rotate_y(ph: f32) -> Mat3x4 {
		Mat3x4::rotate_y_translate(ph, Vec3::zero())
	}

	pub fn rotate_z(ph: f32) -> Mat3x4 {
		Mat3x4::rotate_z_translate(ph, Vec3::zero())
	}

	pub fn to_mat4(&self) -> Mat4 {
		let [a,b,c] = self.rows;
		Mat4::from_rows([a, b, c, Vec4::from_w(1.0)])
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

	pub fn column_w(&self) -> Vec3 {
		let [a,b,c] = &self.rows;
		Vec3::new(a.w, b.w, c.w)
	}

	pub fn columns(&self) -> [Vec3; 4] {
		[self.column_x(), self.column_y(), self.column_z(), self.column_w()]
	}

	pub fn determinant(&self) -> f32 {
		let [a,b,c] = self.rows;

		  a.x * b.y * c.z
		+ a.y * b.z * c.x
		+ a.z * b.x * c.y

		- a.x * b.z * c.y
		- a.y * b.x * c.z
		- a.z * b.y * c.x
	}

	pub fn inverse(&self) -> Mat3x4 {
		let [a,b,c] = self.rows;
		let inv_det = 1.0 / self.determinant();

		Mat3x4::from_rows([
			Vec4::new(
				b.y * c.z - b.z * c.y,
				a.z * c.y - a.y * c.z,
				a.y * b.z - a.z * b.y,

				a.y * b.w * c.z
				+ a.z * b.y * c.w
				+ a.w * b.z * c.y
				- a.y * b.z * c.w
				- a.z * b.w * c.y
				- a.w * b.y * c.z
			) * inv_det,

			Vec4::new(
				b.z * c.x - b.x * c.z,
				a.x * c.z - a.z * c.x,
				a.z * b.x - a.x * b.z,

				a.x * b.z * c.w
				+ a.z * b.w * c.x
				+ a.w * b.x * c.z
				- a.x * b.w * c.z
				- a.z * b.x * c.w
				- a.w * b.z * c.x
			) * inv_det,

			Vec4::new(
				b.x * c.y - b.y * c.x,
				a.y * c.x - a.x * c.y,
				a.x * b.y - a.y * b.x,

				a.x * b.w * c.y
				+ a.y * b.x * c.w
				+ a.w * b.y * c.x
				- a.x * b.y * c.w
				- a.y * b.w * c.x
				- a.w * b.x * c.y
			) * inv_det
		])
	}
}


impl Mul<Mat3x4> for Mat3x4 {
	type Output = Mat3x4;
	fn mul(self, o: Mat3x4) -> Mat3x4 {
		let mut d = [0.0f32; 12];
		let ot = [
			o.column_x().extend(0.0),
			o.column_y().extend(0.0),
			o.column_z().extend(0.0),
			o.column_w().extend(1.0),
		];

		for j in 0..3 {
			for i in 0..4 {
				d[j*4 + i] = self.rows[j].dot(ot[i]);
			}
		}

		Mat3x4::new(&d)
	}
}

impl Mul<Vec4> for Mat3x4 {
	type Output = Vec4;
	fn mul(self, o: Vec4) -> Vec4 {
		Vec4::new(
			self.rows[0].dot(o),
			self.rows[1].dot(o),
			self.rows[2].dot(o),
			o.w,
		)
	}
}

impl Mul<Vec3> for Mat3x4 {
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


#[cfg(test)]
mod tests {
	use crate::*;

	#[test]
	fn test_rotate_x() {
		let ident = Mat3x4::rotate_x(0.0);
		assert_vec_eq!(ident.column_x(), Vec3::from_x(1.0));
		assert_vec_eq!(ident.column_y(), Vec3::from_y(1.0));
		assert_vec_eq!(ident.column_z(), Vec3::from_z(1.0));

		let r90 = Mat3x4::rotate_x(PI/2.0);
		assert_vec_eq!(r90.column_x(), Vec3::from_x(1.0));
		assert_vec_eq!(r90.column_y(), Vec3::from_z(1.0));
		assert_vec_eq!(r90.column_z(), Vec3::from_y(-1.0));
		assert_vec_eq!(r90 * Vec3::from_y(3.0), Vec3::from_z(3.0));
		assert_vec_eq!(r90 * Vec3::from_x(3.0), Vec3::from_x(3.0));

		let r180 = Mat3x4::rotate_x(PI);
		assert_vec_eq!(r180.column_x(), Vec3::from_x(1.0));
		assert_vec_eq!(r180.column_y(), Vec3::from_y(-1.0));
		assert_vec_eq!(r180.column_z(), Vec3::from_z(-1.0));
	}

	#[test]
	fn test_rotate_y() {
		let ident = Mat3x4::rotate_y(0.0);
		assert_vec_eq!(ident.column_x(), Vec3::from_x(1.0));
		assert_vec_eq!(ident.column_y(), Vec3::from_y(1.0));
		assert_vec_eq!(ident.column_z(), Vec3::from_z(1.0));

		let r45 = Mat3x4::rotate_y(PI/4.0);
		assert_vec_eq!(r45.column_x(), Vec3::new(INV_SQRT_2, 0.0, -INV_SQRT_2));
		assert_vec_eq!(r45.column_y(), Vec3::from_y(1.0));
		assert_vec_eq!(r45.column_z(), Vec3::new(INV_SQRT_2, 0.0, INV_SQRT_2));

		let r90 = Mat3x4::rotate_y(PI/2.0);
		assert_vec_eq!(r90.column_x(), Vec3::from_z(-1.0));
		assert_vec_eq!(r90.column_y(), Vec3::from_y(1.0));
		assert_vec_eq!(r90.column_z(), Vec3::from_x(1.0));
		assert_vec_eq!(r90 * Vec3::from_y(3.0), Vec3::from_y(3.0));
		assert_vec_eq!(r90 * Vec3::from_x(3.0), Vec3::from_z(-3.0));

		let r180 = Mat3x4::rotate_y(PI);
		assert_vec_eq!(r180.column_x(), Vec3::from_x(-1.0));
		assert_vec_eq!(r180.column_y(), Vec3::from_y(1.0));
		assert_vec_eq!(r180.column_z(), Vec3::from_z(-1.0));
	}

	#[test]
	fn test_rotate_z() {
		let ident = Mat3x4::rotate_z(0.0);
		assert_vec_eq!(ident.column_x(), Vec3::from_x(1.0));
		assert_vec_eq!(ident.column_y(), Vec3::from_y(1.0));
		assert_vec_eq!(ident.column_z(), Vec3::from_z(1.0));

		let r90 = Mat3x4::rotate_z(PI/2.0);
		assert_vec_eq!(r90.column_x(), Vec3::from_y(1.0));
		assert_vec_eq!(r90.column_y(), Vec3::from_x(-1.0));
		assert_vec_eq!(r90.column_z(), Vec3::from_z(1.0));
		assert_vec_eq!(r90 * Vec3::from_z(3.0), Vec3::from_z(3.0));
		assert_vec_eq!(r90 * Vec3::from_x(3.0), Vec3::from_y(3.0));

		let r180 = Mat3x4::rotate_z(PI);
		assert_vec_eq!(r180.column_x(), Vec3::from_x(-1.0));
		assert_vec_eq!(r180.column_y(), Vec3::from_y(-1.0));
		assert_vec_eq!(r180.column_z(), Vec3::from_z(1.0));
	}
}
