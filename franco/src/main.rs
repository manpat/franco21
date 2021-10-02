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
	let mut ui_ctl = controller::UiController::new(&mut engine);

	let mut model = model::Model::new()?;

	let mut boat_view = view::BoatView::new(&mut engine.gfx, &model.resources)?;
	let mut water_view = view::WaterView::new(&mut engine.gfx, &model.resources)?;
	let mut island_view = view::IslandView::new(&mut engine.gfx, &model.resources)?;
	let mut ui_view = view::UiView::new(&mut engine.gfx, &model.resources)?;

	'main: loop {
		engine.process_events();
		if engine.should_quit() || model.global.wants_hard_quit {
			break 'main
		}

		debug_ctl.update(&mut engine, &mut model);
		global_ctl.update(&mut engine, &mut model.global);
		camera_ctl.update(&mut engine, &mut model.camera);
		player_ctl.update(&mut engine, &mut model);
		ui_ctl.update(&mut engine, &mut model);

		boat_view.update(&model);
		water_view.update(&model);
		island_view.update(&model);
		ui_view.update(&model);


		{
			let mut mb = debug::mesh_builder();

			let player_pos_map = model.player.map_position;

			// mb.set_color(Color::rgb(1.0, 1.0, 0.5));

			// let txform = Mat3x4::scale_translate(
			// 	Vec3{y: 0.1, .. model.world.map.size.to_x0z() * model::MAP_SCALE},
			// 	model::map_to_world(-player_pos_map).to_x0z()
			// );

			// mb.build(gfx::geom::Cuboid::from_matrix(txform));


			mb.set_color(Color::rgb(1.0, 0.0, 0.0));
			for object in model.world.map.objects.iter() {
				let txform = Mat3x4::scale_translate(
					Vec3::splat(4.0),
					model::map_to_world(object.map_position - player_pos_map).to_x0z() + Vec3::from_y(8.0)
				);

				mb.build(gfx::geom::Cuboid::from_matrix(txform));
			}
		}
 

		let camera_uniforms = build_camera_uniforms(&model.camera, engine.gfx.aspect());
		main_camera_ubo.upload(&[camera_uniforms]);

		let camera_uniforms = build_ui_camera_uniforms(engine.gfx.aspect());
		ui_camera_ubo.upload(&[camera_uniforms]);

		let world_uniforms = build_world_uniforms(&model);
		main_world_ubo.upload(&[world_uniforms]);


		let mut view_ctx = view::ViewContext::new(engine.gfx.render_state());

		view_ctx.gfx.set_clear_color(Color::hsv(220.0, 0.5, 0.9));
		view_ctx.gfx.clear(gfx::ClearMode::ALL);

		view_ctx.gfx.bind_uniform_buffer(0, main_camera_ubo);
		view_ctx.gfx.bind_uniform_buffer(1, main_world_ubo);

		boat_view.draw(&mut view_ctx);
		island_view.draw(&mut view_ctx);
		water_view.draw(&mut view_ctx);

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
			Mat4::ortho_aspect(model::UI_SAFE_REGION, aspect, 0.0, 10.0)
			// Mat4::perspective(PI/3.0, aspect, 1.0, 20.0)
			// 	* Mat4::scale_translate(Vec3::splat(0.5 / model::UI_SAFE_REGION), Vec3::from_z(-1.0))
		}
	}
}



#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct WorldUniforms {
	player_position: Vec2,
	// _pad: [f32; 2],
	// NOTE: align to Vec4s
}

fn build_world_uniforms(model: &model::Model) -> WorldUniforms {
	WorldUniforms {
		player_position: model.player.map_position,
	}
}