#![feature(backtrace, array_chunks)]

pub mod prelude;
pub mod gl;
pub mod perf;
pub mod window;
pub mod input;
pub mod audio;

pub use crate::prelude::*;

use std::error::Error;

pub struct Engine {
	pub sdl_ctx: sdl2::Sdl,
	pub event_pump: sdl2::EventPump,
	pub window: sdl2::video::Window,
	pub gl_ctx: gl::Context,
	pub input: input::InputSystem,
	pub audio: audio::AudioSystem,
	pub instrumenter: perf::Instrumenter,

	should_quit: bool,
}


impl Engine {
	pub fn new(window_name: &str) -> Result<Engine, Box<dyn Error>> {
		let sdl_ctx = sdl2::init()?;
		let sdl_video = sdl_ctx.video()?;
		let sdl_audio = sdl_ctx.audio()?;

		let (window, gl_ctx) = window::init_window(&sdl_video, window_name)?;
		let event_pump = sdl_ctx.event_pump()?;
		let input = input::InputSystem::new(sdl_ctx.mouse());
		let audio = audio::AudioSystem::new(sdl_audio)?;

		let instrumenter = perf::Instrumenter::new(&gl_ctx);

		Ok(Engine {
			sdl_ctx,
			event_pump,
			window,
			gl_ctx,
			input,
			audio,
			instrumenter,

			should_quit: false,
		})
	}

	pub fn should_quit(&self) -> bool { self.should_quit }

	pub fn process_events(&mut self) {
		self.input.clear();

		for event in self.event_pump.poll_iter() {
			use sdl2::event::{Event, WindowEvent};

			match event {
				Event::Quit {..} => { self.should_quit = true }
				Event::Window{ win_event: WindowEvent::Resized(..), .. } => {
					let (w, h) = self.window.drawable_size();
					self.gl_ctx.on_resize(w, h);
					self.input.handle_event(&event)
				}

				_ => {
					self.input.handle_event(&event)
				},
			}
		}

		self.input.process_events();
	}

	pub fn end_frame(&mut self) {
		{
			// TODO(pat.m): should probably add CPU timing to Instrumenter so that this actually makes sense
			let _guard = self.instrumenter.scoped_section("audio");
			self.audio.update();
		}

		self.instrumenter.end_frame();
		self.window.gl_swap_window();
	}
}