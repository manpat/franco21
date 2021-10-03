pub mod prelude;
pub mod controller;
pub mod model;
pub mod view;
pub mod shaders;
pub mod debug;

use prelude::*;

fn main() -> Result<()> {
	std::env::set_var("RUST_BACKTRACE", "1");

	let mut engine = toybox::Engine::new("franco21")?;

	engine.gfx.add_shader_import("3d", shaders::THREE_D_COMMON);
	engine.gfx.add_shader_import("water", shaders::WATER_COMMON);

	let mut main_camera_ubo = engine.gfx.new_buffer(gfx::BufferUsage::Stream);
	let mut ui_camera_ubo = engine.gfx.new_buffer(gfx::BufferUsage::Stream);
	let mut main_world_ubo = engine.gfx.new_buffer(gfx::BufferUsage::Stream);

	debug::init(&mut engine.gfx);

	let mut debug_ctl = controller::DebugController::new(&mut engine);
	let mut global_ctl = controller::GlobalController::new(&mut engine);
	let mut camera_ctl = controller::CameraController::new(&mut engine);
	let mut player_ctl = controller::PlayerController::new(&mut engine);
	let mut friend_ctl = controller::FriendController::new(&mut engine);
	let mut ui_ctl = controller::UiController::new(&mut engine);

	let mut model = model::Model::new()?;

	let mut boat_view = view::BoatView::new(&mut engine.gfx, &model.resources)?;
	let mut water_view = view::WaterView::new(&mut engine.gfx, &model.resources)?;
	let mut island_view = view::IslandView::new(&mut engine.gfx, &model.resources)?;
	let mut friend_view = view::FriendView::new(&mut engine.gfx, &model.resources)?;
	let mut ui_view = view::UiView::new(&mut engine.gfx, &model.resources)?;

	let main_fbo = engine.gfx.new_framebuffer(
		gfx::FramebufferSettings::new(gfx::TextureSize::Backbuffer)
			.add_depth()
			.add_color(0, gfx::TextureFormat::color())
	);

	let friend_fbo = engine.gfx.new_framebuffer(
		gfx::FramebufferSettings::new(gfx::TextureSize::Backbuffer)
			.add_depth()
			.add_color(0, gfx::TextureFormat::color())
	);

	let composite_shader = engine.gfx.new_simple_shader(
		include_str!("shaders/fullscreen_quad.vert.glsl"),
		include_str!("shaders/final_composite.frag.glsl")
	)?;

	'main: loop {
		engine.process_events();
		if engine.should_quit() || model.global.wants_hard_quit {
			break 'main
		}

		debug_ctl.update(&mut engine, &mut model);
		global_ctl.update(&mut engine, &mut model);
		camera_ctl.update(&mut engine, &mut model);
		player_ctl.update(&mut model);
		friend_ctl.update(&mut model);
		ui_ctl.update(&mut engine, &mut model);

		boat_view.update(&model);
		water_view.update(&model);
		island_view.update(&model);
		friend_view.update(&model);
		ui_view.update(&model);
 

		let camera_uniforms = build_camera_uniforms(&model.camera, engine.gfx.aspect());
		main_camera_ubo.upload(&[camera_uniforms]);

		let camera_uniforms = build_ui_camera_uniforms(engine.gfx.aspect());
		ui_camera_ubo.upload(&[camera_uniforms]);

		let world_uniforms = build_world_uniforms(&model);
		main_world_ubo.upload(&[world_uniforms]);


		let mut view_ctx = view::ViewContext::new(engine.gfx.render_state());

		view_ctx.gfx.bind_uniform_buffer(0, main_camera_ubo);
		view_ctx.gfx.bind_uniform_buffer(1, main_world_ubo);

		view_ctx.gfx.set_wireframe(model.global.wireframe_enabled);

		view_ctx.gfx.bind_framebuffer(main_fbo);
		view_ctx.gfx.set_clear_color(Color{a: 0.0, ..model.world.sky_color});
		view_ctx.gfx.clear(gfx::ClearMode::ALL);

		boat_view.draw(&mut view_ctx);
		island_view.draw(&mut view_ctx);
		water_view.draw(&mut view_ctx);

		// Draw friends into separate fbo so we can draw them underwater
		view_ctx.gfx.bind_framebuffer(friend_fbo);
		view_ctx.gfx.set_clear_color(Color::grey_a(0.0, 0.0));
		view_ctx.gfx.clear(gfx::ClearMode::ALL);
		friend_view.draw(&mut view_ctx);

		view_ctx.gfx.bind_framebuffer(None);

		// Composite and draw world
		{
			view_ctx.gfx.set_wireframe(false);

			view_ctx.gfx.set_clear_color(model.world.sky_color);
			view_ctx.gfx.clear(gfx::ClearMode::ALL);

			let resources = view_ctx.resources;

			let color_0 = resources.get(main_fbo).color_attachment(0).unwrap();
			let depth_0 = resources.get(main_fbo).depth_stencil_attachment().unwrap();
			let color_1 = resources.get(friend_fbo).color_attachment(0).unwrap();
			let depth_1 = resources.get(friend_fbo).depth_stencil_attachment().unwrap();

			view_ctx.gfx.bind_texture(0, color_0);
			view_ctx.gfx.bind_texture(1, color_1);
			view_ctx.gfx.bind_texture(2, depth_0);
			view_ctx.gfx.bind_texture(3, depth_1);
			view_ctx.gfx.bind_shader(composite_shader);
			view_ctx.gfx.draw_arrays(gfx::DrawMode::Triangles, 6);
		}

		view_ctx.gfx.set_wireframe(model.global.wireframe_enabled);

		debug::draw(&mut view_ctx.gfx);

		view_ctx.gfx.clear(gfx::ClearMode::DEPTH);
		view_ctx.gfx.bind_uniform_buffer(0, ui_camera_ubo);

		ui_view.draw(&mut view_ctx);

		engine.end_frame();
	}

	Ok(())
}





#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct CameraUniforms {
	projection_view: Mat4,
	// NOTE: align to Vec4s
}

fn build_camera_uniforms(camera: &model::Camera, aspect: f32) -> CameraUniforms {
	CameraUniforms {
		projection_view: {
			let camera_orientation = Quat::from_pitch(-camera.pitch) * Quat::from_yaw(-camera.yaw);
			let camera_orientation = camera_orientation.to_mat4();

			Mat4::perspective(PI/3.0, aspect, 0.1, 1000.0)
				* camera_orientation
				* Mat4::translate(-camera.position)
		},
	}
}


fn build_ui_camera_uniforms(aspect: f32) -> CameraUniforms {
	CameraUniforms {
		projection_view: {
			Mat4::ortho_aspect(model::UI_SAFE_REGION, aspect, -10.0, 10.0)
			// Mat4::perspective(PI/3.0, aspect, 1.0, 20.0)
			// 	* Mat4::scale_translate(Vec3::splat(0.5 / model::UI_SAFE_REGION), Vec3::from_z(-1.0))
		}
	}
}



#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct WorldUniforms {
	sky_color: Color,
	water_obscure_color: Color,
	player_position: Vec2,
	fog_start: f32,
	fog_distance: f32,
	// _pad: [f32; 2],
	// NOTE: align to Vec4s
}

fn build_world_uniforms(model: &model::Model) -> WorldUniforms {
	WorldUniforms {
		sky_color: model.world.sky_color,
		water_obscure_color: Color::hsv(220.0, 0.6, 0.7),
		player_position: model.player.map_position,

		fog_start: 80.0,
		fog_distance: 200.0,
	}
}