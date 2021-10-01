use std::ops::Mul;
use crate::vector::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Mat2{pub rows: [Vec2; 2]}

impl Mat2 {
	pub fn new(d: [f32; 4]) -> Mat2 {
		Mat2 {
			rows: [
				Vec2::new(d[0], d[1]),
				Vec2::new(d[2], d[3]),
			]
		}
	}

	pub fn from_rows(rows: [Vec2; 2]) -> Mat2 { Mat2 { rows } }
	pub fn from_columns(columns: [Vec2; 2]) -> Mat2 {
		Mat2::from_rows(columns).transpose()
	}

	pub fn identity() -> Mat2 { Mat2::uniform_scale(1.0) }
	pub fn uniform_scale(s: f32) -> Mat2 { Mat2::scale(Vec2::splat(s)) }

	pub fn scale(s: Vec2) -> Mat2 {
		Mat2::new([
			s.x, 0.0,
			0.0, s.y,
		])
	}
	
	pub fn rotate(ph: f32) -> Mat2 {
		let (rx, ry) = (ph.cos(), ph.sin());
		Mat2::new([
			rx, -ry,
			ry,  rx,
		])
	}

	#[deprecated]
	pub fn rot(ph: f32) -> Mat2 { Mat2::rotate(ph) }

	pub fn transpose(&self) -> Mat2 {
		let [Vec2{x: a, y: b}, Vec2{x: c, y: d}] = self.rows;
		Mat2::new([ a, c, b, d ])
	}

	pub fn adjugate(&self) -> Mat2 {
		let [Vec2{x: a, y: b}, Vec2{x: c, y: d}] = self.rows;
		Mat2::new([ d, -b, -c,  a ])
	}

	pub fn column_x(&self) -> Vec2 {
		let [a, b] = self.rows;
		Vec2::new(a.x, b.x)
	}

	pub fn column_y(&self) -> Vec2 {
		let [a, b] = self.rows;
		Vec2::new(a.y, b.y)
	}

	pub fn columns(&self) -> [Vec2; 2] {
		self.transpose().rows
	}

	pub fn determinant(&self) -> f32 {
		let [a, b] = self.columns();
		a.wedge(b)
	}

	pub fn inverse(&self) -> Mat2 {
		let inv_det = 1.0 / self.determinant();
		self.adjugate() * inv_det
	}
}


impl Mul<Mat2> for Mat2 {
	type Output = Mat2;
	fn mul(self, o: Mat2) -> Mat2 {
		let ot = o.transpose();
		Mat2::new([
			self.rows[0].dot(ot.rows[0]),
			self.rows[0].dot(ot.rows[1]),
			self.rows[1].dot(ot.rows[0]),
			self.rows[1].dot(ot.rows[1]),
		])
	}
}

impl Mul<Vec2> for Mat2 {
	type Output = Vec2;
	fn mul(self, o: Vec2) -> Vec2 {
		Vec2::new(
			self.rows[0].dot(o),
			self.rows[1].dot(o),
		)
	}
}

impl Mul<f32> for Mat2 {
	type Output = Mat2;
	fn mul(self, o: f32) -> Mat2 {
		Mat2::from_rows([
			self.rows[0] * o,
			self.rows[1] * o
		])
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	fn assert_almost_eq(a: f32, b: f32) {
		assert!((a - b).abs() < 0.0001);
	}

	fn assert_almost_eq_vec(a: Vec2, b: Vec2) {
		assert_almost_eq(a.x, b.x);
		assert_almost_eq(a.y, b.y);
	}

	fn assert_almost_eq_mat(a: Mat2, b: Mat2) {
		assert_almost_eq_vec(a.rows[0], b.rows[0]);
		assert_almost_eq_vec(a.rows[1], b.rows[1]);		
	}

	#[test]
	fn test_inverse() {
		let a = Mat2::new([1.0, 1.0, 0.0, 2.0]);
		let i = Mat2::ident();

		println!("A = {:?}", a);

		let det = a.determinant();
		println!("det A = {:?}", det);
		assert_almost_eq(det, 2.0);

		println!("AI");
		assert_almost_eq_mat(a, a * i);
		println!("IA");
		assert_almost_eq_mat(a, i * a);

		let a_adj_a = a * a.adjugate();
		println!("A adj(A) = adj(A) A");
		assert_almost_eq_mat(a_adj_a, a.adjugate() * a);

		let det_a_i = Mat2::uniform_scale(det);
		println!("det(A) I = {:?}", det_a_i);

		println!("A adj(A) = det(A) I");
		assert_almost_eq_mat(a_adj_a, det_a_i);

		println!("adj a {:?}", a.adjugate());
		println!("inv a {:?}", a.inverse());
		println!("a a-1 {:?}", a * a.inverse());
		println!("a-1 a {:?}", a.inverse() * a);

		assert_almost_eq_mat(i, a * a.inverse());
		assert_almost_eq_mat(i, a.inverse() * a);
	}

}