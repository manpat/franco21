use crate::prelude::*;

#[derive(Debug)]
pub struct Map {
	pub state: model::UiPanelState,
}

impl Map {
	pub fn new() -> Map {
		Map {
			state: model::UiPanelState::Closed,
		}
	}
}

