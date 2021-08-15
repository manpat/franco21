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
			translation + ux*(TAU/3.0).cos() + uz*(TAU/3.0).sin(),
			translation + ux*(TAU/3.0).cos() - uz*(TAU/3.0).sin(),
			translation + uy,
		];

		let indices = [
			0, 1, 2,

			3, 0, 1,
			3, 1, 2,
			3, 2, 0,
		];

		mb.extend_3d(verts, indices);
	}
}


// pub struct Cuboid {
// 	basis: Mat3x4,
// }

// impl Cuboid {
// 	pub fn unit() -> Cuboid {
// 		Cuboid {
// 			basis: Mat3x4::identity(),
// 		}
// 	}


// 	pub fn build<MB: PolyBuilder3D>(&self, mb: &mut MB) {
// 		let verts = [

// 		];

// 		let indices = [
		
// 		];

// 		mb.extend_3d(verts, indices);
// 	}
// }