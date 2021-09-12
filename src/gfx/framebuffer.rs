use crate::gfx::{self, raw};
use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub enum FramebufferSize {
	/// Automatically resize to match backbuffer size
	Backbuffer,

	/// Automatically resize to match some division of backbuffer size
	BackbufferDivisor(usize),

	/// One size forever
	Fixed(Vec2i),
}

impl FramebufferSize {
	pub fn resolve(&self, backbuffer_size: Vec2i) -> Vec2i {
		match *self {
			FramebufferSize::Backbuffer => backbuffer_size,
			FramebufferSize::BackbufferDivisor(d) => backbuffer_size / d as i32,
			FramebufferSize::Fixed(fixed_size) => fixed_size,
		}
	} 
}


#[derive(Debug)]
pub struct Framebuffer {
	pub(super) handle: u32,
	pub(super) size_mode: FramebufferSize,
}


impl Framebuffer {
	pub(super) fn new(settings: FramebufferSettings, canvas_size: Vec2i) -> Framebuffer {
		let FramebufferSettings {
			size_mode,
			depth_attachment,
			stencil_attachment,
			color_attachments,
		} = settings;

		let (format, attachment_point) = match (depth_attachment, stencil_attachment) {
			(false, false) => todo!(),
			(true, false) => (raw::DEPTH_COMPONENT24, raw::DEPTH_ATTACHMENT),
			(false, true) => todo!(),
			(true, true) => (raw::DEPTH24_STENCIL8, raw::DEPTH_STENCIL_ATTACHMENT),
		};

		let Vec2i{x: backbuffer_width, y: backbuffer_height} = size_mode.resolve(canvas_size);

		let mut fbo = 0;
		let mut depth_stencil_tex = 0;

		unsafe {
			raw::CreateFramebuffers(1, &mut fbo);

			raw::CreateTextures(raw::TEXTURE_2D, 1, &mut depth_stencil_tex);
			raw::TextureParameteri(depth_stencil_tex, raw::TEXTURE_WRAP_S, raw::CLAMP_TO_EDGE as _);
			raw::TextureParameteri(depth_stencil_tex, raw::TEXTURE_WRAP_T, raw::CLAMP_TO_EDGE as _);
			raw::TextureParameteri(depth_stencil_tex, raw::TEXTURE_MAG_FILTER, raw::NEAREST as _);
			raw::TextureParameteri(depth_stencil_tex, raw::TEXTURE_MIN_FILTER, raw::NEAREST as _);
			raw::TextureStorage2D(depth_stencil_tex, 1, format, backbuffer_width, backbuffer_height);

			raw::NamedFramebufferTexture(fbo, attachment_point, depth_stencil_tex, 0);

			for (slot, format) in color_attachments.iter()
				.enumerate()
				.filter_map(|(s, f)| f.map(|f| (s, f)))
			{
				let mut color_tex = 0;

				raw::CreateTextures(raw::TEXTURE_2D, 1, &mut color_tex);
				raw::TextureParameteri(color_tex, raw::TEXTURE_WRAP_S, raw::CLAMP_TO_EDGE as _);
				raw::TextureParameteri(color_tex, raw::TEXTURE_WRAP_T, raw::CLAMP_TO_EDGE as _);
				raw::TextureParameteri(color_tex, raw::TEXTURE_MAG_FILTER, raw::NEAREST as _);
				raw::TextureParameteri(color_tex, raw::TEXTURE_MIN_FILTER, raw::NEAREST as _);
				raw::TextureStorage2D(color_tex, 1, format, backbuffer_width, backbuffer_height);
				
				raw::NamedFramebufferTexture(fbo, raw::COLOR_ATTACHMENT0 + slot as u32, color_tex, 0);
			}

			let draw_buffers: Vec<_> = color_attachments.iter().enumerate()
				.filter(|(_, &f)| f.is_some())
				.map(|(i, _)| raw::COLOR_ATTACHMENT0 + i as u32)
				.collect();

			raw::NamedFramebufferDrawBuffers(fbo, draw_buffers.len() as _, draw_buffers.as_ptr());
		}

		Framebuffer {
			handle: fbo,
			size_mode,
		}
	}

	// pub(super) fn resize(&mut self, canvas_size: Vec2i) {

	// }

	pub fn is_complete(&self) -> bool {
		let status = unsafe {raw::CheckNamedFramebufferStatus(self.handle, raw::FRAMEBUFFER)};
		match status {
			0 => true,
			_ => false,
		}
	}
}




pub struct FramebufferSettings {
	size_mode: FramebufferSize,
	depth_attachment: bool,
	stencil_attachment: bool,
	color_attachments: [Option<u32>; 8],
}

impl FramebufferSettings {
	pub fn new(size_mode: FramebufferSize) -> FramebufferSettings {
		FramebufferSettings {
			size_mode,
			depth_attachment: false,
			stencil_attachment: false,
			color_attachments: [None; 8],
		}
	}

	pub fn add_depth(self) -> Self {
		FramebufferSettings {depth_attachment: true, ..self}
	}

	pub fn add_stencil(self) -> Self {
		FramebufferSettings {stencil_attachment: true, ..self}
	}

	pub fn add_depth_stencil(self) -> Self {
		self.add_depth().add_stencil()
	}

	pub fn add_color(mut self, attachment_point: u32, gl_format: u32) -> Self {
		assert!(attachment_point < 8);
		self.color_attachments[attachment_point as usize] = Some(gl_format);
		self
	}
}




// fn texture_format_to_unsized(sized: u32) -> u32 {
// 	match sized {
// 		raw::DEPTH_COMPONENT16 | raw::DEPTH_COMPONENT24 | raw::DEPTH_COMPONENT32F => raw::DEPTH_COMPONENT,
// 		raw::DEPTH24_STENCIL8 | raw::DEPTH32F_STENCIL8 => raw::DEPTH_STENCIL,
// 		raw::STENCIL_INDEX8 => raw::STENCIL_INDEX,

// 		raw::R8 | raw::R8I | raw::R8UI | raw::R8_SNORM
// 			| raw::R16 | raw::R16I | raw::R16UI | raw::R16_SNORM | raw::R16F
// 			| raw::R32I | raw::R32UI | raw::R32F => raw::RED,

// 		raw::RG8 | raw::RG8I | raw::RG8UI | raw::RG8_SNORM
// 			| raw::RG16 | raw::RG16I | raw::RG16UI | raw::RG16_SNORM | raw::RG16F
// 			| raw::RG32I | raw::RG32UI | raw::RG32F => raw::RG,

// 		raw::RGB8 | raw::RGB8I | raw::RGB8UI | raw::RGB8_SNORM
// 			| raw::RGB16 | raw::RGB16I | raw::RGB16UI | raw::RGB16_SNORM | raw::RGB16F
// 			| raw::RGB32I | raw::RGB32UI | raw::RGB32F
// 			| raw::R11F_G11F_B10F | raw::SRGB8 => raw::RGB,

// 		raw::RGBA8 | raw::RGBA8I | raw::RGBA8UI | raw::RGBA8_SNORM
// 			| raw::RGBA16 | raw::RGBA16I | raw::RGBA16UI | raw::RGBA16_SNORM | raw::RGBA16F
// 			| raw::RGBA32I | raw::RGBA32UI | raw::RGBA32F
// 			| raw::SRGB8_ALPHA8 | raw::RGB10_A2 | raw::RGB10_A2UI => raw::RGBA,

// 		_ => panic!("unhandled"),
// 	}
// }