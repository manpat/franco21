use crate::gfx::*;

pub struct Context {
	_sdl_ctx: sdl2::video::GLContext,
	shader_manager: ShaderManager,
	capabilities: Capabilities,
	backbuffer_size: Vec2i,

	resources: Resources,
}

impl Context {
	pub fn new(sdl_ctx: sdl2::video::GLContext) -> Self {
		unsafe {
			raw::DebugMessageCallback(Some(gl_message_callback), std::ptr::null());
			raw::Enable(raw::DEBUG_OUTPUT_SYNCHRONOUS);
			raw::Enable(raw::PROGRAM_POINT_SIZE);

			raw::Enable(raw::FRAMEBUFFER_SRGB);

			raw::Enable(raw::DEPTH_TEST);
			// raw::Enable(raw::BLEND);
			// raw::BlendFunc(raw::DST_COLOR, raw::ZERO);
			// raw::BlendEquation(raw::FUNC_ADD);

			// raw::Enable(raw::CULL_FACE);
			raw::FrontFace(raw::CCW);
			raw::CullFace(raw::BACK);

			// Disable performance messages
			raw::DebugMessageControl(
				raw::DONT_CARE,
				raw::DEBUG_TYPE_PERFORMANCE,
				raw::DONT_CARE,
				0, std::ptr::null(),
				0 // false
			);

			// Disable notification messages
			raw::DebugMessageControl(
				raw::DONT_CARE,
				raw::DONT_CARE,
				raw::DEBUG_SEVERITY_NOTIFICATION,
				0, std::ptr::null(),
				0 // false
			);
		}

		Context {
			_sdl_ctx: sdl_ctx,
			shader_manager: ShaderManager::new(),
			capabilities: Capabilities::new(),
			backbuffer_size: Vec2i::splat(1),

			resources: Resources::new(),
		}
	}

	pub(crate) fn on_resize(&mut self, w: u32, h: u32) {
		unsafe {
			raw::Viewport(0, 0, w as _, h as _);
		}

		self.backbuffer_size = Vec2i::new(w as _, h as _);
		self.resources.on_resize(self.backbuffer_size);
	}

	pub fn backbuffer_size(&self) -> Vec2i { self.backbuffer_size }
	pub fn aspect(&self) -> f32 {
		let Vec2{x, y} = self.backbuffer_size.to_vec2();
		x / y
	}

	pub fn capabilities(&self) -> &Capabilities { &self.capabilities }

	pub fn render_state(&mut self) -> RenderState<'_> {
		RenderState {
			resources: &self.resources,
			backbuffer_size: self.backbuffer_size,
		}
	}


	pub fn new_untyped_buffer(&mut self, usage: BufferUsage) -> UntypedBuffer {
		unsafe {
			let mut handle = 0;
			raw::CreateBuffers(1, &mut handle);
			UntypedBuffer {
				handle,
				size_bytes: 0,
				usage,
			}
		}
	}

	pub fn new_buffer<T: Copy>(&mut self, usage: BufferUsage) -> Buffer<T> {
		self.new_untyped_buffer(usage).into_typed()
	}

	pub fn new_texture(&mut self, size: impl Into<TextureSize>, format: TextureFormat) -> TextureKey {
		let texture = Texture::new(size.into(), self.backbuffer_size, format);
		self.resources.insert_texture(texture)
	}

	pub fn new_framebuffer(&mut self, settings: FramebufferSettings) -> FramebufferKey {
		let framebuffer = Framebuffer::new(settings, &mut self.resources, self.backbuffer_size);
		self.resources.insert_framebuffer(framebuffer)
	}

	pub fn new_vao(&mut self) -> Vao {
		unsafe {
			let mut vao = 0;
			raw::CreateVertexArrays(1, &mut vao);
			Vao::new(vao)
		}
	}

	pub fn new_query(&mut self) -> QueryObject {
		unsafe {
			let mut handle = 0;
			raw::GenQueries(1, &mut handle);
			QueryObject(handle)
		}
	}


	pub fn add_shader_import(&mut self, name: impl Into<String>, src: impl Into<String>) {
		self.shader_manager.add_import(name, src)
	}

	pub fn new_shader(&mut self, shaders: &[(u32, &str)]) -> Result<Shader, shader::CompilationError> {
		self.shader_manager.get_shader(shaders)
	}

	pub fn new_simple_shader(&mut self, vsrc: &str, fsrc: &str) -> Result<Shader, shader::CompilationError> {
		self.shader_manager.get_shader(&[
			(raw::VERTEX_SHADER, vsrc),
			(raw::FRAGMENT_SHADER, fsrc),
		])
	}

	pub fn new_compute_shader(&mut self, csrc: &str) -> Result<Shader, shader::CompilationError> {
		self.shader_manager.get_shader(&[
			(raw::COMPUTE_SHADER, csrc)
		])
	}
}



pub struct RenderState<'ctx> {
	resources: &'ctx Resources,
	backbuffer_size: Vec2i,
}

impl<'ctx> RenderState<'ctx> {
	pub fn set_wireframe(&mut self, wireframe_enabled: bool) {
		let mode = match wireframe_enabled {
			false => raw::FILL,
			true => raw::LINE,
		};

		unsafe {
			raw::PolygonMode(raw::FRONT_AND_BACK, mode);
		}
	}

	pub fn set_clear_color(&mut self, color: impl Into<Color>) {
		let (r,g,b,a) = color.into().to_tuple();
		unsafe {
			raw::ClearColor(r, g, b, a);
		}
	}

	pub fn clear(&mut self, mode: ClearMode) {
		unsafe {
			raw::Clear(mode.into_gl());
		}
	}

	pub fn resources(&self) -> &'ctx Resources {
		self.resources
	}

	pub fn get_framebuffer(&self, framebuffer: FramebufferKey) -> ResourceLock<Framebuffer> {
		self.resources.get(framebuffer)
	}

	pub fn get_texture(&self, texture: TextureKey) -> ResourceLock<Texture> {
		self.resources.get(texture)
	}

	pub fn bind_uniform_buffer(&mut self, binding: u32, buffer: impl Into<UntypedBuffer>) {
		let buffer = buffer.into();
		unsafe {
			raw::BindBufferBase(raw::UNIFORM_BUFFER, binding, buffer.handle);
		}
	}

	pub fn bind_shader_storage_buffer(&mut self, binding: u32, buffer: impl Into<UntypedBuffer>) {
		let buffer = buffer.into();
		unsafe {
			raw::BindBufferBase(raw::SHADER_STORAGE_BUFFER, binding, buffer.handle);
		}
	}

	pub fn bind_image_for_rw(&mut self, binding: u32, texture: TextureKey) {
		// https://www.khronos.org/opengl/wiki/Image_Load_Store#Images_in_the_context
		let (level, layered, layer) = (0, 0, 0);
		let texture = texture.get(self.resources);

		unsafe {
			raw::BindImageTexture(binding, texture.texture_handle, level, layered, layer,
				raw::READ_WRITE, texture.format().to_gl());
		}
	}

	pub fn bind_image_for_read(&mut self, binding: u32, texture: TextureKey) {
		let (level, layered, layer) = (0, 0, 0);
		let texture = texture.get(self.resources);

		unsafe {
			raw::BindImageTexture(binding, texture.texture_handle, level, layered, layer,
				raw::READ_ONLY, texture.format().to_gl());
		}
	}

	pub fn bind_image_for_write(&mut self, binding: u32, texture: TextureKey) {
		let (level, layered, layer) = (0, 0, 0);
		let texture = texture.get(self.resources);

		unsafe {
			raw::BindImageTexture(binding, texture.texture_handle, level, layered, layer,
				raw::WRITE_ONLY, texture.format().to_gl());
		}
	}

	pub fn bind_texture(&mut self, binding: u32, texture: TextureKey) {
		let texture = texture.get(self.resources);

		unsafe {
			raw::BindTextureUnit(binding, texture.texture_handle);
			raw::BindSampler(binding, texture.sampler_handle);
		}
	}

	pub fn bind_vao(&mut self, vao: Vao) {
		unsafe {
			raw::BindVertexArray(vao.handle);
		}
	}

	pub fn bind_shader(&mut self, shader: Shader) {
		unsafe {
			raw::UseProgram(shader.0);
		}
	}

	pub fn bind_framebuffer(&mut self, framebuffer: impl Into<Option<FramebufferKey>>) {
		if let Some(framebuffer) = framebuffer.into() {
			let framebuffer = framebuffer.get(self.resources);
			let Vec2i{x,y} = framebuffer.size_mode.resolve(self.backbuffer_size);

			unsafe {
				raw::Viewport(0, 0, x, y);
				raw::BindFramebuffer(raw::DRAW_FRAMEBUFFER, framebuffer.handle);
			}
		} else {
			let Vec2i{x,y} = self.backbuffer_size;

			unsafe {
				raw::Viewport(0, 0, x, y);
				raw::BindFramebuffer(raw::DRAW_FRAMEBUFFER, 0);
			}
		}
	}

	pub fn draw_indexed(&self, draw_mode: DrawMode, num_elements: u32) {
		if num_elements == 0 {
			return
		}

		unsafe {
			raw::DrawElements(draw_mode.into_gl(), num_elements as i32, raw::UNSIGNED_SHORT, std::ptr::null());
		}
	}

	pub fn draw_instances_indexed(&self, draw_mode: DrawMode, num_elements: u32, num_instances: u32) {
		if num_elements == 0 || num_instances == 0 {
			return
		}

		unsafe {
			raw::DrawElementsInstanced(draw_mode.into_gl(), num_elements as i32, raw::UNSIGNED_SHORT, std::ptr::null(), num_instances as i32);
		}
	}

	pub fn draw_arrays(&self, draw_mode: DrawMode, num_vertices: u32) {
		if num_vertices == 0 {
			return
		}

		unsafe {
			raw::DrawArrays(draw_mode.into_gl(), 0, num_vertices as i32);
		}
	}

	pub fn dispatch_compute(&self, x: u32, y: u32, z: u32) {
		// see: GL_MAX_COMPUTE_WORK_GROUP_COUNT
		assert!(x < 65535, "Work group exceeds guaranteed minimum size along x axis");
		assert!(y < 65535, "Work group exceeds guaranteed minimum size along y axis");
		assert!(z < 65535, "Work group exceeds guaranteed minimum size along z axis");

		unsafe {
			raw::DispatchCompute(x, y, z);
		}
	}
}





extern "system" fn gl_message_callback(source: u32, ty: u32, _id: u32, severity: u32,
	_length: i32, msg: *const i8, _ud: *mut std::ffi::c_void)
{
	let severity = match severity {
		raw::DEBUG_SEVERITY_LOW => "low",
		raw::DEBUG_SEVERITY_MEDIUM => "medium",
		raw::DEBUG_SEVERITY_HIGH => "high",
		raw::DEBUG_SEVERITY_NOTIFICATION => "notification",
		_ => panic!("Unknown severity {}", severity),
	};

	let ty = match ty {
		raw::DEBUG_TYPE_ERROR => "error",
		raw::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "deprecated behaviour",
		raw::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "undefined behaviour",
		raw::DEBUG_TYPE_PORTABILITY => "portability",
		raw::DEBUG_TYPE_PERFORMANCE => "performance",
		raw::DEBUG_TYPE_OTHER => "other",
		_ => panic!("Unknown type {}", ty),
	};

	let source = match source {
		raw::DEBUG_SOURCE_API => "api",
		raw::DEBUG_SOURCE_WINDOW_SYSTEM => "window system",
		raw::DEBUG_SOURCE_SHADER_COMPILER => "shader compiler",
		raw::DEBUG_SOURCE_THIRD_PARTY => "third party",
		raw::DEBUG_SOURCE_APPLICATION => "application",
		raw::DEBUG_SOURCE_OTHER => "other",
		_ => panic!("Unknown source {}", source),
	};

	eprintln!("GL ERROR!");
	eprintln!("Source:   {}", source);
	eprintln!("Severity: {}", severity);
	eprintln!("Type:     {}", ty);

	unsafe {
		let msg = std::ffi::CStr::from_ptr(msg as _).to_str().unwrap();
		eprintln!("Message: {}", msg);
	}

	panic!("GL ERROR!");
}