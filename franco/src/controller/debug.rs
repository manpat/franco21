use crate::prelude::*;


toybox::declare_input_context! {
	struct Actions "Debug" {
		trigger toggle_active { "Toggle" [Scancode::Grave] }
		trigger toggle_flycam { "Toggle Fly Cam" [Scancode::V] }
		trigger toggle_wireframe { "Toggle Wireframe" [Scancode::Z] }

		trigger dump_model { "Dump Model" [Scancode::F12] }
	}
}

toybox::declare_input_context! {
	struct ActiveActions "Active Debug" {
		state left_mouse { "Interact" [MouseButton::Left] }
		pointer mouse { "Mouse" }
	}
}


pub struct DebugController {
	actions: Actions,
	active_actions: ActiveActions,

	wireframe_enabled: bool,
}

impl DebugController {
	pub fn new(engine: &mut toybox::Engine) -> DebugController {
		DebugController {
			actions: Actions::new_active(&mut engine.input),
			active_actions: ActiveActions::new(&mut engine.input),

			wireframe_enabled: false,
		}
	}

	pub fn update(&mut self, engine: &mut toybox::Engine, model: &mut model::Model) {
		let currently_active = engine.input.is_context_active(self.active_actions.context_id());

		if engine.input.frame_state().active(self.actions.toggle_active) {
			if currently_active {
				engine.input.leave_context(self.active_actions.context_id());
			} else {
				engine.input.enter_context(self.active_actions.context_id());
			}

			// debug_model.active = !currently_active;
		}

		let input_state = engine.input.frame_state();

		// if let Some(pos) = input_state.mouse(self.active_actions.mouse) {
		// 	debug_model.mouse_pos = pos;
		// }

		if input_state.active(self.actions.toggle_flycam) {
			use model::camera::ControlMode;

			model.camera.control_mode = match model.camera.control_mode {
				ControlMode::OrbitPlayer => ControlMode::FreeFly,
				ControlMode::FreeFly => ControlMode::OrbitPlayer,
			};
		}

		if input_state.active(self.actions.dump_model) {
			println!("{:#?}", model.global);
			println!("{:#?}", model.camera);
			println!("{:#?}", model.world);
			println!("{:#?}", model.player);
		}

		if input_state.active(self.actions.toggle_wireframe) {
			self.wireframe_enabled = !self.wireframe_enabled;
			engine.gfx.render_state().set_wireframe(self.wireframe_enabled);
		}
	}
}