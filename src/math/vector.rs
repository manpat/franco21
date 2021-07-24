use std::ops::{Add, Sub, Mul, Div, Neg};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};
use std::iter::{Sum, Product};
use crate::easing::*;

pub mod vec2;
pub mod vec3;
pub mod vec4;
pub mod vec2i;
pub mod vec3i;

pub use vec2::*;
pub use vec3::*;
pub use vec4::*;
pub use vec2i::*;
pub use vec3i::*;


#[macro_export]
macro_rules! internal_vec_map {
	(@apply ($v:expr, $el:tt), @[$($body:tt)*] element $($tail:tt)* ) => {
		internal_vec_map!(@apply ($v, $el), @[$($body)* $v.$el] $($tail)*)
	};

	(@apply ($v:expr, $el:tt), @[$($body:tt)*] $name:ident.element $($tail:tt)* ) => {
		internal_vec_map!(@apply ($v, $el), @[$($body)* $name.$el] $($tail)*)
	};

	(@apply ($v:expr, $el:tt), @[$($body:tt)*] ( $($subexpr:tt)+ ) $($tail:tt)* ) => {
		internal_vec_map!(@apply ($v, $el), @[
			$($body)*
			( internal_vec_map!(@apply ($v, $el), @[] $($subexpr)+) )
		] $($tail)*)
	};

	(@apply ($v:expr, $el:tt), @[$($body:tt)*] { $($subexpr:tt)+ } $($tail:tt)* ) => {
		internal_vec_map!(@apply ($v, $el), @[
			$($body)*
			{ internal_vec_map!(@apply ($v, $el), @[] $($subexpr)+) }
		] $($tail)*)
	};

	(@apply ($v:expr, $el:tt), @[$($body:tt)*] $next:tt $($tail:tt)* ) => {
		internal_vec_map!(@apply ($v, $el), @[$($body)* $next] $($tail)*)
	};

	(@apply ($v:expr, $el:tt), @[$($body:tt)*]) => { $($body)* };

	(Vec2 $v:expr, $($func:tt)+) => {{
		let v = $v;
		Vec2 {
			x: internal_vec_map!(@apply (v, x), @[] $($func)+),
			y: internal_vec_map!(@apply (v, y), @[] $($func)+),
		}
	}};

	(Vec2i $v:expr, $($func:tt)+) => {{
		let v = $v;
		Vec2i {
			x: internal_vec_map!(@apply (v, x), @[] $($func)+),
			y: internal_vec_map!(@apply (v, y), @[] $($func)+),
		}
	}};

	(Vec3i $v:expr, $($func:tt)+) => {{
		let v = $v;
		Vec3i {
			x: internal_vec_map!(@apply (v, x), @[] $($func)+),
			y: internal_vec_map!(@apply (v, y), @[] $($func)+),
			z: internal_vec_map!(@apply (v, z), @[] $($func)+),
		}
	}};

	(Vec3 $v:expr, $($func:tt)+) => {{
		let v = $v;
		Vec3 {
			x: internal_vec_map!(@apply (v, x), @[] $($func)+),
			y: internal_vec_map!(@apply (v, y), @[] $($func)+),
			z: internal_vec_map!(@apply (v, z), @[] $($func)+),
		}
	}};

	(Vec4 $v:expr, $($func:tt)+) => {{
		let v = $v;
		Vec4 {
			x: internal_vec_map!(@apply (v, x), @[] $($func)+),
			y: internal_vec_map!(@apply (v, y), @[] $($func)+),
			z: internal_vec_map!(@apply (v, z), @[] $($func)+),
			w: internal_vec_map!(@apply (v, w), @[] $($func)+),
		}
	}};
}

#[macro_export]
macro_rules! vec2_map { ($($tt:tt)+) => { internal_vec_map!(Vec2 $($tt)+) } }

#[macro_export]
macro_rules! vec2i_map { ($($tt:tt)+) => { internal_vec_map!(Vec2i $($tt)+) } }

#[macro_export]
macro_rules! vec3i_map { ($($tt:tt)+) => { internal_vec_map!(Vec3i $($tt)+) } }

#[macro_export]
macro_rules! vec3_map { ($($tt:tt)+) => { internal_vec_map!(Vec3 $($tt)+) } }

#[macro_export]
macro_rules! vec4_map { ($($tt:tt)+) => { internal_vec_map!(Vec4 $($tt)+) } }

macro_rules! impl_vector_bin_op {
	($ty:ident, $trait:ident<$scalar:ty>, $fn:ident, $op:tt, $($els:ident),+) => {
		impl $trait for $ty {
			type Output = $ty;
			fn $fn(self, o: $ty) -> $ty {
				$ty::new($(self.$els $op o.$els),+)
			}
		}

		impl $trait<$scalar> for $ty {
			type Output = $ty;
			fn $fn(self, o: $scalar) -> $ty {
				$ty::new($(self.$els $op o),+)
			}
		}

		impl $trait<$ty> for $scalar {
			type Output = $ty;
			fn $fn(self, o: $ty) -> $ty {
				$ty::new($(self $op o.$els),+)
			}
		}
	};

	(ass $ty:ident, $trait:ident<$scalar:ty>, $fn:ident, $op:tt, $($els:ident),+) => {
		impl $trait for $ty {
			fn $fn(&mut self, o: $ty) {
				$(
					self.$els $op o.$els;
				)+
			}
		}

		impl $trait<$scalar> for $ty {
			fn $fn(&mut self, o: $scalar) {
				$(
					self.$els $op o;
				)+
			}
		}
	};
}

macro_rules! bulk_impl_vector_ops {
	($ty:ident, $scalar:ty, $($els:ident),+) => {
		impl_vector_bin_op!($ty, Add<$scalar>, add, +, $($els),+);
		impl_vector_bin_op!($ty, Sub<$scalar>, sub, -, $($els),+);
		impl_vector_bin_op!($ty, Mul<$scalar>, mul, *, $($els),+);
		impl_vector_bin_op!($ty, Div<$scalar>, div, /, $($els),+);

		impl_vector_bin_op!(ass $ty, AddAssign<$scalar>, add_assign, +=, $($els),+);
		impl_vector_bin_op!(ass $ty, SubAssign<$scalar>, sub_assign, -=, $($els),+);
		impl_vector_bin_op!(ass $ty, MulAssign<$scalar>, mul_assign, *=, $($els),+);
		impl_vector_bin_op!(ass $ty, DivAssign<$scalar>, div_assign, /=, $($els),+);

		impl Neg for $ty {
			type Output = $ty;
			fn neg(self) -> $ty {
				$ty::new($(-self.$els),+)
			}
		}

		impl Sum for $ty {
			fn sum<I>(iter: I) -> $ty where I: Iterator<Item=$ty> {
				iter.fold($ty::zero(), |a, v| a + v)
			}
		}
		
		impl<'a> Sum<&'a $ty> for $ty {
			fn sum<I>(iter: I) -> $ty where I: Iterator<Item=&'a $ty> {
				iter.fold($ty::zero(), |a, &v| a + v)
			}
		}

		impl Product for $ty {
			fn product<I>(iter: I) -> $ty where I: Iterator<Item=$ty> {
				iter.fold($ty::splat(1 as $scalar), |a, v| a * v)
			}
		}

		impl<'a> Product<&'a $ty> for $ty {
			fn product<I>(iter: I) -> $ty where I: Iterator<Item=&'a $ty> {
				iter.fold($ty::splat(1 as $scalar), |a, &v| a * v)
			}
		}
	};
}

bulk_impl_vector_ops!(Vec2, f32, x, y);
bulk_impl_vector_ops!(Vec3, f32, x, y, z);
bulk_impl_vector_ops!(Vec4, f32, x, y, z, w);
bulk_impl_vector_ops!(Vec2i, i32, x, y);
bulk_impl_vector_ops!(Vec3i, i32, x, y, z);

macro_rules! impl_ease_for_vec {
	(fn $func: ident, $ty:ident, $($els:ident),+) => (
		fn $func(&self, start: $ty, end: $ty) -> $ty {
			$ty {
				$($els: self.$func(start.$els, end.$els)),+
			}
		}
	);

	($ty:ident, $($els:ident),+) => {
		impl Ease<$ty> for f32 {
			impl_ease_for_vec!(fn ease_linear, $ty, $($els),+);

			impl_ease_for_vec!(fn ease_quad_in, $ty, $($els),+);
			impl_ease_for_vec!(fn ease_quad_out, $ty, $($els),+);
			impl_ease_for_vec!(fn ease_quad_inout, $ty, $($els),+);

			impl_ease_for_vec!(fn ease_exp_in, $ty, $($els),+);
			impl_ease_for_vec!(fn ease_exp_out, $ty, $($els),+);
			impl_ease_for_vec!(fn ease_exp_inout, $ty, $($els),+);

			impl_ease_for_vec!(fn ease_elastic_in, $ty, $($els),+);
			impl_ease_for_vec!(fn ease_elastic_out, $ty, $($els),+);
			impl_ease_for_vec!(fn ease_elastic_inout, $ty, $($els),+);

			impl_ease_for_vec!(fn ease_back_in, $ty, $($els),+);
			impl_ease_for_vec!(fn ease_back_out, $ty, $($els),+);
			impl_ease_for_vec!(fn ease_back_inout, $ty, $($els),+);

			impl_ease_for_vec!(fn ease_bounce_in, $ty, $($els),+);
			impl_ease_for_vec!(fn ease_bounce_out, $ty, $($els),+);
			impl_ease_for_vec!(fn ease_bounce_inout, $ty, $($els),+);
		}
	};
}

impl_ease_for_vec!(Vec2, x, y);
impl_ease_for_vec!(Vec3, x, y, z);
impl_ease_for_vec!(Vec4, x, y, z, w);

