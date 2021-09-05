use common::*;
use crate::gfx::vertex::ColorVertex;
use crate::gfx::mesh::{MeshData, PolyBuilder3D, ColoredPolyBuilder, PlaneMeshBuilderAdaptor};


pub struct ColorMeshBuilder<'md> {
	data: &'md mut MeshData<ColorVertex>,
	color: Color,
}


impl<'md> ColorMeshBuilder<'md> {
	pub fn new(data: &'md mut MeshData<ColorVertex>) -> Self {
		ColorMeshBuilder {
			data,
			color: Color::white(),
		}
	}

	pub fn set_color(&mut self, color: impl Into<Color>) {
		self.color = color.into();
	}

	pub fn on_plane(self, uvw: Mat3) -> PlaneMeshBuilderAdaptor<Self> {
		PlaneMeshBuilderAdaptor::new(self, uvw)
	}

	pub fn on_plane_ref(&mut self, uvw: Mat3) -> PlaneMeshBuilderAdaptor<&'_ mut Self> {
		PlaneMeshBuilderAdaptor::new(self, uvw)
	}
}


impl PolyBuilder3D for ColorMeshBuilder<'_> {
	fn extend_3d(&mut self, vs: impl IntoIterator<Item=Vec3>, is: impl IntoIterator<Item=u16>) {
		let color = self.color.into();
		self.data.extend(vs.into_iter().map(|v| ColorVertex::new(v, color)), is);
	}
}


impl ColoredPolyBuilder for ColorMeshBuilder<'_> {
	fn set_color(&mut self, color: impl Into<Color>) {
		self.set_color(color);
	}
}
