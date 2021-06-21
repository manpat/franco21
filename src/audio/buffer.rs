use crate::prelude::*;
use std::io::Cursor;

use lewton::inside_ogg::OggStreamReader;
use lewton::samples::Sample;

/// A complete sound ready to be played in its entirety
pub struct Buffer {
	pub data: Vec<i16>,
	pub channels: usize,
}

impl Buffer {
	pub fn from_vorbis(data: &[u8]) -> Result<Buffer, Box<dyn Error>> {
		// https://github.com/RustAudio/lewton/blob/master/examples/player.rs
		let cursor = Cursor::new(data);
		let mut reader = OggStreamReader::new(cursor)?;

		dbg!(&reader.ident_hdr.audio_sample_rate);
		dbg!(&reader.ident_hdr.audio_channels);

		assert!(reader.ident_hdr.audio_sample_rate == 44100);
		assert!(matches!(reader.ident_hdr.audio_channels, 1 | 2));

		let mut data = Vec::new();
		while let Some(packet) = reader.read_dec_packet_itl()? {
			data.extend_from_slice(&packet);
		}

		Ok(Buffer {
			data,
			channels: reader.ident_hdr.audio_channels as usize,
		})
	}

	pub fn from_samples(data: impl Iterator<Item=f32>, channels: usize) -> Buffer {
		let data = data.map(i16::from_float).collect();

		Buffer {
			data,
			channels,
		}
	}

	pub fn from_mono_samples(data: impl Iterator<Item=f32>) -> Buffer {
		Buffer::from_samples(data, 1)
	}

	pub fn from_stereo_samples(data: impl Iterator<Item=f32>) -> Buffer {
		Buffer::from_samples(data, 2)
	}

	pub fn samples(&self) -> usize {
		self.data.len() / self.channels
	}
}