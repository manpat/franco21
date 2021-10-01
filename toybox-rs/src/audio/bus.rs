use crate::prelude::*;
use crate::audio::{
	system::StreamUpdateRequest,
	system::Assets,
	system::SoundAssetID,
	system::SoundAssetType,
	system::STREAM_PREFETCH_FACTOR,
	mixer::Mixer,
};

use std::num::Wrapping;


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BusID(pub(super) usize);


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SoundInstanceID {
	pub(super) bus_id: BusID,
	instance_id: usize
}


#[derive(Copy, Clone, Debug)]
struct SoundInstance {
	instance_id: SoundInstanceID,
	asset_id: SoundAssetID,
	position: usize,
	playing: bool,
}



pub struct Bus {
	bus_id: BusID,
	name: String,
	send_bus: Option<BusID>,

	active_sounds: Vec<SoundInstance>,
	mixer: Mixer,

	sound_instance_counter: Wrapping<usize>,
}

impl Bus {
	pub fn bus_id(&self) -> BusID { self.bus_id }
	pub fn name(&self) -> &str { &self.name }

	pub fn set_send_bus(&mut self, bus_id: impl Into<Option<BusID>>) {
		self.send_bus = bus_id.into();
	}

	pub fn send_bus(&self) -> Option<BusID> { self.send_bus }

	pub fn set_gain(&mut self, gain: f32) { self.mixer.set_gain(gain); }
	pub fn gain(&self) -> f32 { self.mixer.gain() }

	pub fn start_sound(&mut self, asset_id: SoundAssetID) -> SoundInstanceID {
		let instance_id = SoundInstanceID {
			bus_id: self.bus_id,
			instance_id: self.sound_instance_counter.0,
		};

		self.sound_instance_counter += Wrapping(1);

		self.active_sounds.push(SoundInstance {
			instance_id,
			asset_id,
			position: 0,
			playing: true,
		});

		instance_id
	}

	pub fn kill_sound(&mut self, instance_id: SoundInstanceID) {
		self.active_sounds.retain(|s| s.instance_id != instance_id);
	}

	pub fn set_playing(&mut self, instance_id: SoundInstanceID, playing: bool) {
		if let Some(instance) = self.active_sounds.iter_mut()
			.find(|s| s.instance_id == instance_id)
		{
			instance.playing = playing;
		}
	}
}


impl Bus {
	pub(super) fn new(name: String, buffer_size: usize, bus_id: BusID) -> Bus {
		let mixer = Mixer::new(buffer_size);

		Bus {
			bus_id,
			name,
			send_bus: None,

			active_sounds: Vec::new(),
			mixer,

			sound_instance_counter: Wrapping(0),
		}
	}

	pub(super) fn update(&mut self, assets: &Assets, stream_updates: &mut Vec<StreamUpdateRequest>) {
		let mix_buffer_samples = self.mixer.buffer_samples();
		let stream_prefetch_size = STREAM_PREFETCH_FACTOR * mix_buffer_samples;

		// Drop inactive sounds
		self.active_sounds.retain(|sound| {
			match sound.asset_id.ty {
				SoundAssetType::Buffer => {
					let buffer = &assets.buffers[sound.asset_id.index];
					sound.position * buffer.channels < buffer.data.len()
				}

				SoundAssetType::FileStream => {
					let stream = &assets.streams[sound.asset_id.index];
					let buffer = &stream.resident_buffer;
					!stream.fully_resident || sound.position * buffer.channels < buffer.data.len()
				}
			}
		});

		// Clear mix buffer
		self.mixer.clear();

		// Mix each sound into the mix buffer
		for SoundInstance {asset_id, position, playing, ..} in self.active_sounds.iter_mut() {
			if !*playing {
				continue
			}

			match asset_id.ty {
				SoundAssetType::Buffer => {
					let buffer = &assets.buffers[asset_id.index];
					let buffer_consumption = self.mixer.mix_buffer(buffer, *position);
					*position += buffer_consumption;
				}

				SoundAssetType::FileStream => {
					let stream = &assets.streams[asset_id.index];
					let buffer_consumption = self.mixer.mix_buffer(&stream.resident_buffer, *position);
					*position += buffer_consumption;

					// If the stream is running low on samples, queue it for update
					if !stream.fully_resident && stream.resident_buffer.samples() - *position < stream_prefetch_size {
						stream_updates.push(StreamUpdateRequest {
							index: asset_id.index,
							position: *position,
						});
					}
				}
			}
		}
	}

	pub(super) fn mix_subbus(&mut self, bus: &Bus) {
		self.mixer.mix_bus(bus);
	}

	pub(super) fn buffer(&self) -> &[f32] {
		self.mixer.buffer()
	}
}

