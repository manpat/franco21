use crate::prelude::*;
use crate::gfx::{
	self, raw,
	Texture, TextureSize, TextureFormat, TextureKey,
};

#[derive(Debug)]
struct Attachment {
	attachment_point: u32,
	texture_key: TextureKey,
}

#[derive(Debug)]
pub struct Framebuffer {
	pub(super) handle: u32,
	pub(super) size_mode: TextureSize,

	depth_stencil_attachment: Option<Attachment>,
	color_attachments: Vec<Attachment>,
}

impl Framebuffer {
	pub(super) fn new(settings: FramebufferSettings, resources: &mut gfx::Resources, backbuffer_size: Vec2i) -> Framebuffer {
		let FramebufferSettings {
			size_mode,
			depth_attachment,
			stencil_attachment,
			color_attachments,
		} = settings;

		let mut generate_depth_stencil_attachment = || {
			let (depth_stencil_format, depth_stencil_attachment_point) = match (depth_attachment, stencil_attachment) {
				(false, false) => return None,
				(true, false) => (TextureFormat::Depth, raw::DEPTH_ATTACHMENT),
				(false, true) => (TextureFormat::Stencil, raw::STENCIL_ATTACHMENT),
				(true, true) => (TextureFormat::DepthStencil, raw::DEPTH_STENCIL_ATTACHMENT),
			};

			let mut depth_stencil_tex = Texture::new(size_mode, backbuffer_size, depth_stencil_format);
			depth_stencil_tex.set_filter(false, false);

			Some(Attachment {
				attachment_point: depth_stencil_attachment_point,
				texture_key: resources.insert_texture(depth_stencil_tex)
			})
		};

		let depth_stencil_attachment = generate_depth_stencil_attachment();

		let color_attachments = color_attachments.iter()
			.enumerate()
			.filter_map(|(s, maybe_f)| maybe_f.map(|f| (s, f))) // (attachment_point, format)
			.map(|(attachment_point, format)| {
				let color_tex = Texture::new(size_mode, backbuffer_size, format);
				let texture_key = resources.insert_texture(color_tex);
				let attachment_point = raw::COLOR_ATTACHMENT0 + attachment_point as u32;
				Attachment {attachment_point, texture_key}
			})
			.collect(): Vec<_>;


		// TODO(pat.m): this isn't actually correct I don't think
		// draw_buffers should effectively match the format of settings.color_attachments, with raw::NONE in gaps
		let draw_buffers: Vec<_> = color_attachments.iter()
			.map(|Attachment{attachment_point, ..}| *attachment_point)
			.collect();


		let mut fbo = 0;

		unsafe {
			raw::CreateFramebuffers(1, &mut fbo);

			if let Some(Attachment{attachment_point, texture_key}) = depth_stencil_attachment {
				let handle = resources.get(texture_key).texture_handle;
				raw::NamedFramebufferTexture(fbo, attachment_point, handle, 0);
			}

			for &Attachment{attachment_point, texture_key} in color_attachments.iter() {
				let handle = resources.get(texture_key).texture_handle;
				raw::NamedFramebufferTexture(fbo, attachment_point, handle, 0);
			}

			raw::NamedFramebufferDrawBuffers(fbo, draw_buffers.len() as _, draw_buffers.as_ptr());
		}

		Framebuffer {
			handle: fbo,
			size_mode,

			depth_stencil_attachment,
			color_attachments,
		}
	}

	// HACK: this should take &mut self probably, but can't while Resources uses RefCell nonsense
	pub(super) fn rebind_attachments(&self, resources: &gfx::Resources) {
		if let Some(Attachment{attachment_point, texture_key}) = self.depth_stencil_attachment {
			let texture_handle = resources.get(texture_key).texture_handle;
			unsafe {
				raw::NamedFramebufferTexture(self.handle, attachment_point, texture_handle, 0);
			}
		}

		for &Attachment{attachment_point, texture_key} in self.color_attachments.iter() {
			let handle = resources.get(texture_key).texture_handle;
			unsafe {
				raw::NamedFramebufferTexture(self.handle, attachment_point, handle, 0);
			}
		}
	}

	pub fn is_complete(&self) -> bool {
		let status = unsafe {raw::CheckNamedFramebufferStatus(self.handle, raw::DRAW_FRAMEBUFFER)};
		match status {
			0 => true,
			_ => false,
		}
	}

	pub fn depth_stencil_attachment(&self) -> Option<TextureKey> {
		self.depth_stencil_attachment.as_ref().map(|attachment| attachment.texture_key)
	}

	pub fn color_attachment(&self, attachment_point: u32) -> Option<TextureKey> {
		let attachment_point = raw::COLOR_ATTACHMENT0 + attachment_point;

		self.color_attachments.iter()
			.find(|attachment| attachment.attachment_point == attachment_point)
			.map(|attachment| attachment.texture_key)
	}
}



#[derive(Copy, Clone, Debug)]
pub struct FramebufferSettings {
	size_mode: TextureSize,
	depth_attachment: bool,
	stencil_attachment: bool,
	color_attachments: [Option<TextureFormat>; 8],
}

impl FramebufferSettings {
	pub fn new(size_mode: TextureSize) -> FramebufferSettings {
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

	pub fn add_color(mut self, attachment_point: u32, format: TextureFormat) -> Self {
		assert!(attachment_point < 8);
		self.color_attachments[attachment_point as usize] = Some(format);
		self
	}
}


