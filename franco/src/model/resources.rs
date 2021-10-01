use crate::prelude::*;


pub struct Resources {
	pub main_project: toy::Project,
}

impl Resources {
	pub fn new() -> Result<Resources> {
		let main_project_data = std::fs::read("assets/main.toy")?;
		let main_project = toy::load(&main_project_data)?;

		Ok(Resources {
			main_project
		})
	}
}