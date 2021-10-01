use crate::gfx::{self, raw};

#[derive(Copy, Clone, Debug)]
pub struct Vao {
	pub(super) handle: u32,
}



impl Vao {
	pub(super) fn new(handle: u32) -> Vao {
		Vao {
			handle,
		}
	}


	pub fn bind_index_buffer(&mut self, index_buffer: impl Into<gfx::UntypedBuffer>) {
		unsafe {
			raw::VertexArrayElementBuffer(self.handle, index_buffer.into().handle);
		}
	}

	pub fn bind_vertex_buffer<V: gfx::Vertex>(&mut self, binding: u32, vertex_buffer: gfx::Buffer<V>) {
		let descriptor = V::descriptor();
		let stride = descriptor.size_bytes as i32;

		for (attribute_index, attribute) in descriptor.attributes.iter().enumerate() {
			let attribute_index = attribute_index as u32;

			let &gfx::vertex::Attribute{
				offset_bytes,
				num_elements,
				gl_type,
			} = attribute;

			unsafe {
				raw::EnableVertexArrayAttrib(self.handle, attribute_index);
				raw::VertexArrayAttribBinding(self.handle, attribute_index, binding);
				raw::VertexArrayAttribFormat(self.handle, attribute_index, num_elements as i32, gl_type, raw::FALSE, offset_bytes);
			}
		}

		unsafe {
			raw::VertexArrayVertexBuffer(self.handle, binding, vertex_buffer.handle, 0, stride);
		}
	}
}
