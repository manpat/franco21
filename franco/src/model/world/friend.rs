use crate::prelude::*;


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum FriendName {
	Dolphin,
	Fish,

	BoatBoy,
	BoatBoy2,
}

#[derive(Copy, Clone, Debug)]
pub enum FriendState {
	HangingOut,
	Following,
	DoingTricks(f32),
}


#[derive(Debug)]
pub struct Friend {
	pub name: FriendName,
	pub state: FriendState,

	pub map_position: Vec2,
	pub heading: f32,
	pub speed: f32,

	pub decision_timer: f32,
	pub met_player: bool,

	pub heading_wander: f32,
	pub bob_phase: f32,
}


impl FriendName {
	pub fn from_name(name: &str) -> FriendName {
		let name = name.trim_start_matches("FRIEND_");
		let (name, _) = name.split_once('.').unwrap_or((name, ""));

		match name {
			"dolphin" => FriendName::Dolphin,
			"fish" => FriendName::Fish,
			"boat_boy" => FriendName::BoatBoy,
			"boat_boy2" => FriendName::BoatBoy2,
			_ => panic!("Unknown friend name {}", name),
		}
	}

	pub fn is_fish(&self) -> bool {
		matches!(self, FriendName::Dolphin | FriendName::Fish)
	}
}


