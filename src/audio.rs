use crate::prelude::*;

// https://splice.com/blog/dynamic-game-audio-mix/
// https://www.youtube.com/watch?v=UuqcgQxpfO8

pub mod buffer;
pub mod mixer;

use buffer::Buffer;
use mixer::Mixer;

#[derive(Copy, Clone, Debug)]
pub struct SoundAssetID(usize);


pub struct AudioSystem {
	audio_queue: sdl2::audio::AudioQueue<f32>,
	sound_assets: Vec<Buffer>,
	active_sounds: Vec<Sound>,

	mixer: Mixer,
}

impl AudioSystem {
	pub fn new(sdl_audio: sdl2::AudioSubsystem) -> Result<AudioSystem, Box<dyn Error>> {
		let desired_spec = sdl2::audio::AudioSpecDesired {
			freq: Some(44100),
			channels: Some(2),
			samples: None,
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
			sound_assets: Vec::new(),
			active_sounds: Vec::new(),
			
			mixer,
		})
	}

	pub fn register_buffer(&mut self, buffer: Buffer) -> SoundAssetID {
		let asset_id = SoundAssetID(self.sound_assets.len());
		self.sound_assets.push(buffer);
		asset_id
	}

	pub fn play_one_shot(&mut self, asset_id: SoundAssetID) {
		self.active_sounds.push(Sound {
			asset_id,
			position: 0,
		});
	}

	pub fn update(&mut self) {
		let spec = self.audio_queue.spec();

		let threshold_size = 1.0 / 60.0 * spec.freq as f32 * spec.channels as f32;
		let threshold_size = threshold_size as u32 * std::mem::size_of::<f32>() as u32;

		if self.audio_queue.size() < threshold_size {
			// Drop inactive sounds
			let sound_assets = &self.sound_assets;
			self.active_sounds.retain(|sound| {
				let buffer = &sound_assets[sound.asset_id.0];
				sound.position * buffer.channels < buffer.data.len()
			});

			// Clear mix buffer
			self.mixer.clear();

			// Mix each sound into the mix buffer
			for Sound {asset_id, position} in self.active_sounds.iter_mut() {
				let buffer = &self.sound_assets[asset_id.0];
				let buffer_consumption = self.mixer.mix_buffer(buffer, *position);
				*position += buffer_consumption;
			}

			self.audio_queue.queue(&self.mixer.buffer());
		}
	}
}



#[derive(Copy, Clone, Debug)]
pub struct Sound {
	asset_id: SoundAssetID,
	position: usize,
}