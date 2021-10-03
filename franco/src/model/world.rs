use crate::prelude::*;


pub const MAP_SCALE: f32 = 10.0;


#[derive(Debug)]
pub struct World {
	pub map: Map,

	pub sky_color: Color,
}

impl World {
	pub fn new(resources: &model::Resources) -> Result<World> {
		let map_scene = resources.main_project.find_scene("map").unwrap();

		Ok(World {
			sky_color: Color::hsv(200.0, 0.5, 0.9),
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
				map_position: entity.position.to_xz() * Vec2::new(1.0, -1.0),
				rotation: entity.rotation.yaw(),
				ty: MapObjectType::from_name(&entity.name),
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
	pub rotation: f32,
	pub ty: MapObjectType,
}


#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MapObjectType {
	SmallIsland,
	Rocks,
	Rocks2,
}

impl MapObjectType {
	pub fn from_name(name: &str) -> MapObjectType {
		let name = name.trim_start_matches("MAP_");
		let name = name.trim_start_matches("ISLAND_");
		let (name, _) = name.split_once('.').unwrap_or((name, ""));

		match name {
			"small" => MapObjectType::SmallIsland,
			"rocks" => MapObjectType::Rocks,
			"rocks2" => MapObjectType::Rocks2,
			_ => panic!("Unknown map object type {}", name),
		}
	}
}



pub fn map_to_world(map: Vec2) -> Vec2 {
	map * MAP_SCALE * Vec2::new(1.0, -1.0)
}
