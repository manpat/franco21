use crate::gfx;
use std::error::Error;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub struct Shader (pub(super) u32);

pub(super) struct ShaderManager {
	imports: HashMap<String, String>,
	cache: RefCell<HashMap<u64, Shader>>,
}


impl ShaderManager {
	pub fn new() -> Self {
		ShaderManager {
			imports: HashMap::new(),
			cache: HashMap::new().into(),
		}
	}

	pub fn add_import(&mut self, name: impl Into<String>, src: impl Into<String>) {
		let existing_import_src = self.imports.insert(name.into(), src.into());
		assert!(existing_import_src.is_none());
	}

	pub fn get_shader(&self, shaders: &[(u32, &str)]) -> Result<Shader, CompilationError> {
		use std::collections::hash_map::*;
		use std::hash::Hasher;

		let mut hasher = DefaultHasher::new();
		for &(ty, contents) in shaders {
			hasher.write_u32(ty);
			hasher.write(contents.as_bytes());
		}

		let hash = hasher.finish();
		let mut cache = self.cache.borrow_mut();

		match cache.entry(hash) {
			Entry::Occupied(entry) => Ok(*entry.get()),
			Entry::Vacant(entry) => {
				let shader = compile_shaders(shaders, &self.imports)?;
				Ok(*entry.insert(shader))
			}
		}
	}
}



fn compile_shaders(shaders: &[(u32, &str)], imports: &HashMap<String, String>) -> Result<Shader, CompilationError> {
	use std::ffi::CString;
	use std::str;

	unsafe {
		let program_handle = gfx::raw::CreateProgram();

		for &(ty, src) in shaders.iter() {
			let src = resolve_imports(&src, imports);
			let src = CString::new(src.as_bytes()).unwrap();

			let shader_handle = gfx::raw::CreateShader(ty);

			gfx::raw::ShaderSource(shader_handle, 1, &src.as_ptr(), std::ptr::null());
			gfx::raw::CompileShader(shader_handle);

			let mut status = 0;
			gfx::raw::GetShaderiv(shader_handle, gfx::raw::COMPILE_STATUS, &mut status);

			if status == 0 {
				let mut length = 0;
				gfx::raw::GetShaderiv(shader_handle, gfx::raw::INFO_LOG_LENGTH, &mut length);

				let mut buffer = vec![0u8; length as usize];
				gfx::raw::GetShaderInfoLog(
					shader_handle,
					length,
					std::ptr::null_mut(),
					buffer.as_mut_ptr() as *mut _
				);

				let error = str::from_utf8(&buffer[..buffer.len()-1])
					.map_err(|_| CompilationError::new("shader compilation", "error message invalid utf-8"))?;

				return Err(CompilationError::new("shader compilation", error));
			}

			gfx::raw::AttachShader(program_handle, shader_handle);
			gfx::raw::DeleteShader(shader_handle);
		}

		gfx::raw::LinkProgram(program_handle);

		let mut status = 0;
		gfx::raw::GetProgramiv(program_handle, gfx::raw::LINK_STATUS, &mut status);

		if status == 0 {
			let mut buf = [0u8; 1024];
			let mut len = 0;
			gfx::raw::GetProgramInfoLog(program_handle, buf.len() as _, &mut len, buf.as_mut_ptr() as _);

			let error = str::from_utf8(&buf[..len as usize])
				.map_err(|_| CompilationError::new("shader linking", "error message invalid utf-8"))?;

			return Err(CompilationError::new("shader link", error));
		}

		Ok(Shader(program_handle))
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



#[derive(Debug)]
pub struct CompilationError {
	what: String,
	description: String,
	backtrace: std::backtrace::Backtrace,
}

impl CompilationError {
	fn new(what: &str, description: &str) -> CompilationError {
		CompilationError {
			what: what.into(),
			description: description.into(),
			backtrace: std::backtrace::Backtrace::capture(),
		}
	}
}

impl std::fmt::Display for CompilationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} failed\n", self.what)?;
        write!(f, "{}\n", self.description)
    }
}


impl Error for CompilationError {
	fn backtrace(&self) -> Option<&'_ std::backtrace::Backtrace> {
		Some(&self.backtrace)
	}
}