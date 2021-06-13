use crate::prelude::*;


pub struct AudioSystem {
	audio_queue: sdl2::audio::AudioQueue<f32>,
}

impl AudioSystem {
	pub fn new(sdl_audio: sdl2::AudioSubsystem) -> Result<AudioSystem, Box<dyn Error>> {
		let desired_spec = sdl2::audio::AudioSpecDesired {
			freq: Some(44100),
			channels: Some(1),
			samples: None,
		};

		let audio_queue = sdl_audio.open_queue(None, &desired_spec)?;
		audio_queue.resume();

		Ok(AudioSystem {
			audio_queue
		})
	}

	pub fn update(&mut self) {
		let spec = self.audio_queue.spec();

		let threshold_size = 3.0 / 60.0 * spec.freq as f32;
		let threshold_size = threshold_size as u32 * std::mem::size_of::<f32>() as u32;

		if self.audio_queue.size() < threshold_size {
			let data: Vec<_> = (0..threshold_size*3)
				.map(|x| (x as f32 * 110.0 / spec.freq as f32 * PI).sin())
				.collect();

			self.audio_queue.queue(&data);
		}


		dbg!(self.audio_queue.size(), self.audio_queue.size() as f32 / spec.freq as f32);
	}
}