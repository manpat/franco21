use crate::prelude::*;
use std::cell::{RefCell, RefMut};


static mut DRAW_STATE: Option<RefCell<DebugState>> = None;
static mut MESH_DATA: RefCell<gfx::MeshData<gfx::ColorVertex>> = RefCell::new(gfx::MeshData {
	vertices: Vec::new(),
	indices: Vec::new()
});


pub fn init(gfx: &mut gfx::Context) {
	let shader = gfx.new_simple_shader(shaders::COLOR_3D_VERT, shaders::FLAT_COLOR_FRAG).unwrap();

	let state = DebugState {
		shader,
		mesh: gfx::Mesh::new(gfx),
	};
	
	unsafe {
		DRAW_STATE = Some(state.into());
	}
}


pub fn draw(gfx: &mut gfx::RenderState<'_>) {
	let mut state = unsafe {
		DRAW_STATE.as_ref()
			.expect("debug module not initialised!")
			.borrow_mut()
	};

	let mut mesh_data = unsafe { MESH_DATA.borrow_mut() };

	state.mesh.upload(&mesh_data);
	mesh_data.clear();

	gfx.bind_shader(state.shader);
	state.mesh.draw(gfx, gfx::DrawMode::Triangles);
}


pub fn mesh_builder() -> gfx::ColorMeshBuilder<RefMut<'static, gfx::MeshData<gfx::ColorVertex>>> {
	let mesh_data = unsafe { MESH_DATA.borrow_mut() };
	gfx::ColorMeshBuilder::new(mesh_data)
}



struct DebugState {
	shader: gfx::Shader,
	mesh: gfx::Mesh<gfx::ColorVertex>,
}
