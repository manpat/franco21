pub use std::f32::consts::PI;
pub use std::f32::consts::TAU;
pub use std::f32::consts::SQRT_2;
pub use std::f32::consts::FRAC_1_SQRT_2 as INV_SQRT_2;

pub mod aabb;
pub mod color;
pub mod plane;
pub mod vector;
pub mod matrix;
pub mod quaternion;

pub use aabb::*;
pub use plane::*;
pub use color::*;
pub use vector::*;
pub use matrix::*;
pub use quaternion::*;



#[cfg(test)]
#[macro_export]
macro_rules! assert_vec_eq {
	($a:expr, $b:expr) => {{
		let diff = $a - $b;
		assert!(diff.length() < 0.001, "{:?} != {:?}", $a, $b);
	}};

	($a:expr, $b:expr, $msg:expr) => {{
		let diff = $a - $b;
		assert!(diff.length() < 0.001, $msg);
	}}
}


#[cfg(test)]
#[macro_export]
macro_rules! assert_almost_eq {
	($a:expr, $b:expr) => {{
		let diff = $a - $b;
		assert!(diff.abs() < 0.001, "{:?} != {:?}", $a, $b);
	}};

	($a:expr, $b:expr, $msg:expr) => {{
		let diff = $a - $b;
		assert!(diff.abs() < 0.001, $msg);
	}}
}
