use crate::prelude::*;


#[derive(Debug)]
pub struct World {
	pub map: Map,
}

impl World {
	pub fn new(resources: &model::Resources) -> Result<World> {
		let map_scene = resources.main_project.find_scene("map").unwrap();

		Ok(World {
			map: Map::new(map_scene),
		})
	}
}


#[derive(Debug)]
pub struct Map {
	pub size: Vec2,
	pub objects: Vec<MapObject>,
}


impl Map {
	pub fn new(scene: toy::SceneRef<'_>) -> Map {
		let scale_ent = scene.find_entity("REF_map_scale").unwrap();
		let size = scale_ent.scale.to_xz();

		let objects = scene.entities_with_prefix("MAP_")
			.map(|entity| MapObject {
				map_position: entity.position.to_xz(),
			})
			.collect();

		Map {
			size,
			objects,
		}
	}
}




#[derive(Debug)]
pub struct MapObject {
	pub map_position: Vec2,

}
