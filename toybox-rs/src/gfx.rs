use common::math::*;

pub mod raw {
	include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

pub mod context;
pub mod resources;
pub mod vao;
pub mod buffer;
pub mod texture;
pub mod framebuffer;
pub mod vertex;
pub mod shader;
pub mod query;
pub mod capabilities;
pub mod mesh;

pub use self::context::*;
pub use self::resources::*;
pub use self::vao::*;
pub use self::buffer::*;
pub use self::texture::*;
pub use self::framebuffer::*;
pub use self::vertex::*;
pub use self::shader::*;
pub use self::query::*;
pub use self::capabilities::*;
pub use self::mesh::*;

pub enum DrawMode {
	Points,
	Lines,
	Triangles,
}


bitflags::bitflags! {
	pub struct ClearMode : u32 {
		const COLOR = 0b001;
		const DEPTH = 0b010;
		const STENCIL = 0b100;
		const ALL = 0b111;
	}
}


impl DrawMode {
	fn into_gl(self) -> u32 {
		match self {
			DrawMode::Points => raw::POINTS,
			DrawMode::Lines => raw::LINES,
			DrawMode::Triangles => raw::TRIANGLES,
		}
	}
}

impl ClearMode {
	fn into_gl(self) -> u32 {
		let mut gl_value = 0;
		if self.contains(Self::COLOR) { gl_value |= raw::COLOR_BUFFER_BIT }
		if self.contains(Self::DEPTH) { gl_value |= raw::DEPTH_BUFFER_BIT }
		if self.contains(Self::STENCIL) { gl_value |= raw::STENCIL_BUFFER_BIT }
		gl_value
	}
}
