use crate::prelude::*;

pub mod global;
pub mod resources;

pub mod camera;
pub mod world;
pub mod player;

pub use global::*;
pub use resources::*;
pub use camera::*;
pub use world::*;
pub use player::*;

pub struct Model {
	pub resources: Resources,
	pub global: Global,

	pub camera: Camera,

	pub world: World,
	pub player: Player,
}

impl Model {
	pub fn new() -> Result<Model> {
		let resources = Resources::new()?;
		let world = World::new(&resources)?;

		Ok(Model {
			resources,
			global: Global::new(),
			camera: Camera::new(),

			world,
			player: Player::new(),
		})
	}
}