use common::*;
use crate::gfx::mesh::{PolyBuilder2D, traits::BuildableGeometry2D};


pub struct Quad {
	basis: Mat2x3,
}

impl Quad {
	pub fn from_matrix(basis: Mat2x3) -> Quad {
		Quad {basis}
	}

	pub fn unit() -> Quad {
		Quad::from_matrix(Mat2x3::identity())
	}
}

impl BuildableGeometry2D for Quad {
	fn build<MB: PolyBuilder2D>(&self, mb: &mut MB) {
		let [ux, uy, translation] = self.basis.columns();
		let (hx, hy) = (ux/2.0, uy/2.0);

		mb.extend_2d_fan(4, [
			translation - hx - hy,
			translation + hx - hy,
			translation + hx + hy,
			translation - hx + hy,
		]);
	}
}


pub struct Polygon {
	basis: Mat2x3,
	num_faces: u32,
}

impl Polygon {
	pub fn from_matrix(num_faces: u32, basis: Mat2x3) -> Polygon {
		Polygon {basis, num_faces}
	}

	pub fn unit(num_faces: u32) -> Polygon {
		Polygon::from_matrix(num_faces, Mat2x3::identity())
	}

	pub fn from_pos_scale(num_faces: u32, pos: Vec2, scale: Vec2) -> Polygon {
		Polygon::from_matrix(num_faces, Mat2x3::scale_translate(scale, pos))
	}
}

impl BuildableGeometry2D for Polygon {
	fn build<MB: PolyBuilder2D>(&self, mb: &mut MB) {
		if self.num_faces < 3 {
			return
		}

		let [ux, uy, translation] = self.basis.columns();
		let uxy = Mat2::from_columns([ux/2.0, uy/2.0]);

		let angle_increment = TAU / (self.num_faces as f32);
		let vertices = (0..self.num_faces)
			.map(|i| {
				let angle = angle_increment * i as f32;
				translation + uxy * Vec2::from_angle(angle)
			});

		mb.extend_2d_fan(self.num_faces, vertices);
	}
}


