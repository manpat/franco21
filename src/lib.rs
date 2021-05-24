#![feature(generators, generator_trait)]
#![feature(const_fn_floating_point_arithmetic)]

pub mod ordified;
pub mod mut_rc; 
pub mod easing; 
pub mod math;
pub mod coro;

pub use rand;

pub use ordified::*;
pub use mut_rc::*;
pub use easing::*;
pub use math::*;
pub use coro::*;

#[macro_export]
#[deprecated(note="use matches! instead")]
macro_rules! match_pattern {
	($v:expr, $p:pat) => {
		match $v {
			$p => true,
			_ => false,
		}
	}
}
