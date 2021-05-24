use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

pub struct Coro<Y> {
	valid: bool,
	coro: Pin<Box<dyn Generator<Yield=Y, Return=()>>>,
}

impl<Y> Coro<Y> {
	pub fn is_valid(&self) -> bool { self.valid }
}

impl<Y, G> From<G> for Coro<Y> where G: 'static + Generator<Yield=Y, Return=()> {
	fn from(gen: G) -> Self {
		Coro {
			coro: Box::pin(gen),
			valid: true,
		}
	}
}

impl<Y> Iterator for Coro<Y> {
	type Item = Y;
	fn next(&mut self) -> Option<Self::Item> {
		if !self.valid { return None }

		if let GeneratorState::Yielded(yielded_value) = Pin::new(&mut self.coro).resume(()) {
			Some(yielded_value)
		} else {
			self.valid = false;
			None
		}
	}
}
