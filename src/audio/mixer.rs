
use crate::audio::buffer::Buffer;
use crate::audio::bus::Bus;

pub const NUM_CHANNELS: usize = 2;

pub(super) struct Mixer {
	mix_buffer: Vec<f32>,
	gain: f32,
}

impl Mixer {
	pub fn new(buffer_samples: usize) -> Mixer {
		Mixer {
			mix_buffer: vec![0.0; buffer_samples * NUM_CHANNELS],
			gain: 1.0,
		}
	}

	pub fn buffer_samples(&self) -> usize { self.mix_buffer.len() / NUM_CHANNELS }
	pub fn buffer(&self) -> &[f32] { &self.mix_buffer }

	pub fn clear(&mut self) {
		for sample in self.mix_buffer.iter_mut() {
			*sample = 0.0;
		}
	}

	pub fn set_gain(&mut self, gain: f32) {
		self.gain = gain;
	}

	pub fn gain(&self) -> f32 { self.gain }

	pub fn mix_buffer(&mut self, buffer: &Buffer, position: usize) -> usize {
		let buffer_samples = buffer.data.len() / buffer.channels;
		let buffer_remaining = buffer_samples - position;
		let buffer_consumption = buffer_remaining.min(self.mix_buffer.len() / 2);

		let mix_chunks = self.mix_buffer.array_chunks_mut::<2>();

		match buffer.channels {
			1 => {
				for ([left, right], sample) in mix_chunks.zip(&buffer.data[position..]) {
					let sample = i16_to_f32(*sample) * self.gain;
					*left += sample;
					*right += sample;
				}
			}

			2 => {
				let buffer_chunks = buffer.data[position*2..].array_chunks::<2>();

				for ([mix_left, mix_right], [buf_left, buf_right]) in mix_chunks.zip(buffer_chunks) {
					*mix_left += i16_to_f32(*buf_left) * self.gain;
					*mix_right += i16_to_f32(*buf_right) * self.gain;
				}
			}

			n => panic!("Buffers with {} channels not supported", n),
		}

		buffer_consumption
	}

	pub fn mix_bus(&mut self, bus: &Bus) {
		assert!(self.mix_buffer.len() == bus.buffer().len(), "Bus buffer size mismatch");

		for (out_sample, in_sample) in self.mix_buffer.iter_mut().zip(bus.buffer()) {
			*out_sample += *in_sample * self.gain;
		}
	}
}


fn i16_to_f32(i: i16) -> f32 {
	let f = i as f32 / i16::MAX as f32;
	f.clamp(-1.0, 1.0)
}
