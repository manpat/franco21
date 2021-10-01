pub use toybox::prelude::*;
pub use toybox::input::raw::{Scancode, MouseButton};

pub use toybox::gfx::mesh::traits::*;

pub use crate::{
	controller,
	model,
	view,
	shaders,
	debug,
};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
