#![feature(const_fn)]
#![feature(box_syntax)]
#![feature(ord_max_min)]
#![feature(slice_patterns)]
#![feature(specialization)]
#![feature(inclusive_range_syntax)]
#![feature(generators, generator_trait)]

pub extern crate rand;

pub mod mut_rc; 
pub mod easing; 
pub mod math;
pub mod coro;

pub use mut_rc::*;
pub use easing::*;
pub use math::*;
pub use coro::*;

#[macro_export]
macro_rules! match_pattern {
	($v:expr, $p:pat) => {
		match $v {
			$p => true,
			_ => false,
		}
	}
}
