use crate::prelude::*;

pub mod basic_mesh;
pub use basic_mesh::*;

pub mod boat;
pub use boat::*;

pub mod water;
pub use water::*;

pub mod island;
pub use island::*;

pub mod friends;
pub use friends::*;

pub mod ui;
pub use ui::*;


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
