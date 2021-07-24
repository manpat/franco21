use std::ops::Mul;
use crate::vector::*;
use crate::matrix::Mat3x4;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Mat4 {
	pub rows: [Vec4; 4]
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
	pub fn from_columns(columns: [Vec4; 4]) -> Mat4 {
		Mat4::from_rows(columns).transpose()
	}

	#[deprecated]
	pub fn ident() -> Mat4 { Mat4::identity() }
	pub fn identity() -> Mat4 { Mat4::uniform_scale(1.0) }
	pub fn uniform_scale(s: f32) -> Mat4 { Mat4::scale(Vec3::new(s,s,s)) }

	pub fn translate(t: Vec3) -> Mat4 { Mat3x4::translate(t).to_mat4() }
	pub fn scale(s: Vec3) -> Mat4 { Mat3x4::scale(s).to_mat4() }

	pub fn rotate_x(ph: f32) -> Mat4 { Mat3x4::rotate_x(ph).to_mat4() }
	pub fn rotate_y(ph: f32) -> Mat4 { Mat3x4::rotate_y(ph).to_mat4() }
	pub fn rotate_z(ph: f32) -> Mat4 { Mat3x4::rotate_z(ph).to_mat4() }

	pub fn scale_translate(s: Vec3, t: Vec3) -> Mat4 { Mat3x4::scale_translate(s, t).to_mat4() }
	pub fn rotate_x_translate(ph: f32, t: Vec3) -> Mat4 { Mat3x4::rotate_x_translate(ph, t).to_mat4() }
	pub fn rotate_y_translate(ph: f32, t: Vec3) -> Mat4 { Mat3x4::rotate_y_translate(ph, t).to_mat4() }
	pub fn rotate_z_translate(ph: f32, t: Vec3) -> Mat4 { Mat3x4::rotate_z_translate(ph, t).to_mat4() }

	#[deprecated]
	pub fn xrot(ph: f32) -> Mat4 { Mat3x4::rotate_x(ph).to_mat4() }

	#[deprecated]
	pub fn yrot(ph: f32) -> Mat4 { Mat3x4::rotate_y(ph).to_mat4() }

	#[deprecated]
	pub fn zrot(ph: f32) -> Mat4 { Mat3x4::rotate_z(ph).to_mat4() }

	pub fn to_mat3x4(&self) -> Mat3x4 {
		let [a,b,c,_] = self.rows;
		Mat3x4::from_rows([a, b, c])
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

	pub fn column_x(&self) -> Vec4 {
		let [a,b,c,d] = self.rows;
		Vec4::new(a.x, b.x, c.x, d.x)
	}

	pub fn column_y(&self) -> Vec4 {
		let [a,b,c,d] = self.rows;
		Vec4::new(a.y, b.y, c.y, d.y)
	}

	pub fn column_z(&self) -> Vec4 {
		let [a,b,c,d] = self.rows;
		Vec4::new(a.z, b.z, c.z, d.z)
	}

	pub fn column_w(&self) -> Vec4 {
		let [a,b,c,d] = self.rows;
		Vec4::new(a.w, b.w, c.w, d.w)
	}

	pub fn columns(&self) -> [Vec4; 4] {
		self.transpose().rows
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
			0.0, 0.0,-1.0, 0.0,
		])
	}

	pub fn ortho(l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) -> Mat4 {
		let xsc = 2.0 / (r - l);
		let ysc = 2.0 / (t - b);
		let zsc =-2.0 / (f - n);

		let xtr = -(r + l) / (r - l);
		let ytr = -(t + b) / (t - b);
		let ztr = -(f + n) / (f - n);

		Mat4::new(&[
			xsc, 0.0, 0.0, xtr,
			0.0, ysc, 0.0, ytr,
			0.0, 0.0, zsc, ztr,
			0.0, 0.0, 0.0, 1.0,
		])
	}

	pub fn perspective(fov: f32, aspect: f32, n: f32, f: f32) -> Mat4 {
		let scale = (fov / 2.0).tan() * n;

		// maintain at least 1x1 safe region in portrait
		let (r, t) = if aspect > 1.0 {
			(scale * aspect, scale)
		} else {
			(scale, scale / aspect)
		};

		Mat4::frustum(-r, r,-t, t, n, f)
	}

	pub fn ortho_aspect(scale: f32, aspect: f32, n: f32, f: f32) -> Mat4 {
		// maintain at least 1x1 safe region in portrait
		let (r, t) = if aspect > 1.0 {
			(scale * aspect, scale)
		} else {
			(scale, scale / aspect)
		};

		Mat4::ortho(-r, r,-t, t, n, f)
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