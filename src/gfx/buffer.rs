use crate::gfx;
use std::marker::PhantomData;

#[derive(Copy, Clone, Debug)]
pub enum BufferUsage {
	Static,
	Dynamic,
	Stream,
}

#[derive(Copy, Clone, Debug)]
pub struct UntypedBuffer {
	pub(super) handle: u32,
	pub(super) size_bytes: usize,
	pub(super) usage: BufferUsage,
}


#[derive(Copy, Clone, Debug)]
pub struct Buffer<T: Copy> {
	pub(super) handle: u32,
	length: u32,
	usage: BufferUsage,
	_phantom: PhantomData<*const T>,
}


impl UntypedBuffer {
	pub fn upload<T: Copy>(&mut self, data: &[T]) {
		upload_untyped(self.handle, data, self.usage);
		self.size_bytes = data.len() * std::mem::size_of::<T>();
	}

	pub fn into_typed<T: Copy>(self) -> Buffer<T> {
		Buffer {
			handle: self.handle,
			length: (self.size_bytes / std::mem::size_of::<T>()) as u32,
			usage: self.usage,
			_phantom: PhantomData,
		}
	}
}


impl<T: Copy> Buffer<T> {
	pub fn upload(&mut self, data: &[T]) {
		upload_untyped(self.handle, data, self.usage);
		self.length = data.len() as u32;
	}

	pub fn len(&self) -> u32 {
		self.length
	}

	pub fn is_empty(&self) -> bool {
		self.length == 0
	}
}



impl<T: Copy> From<Buffer<T>> for UntypedBuffer {
	fn from(Buffer{handle, length, usage, ..}: Buffer<T>) -> UntypedBuffer {
		UntypedBuffer {
			handle,
			size_bytes: length as usize * std::mem::size_of::<T>(),
			usage,
		}
	}
}




fn upload_untyped<T: Copy>(handle: u32, data: &[T], usage: BufferUsage) {
	if data.is_empty() {
		// TODO(pat.m): is this what I want? 
		return
	}

	let usage = match usage {
		BufferUsage::Static => gfx::raw::STATIC_DRAW,
		BufferUsage::Dynamic => gfx::raw::DYNAMIC_DRAW,
		BufferUsage::Stream => gfx::raw::STREAM_DRAW,
	};

	let size_bytes = data.len() * std::mem::size_of::<T>();

	unsafe {
		gfx::raw::NamedBufferData(
			handle,
			size_bytes as _,
			data.as_ptr() as *const _,
			usage
		);
	}
}