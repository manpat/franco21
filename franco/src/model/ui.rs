use crate::prelude::*;

pub mod wheel;
pub use wheel::*;

pub mod map;
pub use map::*;

pub const UI_SAFE_REGION: f32 = 10.0;


#[derive(Debug)]
pub struct Ui {
	pub aspect: f32,
	pub map_button: Button,
	pub wheel: Wheel,
	pub map: Map,

	pub dragging_unclaimed_area: bool,
}

impl Ui {
	pub fn new(resources: &model::Resources) -> Ui {
		Ui {
			aspect: 1.0,
			map_button: Button {
				position: UiPosition::TopLeft(Vec2::splat(2.0)),
				state: UiPanelState::Closed,
			},

			wheel: Wheel::new(),
			map: Map::new(),

			dragging_unclaimed_area: false,
		}
	}
}




#[derive(Debug)]
pub struct Button {
	pub position: UiPosition,
	pub state: UiPanelState,
}


#[derive(Copy, Clone, Debug)]
pub enum UiPosition {
	TopLeft(Vec2),
	BottomLeft(Vec2),
	TopRight(Vec2),
	BottomRight(Vec2),

	Top(f32),
	Bottom(f32),

	Center(Vec2),
}

impl UiPosition {
	pub fn resolve(&self, aspect: f32) -> Vec2 {
		let screen_extents = if aspect < 1.0 {
			Vec2::new(UI_SAFE_REGION, UI_SAFE_REGION / aspect)
		} else {
			Vec2::new(UI_SAFE_REGION * aspect, UI_SAFE_REGION)
		};

		let corner_dir = match self {
			UiPosition::TopLeft(_) => Vec2::new(-1.0, 1.0),
			UiPosition::BottomLeft(_) => Vec2::new(-1.0,-1.0),
			UiPosition::TopRight(_) => Vec2::new(1.0, 1.0),
			UiPosition::BottomRight(_) => Vec2::new(1.0,-1.0),
			UiPosition::Top(_) => Vec2::new(0.0,1.0),
			UiPosition::Bottom(_) => Vec2::new(0.0,-1.0),
			_ => Vec2::zero(),
		};

		match *self {
			UiPosition::TopLeft(offset)
				| UiPosition::BottomLeft(offset)
				| UiPosition::TopRight(offset)
				| UiPosition::BottomRight(offset) => (screen_extents - offset) * corner_dir,

			UiPosition::Top(offset)
				| UiPosition::Bottom(offset) => (screen_extents - Vec2::splat(offset)) * corner_dir,

			UiPosition::Center(pos) => pos,
		}
	}

	pub fn diff_to(&self, other: Vec2, aspect: f32) -> Vec2 {
		other - self.resolve(aspect)
	}

	pub fn distance_to(&self, other: Vec2, aspect: f32) -> f32 {
		self.diff_to(other, aspect).length()
	}
}



#[derive(Copy, Clone, Debug)]
pub enum UiPanelState {
	Closed,
	Open,

	Opening { phase: f32, rate: f32, },
	Closing { phase: f32, rate: f32, },
}

impl UiPanelState {
	pub fn update(&mut self) {
		*self = match *self {
			UiPanelState::Opening{phase, rate} => {
				let phase = phase + rate / 60.0;

				if phase >= 1.0 {
					UiPanelState::Open
				} else {
					UiPanelState::Opening {phase, rate}
				}
			}

			UiPanelState::Closing{phase, rate} => {
				let phase = phase - rate / 60.0;

				if phase < 0.0 {
					UiPanelState::Closed
				} else {
					UiPanelState::Closing {phase, rate}
				}
			}

			x => x
		}
	}

	pub fn open(&mut self, time: f32) {
		*self = match *self {
			UiPanelState::Opening{phase, ..} => UiPanelState::Opening {phase, rate: 1.0/time},
			UiPanelState::Closing{phase, ..} => UiPanelState::Opening {phase, rate: 1.0/time},
			UiPanelState::Closed => UiPanelState::Opening {phase: 0.0, rate: 1.0/time},
			UiPanelState::Open => UiPanelState::Open,
		}
	}

	pub fn close(&mut self, time: f32) {
		*self = match *self {
			UiPanelState::Opening{phase, ..} => UiPanelState::Closing {phase, rate: 1.0/time},
			UiPanelState::Closing{phase, ..} => UiPanelState::Closing {phase, rate: 1.0/time},
			UiPanelState::Open => UiPanelState::Closing {phase: 1.0, rate: 1.0/time},
			UiPanelState::Closed => UiPanelState::Closed,
		}
	}

	pub fn as_phase(&self) -> f32 {
		match *self {
			UiPanelState::Opening{phase, ..} => phase,
			UiPanelState::Closing{phase, ..} => phase,
			UiPanelState::Open => 1.0,
			UiPanelState::Closed => 0.0,
		}
	}

	pub fn is_open(&self) -> bool { 
		matches!(self, UiPanelState::Open | UiPanelState::Opening{..})
	}

	pub fn is_closed(&self) -> bool { 
		matches!(self, UiPanelState::Closed | UiPanelState::Closing{..})
	}
}