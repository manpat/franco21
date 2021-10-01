use common::*;
use crate::gfx::mesh::PlaneMeshBuilderAdaptor;

pub trait PolyBuilder2D {
	fn extend_2d(&mut self, vs: impl IntoIterator<Item=Vec2>, is: impl IntoIterator<Item=u16>);
	fn build(&mut self, geom: impl BuildableGeometry2D) where Self: Sized { geom.build(self) }

	fn extend_2d_fan(&mut self, num_vertices: u32, vs: impl IntoIterator<Item=Vec2>) {
		if num_vertices < 3 {
			return
		}

		let indices = (0..num_vertices as u16-2)
			.flat_map(|i| [0, i+1, i+2]);

		self.extend_2d(vs, indices);
	}

	fn extend_2d_fan_closed(&mut self, num_vertices: u32, vs: impl IntoIterator<Item=Vec2>) {
		if num_vertices < 3 {
			return
		}

		let indices = (0..num_vertices as u16-2)
			.flat_map(|i| [0, i+1, i+2])
			.chain([0, num_vertices as u16-1, 1]);

		self.extend_2d(vs, indices);
	}
}

impl<PB: PolyBuilder2D> PolyBuilder2D for &mut PB {
	fn extend_2d(&mut self, vs: impl IntoIterator<Item=Vec2>, is: impl IntoIterator<Item=u16>) {
		(*self).extend_2d(vs, is);
	}
}


pub trait PolyBuilder3D {
	fn extend_3d(&mut self, vs: impl IntoIterator<Item=Vec3>, is: impl IntoIterator<Item=u16>);

	fn build(&mut self, geom: impl BuildableGeometry3D) where Self: Sized {
		geom.build(self)
	}

	fn on_plane(self, uvw: Mat3) -> PlaneMeshBuilderAdaptor<Self> where Self: Sized {
		PlaneMeshBuilderAdaptor::new(self, uvw)
	}
	
	fn on_plane_ref(&mut self, uvw: Mat3) -> PlaneMeshBuilderAdaptor<&'_ mut Self> where Self: Sized {
		PlaneMeshBuilderAdaptor::new(self, uvw)
	}

	fn extend_3d_fan(&mut self, num_vertices: u32, vs: impl IntoIterator<Item=Vec3>) {
		if num_vertices < 3 {
			return
		}

		let indices = (0..num_vertices as u16-2)
			.flat_map(|i| [0, i+1, i+2]);

		self.extend_3d(vs, indices);
	}

	fn extend_3d_fan_closed(&mut self, num_vertices: u32, vs: impl IntoIterator<Item=Vec3>) {
		if num_vertices < 3 {
			return
		}

		let indices = (0..num_vertices as u16-2)
			.flat_map(|i| [0, i+1, i+2])
			.chain([0, num_vertices as u16-1, 1]);

		self.extend_3d(vs, indices);
	}
}

impl<PB: PolyBuilder3D> PolyBuilder3D for &mut PB {
	fn extend_3d(&mut self, vs: impl IntoIterator<Item=Vec3>, is: impl IntoIterator<Item=u16>) {
		(*self).extend_3d(vs, is);
	}
}


pub trait ColoredPolyBuilder {
	fn set_color(&mut self, color: impl Into<Color>);
}

impl<PB: ColoredPolyBuilder> ColoredPolyBuilder for &mut PB {
	fn set_color(&mut self, color: impl Into<Color>) {
		(*self).set_color(color);
	}
}



pub trait BuildableGeometry3D {
	fn build<MB: PolyBuilder3D>(&self, mb: &mut MB);
}

pub trait BuildableGeometry2D {
	fn build<MB: PolyBuilder2D>(&self, mb: &mut MB);
}


impl<G: BuildableGeometry2D> BuildableGeometry2D for &G {
	fn build<MB: PolyBuilder2D>(&self, mb: &mut MB) {
		(*self).build(mb);
	}
}

impl<G: BuildableGeometry3D> BuildableGeometry3D for &G {
	fn build<MB: PolyBuilder3D>(&self, mb: &mut MB) {
		(*self).build(mb);
	}
}

