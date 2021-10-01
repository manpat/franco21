use crate::prelude::*;

use model::camera::ControlMode;

const ORBIT_CAMERA_PITCH_LIMIT: (f32, f32) = (-PI/2.0, -PI/64.0);
const DEBUG_CAMERA_PITCH_LIMIT: (f32, f32) = (-PI/2.0, PI/2.0);

toybox::declare_input_context! {
	struct OrbitCameraActions "Orbit Camera Control" {
		trigger zoom_out { "Zoom Out" [Scancode::Minus] }
		trigger zoom_in { "Zoom In" [Scancode::Equals] }
		mouse mouse { "Mouse" [1.0] }
	}
}

toybox::declare_input_context! {
	struct DebugCameraActions "Debug Camera Control" {
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
	debug_actions: DebugCameraActions,
	zoom: f32,

	prev_mode: ControlMode,
}


impl CameraController {
	pub fn new(engine: &mut toybox::Engine) -> Self {
		CameraController {
			orbit_actions: OrbitCameraActions::new_active(&mut engine.input),
			debug_actions: DebugCameraActions::new(&mut engine.input),
			zoom: 20.0,

			prev_mode: ControlMode::OrbitPlayer,
		}
	}

	pub fn update(&mut self, engine: &mut toybox::Engine, camera: &mut model::Camera) {
		if camera.control_mode != self.prev_mode {
			engine.input.leave_context(self.orbit_actions.context_id());
			engine.input.leave_context(self.debug_actions.context_id());

			self.prev_mode = camera.control_mode;

			match camera.control_mode {
				ControlMode::OrbitPlayer => engine.input.enter_context(self.orbit_actions.context_id()),
				ControlMode::FreeFly => engine.input.enter_context(self.debug_actions.context_id()),
			}
		}

		let frame_state = engine.input.frame_state();

		match camera.control_mode {
			ControlMode::OrbitPlayer => self.update_orbit(camera, frame_state),
			ControlMode::FreeFly => self.update_debug(camera, frame_state),
		}
	}


	fn update_orbit(&mut self, camera: &mut model::Camera, input: &toybox::input::FrameState) {
		if let Some(mouse) = input.mouse(self.orbit_actions.mouse) {
			let (pitch_min, pitch_max) = ORBIT_CAMERA_PITCH_LIMIT;

			camera.yaw -= mouse.x * 0.5;
			camera.pitch = (camera.pitch + mouse.y as f32 * 0.5).clamp(pitch_min, pitch_max);
		}

		if input.active(self.orbit_actions.zoom_out) {
			self.zoom *= 1.2;
		} else if input.active(self.orbit_actions.zoom_in) {
			self.zoom /= 1.2;
		}

		let camera_orientation = Quat::from_yaw(camera.yaw) * Quat::from_pitch(camera.pitch);

		camera.position = Vec3::from_y(1.0) - camera_orientation.forward() * self.zoom;
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

