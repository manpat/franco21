use crate::math::vector::Vec2;

/// A Closed 2D Range - that is min and max count as being inside the bounds of the Aabb2
#[derive(Debug, Copy, Clone)]
pub struct Aabb2 {
	pub min: Vec2,
	pub max: Vec2,
}

impl Aabb2 {
	pub fn new(min: Vec2, max: Vec2) -> Aabb2 {
		Aabb2 { min, max }
	}

	pub fn new_empty() -> Aabb2 {
		Aabb2::new(
			Vec2::splat(f32::INFINITY),
			Vec2::splat(-f32::INFINITY)
		)
	}

	pub fn around_point(center: Vec2, extents: Vec2) -> Aabb2 {
		Aabb2::new(center - extents, center + extents)
	}

	pub fn is_empty(&self) -> bool {
		self.min.x >= self.max.x
		|| self.min.y >= self.max.y
	}

	pub fn contains_point(&self, point: Vec2) -> bool {
		self.min.x <= point.x && point.x <= self.max.x
		&& self.min.y <= point.y && point.y <= self.max.y
	}
}