use crate::gfx::{self, raw};
use crate::prelude::*;

#[derive(Debug)]
pub enum FramebufferSize {
	/// Automatically resize to match backbuffer size
	Backbuffer,

	/// Automatically resize to match some division of backbuffer size
	BackbufferDivisor(usize),

	/// One size forever
	Fixed(Vec2i),
}


#[derive(Debug)]
pub struct Framebuffer {
	pub(super) handle: u32,
	pub(super) size_mode: FramebufferSize,
}


impl Framebuffer {
	pub(super) fn new(size_mode: FramebufferSize) -> Framebuffer {
		Framebuffer {
			handle: unsafe {
				let mut handle = 0;
				raw::CreateFramebuffers(1, &mut handle);
				handle
			},

			size_mode,
		}
	}

	pub fn is_complete(&self) -> bool {
		let status = unsafe {raw::CheckNamedFramebufferStatus(self.handle, raw::FRAMEBUFFER)};
		match status {
			0 => true,
			_ => false,
		}
	}

	// pub fn attach
}