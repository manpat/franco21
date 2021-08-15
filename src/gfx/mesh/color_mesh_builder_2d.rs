use common::*;
use crate::gfx::vertex::ColorVertex2D;
use crate::gfx::mesh::{MeshData, PolyBuilder2D, ColoredPolyBuilder};


pub struct ColorMeshBuilder2D<'md> {
	data: &'md mut MeshData<ColorVertex2D>,
	color: Color,
}


impl<'md> ColorMeshBuilder2D<'md> {
	pub fn new(data: &'md mut MeshData<ColorVertex2D>) -> Self {
		ColorMeshBuilder2D {
			data,
			color: Color::white(),
		}
	}

	pub fn set_color(&mut self, color: impl Into<Color>) {
		self.color = color.into();
	}
}


impl PolyBuilder2D for ColorMeshBuilder2D<'_> {
	fn extend_2d(&mut self, vs: impl IntoIterator<Item=Vec2>, is: impl IntoIterator<Item=u16>) {
		let color = self.color.into();
		self.data.extend(vs.into_iter().map(|v| ColorVertex2D::new(v, color)), is);
	}
}


impl ColoredPolyBuilder for ColorMeshBuilder2D<'_> {
	fn set_color(&mut self, color: impl Into<Color>) {
		self.set_color(color);
	}
}
