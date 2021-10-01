use crate::prelude::*;
use crate::audio::{*, bus::Bus, bus::BusID};

pub(super) const STREAM_PREFETCH_FACTOR: usize = 1;
pub const MASTER_BUS: BusID = BusID(0);


#[derive(Copy, Clone, Debug)]
pub struct SoundAssetID {
	pub(super) ty: SoundAssetType,
	pub(super) index: usize,
}


#[derive(Copy, Clone, Debug)]
pub(super) enum SoundAssetType {
	Buffer,
	FileStream,
}




pub(super) struct StreamUpdateRequest {
	pub index: usize,
	pub position: usize,
}


pub(super) struct Assets {
	pub(super) buffers: Vec<Buffer>,
	pub(super) streams: Vec<FileStream>,
}


pub struct AudioSystem {
	audio_queue: sdl2::audio::AudioQueue<f32>,
	assets: Assets,

	master_bus: Bus,
	busses: Vec<Bus>,
	bus_counter: usize,

	stream_updates: Vec<StreamUpdateRequest>,
	buffer_size: usize,
}

impl AudioSystem {
	pub fn new(sdl_audio: sdl2::AudioSubsystem) -> Result<AudioSystem, Box<dyn Error>> {
		let desired_spec = sdl2::audio::AudioSpecDesired {
			freq: Some(44100),
			channels: Some(2),
			samples: Some(512),
		};

		let audio_queue = sdl_audio.open_queue(None, &desired_spec)?;
		audio_queue.resume();

		let spec = audio_queue.spec();
		assert!(spec.freq == 44100);
		assert!(spec.channels == 2);

		let buffer_size = spec.samples as usize * spec.channels as usize;

		Ok(AudioSystem {
			audio_queue,
			assets: Assets {
				buffers: Vec::new(),
				streams: Vec::new(),
			},

			master_bus: Bus::new("Master".into(), buffer_size, BusID(0)),
			busses: Vec::new(),
			bus_counter: 1,

			stream_updates: Vec::new(),
			buffer_size,
		})
	}



	pub fn register_buffer(&mut self, buffer: Buffer) -> SoundAssetID {
		let asset_id = SoundAssetID {
			ty: SoundAssetType::Buffer,
			index: self.assets.buffers.len(),
		};

		self.assets.buffers.push(buffer);
		asset_id
	}

	pub fn register_file_stream(&mut self, stream: FileStream) -> SoundAssetID {
		let asset_id = SoundAssetID {
			ty: SoundAssetType::FileStream,
			index: self.assets.streams.len(),
		};

		self.assets.streams.push(stream);
		asset_id
	}



	pub fn new_bus(&mut self, name: impl Into<String>) -> BusID {
		let bus_id = BusID(self.bus_counter);
		let name = name.into();

		self.busses.push(Bus::new(name, self.buffer_size, bus_id));
		self.bus_counter += 1;
		bus_id
	}

	pub fn destroy_bus(&mut self, bus_id: BusID) {
		self.busses.retain(|bus| bus.bus_id() != bus_id);
		// TODO(pat.m): once busses can be parented, destroy or reparent children busses
	}


	pub fn get_bus(&mut self, bus_id: impl Into<Option<BusID>>) -> Option<&Bus> {
		let bus_id = bus_id.into().unwrap_or(MASTER_BUS);

		if bus_id == MASTER_BUS {
			Some(&self.master_bus)
		} else {
			self.busses.iter()
				.find(move |bus| bus.bus_id() == bus_id)
		}
	}

	pub fn get_bus_mut(&mut self, bus_id: impl Into<Option<BusID>>) -> Option<&mut Bus> {
		let bus_id = bus_id.into().unwrap_or(MASTER_BUS);

		if bus_id == MASTER_BUS {
			Some(&mut self.master_bus)
		} else {
			self.busses.iter_mut()
				.find(move |bus| bus.bus_id() == bus_id)
		}
	}



	pub fn start_sound(&mut self, bus_id: impl Into<Option<BusID>>, asset_id: SoundAssetID) -> SoundInstanceID {
		self.get_bus_mut(bus_id.into().unwrap_or(MASTER_BUS))
			.expect("Invalid BusID")
			.start_sound(asset_id)
	}

	pub fn kill_sound(&mut self, instance_id: SoundInstanceID) {
		if let Some(bus) = self.get_bus_mut(instance_id.bus_id) {
			bus.kill_sound(instance_id);
		}
	}

	pub fn set_playing(&mut self, instance_id: SoundInstanceID, playing: bool) {
		if let Some(bus) = self.get_bus_mut(instance_id.bus_id) {
			bus.set_playing(instance_id, playing)
		}
	}



	pub fn update(&mut self) {
		let spec = self.audio_queue.spec();

		let threshold_size = 1.0 / 60.0 * spec.freq as f32 * spec.channels as f32;
		let threshold_size = threshold_size as u32 * std::mem::size_of::<f32>() as u32;

		let stream_prefetch_size = STREAM_PREFETCH_FACTOR * self.buffer_size;

		if self.audio_queue.size() < threshold_size {
			// Mix sound instances
			for bus in self.busses.iter_mut() {
				bus.update(&self.assets, &mut self.stream_updates);
			}

			self.master_bus.update(&self.assets, &mut self.stream_updates);

			// Mix child busses into parent busses
			let mut child_bus_sends = self.busses.iter()
				.enumerate()
				.map(|(idx, bus)| {
					let send_idx = bus.send_bus()
						.and_then(|send_bus| {
							self.busses.iter()
								.position(|bus| bus.bus_id() == send_bus)
						});

					(idx, send_idx)
				})
				.collect(): Vec<(usize, Option<usize>)>;

			// TODO(pat.m): need to sort so children are mixed up before sends
			child_bus_sends.sort_by(|a, b| b.1.cmp(&a.1));

			for (child_idx, send_idx) in child_bus_sends {
				if let Some(send_idx) = send_idx {
					assert!(child_idx != send_idx);

					let (child, send) = get_mut_disjoint(&mut self.busses, child_idx, send_idx);
					send.mix_subbus(&child);
					
				} else {
					self.master_bus.mix_subbus(&self.busses[child_idx]);
				}
			}

			// Submit audio frame
			self.audio_queue.queue(&self.master_bus.buffer());

			// Remove duplicate update requests - prioritising those with the highest `position`
			self.stream_updates.sort_by(|a, b| a.index.cmp(&b.index).then(b.position.cmp(&a.position)));
			self.stream_updates.dedup_by_key(|r| r.index);
		}

		for StreamUpdateRequest{index, position} in self.stream_updates.drain(..) {
			let stream = &mut self.assets.streams[index];

			loop {
				stream.load_chunk().expect("Chunk load failed!");

				// break if we've hit end of stream
				if stream.fully_resident {
					break
				}

				// or if we've loaded enough samples to cover a couple frames of audio at least
				if stream.resident_buffer.samples() - position >= stream_prefetch_size {
					break
				}
			}
		}
	}
}



fn get_mut_disjoint<T>(slice: &mut [T], a: usize, b: usize) -> (&mut T, &mut T) {
	assert!(a != b);

	let (left, right) = slice.split_at_mut(a.max(b));

	if a < b {
		(&mut left[a], &mut right[0])
	} else {
		(&mut right[0], &mut left[b])
	}
}