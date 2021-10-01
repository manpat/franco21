use crate::prelude::*;

pub mod boat;
pub mod water;

pub use boat::*;
pub use water::*;


pub struct ViewContext<'engine> {
	pub gfx: gfx::RenderState<'engine>,
	pub resources: &'engine gfx::Resources,
}

impl<'engine> ViewContext<'engine> {
	pub fn new(gfx: gfx::RenderState<'engine>) -> ViewContext<'engine> {
		let resources = gfx.resources();
		ViewContext {
			gfx,
			resources,
		}
	}
}
