use common::*;
use crate::gfx::mesh::{PolyBuilder3D, traits::BuildableGeometry3D};

pub struct Tetrahedron {
	basis: Mat3x4,
}

impl Tetrahedron {
	pub fn from_matrix(basis: Mat3x4) -> Tetrahedron {
		Tetrahedron {basis}
	}

	pub fn unit() -> Tetrahedron {
		Tetrahedron::from_matrix(Mat3x4::identity())
	}
}

impl BuildableGeometry3D for Tetrahedron {
	fn build<MB: PolyBuilder3D>(&self, mb: &mut MB) {
		let [ux, uy, uz, translation] = self.basis.columns();

		let verts = [
			translation + ux,
			translation + ux*(TAU/3.0).cos() - uz*(TAU/3.0).sin(),
			translation + ux*(TAU/3.0).cos() + uz*(TAU/3.0).sin(),
			translation + uy,
		];

		let indices = [
			0, 2, 1,

			3, 0, 1,
			3, 1, 2,
			3, 2, 0,
		];

		mb.extend_3d(verts, indices);
	}
}


pub struct Cuboid {
	basis: Mat3x4,
}

impl Cuboid {
	pub fn from_matrix(basis: Mat3x4) -> Cuboid {
		Cuboid {basis}
	}

	pub fn unit() -> Cuboid {
		Cuboid::from_matrix(Mat3x4::identity())
	}
}

impl BuildableGeometry3D for Cuboid {
	fn build<MB: PolyBuilder3D>(&self, mb: &mut MB) {
		let [ux, uy, uz, translation] = self.basis.columns();
		let (hx, hy, hz) = (ux/2.0, uy/2.0, uz/2.0);

		let verts = [
			translation - hx - hy - hz,
			translation - hx + hy - hz,
			translation + hx + hy - hz,
			translation + hx - hy - hz,

			translation - hx - hy + hz,
			translation - hx + hy + hz,
			translation + hx + hy + hz,
			translation + hx - hy + hz,
		];

		let indices = [
			// -Z, +Z
			3, 0, 1,  3, 1, 2,
			4, 7, 6,  4, 6, 5,

			// -X, +X
			0, 4, 5,  0, 5, 1,
			7, 3, 2,  7, 2, 6,

			// -Y, +Y
			0, 3, 7,  0, 7, 4,
			5, 6, 2,  5, 2, 1,
		];

		mb.extend_3d(verts, indices);
	}
}