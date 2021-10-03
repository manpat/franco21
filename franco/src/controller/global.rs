use crate::prelude::*;

toybox::declare_input_context! {
	struct GlobalActions "Global" {
		trigger quit { "Quit" [Scancode::Escape] }
	}
}



pub struct GlobalController {
	actions: GlobalActions,
}

impl GlobalController {
	pub fn new(engine: &mut toybox::Engine) -> GlobalController {
		GlobalController {
			actions: GlobalActions::new_active(&mut engine.input),
		}
	}

	pub fn update(&mut self, engine: &mut toybox::Engine, model: &mut model::Model) {
		let frame_state = engine.input.frame_state();

		if frame_state.active(self.actions.quit) {
			model.global.wants_hard_quit = true;
		}

		model.global.game_state.update();

		if !model.global.game_state.has_ended() && model.world.friends.iter().all(|f| f.met_player) {
			model.global.game_state.notify_end_game();
		}
	}
}

