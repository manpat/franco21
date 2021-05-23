pub mod prelude;
pub mod gl;
pub mod perf;
pub mod window;

pub use crate::prelude::*;

use std::error::Error;

pub struct Engine {
	pub sdl_ctx: sdl2::Sdl,
	pub event_pump: sdl2::EventPump,
	pub window: sdl2::video::Window,
	pub gl_ctx: gl::Context,
}


impl Engine {
	pub fn new(window_name: &str) -> Result<Engine, Box<dyn Error>> {
		let sdl_ctx = sdl2::init()?;
		let sdl_video = sdl_ctx.video()?;

		let (window, gl_ctx) = window::init_window(&sdl_video, window_name)?;

		let event_pump = sdl_ctx.event_pump()?;

		Ok(Engine {
			sdl_ctx,
			event_pump,
			window,
			gl_ctx,
		})
	}
}