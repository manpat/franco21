use std::cmp::*;

pub struct Ordified<T>(pub T) where T: PartialOrd + PartialEq;

pub fn ordify<T: PartialOrd+PartialEq+Clone>(a: &T) -> Ordified<T> { Ordified(a.clone()) }

impl<T: PartialOrd + PartialEq> PartialOrd for Ordified<T> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.0.partial_cmp(&other.0)
	}
}

impl<T: PartialOrd + PartialEq> PartialEq for Ordified<T> {
	fn eq(&self, other: &Self) -> bool {
		self.0.eq(&other.0)
	}
}

impl<T: PartialOrd + PartialEq> Eq for Ordified<T> {}

impl<T: PartialOrd + PartialEq> Ord for Ordified<T> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
	}
}

pub trait Ordifiable: PartialOrd + PartialEq + Clone {
	fn ordify(&self) -> Ordified<Self>;
}

impl<T> Ordifiable for T where T: PartialOrd + PartialEq + Clone {
	fn ordify(&self) -> Ordified<Self> { Ordified(self.clone()) }
}
