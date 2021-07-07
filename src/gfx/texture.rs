use crate::gfx::raw;


#[derive(Copy, Clone, Debug)]
pub struct Texture (pub(super) u32);


impl Texture {
	pub fn clear(&self) {
		unsafe {
			raw::ClearTexImage(self.0, 0, raw::RED, raw::FLOAT, &0.0f32 as *const f32 as _);
		}
	}

	pub fn set_filter(&self, min_linear: bool, mag_linear: bool) {
		let min = match min_linear {
			true => raw::LINEAR,
			false => raw::NEAREST,
		};

		let mag = match mag_linear {
			true => raw::LINEAR,
			false => raw::NEAREST,
		};

		unsafe {
			raw::TextureParameteri(self.0, raw::TEXTURE_MIN_FILTER, min as _);
			raw::TextureParameteri(self.0, raw::TEXTURE_MAG_FILTER, mag as _);
		}
	}

	pub fn set_wrap(&self, wrap: bool) {
		let mode = match wrap {
			true => raw::REPEAT,
			false => raw::CLAMP_TO_EDGE,
		};

		unsafe {
			raw::TextureParameteri(self.0, raw::TEXTURE_WRAP_S, mode as _);
			raw::TextureParameteri(self.0, raw::TEXTURE_WRAP_T, mode as _);
		}
	}
}