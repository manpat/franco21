use crate::prelude::*;

use model::camera::ControlMode;

const ORBIT_CAMERA_PITCH_LIMIT: (f32, f32) = (-PI/2.0, -PI/64.0);
const DEBUG_CAMERA_PITCH_LIMIT: (f32, f32) = (-PI/2.0, PI/2.0);

toybox::declare_input_context! {
	struct OrbitCameraActions "Orbit Camera Control" {
	}
}

toybox::declare_input_context! {
	struct ActiveOrbitCameraActions "ActiveOrbit Camera Control" {
		mouse mouse { "Mouse" [1.0] }
	}
}

toybox::declare_input_context! {
	struct DebugCameraActions "Debug Camera Control" {
		priority [20]

		state forward { "Forward" [Scancode::W] }
		state back { "Back" [Scancode::S] }
		state left { "Left" [Scancode::A] }
		state right { "Right" [Scancode::D] }
		state shift { "Sprint" [Scancode::LShift] }
		mouse mouse { "Mouse" [1.0] }
	}
}

pub struct CameraController {
	orbit_actions: OrbitCameraActions,
	active_orbit_actions: ActiveOrbitCameraActions,
	debug_actions: DebugCameraActions,

	prev_mode: ControlMode,
}


impl CameraController {
	pub fn new(engine: &mut toybox::Engine) -> Self {
		CameraController {
			orbit_actions: OrbitCameraActions::new_active(&mut engine.input),
			active_orbit_actions: ActiveOrbitCameraActions::new(&mut engine.input),
			debug_actions: DebugCameraActions::new(&mut engine.input),

			prev_mode: ControlMode::OrbitPlayer,
		}
	}

	pub fn update(&mut self, engine: &mut toybox::Engine, model: &mut model::Model) {
		let camera = &mut model.camera;

		if camera.control_mode != self.prev_mode {
			engine.input.leave_context(self.orbit_actions.context_id());
			engine.input.leave_context(self.debug_actions.context_id());

			self.prev_mode = camera.control_mode;

			match camera.control_mode {
				ControlMode::OrbitPlayer => engine.input.enter_context(self.orbit_actions.context_id()),
				ControlMode::FreeFly => engine.input.enter_context(self.debug_actions.context_id()),
			}
		}

		match camera.control_mode {
			ControlMode::OrbitPlayer => self.update_orbit(camera, &mut engine.input, model.ui.dragging_unclaimed_area),
			ControlMode::FreeFly => self.update_debug(camera, engine.input.frame_state()),
		}
	}


	fn update_orbit(&mut self, camera: &mut model::Camera, input: &mut toybox::input::InputSystem, dragging: bool) {
		if dragging && !input.is_context_active(self.active_orbit_actions.context_id()) {
			input.enter_context(self.active_orbit_actions.context_id());
		} else if !dragging && input.is_context_active(self.active_orbit_actions.context_id()) {
			input.leave_context(self.active_orbit_actions.context_id());
		}

		let input_state = input.frame_state();
		if let Some(mouse) = input_state.mouse(self.active_orbit_actions.mouse) {
			let (pitch_min, pitch_max) = ORBIT_CAMERA_PITCH_LIMIT;

			camera.yaw -= mouse.x * 0.5;
			camera.pitch = (camera.pitch + mouse.y as f32 * 0.5).clamp(pitch_min, pitch_max);
		}

		let camera_orientation = Quat::from_yaw(camera.yaw) * Quat::from_pitch(camera.pitch);
		camera.position = Vec3::from_y(1.0) - camera_orientation.forward() * camera.orbit_zoom;
	}


	fn update_debug(&mut self, camera: &mut model::Camera, input: &toybox::input::FrameState) {
		if let Some(mouse) = input.mouse(self.debug_actions.mouse) {
			let (pitch_min, pitch_max) = DEBUG_CAMERA_PITCH_LIMIT;

			camera.yaw -= mouse.x * 0.5;
			camera.pitch = (camera.pitch + mouse.y as f32 * 0.5).clamp(pitch_min, pitch_max);
		}

		let camera_orientation = Quat::from_yaw(camera.yaw) * Quat::from_pitch(camera.pitch);
		let mut move_direction = Vec3::zero();

		if input.active(self.debug_actions.forward) { move_direction += camera_orientation.forward() }
		if input.active(self.debug_actions.back) { move_direction -= camera_orientation.forward() }
		if input.active(self.debug_actions.left) { move_direction -= camera_orientation.right() }
		if input.active(self.debug_actions.right) { move_direction += camera_orientation.right() }

		let move_speed = match input.active(self.debug_actions.shift) {
			true => 50.0,
			false => 10.0,
		};

		camera.position += move_speed * move_direction / 60.0;
	}
}

