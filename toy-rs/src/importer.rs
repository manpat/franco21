use crate::types::*;
use crate::ToyResult;
use std::convert::TryInto;

use common::*;
use failure::{ensure, bail, format_err};

// version 4: convert vertex colours from srgb to linear
const SCENE_VERSION: u8 = 4;

pub fn load(data: &[u8]) -> ToyResult<Project> {
	let reader = ToyReader { buf: data };
	reader.read_all()
}


type Tag = [u8; 4];

struct ToyReader<'data> { buf: &'data [u8] }

impl<'d> ToyReader<'d> {
	fn read_all(mut self) -> ToyResult<Project> {
		self.read_magic()?;

		let mut meshes = Vec::new();
		let mut entities = Vec::new();
		let mut scenes = Vec::new();

		while !self.buf.is_empty() {
			let (tag, mut section) = self.read_section()?;

			let to_err = |e| format_err!("While parsing '{}' section: {}", tag_to_string(&tag), e);

			match &tag {
				b"SCNE" => scenes.push(section.read_scene().map_err(to_err)?),
				b"MESH" => meshes.push(section.read_mesh().map_err(to_err)?),
				b"ENTY" => entities.push(section.read_entity().map_err(to_err)?),
				_ => { bail!("Unexpected tag '{}' encountered", tag_to_string(&tag)); }
			}
		}

		Ok(Project {
			scenes,
			entities,
			meshes,
		})
	}

	fn read_magic(&mut self) -> ToyResult<()> {
		ensure!(&self.buf[..3] == b"TOY", "Expected magic string");
		self.buf = &self.buf[3..];

		let version = self.read_u8()?;
		ensure!(version == SCENE_VERSION, "Version mismatch ({}/{})", version, SCENE_VERSION);

		Ok(())
	}

	fn read_section(&mut self) -> ToyResult<(Tag, ToyReader<'_>)> {
		let tag = self.read_tag()?;
		let section_size = self.read_u32()? as usize;
		ensure!(section_size <= self.buf.len(), "Invalid section size for '{}'", tag_to_string(&tag));

		let (section, rest) = self.buf.split_at(section_size);
		self.buf = rest;

		Ok((tag, ToyReader{ buf: section }))
	}

	fn expect_section(&mut self, expected: &Tag) -> ToyResult<ToyReader<'_>> {
		let (tag, section) = self.read_section()?;
		ensure!(&tag == expected, "Encountered unexpected tag '{}'", tag_to_string(&tag));
		Ok(section)
	}

	fn read_mesh(&mut self) -> ToyResult<MeshData> {
		let num_vertices = self.read_u16()? as usize;
		let mut vertices = Vec::with_capacity(num_vertices);
		for _ in 0..num_vertices {
			vertices.push(self.read_vec3()?);
		}

		let wide_indices = num_vertices >= 256;

		let num_triangles = self.read_u16()? as usize;
		let num_indices = num_triangles * 3;
		let mut indices = Vec::with_capacity(num_indices);

		if wide_indices {
			for _ in 0..num_indices {
				indices.push(self.read_u16()?);
			}

		} else {
			for _ in 0..num_indices {
				indices.push(self.read_u8()? as u16);
			}
		}

		let num_color_layers = self.read_u8()? as usize;
		let mut color_data = Vec::with_capacity(num_color_layers);
		for _ in 0..num_color_layers {
			self.expect_tag(b"MDTA")?;

			let layer_name = self.read_string()?;
			let num_points = self.read_u16()? as usize;
			ensure!(num_points == num_vertices, "Color layer '{}' different size to vertex list", layer_name);

			let mut layer_data = Vec::with_capacity(num_points);
			for _ in 0..num_points {
				layer_data.push(self.read_vec4()?);
			}

			color_data.push(MeshColorData {
				name: layer_name,
				data: layer_data,
			})
		}

		let mut animation_data = None;

		if !self.buf.is_empty() {
			let mut section = self.expect_section(b"SKIN")?;
			animation_data = Some(section.read_animation_data()?);
		}

		Ok(MeshData {
			positions: vertices,
			indices,
			color_data,
			animation_data,
		})
	}

	fn read_animation_data(&mut self) -> ToyResult<MeshAnimationData> {
		let num_bones = self.read_u8()? as usize;
		let mut bones = Vec::with_capacity(num_bones);
		for _ in 0..num_bones {
			bones.push(MeshBone {
				name: self.read_string()?,
				head: self.read_vec3()?,
				tail: self.read_vec3()?,
			});
		}

		let num_vertices = self.read_u16()? as usize;
		let mut weights = Vec::with_capacity(num_vertices);

		// I'm sorry
		for _ in 0..((num_vertices+3) / 4) {
			let counts_packed = self.read_u8()?;
			let counts = [
				(counts_packed as usize >> 6) & 0b11,
				(counts_packed as usize >> 4) & 0b11,
				(counts_packed as usize >> 2) & 0b11,
				(counts_packed as usize >> 0) & 0b11,
			];

			for &count in counts.iter() {
				let mut vertex = MeshWeightVertex::default();
				for (index, weight) in vertex.indices.iter_mut().zip(&mut vertex.weights).take(count) {
					*index = self.read_u8()?;
					*weight = self.read_uf16()?;
				}
				weights.push(vertex);
			}
		}

		// Remove extra unnecessary vertices
		for _ in 0..(num_vertices%4) {
			weights.pop();
		}

		let mut section = self.expect_section(b"ANMS")?;
		let animations = section.read_animations()?;

		Ok(MeshAnimationData {
			bones,
			weights,
			animations
		})
	}

	fn read_animations(&mut self) -> ToyResult<Vec<MeshAnimation>> {
		let mut animations = Vec::new();

		while !self.buf.is_empty() {
			let mut section = self.expect_section(b"ANIM")?;
			animations.push(MeshAnimation {
				name: section.read_string()?,
				fps: section.read_f32()?,
				channels: section.read_animation_channels()?,
			});
		}

		Ok(animations)
	}

	fn read_animation_channels(&mut self) -> ToyResult<Vec<MeshAnimationChannel>> {
		let num_frames = self.read_u16()? as usize;
		let num_channels = self.read_u8()? as usize;
		let mut channels = Vec::with_capacity(num_channels);

		for _ in 0..num_channels {
			channels.push(MeshAnimationChannel {
				bone: self.read_string()?,
				frames: self.read_animation_frames(num_frames)?,
			})
		}

		Ok(channels)
	}

	fn read_animation_frames(&mut self, num_frames: usize) -> ToyResult<Vec<MeshAnimationFrame>> {
		let mut frames = Vec::with_capacity(num_frames);

		for _ in 0..num_frames {
			frames.push(MeshAnimationFrame {
				position: self.read_vec3()?,
				rotation: self.read_quat()?,
				scale: self.read_vec3()?,
			});
		}

		Ok(frames)
	}

	fn read_entity(&mut self) -> ToyResult<EntityData> {
		Ok(EntityData {
			name: self.read_string()?,
			position: self.read_vec3()?,
			rotation: self.read_quat()?,
			scale: self.read_vec3()?,
			mesh_id: self.read_u16()?,
		})
	}

	fn read_scene(&mut self) -> ToyResult<SceneData> {
		let name = self.read_string()?;
		let num_entities = self.read_u32()? as usize;
		let mut entities = Vec::with_capacity(num_entities);
		for _ in 0..num_entities {
			entities.push(self.read_u32()?);
		}

		Ok(SceneData {
			name,
			entities
		})
	}

	fn expect_tag(&mut self, tag: &Tag) -> ToyResult<()> {
		ensure!(self.buf.len() >= 4, "Unexpected EOF while expecting tag '{}'", tag_to_string(tag));
		ensure!(&self.buf[..4] == tag, "Expected tag '{}'", tag_to_string(tag));
		self.buf = &self.buf[4..];
		Ok(())
	}

	fn read_tag(&mut self) -> ToyResult<Tag> {
		ensure!(self.buf.len() >= 4, "Unexpected EOF while expecting tag");
		let (tag, rest) = self.buf.split_at(4);
		self.buf = rest;
		Ok(tag.try_into()?)
	}

	fn read_u8(&mut self) -> ToyResult<u8> {
		ensure!(self.buf.len() >= 1, "Unexpected EOF while expecting u8");
		let b = self.buf[0];
		self.buf = &self.buf[1..];
		Ok(b)
	}

	fn read_u16(&mut self) -> ToyResult<u16> {
		ensure!(self.buf.len() >= 2, "Unexpected EOF while expecting u16");
		let (b, rest) = self.buf.split_at(2);
		self.buf = rest;
		Ok(u16::from_le_bytes(b.try_into()?))
	}

	fn read_u32(&mut self) -> ToyResult<u32> {
		ensure!(self.buf.len() >= 4, "Unexpected EOF while expecting u32");
		let (b, rest) = self.buf.split_at(4);
		self.buf = rest;
		Ok(u32::from_le_bytes(b.try_into()?))
	}

	fn read_f32(&mut self) -> ToyResult<f32> {
		Ok(f32::from_bits(self.read_u32()?))
	}

	fn read_uf16(&mut self) -> ToyResult<f32> {
		Ok(self.read_u16()? as f32 / 65535.0)
	}

	fn read_vec3(&mut self) -> ToyResult<Vec3> {
		Ok(Vec3::new(
			self.read_f32()?,
			self.read_f32()?,
			self.read_f32()?
		))
	}

	fn read_vec4(&mut self) -> ToyResult<Vec4> {
		Ok(Vec4::new(
			self.read_f32()?,
			self.read_f32()?,
			self.read_f32()?,
			self.read_f32()?
		))
	}

	fn read_quat(&mut self) -> ToyResult<Quat> {
		Ok(Quat::from_raw(
			self.read_f32()?,
			self.read_f32()?,
			self.read_f32()?,
			self.read_f32()?
		))
	}

	fn read_string(&mut self) -> ToyResult<String> {
		let length = self.read_u8()? as usize;

		ensure!(self.buf.len() >= length, "Unexpected EOF while reading string");
		let (utf8, tail) = self.buf.split_at(length);
		self.buf = tail;

		std::str::from_utf8(utf8)
			.map(Into::into)
			.map_err(Into::into)
	}
}


fn tag_to_string(tag: &Tag) -> String {
	unsafe {
		std::str::from_utf8_unchecked(tag).into()
	}
}