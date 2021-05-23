use crate::gl;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub struct Shader (pub(super) u32);


pub(super) fn compile_shaders(shaders: &[(u32, &str)], imports: &HashMap<String, String>) -> Shader {
	use std::ffi::CString;
	use std::str;

	unsafe {
		let program_handle = gl::raw::CreateProgram();

		for &(ty, src) in shaders {
			let src = resolve_imports(&src, imports);
			let src = CString::new(src.as_bytes()).unwrap();

			let shader_handle = gl::raw::CreateShader(ty);

			gl::raw::ShaderSource(shader_handle, 1, &src.as_ptr(), std::ptr::null());
			gl::raw::CompileShader(shader_handle);

			let mut status = 0;
			gl::raw::GetShaderiv(shader_handle, gl::raw::COMPILE_STATUS, &mut status);

			if status == 0 {
				let mut length = 0;
				gl::raw::GetShaderiv(shader_handle, gl::raw::INFO_LOG_LENGTH, &mut length);

				let mut buffer = vec![0u8; length as usize];
				gl::raw::GetShaderInfoLog(
					shader_handle,
					length,
					std::ptr::null_mut(),
					buffer.as_mut_ptr() as *mut _
				);

				let error = str::from_utf8(&buffer[..buffer.len()-1]).unwrap();

				panic!("Shader compile failed!\n{}", error);
			}

			gl::raw::AttachShader(program_handle, shader_handle);
			gl::raw::DeleteShader(shader_handle);
		}

		gl::raw::LinkProgram(program_handle);

		let mut status = 0;
		gl::raw::GetProgramiv(program_handle, gl::raw::LINK_STATUS, &mut status);

		if status == 0 {
			let mut buf = [0u8; 1024];
			let mut len = 0;
			gl::raw::GetProgramInfoLog(program_handle, buf.len() as _, &mut len, buf.as_mut_ptr() as _);

			panic!("shader link failed: {}", std::str::from_utf8(&buf[..len as usize]).unwrap());
		}

		Shader(program_handle)
	}
}




fn resolve_imports(mut src: &str, imports: &HashMap<String, String>) -> String {
	let search_pattern = "#import";
	let mut result = String::with_capacity(src.len());

	while !src.is_empty() {
		let (prefix, suffix) = match src.split_once(search_pattern) {
			Some(pair) => pair,
			None => {
				result.push_str(src);
				break
			}
		};

		let (import_name, suffix) = suffix.split_once('\n')
			.expect("Expected '#common <name>'");
		src = suffix;

		let import_name = import_name.trim();
		let import_str = imports.get(import_name)
			.expect("Unknown import");

		result.push_str(prefix);
		result.push_str(import_str);
	}

	result
}