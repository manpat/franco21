use crate::prelude::*;


pub struct UiController {

}

impl UiController {
	pub fn new(_engine: &mut toybox::Engine) -> UiController {
		UiController {}
	}

	pub fn update(&mut self, engine: &mut toybox::Engine, model: &mut model::Model) {
		model.ui.aspect = engine.gfx.aspect();

		model.ui.wheel.angle += 1.0 / 60.0;
	}
}