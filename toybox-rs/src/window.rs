use crate::gfx;
use std::error::Error;


pub fn init_window(sdl_video: &sdl2::VideoSubsystem, window_name: &str) -> Result<(sdl2::video::Window, gfx::Context), Box<dyn Error>> {
	let gl_attr = sdl_video.gl_attr();
	gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
	gl_attr.set_context_version(4, 5);
	gl_attr.set_context_flags().debug().set();
	
	gl_attr.set_framebuffer_srgb_compatible(true);
	gl_attr.set_stencil_size(8);

	let window = sdl_video.window(window_name, 1366, 768)
		.position_centered()
		.resizable()
		.opengl()
		.build()?;

	let gl_ctx = window.gl_create_context()?;
	window.gl_make_current(&gl_ctx)?;

	gfx::raw::load_with(|s| sdl_video.gl_get_proc_address(s) as *const _);

	Ok((window, gfx::Context::new(gl_ctx)))
}


