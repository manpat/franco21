use crate::prelude::*;
use crate::audio::{*, mixer::Mixer};

const STREAM_PREFETCH_FACTOR: usize = 1;

#[derive(Copy, Clone, Debug)]
pub struct SoundAssetID {
	ty: SoundAssetType,
	index: usize,
}

#[derive(Copy, Clone, Debug)]
enum SoundAssetType {
	Buffer,
	FileStream,
}


pub struct AudioSystem {
	audio_queue: sdl2::audio::AudioQueue<f32>,
	buffers: Vec<Buffer>,
	streams: Vec<FileStream>,
	active_sounds: Vec<SoundInstance>,

	stream_updates: Vec<StreamUpdateRequest>,

	mixer: Mixer,
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

		let mixer = Mixer::new(buffer_size);

		Ok(AudioSystem {
			audio_queue,
			buffers: Vec::new(),
			streams: Vec::new(),
			active_sounds: Vec::new(),

			stream_updates: Vec::new(),
			
			mixer,
		})
	}

	pub fn register_buffer(&mut self, buffer: Buffer) -> SoundAssetID {
		let asset_id = SoundAssetID {
			ty: SoundAssetType::Buffer,
			index: self.buffers.len(),
		};

		self.buffers.push(buffer);
		asset_id
	}

	pub fn register_file_stream(&mut self, stream: FileStream) -> SoundAssetID {
		let asset_id = SoundAssetID {
			ty: SoundAssetType::FileStream,
			index: self.streams.len(),
		};

		self.streams.push(stream);
		asset_id
	}

	pub fn play_one_shot(&mut self, asset_id: SoundAssetID) {
		self.active_sounds.push(SoundInstance {
			asset_id,
			position: 0,
		});
	}

	pub fn update(&mut self) {
		let spec = self.audio_queue.spec();

		let threshold_size = 1.0 / 60.0 * spec.freq as f32 * spec.channels as f32;
		let threshold_size = threshold_size as u32 * std::mem::size_of::<f32>() as u32;

		let mix_buffer_samples = self.mixer.buffer_samples();
		let stream_prefetch_size = STREAM_PREFETCH_FACTOR * mix_buffer_samples;

		if self.audio_queue.size() < threshold_size {
			// Drop inactive sounds
			let buffers = &self.buffers;
			let streams = &self.streams;

			self.active_sounds.retain(|sound| {
				match sound.asset_id.ty {
					SoundAssetType::Buffer => {
						let buffer = &buffers[sound.asset_id.index];
						sound.position * buffer.channels < buffer.data.len()
					}

					SoundAssetType::FileStream => {
						let stream = &streams[sound.asset_id.index];
						let buffer = &stream.resident_buffer;
						!stream.fully_resident || sound.position * buffer.channels < buffer.data.len()
					}
				}
			});

			// Clear mix buffer
			self.mixer.clear();

			// Mix each sound into the mix buffer
			for SoundInstance {asset_id, position} in self.active_sounds.iter_mut() {
				match asset_id.ty {
					SoundAssetType::Buffer => {
						let buffer = &self.buffers[asset_id.index];
						let buffer_consumption = self.mixer.mix_buffer(buffer, *position);
						*position += buffer_consumption;
					}

					SoundAssetType::FileStream => {
						let stream = &self.streams[asset_id.index];
						let buffer_consumption = self.mixer.mix_buffer(&stream.resident_buffer, *position);
						*position += buffer_consumption;

						// If the stream is running low on samples, queue it for update
						if !stream.fully_resident && stream.resident_buffer.samples() - *position < stream_prefetch_size {
							self.stream_updates.push(StreamUpdateRequest {
								index: asset_id.index,
								position: *position,
							});
						}
					}
				}
			}

			self.audio_queue.queue(&self.mixer.buffer());

			// Remove duplicate update requests - prioritising those with the highest `position`
			self.stream_updates.sort_by(|a, b| a.index.cmp(&b.index).then(b.position.cmp(&a.position)));
			self.stream_updates.dedup_by_key(|r| r.index);
		}


		for StreamUpdateRequest{index, position} in self.stream_updates.drain(..) {
			let stream = &mut self.streams[index];

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



#[derive(Copy, Clone, Debug)]
struct SoundInstance {
	asset_id: SoundAssetID,
	position: usize,
}


struct StreamUpdateRequest {
	index: usize,
	position: usize,
}

