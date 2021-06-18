
use crate::audio::buffer::Buffer;


pub struct Mixer {
	mix_buffer: Vec<f32>,
	gain: f32,
}

impl Mixer {
	pub fn new(buffer_size: usize) -> Mixer {
		Mixer {
			mix_buffer: vec![0.0; buffer_size],
			gain: 0.2,
		}
	}

	pub fn buffer_size(&self) -> usize { self.mix_buffer.len() }

	pub fn clear(&mut self) {
		for sample in self.mix_buffer.iter_mut() {
			*sample = 0.0;
		}
	}

	pub fn mix_buffer(&mut self, buffer: &Buffer, position: usize) -> usize {
		let buffer_size = buffer.data.len() / buffer.channels;
		let buffer_remaining = buffer_size - position;
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

	pub fn buffer(&self) -> &[f32] { &self.mix_buffer }
}


fn i16_to_f32(i: i16) -> f32 {
	let f = i as f32 / i16::MAX as f32;
	f.clamp(-1.0, 1.0)
}
