use common::*;
use crate::gfx::mesh::{PolyBuilder2D, PolyBuilder3D, ColoredPolyBuilder};


pub struct PlaneMeshBuilderAdaptor<'mb, MB: PolyBuilder3D> {
	builder_3d: &'mb mut MB,
	uvw: Mat3,
}


impl<'mb, MB: PolyBuilder3D> PlaneMeshBuilderAdaptor<'mb, MB> {
	pub fn new(builder_3d: &'mb mut MB, uvw: Mat3) -> Self {
		PlaneMeshBuilderAdaptor {
			builder_3d,
			uvw,
		}
	}
}

impl<'mb, MB> PlaneMeshBuilderAdaptor<'mb, MB>
	where MB: PolyBuilder3D + ColoredPolyBuilder
{
	pub fn set_color(&mut self, color: impl Into<Color>) {
		self.builder_3d.set_color(color);
	}
}

impl<MB: PolyBuilder3D> PolyBuilder2D for PlaneMeshBuilderAdaptor<'_, MB> {
	fn extend_2d(&mut self, vs: impl IntoIterator<Item=Vec2>, is: impl IntoIterator<Item=u16>) {
		let uvw = self.uvw;
		
		let vertices_3d = vs.into_iter().map(move |v2| {
			uvw * v2.extend(1.0)
		});

		self.builder_3d.extend_3d(vertices_3d, is);
	}
}

impl<MB> ColoredPolyBuilder for PlaneMeshBuilderAdaptor<'_, MB>
	where MB: PolyBuilder3D + ColoredPolyBuilder
{
	fn set_color(&mut self, color: impl Into<Color>) {
		self.builder_3d.set_color(color);
	}
}