

#[derive(Debug)]
pub struct Global {
	pub wants_hard_quit: bool
}

impl Global {
	pub fn new() -> Global {
		Global {
			wants_hard_quit: false,
		}
	}
}