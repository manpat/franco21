use crate::math::vector::*;
use crate::easing::Ease;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Color {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32,
}

#[allow(dead_code)]
impl Color {
	pub const fn rgba(r:f32, g:f32, b:f32, a:f32) -> Color {
		Color {r,g,b,a}
	}
	pub const fn rgb(r:f32, g:f32, b:f32) -> Color {
		Color::rgba(r,g,b, 1.0)
	}

	pub const fn rgba8(r:u8, g:u8, b:u8, a:u8) -> Color {
		Color {
			r: r as f32 / 255.0,
			g: g as f32 / 255.0,
			b: b as f32 / 255.0,
			a: a as f32 / 255.0,
		}
	}
	pub const fn rgb8(r:u8, g:u8, b:u8) -> Color {
		Color::rgba8(r,g,b, 255)
	}

	pub fn hsva(h: f32, s: f32, v: f32, a: f32) -> Color {
		use crate::easing::Clamp;

		let h = h % 360.0 - h.signum().min(0.0) * 360.0;
		// if h < 0.0, add 360.0

		let s = Clamp::clamp(&s, 0.0, 1.0);
		let v = Clamp::clamp(&v, 0.0, 1.0);

		let c = v * s;
		let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
		let m = v - c;

		let seg = (h / 60.0) as u32 % 6;
		let (r,g,b) = match seg {
			0 => (c, x, 0.0),
			1 => (x, c, 0.0),
			2 => (0.0, c, x),
			3 => (0.0, x, c),
			4 => (x, 0.0, c),
			5 => (c, 0.0, x),
			_ => return Color::black()
		};

		Color::rgba(r+m, g+m, b+m, a)
	}

	pub fn hsv(h: f32, s: f32, v: f32) -> Color {
		Color::hsva(h,s,v, 1.0)
	}

	pub const fn grey(v: f32) -> Color { Color::rgb(v, v, v) }
	pub const fn grey_a(v: f32, a: f32) -> Color { Color::rgba(v, v, v, a) }
	pub const fn white() -> Color { Color::grey(1.0) }
	pub const fn black() -> Color { Color::grey(0.0) }

	pub fn to_byte_tuple(&self) -> (u8, u8, u8, u8) {
		let Color{r,g,b,a} = *self;
		((r*255.0) as u8, (g*255.0) as u8, (b*255.0) as u8, (a*255.0) as u8)
	}

	pub fn to_tuple(&self) -> (f32, f32, f32, f32) {
		let Color{r,g,b,a} = *self;
		(r,g,b,a)
	}

	pub fn to_vec3(&self) -> Vec3 { Vec3::new(self.r, self.g, self.b) }
	pub fn to_vec4(&self) -> Vec4 { Vec4::new(self.r, self.g, self.b, self.a) }

	pub fn pow(self, exp: f32) -> Color {
		Color::rgba(
			self.r.powf(exp),
			self.g.powf(exp),
			self.b.powf(exp),
			self.a,
		)
	}
}

impl From<&Vec3> for Color {
	fn from(o: &Vec3) -> Color { Color::rgb(o.x, o.y, o.z) }
}
impl From<&Vec4> for Color {
	fn from(o: &Vec4) -> Color { Color::rgba(o.x, o.y, o.z, o.w) }
}

impl From<Vec3> for Color {
	fn from(o: Vec3) -> Color { Color::rgb(o.x, o.y, o.z) }
}
impl From<Vec4> for Color {
	fn from(o: Vec4) -> Color { Color::rgba(o.x, o.y, o.z, o.w) }
}

impl From<(u8,u8,u8)> for Color {
	fn from(o: (u8,u8,u8)) -> Color { Color::rgb8(o.0, o.1, o.2) }
}
impl From<(u8,u8,u8,u8)> for Color {
	fn from(o: (u8,u8,u8,u8)) -> Color { Color::rgba8(o.0, o.1, o.2, o.3) }
}

impl From<Color> for Vec3 {
	fn from(o: Color) -> Vec3 { o.to_vec3() }
}
impl From<Color> for Vec4 {
	fn from(o: Color) -> Vec4 { o.to_vec4() }
}

macro_rules! impl_ease_for_color {
	($func: ident) => (
		fn $func(&self, start: Color, end: Color) -> Color {
			Color {
				r: self.$func(start.r, end.r),
				g: self.$func(start.g, end.g),
				b: self.$func(start.b, end.b),
				a: self.$func(start.a, end.a),
			}
		}
	)
}

impl Ease<Color> for f32 {
	impl_ease_for_color!(ease_linear);

	impl_ease_for_color!(ease_quad_in);
	impl_ease_for_color!(ease_quad_out);
	impl_ease_for_color!(ease_quad_inout);

	impl_ease_for_color!(ease_exp_in);
	impl_ease_for_color!(ease_exp_out);
	impl_ease_for_color!(ease_exp_inout);

	impl_ease_for_color!(ease_elastic_in);
	impl_ease_for_color!(ease_elastic_out);
	impl_ease_for_color!(ease_elastic_inout);

	impl_ease_for_color!(ease_back_in);
	impl_ease_for_color!(ease_back_out);
	impl_ease_for_color!(ease_back_inout);

	impl_ease_for_color!(ease_bounce_in);
	impl_ease_for_color!(ease_bounce_out);
	impl_ease_for_color!(ease_bounce_inout);
}

// fn srgb_to_linear(color: Color) -> Color {
// 	Color {
// 		r: srgb_channel_to_linear(color.r),
// 		g: srgb_channel_to_linear(color.g),
// 		b: srgb_channel_to_linear(color.b),
// 		a: color.a
// 	}
// }


// fn srgb_channel_to_linear(value: f32) -> f32 {
// 	// https://en.wikipedia.org/wiki/SRGB#From_sRGB_to_CIE_XYZ
// 	if value <= 0.04045 {
// 		value / 12.92
// 	} else {
// 		((value + 0.055) / 1.055).powf(2.4)
// 	}
// }