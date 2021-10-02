use crate::prelude::*;

pub mod wheel;
pub use wheel::*;

pub const UI_SAFE_REGION: f32 = 10.0;


#[derive(Debug)]
pub struct Ui {
	pub aspect: f32,
	pub buttons: Vec<Button>,
	pub wheel: Wheel,
}

impl Ui {
	pub fn new(resources: &model::Resources) -> Ui {
		Ui {
			aspect: 1.0,
			buttons: vec![
				Button {
					position: UiPosition::TopLeft(Vec2::new(2.0, 2.0)),
				},
				Button {
					position: UiPosition::Center(Vec2::zero()),
				},
			],

			wheel: Wheel::new(),
		}
	}
}




#[derive(Debug)]
pub struct Button {
	pub position: UiPosition,
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
}