use crate::prelude::*;

#[derive(Debug)]
pub struct Wheel {
	pub state: model::UiPanelState,
	pub angle: f32,
}

impl Wheel {
	pub fn new() -> Wheel {
		Wheel {
			state: model::UiPanelState::Closed,
			angle: 0.0,
		}
	}

	pub fn position(&self) -> model::UiPosition {
		let phase = self.state.as_phase();
		model::UiPosition::Bottom(phase*2.0)
	}
}