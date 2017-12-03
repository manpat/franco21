#![feature(const_fn)]
#![feature(ord_max_min)]
#![feature(slice_patterns)]
#![feature(inclusive_range_syntax)]

pub extern crate rand;

pub mod easing; 
pub mod math; 

pub use easing::*;
pub use math::*;

#[macro_export]
macro_rules! match_pattern {
	($v:expr, $p:pat) => {
		match $v {
			$p => true,
			_ => false,
		}
	}
}
