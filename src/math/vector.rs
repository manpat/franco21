use std::ops::{Add, Sub, Mul, Div, Neg};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};
use std::iter::{Sum, Product};
use easing::*;
use rand::{Rand, Rng};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vec2{pub x: f32, pub y: f32}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vec3{pub x: f32, pub y: f32, pub z: f32}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vec4{pub x: f32, pub y: f32, pub z: f32, pub w: f32}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2i{pub x: i32, pub y: i32}


impl Vec2 {
	pub const fn new(x: f32, y: f32) -> Vec2 { Vec2{x, y} }
	pub const fn splat(x: f32) -> Vec2 { Vec2::new(x, x) }
	pub const fn zero() -> Vec2 { Vec2::splat(0.0) }
	pub fn from_angle(th: f32) -> Vec2 { Vec2::new(th.cos(), th.sin()) }
	pub fn from_slice(o: &[f32]) -> Vec2 {
		assert!(o.len() >= 2);
		Vec2::new(o[0], o[1])
	}

	pub const fn from_x(x: f32) -> Vec2 { Vec2::new(x, 0.0) }
	pub const fn from_y(y: f32) -> Vec2 { Vec2::new(0.0, y) }

	pub fn to_x0z(self) -> Vec3 { Vec3::new(self.x, 0.0, self.y) }
	pub fn to_vec2i(self) -> Vec2i { Vec2i::new(self.x as i32, self.y as i32) }
	pub fn to_tuple(self) -> (f32,f32) { (self.x, self.y) }
	pub fn to_array(self) -> [f32; 2] { [self.x, self.y] }
	pub fn to_angle(self) -> f32 { self.y.atan2(self.x) }
	pub fn extend(self, z: f32) -> Vec3 { Vec3::new(self.x, self.y, z) }

	pub fn length(self) -> f32 { self.dot(self).sqrt() }
	pub fn perp(self) -> Vec2 { Vec2::new(-self.y, self.x) }
	
	pub fn normalize(self) -> Vec2 { self * (1.0/self.length()) }
	pub fn dot(self, o: Vec2) -> f32 { self.x*o.x + self.y*o.y }
}

impl Vec3 {
	pub const fn new(x: f32, y: f32, z: f32) -> Vec3 { Vec3{x, y, z} }
	pub const fn splat(x: f32) -> Vec3 { Vec3::new(x, x, x) }
	pub const fn zero() -> Vec3 { Vec3::splat(0.0) }
	pub fn from_x_angle(th: f32) -> Vec3 { Vec3::new(0.0, th.sin(), th.cos()) }
	pub fn from_y_angle(th: f32) -> Vec3 { Vec3::new(th.cos(), 0.0, th.sin()) }
	pub fn from_slice(o: &[f32]) -> Vec3 {
		assert!(o.len() >= 3);
		Vec3::new(o[0], o[1], o[2])
	}

	pub const fn from_x(x: f32) -> Vec3 { Vec3::new(x, 0.0, 0.0) }
	pub const fn from_y(y: f32) -> Vec3 { Vec3::new(0.0, y, 0.0) }
	pub const fn from_z(z: f32) -> Vec3 { Vec3::new(0.0, 0.0, z) }

	pub fn to_tuple(&self) -> (f32,f32,f32) { (self.x, self.y, self.z) }
	pub fn to_xy(self) -> Vec2 { Vec2::new(self.x, self.y) }
	pub fn extend(&self, w: f32) -> Vec4 { Vec4::new(self.x, self.y, self.z, w) }

	pub fn length(&self) -> f32 { self.dot(*self).sqrt() }
	pub fn normalize(&self) -> Vec3 { *self * (1.0/self.length()) }

	pub fn dot(&self, o: Vec3) -> f32 { self.x*o.x + self.y*o.y + self.z*o.z }
	pub fn cross(&self, o: Vec3) -> Vec3 {
		Vec3::new(
			self.y*o.z - self.z*o.y,
			self.z*o.x - self.x*o.z,
			self.x*o.y - self.y*o.x,
		)
	}
}

impl Vec4 {
	pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 { Vec4{x, y, z, w} }
	pub const fn splat(x: f32) -> Vec4 { Vec4::new(x, x, x, x) }
	pub const fn zero() -> Vec4 { Vec4::splat(0.0) }
	pub fn from_slice(o: &[f32]) -> Vec4 {
		assert!(o.len() >= 4);
		Vec4::new(o[0], o[1], o[2], o[3])
	}

	pub const fn from_x(x: f32) -> Vec4 { Vec4::new(x, 0.0, 0.0, 0.0) }
	pub const fn from_y(y: f32) -> Vec4 { Vec4::new(0.0, y, 0.0, 0.0) }
	pub const fn from_z(z: f32) -> Vec4 { Vec4::new(0.0, 0.0, z, 0.0) }
	pub const fn from_w(w: f32) -> Vec4 { Vec4::new(0.0, 0.0, 0.0, w) }

	pub fn to_tuple(&self) -> (f32,f32,f32,f32) { (self.x, self.y, self.z, self.w) }
	pub fn to_vec3(&self) -> Vec3 { Vec3::new(self.x, self.y, self.z) }

	pub fn length(&self) -> f32 { self.dot(*self).sqrt() }

	pub fn dot(&self, o: Vec4) -> f32 { self.x*o.x + self.y*o.y + self.z*o.z + self.w*o.w }
}

impl Vec2i {
	pub const fn new(x: i32, y: i32) -> Vec2i { Vec2i{x, y} }
	pub const fn splat(x: i32) -> Vec2i { Vec2i::new(x, x) }
	pub const fn zero() -> Vec2i { Vec2i::splat(0) }

	pub fn from_tuple(t: (i32,i32)) -> Vec2i { Vec2i::new(t.0, t.1) }
	pub fn to_tuple(self) -> (i32,i32) { (self.x, self.y) }
	pub fn to_vec2(self) -> Vec2 { Vec2::new(self.x as f32, self.y as f32) }

	pub fn length(self) -> f32 {
		((self.x*self.x + self.y*self.y) as f32).sqrt()
	}
}

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


impl Rand for Vec2 {
	fn rand<R: Rng>(rng: &mut R) -> Self {
		Vec2::new(rng.gen(), rng.gen())
	}
}

impl Rand for Vec3 {
	fn rand<R: Rng>(rng: &mut R) -> Self {
		Vec3::new(rng.gen(), rng.gen(), rng.gen())
	}
}

impl Rand for Vec4 {
	fn rand<R: Rng>(rng: &mut R) -> Self {
		Vec4::new(rng.gen(), rng.gen(), rng.gen(), rng.gen())
	}
}

impl Rand for Vec2i {
	fn rand<R: Rng>(rng: &mut R) -> Self {
		Vec2i::new(rng.gen(), rng.gen())
	}
}