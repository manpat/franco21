use common::*;
use crate::gfx::vertex::{ColorVertex, ColorVertex2D};
use crate::gfx::mesh::{MeshData, PolyBuilder2D, PolyBuilder3D, ColoredPolyBuilder};
use std::borrow::BorrowMut;


pub struct ColorMeshBuilder<MD> {
	pub data: MD,
	color: Color,
}


impl<MD> ColorMeshBuilder<MD> {
	pub fn new(data: MD) -> Self {
		ColorMeshBuilder {
			data,
			color: Color::white(),
		}
	}

	pub fn set_color(&mut self, color: impl Into<Color>) {
		self.color = color.into();
	}
}


impl<MD> ColoredPolyBuilder for ColorMeshBuilder<MD> {
	fn set_color(&mut self, color: impl Into<Color>) {
		self.set_color(color);
	}
}



impl<MD> PolyBuilder2D for ColorMeshBuilder<MD>
	where MD: BorrowMut<MeshData<ColorVertex2D>>
{
	fn extend_2d(&mut self, vs: impl IntoIterator<Item=Vec2>, is: impl IntoIterator<Item=u16>) {
		let color = self.color;
		self.data.borrow_mut().extend(vs.into_iter().map(move |v| ColorVertex2D::new(v, color)), is);
	}
}


impl<MD> PolyBuilder3D for ColorMeshBuilder<MD>
	where MD: BorrowMut<MeshData<ColorVertex>>
{
	fn extend_3d(&mut self, vs: impl IntoIterator<Item=Vec3>, is: impl IntoIterator<Item=u16>) {
		let color = self.color;
		self.data.borrow_mut().extend(vs.into_iter().map(move |v| ColorVertex::new(v, color)), is);
	}
}