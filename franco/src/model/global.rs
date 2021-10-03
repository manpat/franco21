

#[derive(Debug)]
pub struct Global {
	pub wants_hard_quit: bool,
	pub wireframe_enabled: bool,

	pub game_state: GameState,
}

impl Global {
	pub fn new() -> Global {
		Global {
			wants_hard_quit: false,
			wireframe_enabled: false,
			game_state: GameState::PreGame(1.0),
		}
	}
}




#[derive(Copy, Clone, Debug)]
pub enum GameState {
	PreGame(f32),
	Starting(f32),
	Playing,
	GotFriend(f32),
	Ending(f32),
	PostGame,
}

impl GameState {
	pub fn update(&mut self) {
		use GameState::*;

		let dt = 1.0/60.0;

		*self = match *self {
			PreGame(timer) => {
				let new_timer = timer - dt;
				if new_timer < 0.0 {
					Starting(3.0)
				} else {
					PreGame(new_timer)
				}
			}

			Starting(timer) => {
				let new_timer = timer - dt;
				if new_timer < 0.0 {
					Playing
				} else {
					Starting(new_timer)
				}
			}

			GotFriend(timer) => {
				let new_timer = timer - dt;
				if new_timer < 0.0 {
					Playing
				} else {
					GotFriend(new_timer)
				}
			}

			Ending(timer) => {
				let new_timer = timer - dt;
				if new_timer < 0.0 {
					PostGame
				} else {
					Ending(new_timer)
				}
			}

			_ => return
		};
	}

	pub fn has_ended(&self) -> bool {
		matches!(self, GameState::Ending(_) | GameState::PostGame)
	}

	pub fn notify_end_game(&mut self) {
		*self = GameState::Ending(3.0);
	}

	pub fn notify_got_friend(&mut self) {
		*self = GameState::GotFriend(2.0);
	}
}