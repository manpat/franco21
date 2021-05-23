use crate::vector::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Plane {
	pub normal: Vec3,
	pub length: f32,
}

impl Plane {
	pub fn new(n: Vec3, length: f32) -> Self {
		Plane {normal: n.normalize(), length}
	}

	pub fn from_points(a: Vec3, b: Vec3, c: Vec3) -> Self {
		let ab = (b-a).normalize();
		let ac = (c-a).normalize();
		let normal = ab.cross(ac).normalize();
		Plane {
			normal,
			length: normal.dot(a),
		}
	}

	pub fn distance_to(&self, p: Vec3) -> f32 {
		self.normal.dot(p) - self.length
	}

	pub fn project(&self, p: Vec3) -> Vec3 {
		p - self.normal * self.distance_to(p)
	}

	pub fn mirror(&self, p: Vec3) -> Vec3 {
		p - self.normal * self.distance_to(p) * 2.0
	}
}
