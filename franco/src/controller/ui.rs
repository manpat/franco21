use crate::prelude::*;



toybox::declare_input_context! {
	struct UiActions "Ui" {
		state left_mouse { "Interact" [MouseButton::Left] }
		pointer mouse { "Mouse" }

		state wheel_left { "Wheel Left" [Scancode::A] }
		state wheel_right { "Wheel Right" [Scancode::D] }
	}
}

toybox::declare_input_context! {
	struct WheelActions "Wheel" {
		mouse mouse { "Mouse" [1.0] }
	}
}


pub struct UiController {
	actions: UiActions,
	wheel_actions: WheelActions,
	dragging_wheel: bool,
}

impl UiController {
	pub fn new(engine: &mut toybox::Engine) -> UiController {
		UiController {
			actions: UiActions::new_active(&mut engine.input),
			wheel_actions: WheelActions::new(&mut engine.input),
			dragging_wheel: false,
		}
	}

	pub fn update(&mut self, engine: &mut toybox::Engine, model: &mut model::Model) {
		model.ui.aspect = engine.gfx.aspect();

		let input = engine.input.frame_state();

		if input.left(self.actions.left_mouse) {
			self.process_mouse_up(engine, model);

		} else if self.dragging_wheel {
			if let Some(mouse_delta) = input.mouse(self.wheel_actions.mouse) {
				self.process_drag_wheel(model, mouse_delta);
			}
		} else if input.active(self.actions.wheel_left) {
			model.ui.wheel.angle += (PI - model.ui.wheel.angle) / 10.0;
		} else if input.active(self.actions.wheel_right) {
			model.ui.wheel.angle += (-PI - model.ui.wheel.angle) / 10.0;
		} else {
			// Slowly shift wheel back to zero
			let angle = &mut model.ui.wheel.angle;
			*angle -= *angle / angle.abs().max(1.0) / 60.0;
		}

		let input = engine.input.frame_state();
		if let Some(mouse_pos) = input.mouse(self.actions.mouse) {
			let mouse_pos = mouse_pos * model::UI_SAFE_REGION;
			
			if input.entered(self.actions.left_mouse) {
				self.process_mouse_down(engine, model, mouse_pos);
			} else {
				self.process_hover(model, mouse_pos);
			}
		}

		model.ui.map_button.state.update();

		model.ui.map.state.update();
		model.ui.wheel.state.update();
	}

	pub fn process_mouse_down(&mut self, engine: &mut toybox::Engine, model: &mut model::Model, mouse_pos: Vec2) {
		let map_button_pos = model.ui.map_button.position;

		if map_button_pos.distance_to(mouse_pos, model.ui.aspect) < 2.0 {
			let map_state = &mut model.ui.map.state;
			if map_state.is_open() {
				map_state.close(0.3);
			} else {
				map_state.open(0.4);
			}

			return;
		}

		let wheel_pos = model.ui.wheel.position();
		if wheel_pos.distance_to(mouse_pos, model.ui.aspect) < 4.0 {
			self.dragging_wheel = true;
			engine.input.enter_context(self.wheel_actions.context_id());
			return;
		}

		model.ui.dragging_unclaimed_area = true;
	}

	pub fn process_drag_wheel(&mut self, model: &mut model::Model, mouse_delta: Vec2) {
		model.ui.wheel.angle += -mouse_delta.x;
		model.ui.wheel.angle = model.ui.wheel.angle.clamp(-PI, PI);
	}

	pub fn process_mouse_up(&mut self, engine: &mut toybox::Engine, model: &mut model::Model) {
		if self.dragging_wheel {
			engine.input.leave_context(self.wheel_actions.context_id());
		}

		self.dragging_wheel = false;
		model.ui.dragging_unclaimed_area = false;
	}

	pub fn process_hover(&mut self, model: &mut model::Model, mouse_pos: Vec2) {
		let wheel_pos = model.ui.wheel.position();
		let wheel_state = &mut model.ui.wheel.state;

		if wheel_pos.distance_to(mouse_pos, model.ui.aspect) < 4.0 {
			wheel_state.open(0.2);
		} else {
			wheel_state.close(1.0);
		}

		let map_button_pos = model.ui.map_button.position;
		let map_button_state = &mut model.ui.map_button.state;

		if map_button_pos.distance_to(mouse_pos, model.ui.aspect) < 2.0 {
			map_button_state.open(0.1);
		} else {
			map_button_state.close(0.1);
		}
	}
}