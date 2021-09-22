use crate::prelude::*;
use crate::gfx::raw;


#[derive(Debug)]
pub struct Texture {
	pub(super) handle: u32,
	size_mode: TextureSize,
	format: TextureFormat,

	min_filter_linear: bool,
	mag_filter_linear: bool,
	wrap_repeat: bool,

	current_size: Vec2i,
}


impl Texture {
	pub fn clear(&mut self) {
		unsafe {
			let format = self.format.to_gl_unsized();
			raw::ClearTexImage(self.handle, 0, format, raw::FLOAT, std::ptr::null());
		}
	}

	pub fn set_filter(&mut self, min_linear: bool, mag_linear: bool) {
		self.min_filter_linear = min_linear;
		self.mag_filter_linear = mag_linear;
		self.apply_filtering_parameters();
	}

	pub fn set_wrap(&mut self, wrap: bool) {
		self.wrap_repeat = wrap;
		self.apply_filtering_parameters();
	}

	pub fn format(&self) -> TextureFormat { self.format }
	pub fn size(&self) -> Vec2i { self.current_size }
	pub fn size_mode(&self) -> TextureSize { self.size_mode }
}



impl Texture {
	pub(super) fn new(size_mode: TextureSize, backbuffer_size: Vec2i, format: TextureFormat) -> Texture {
		let Vec2i{x:width, y:height} = size_mode.resolve(backbuffer_size);

		let mut texture = unsafe {
			let mut handle = 0;
			raw::CreateTextures(raw::TEXTURE_2D, 1, &mut handle);
			raw::TextureStorage2D(handle, 1, format.to_gl(), width, height);
			Texture {
				handle,
				size_mode,
				format,

				min_filter_linear: false,
				mag_filter_linear: false,
				wrap_repeat: false,

				current_size: size_mode.resolve(backbuffer_size),
			}
		};

		texture.apply_filtering_parameters();
		texture
	}

	/// Called when the size of the backbuffer changes.
	/// Returns whether or not the gl handle was invalidated.
	pub(super) fn on_resize(&mut self, backbuffer_size: Vec2i) -> bool {
		// fixed size textures don't care about backbuffer changes
		if matches!(self.size_mode, TextureSize::Fixed(_)) {
			return false
		}

		self.current_size = self.size_mode.resolve(backbuffer_size);
		let Vec2i{x: new_width, y: new_height} = self.current_size;

		unsafe {
			raw::DeleteTextures(1, &mut self.handle);
			raw::CreateTextures(raw::TEXTURE_2D, 1, &mut self.handle);
			raw::TextureStorage2D(self.handle, 1, self.format.to_gl(), new_width, new_height);
		}

		self.apply_filtering_parameters();

		true
	}

	/// Applies all parameters that have changed since last call.
	pub(super) fn apply_filtering_parameters(&mut self) {
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



#[derive(Copy, Clone, Debug)]
pub enum BaseFormat {
	Unorm8, Unorm16,
	I8, I16, I32,
	U8, U16, U32,
	F16, F32,
}

#[derive(Copy, Clone, Debug)]
pub enum SpecialFormat {
	R11G11B10F,
	Rgb10A2,
	Rgb10A2Ui,
	Srgb8,
	Srgba8,
}

#[derive(Copy, Clone, Debug)]
pub enum TextureFormat {
	Rgba(BaseFormat),
	RedGreen(BaseFormat),
	Red(BaseFormat),

	R11G11B10F,
	Rgb10A2,
	Rgb10A2Ui,
	Srgb8,
	Srgba8,

	Depth,
	DepthStencil,
	Stencil,

	Depth16,
	Depth32,
}


impl TextureFormat {
	pub fn color() -> Self { TextureFormat::Rgba(BaseFormat::Unorm8) }
	pub fn hdr_color() -> Self { TextureFormat::Rgba(BaseFormat::F16) }
	pub fn srgb() -> Self { TextureFormat::Srgb8 }
	pub fn srgba() -> Self { TextureFormat::Srgba8 }

	pub fn to_gl(&self) -> u32 {
		match self {
			TextureFormat::Rgba(BaseFormat::Unorm8) => raw::RGBA8,
			TextureFormat::Rgba(BaseFormat::Unorm16) => raw::RGBA16,

			TextureFormat::Rgba(BaseFormat::I8) => raw::RGBA8I,
			TextureFormat::Rgba(BaseFormat::I16) => raw::RGBA16I,
			TextureFormat::Rgba(BaseFormat::I32) => raw::RGBA32I,

			TextureFormat::Rgba(BaseFormat::U8) => raw::RGBA8UI,
			TextureFormat::Rgba(BaseFormat::U16) => raw::RGBA16UI,
			TextureFormat::Rgba(BaseFormat::U32) => raw::RGBA32UI,

			TextureFormat::Rgba(BaseFormat::F16) => raw::RGBA16F,
			TextureFormat::Rgba(BaseFormat::F32) => raw::RGBA32F,

			TextureFormat::RedGreen(BaseFormat::Unorm8) => raw::RG8,
			TextureFormat::RedGreen(BaseFormat::Unorm16) => raw::RG16,

			TextureFormat::RedGreen(BaseFormat::I8) => raw::RG8I,
			TextureFormat::RedGreen(BaseFormat::I16) => raw::RG16I,
			TextureFormat::RedGreen(BaseFormat::I32) => raw::RG32I,

			TextureFormat::RedGreen(BaseFormat::U8) => raw::RG8UI,
			TextureFormat::RedGreen(BaseFormat::U16) => raw::RG16UI,
			TextureFormat::RedGreen(BaseFormat::U32) => raw::RG32UI,

			TextureFormat::RedGreen(BaseFormat::F16) => raw::RG16F,
			TextureFormat::RedGreen(BaseFormat::F32) => raw::RG32F,

			TextureFormat::Red(BaseFormat::Unorm8) => raw::R8,
			TextureFormat::Red(BaseFormat::Unorm16) => raw::R16,

			TextureFormat::Red(BaseFormat::I8) => raw::R8I,
			TextureFormat::Red(BaseFormat::I16) => raw::R16I,
			TextureFormat::Red(BaseFormat::I32) => raw::R32I,

			TextureFormat::Red(BaseFormat::U8) => raw::R8UI,
			TextureFormat::Red(BaseFormat::U16) => raw::R16UI,
			TextureFormat::Red(BaseFormat::U32) => raw::R32UI,

			TextureFormat::Red(BaseFormat::F16) => raw::R16F,
			TextureFormat::Red(BaseFormat::F32) => raw::R32F,

			TextureFormat::R11G11B10F => raw::R11F_G11F_B10F,
			TextureFormat::Rgb10A2 => raw::RGB10_A2,
			TextureFormat::Rgb10A2Ui => raw::RGB10_A2UI,
			TextureFormat::Srgb8 => raw::SRGB8,
			TextureFormat::Srgba8 => raw::SRGB8_ALPHA8,

			TextureFormat::Depth => raw::DEPTH_COMPONENT24,
			TextureFormat::Stencil => raw::STENCIL_INDEX8,
			TextureFormat::DepthStencil => raw::DEPTH24_STENCIL8,

			TextureFormat::Depth16 => raw::DEPTH_COMPONENT16,
			TextureFormat::Depth32 => raw::DEPTH_COMPONENT32F,
		}
	}

	pub fn to_gl_unsized(&self) -> u32 {
		match self {
			TextureFormat::Rgba(_) => raw::RGBA,
			TextureFormat::RedGreen(_) => raw::RG,
			TextureFormat::Red(_) => raw::RED,

			TextureFormat::Rgb10A2 | TextureFormat::Rgb10A2Ui => raw::RGBA,
			TextureFormat::R11G11B10F => raw::RGB,
			TextureFormat::Srgb8 => raw::RGB,
			TextureFormat::Srgba8 => raw::RGBA,

			TextureFormat::Depth | TextureFormat::Depth16 | TextureFormat::Depth32 => raw::DEPTH_COMPONENT,
			TextureFormat::Stencil => raw::STENCIL_INDEX,
			TextureFormat::DepthStencil => raw::DEPTH_STENCIL,
		}
	}
}

