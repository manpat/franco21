use crate::prelude::*;

pub mod resources;
pub use resources::*;

pub mod global;
pub use global::*;

pub mod camera;
pub use camera::*;

pub mod world;
pub use world::*;

pub mod player;
pub use player::*;

pub mod ui;
pub use ui::*;


pub struct Model {
	pub resources: Resources,
	pub global: Global,

	pub camera: Camera,

	pub world: World,
	pub player: Player,

	pub ui: Ui,
}

impl Model {
	pub fn new() -> Result<Model> {
		let resources = Resources::new()?;
		let world = World::new(&resources)?;
		let ui = Ui::new(&resources);

		Ok(Model {
			resources,
			global: Global::new(),
			camera: Camera::new(),

			world,
			player: Player::new(),

			ui,
		})
	}
}