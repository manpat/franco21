use crate::prelude::*;
use crate::gfx::raw;
use std::cell::Cell;


#[derive(Debug)]
pub struct Texture {
	pub(super) handle: u32,
	size_mode: TextureSize,
	format: u32,

	min_filter_linear: bool,
	mag_filter_linear: bool,
	wrap_repeat: bool,

	is_dirty: Cell<bool>,
}


impl Texture {
	pub fn clear(&mut self) {
		unsafe {
			let format = texture_format_to_unsized(self.format);
			raw::ClearTexImage(self.handle, 0, format, raw::FLOAT, std::ptr::null());
		}
	}

	pub fn set_filter(&mut self, min_linear: bool, mag_linear: bool) {
		self.min_filter_linear = min_linear;
		self.mag_filter_linear = mag_linear;
		self.is_dirty.set(true);
	}

	pub fn set_wrap(&mut self, wrap: bool) {
		self.wrap_repeat = wrap;
		self.is_dirty.set(true);
	}
}



impl Texture {
	pub(super) fn new(size_mode: TextureSize, backbuffer_size: Vec2i, format: u32) -> Texture {
		let Vec2i{x:width, y:height} = size_mode.resolve(backbuffer_size);

		unsafe {
			let mut handle = 0;
			raw::CreateTextures(raw::TEXTURE_2D, 1, &mut handle);
			raw::TextureStorage2D(handle, 1, format, width, height);
			Texture {
				handle,
				size_mode,
				format,

				min_filter_linear: false,
				mag_filter_linear: false,
				wrap_repeat: false,

				is_dirty: true.into(),
			}
		}
	}

	/// Called when the size of the backbuffer changes.
	/// Returns whether or not the gl handle was invalidated.
	pub(super) fn on_resize(&mut self, backbuffer_size: Vec2i) -> bool {
		// fixed size textures don't care about backbuffer changes
		if matches!(self.size_mode, TextureSize::Fixed(_)) {
			return false
		}

		let Vec2i{x: new_width, y: new_height} = self.size_mode.resolve(backbuffer_size);
		unsafe {
			raw::DeleteTextures(1, &mut self.handle);
			raw::CreateTextures(raw::TEXTURE_2D, 1, &mut self.handle);
			raw::TextureStorage2D(self.handle, 1, self.format, new_width, new_height);
		}

		self.is_dirty.set(true);

		true
	}

	/// Applies all parameters that have changed since last call.
	/// Must be called before a texture is sampled
	pub(super) fn apply_changes(&mut self) {
		if !self.is_dirty.get() {
			return
		}

		self.is_dirty.set(false);

		let min_filter = match self.min_filter_linear {
			true => raw::LINEAR,
			false => raw::NEAREST,
		};

		let mag_filter = match self.mag_filter_linear {
			true => raw::LINEAR,
			false => raw::NEAREST,
		};

		let wrap_mode = match self.wrap_repeat {
			true => raw::REPEAT,
			false => raw::CLAMP_TO_EDGE,
		};

		unsafe {
			raw::TextureParameteri(self.handle, raw::TEXTURE_MIN_FILTER, min_filter as _);
			raw::TextureParameteri(self.handle, raw::TEXTURE_MAG_FILTER, mag_filter as _);
			raw::TextureParameteri(self.handle, raw::TEXTURE_WRAP_S, wrap_mode as _);
			raw::TextureParameteri(self.handle, raw::TEXTURE_WRAP_T, wrap_mode as _);
		}
	}
}






#[derive(Copy, Clone, Debug)]
pub enum TextureSize {
	/// Automatically resize to match backbuffer size
	Backbuffer,

	/// Automatically resize to match some division of backbuffer size
	BackbufferDivisor(usize),

	/// One size forever
	Fixed(Vec2i),
}

impl TextureSize {
	pub fn resolve(&self, backbuffer_size: Vec2i) -> Vec2i {
		match *self {
			TextureSize::Backbuffer => backbuffer_size,
			TextureSize::BackbufferDivisor(d) => backbuffer_size / d as i32,
			TextureSize::Fixed(fixed_size) => fixed_size,
		}
	} 
}

impl From<Vec2i> for TextureSize {
	fn from(o: Vec2i) -> TextureSize {
		TextureSize::Fixed(o)
	}
}





pub fn texture_format_to_unsized(sized: u32) -> u32 {
	match sized {
		raw::DEPTH_COMPONENT16 | raw::DEPTH_COMPONENT24 | raw::DEPTH_COMPONENT32F => raw::DEPTH_COMPONENT,
		raw::DEPTH24_STENCIL8 | raw::DEPTH32F_STENCIL8 => raw::DEPTH_STENCIL,
		raw::STENCIL_INDEX8 => raw::STENCIL_INDEX,

		raw::R8 | raw::R8I | raw::R8UI | raw::R8_SNORM
			| raw::R16 | raw::R16I | raw::R16UI | raw::R16_SNORM | raw::R16F
			| raw::R32I | raw::R32UI | raw::R32F => raw::RED,

		raw::RG8 | raw::RG8I | raw::RG8UI | raw::RG8_SNORM
			| raw::RG16 | raw::RG16I | raw::RG16UI | raw::RG16_SNORM | raw::RG16F
			| raw::RG32I | raw::RG32UI | raw::RG32F => raw::RG,

		raw::RGB8 | raw::RGB8I | raw::RGB8UI | raw::RGB8_SNORM
			| raw::RGB16 | raw::RGB16I | raw::RGB16UI | raw::RGB16_SNORM | raw::RGB16F
			| raw::RGB32I | raw::RGB32UI | raw::RGB32F
			| raw::R11F_G11F_B10F | raw::SRGB8 => raw::RGB,

		raw::RGBA8 | raw::RGBA8I | raw::RGBA8UI | raw::RGBA8_SNORM
			| raw::RGBA16 | raw::RGBA16I | raw::RGBA16UI | raw::RGBA16_SNORM | raw::RGBA16F
			| raw::RGBA32I | raw::RGBA32UI | raw::RGBA32F
			| raw::SRGB8_ALPHA8 | raw::RGB10_A2 | raw::RGB10_A2UI => raw::RGBA,

		_ => panic!("unhandled"),
	}
}