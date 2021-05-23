use crate::gl;
use std::marker::PhantomData;

#[derive(Copy, Clone, Debug)]
pub enum BufferUsage {
	Static,
	Dynamic,
	Stream,
}

#[derive(Copy, Clone, Debug)]
pub struct UntypedBuffer (pub(super) u32);


#[derive(Copy, Clone, Debug)]
pub struct Buffer<T: Copy> {
	pub(super) handle: u32,
	_phantom: PhantomData<*const T>,
}


impl UntypedBuffer {
	pub fn upload<T: Copy>(&self, data: &[T], usage: BufferUsage) {
		upload_untyped(self.0, data, usage);
	}

	pub fn into_typed<T: Copy>(self) -> Buffer<T> {
		Buffer {
			handle: self.0,
			_phantom: PhantomData,
		}
	}
}


impl<T: Copy> Buffer<T> {
	pub fn upload(&self, data: &[T], usage: BufferUsage) {
		upload_untyped(self.handle, data, usage);
	}
}



impl<T: Copy> From<Buffer<T>> for UntypedBuffer {
	fn from(Buffer{handle, ..}: Buffer<T>) -> UntypedBuffer {
		UntypedBuffer(handle)
	}
}




fn upload_untyped<T: Copy>(handle: u32, data: &[T], usage: BufferUsage) {
	assert!(!data.is_empty());

	let usage = match usage {
		BufferUsage::Static => gl::raw::STATIC_DRAW,
		BufferUsage::Dynamic => gl::raw::DYNAMIC_DRAW,
		BufferUsage::Stream => gl::raw::STREAM_DRAW,
	};

	let size_bytes = data.len() * std::mem::size_of::<T>();

	unsafe {
		gl::raw::NamedBufferData(
			handle,
			size_bytes as _,
			data.as_ptr() as *const _,
			usage
		);
	}
}