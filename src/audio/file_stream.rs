use crate::prelude::*;
use std::io::{self, Cursor, BufReader};
use std::fs::File;

use crate::audio::buffer::Buffer;

use lewton::inside_ogg::OggStreamReader;

pub struct FileStream {
	reader: OggStreamReader<StreamSource>,

	/// Samples that have already been read from the stream
	pub resident_buffer: Buffer,

	/// Whether `reader` has reached end of stream - no more samples will be read once this is true
	pub fully_resident: bool,
}

impl FileStream {
	pub fn from_vorbis_static(data: &'static [u8]) -> Result<FileStream, Box<dyn Error>> {
		let source = StreamSource::Static(Cursor::new(data));
		let reader = OggStreamReader::new(source)?;

		assert!(reader.ident_hdr.audio_sample_rate == 44100);
		assert!(matches!(reader.ident_hdr.audio_channels, 1 | 2));

		let resident_buffer = Buffer {
			data: Vec::with_capacity(reader.ident_hdr.audio_sample_rate as usize),
			channels: reader.ident_hdr.audio_channels as usize,
		};

		Ok(FileStream {
			reader,
			resident_buffer,
			fully_resident: false,
		})
	}

	pub fn from_vorbis_file(filepath: impl AsRef<std::path::Path>) -> Result<FileStream, Box<dyn Error>> {
		let file = File::open(filepath)?;

		let source = StreamSource::File(BufReader::new(file));
		let reader = OggStreamReader::new(source)?;

		assert!(reader.ident_hdr.audio_sample_rate == 44100);
		assert!(matches!(reader.ident_hdr.audio_channels, 1 | 2));

		let resident_buffer = Buffer {
			data: Vec::with_capacity(reader.ident_hdr.audio_sample_rate as usize),
			channels: reader.ident_hdr.audio_channels as usize,
		};

		Ok(FileStream {
			reader,
			resident_buffer,
			fully_resident: false,
		})
	}

	pub fn load_chunk(&mut self) -> Result<(), Box<dyn Error>> {
		if self.fully_resident {
			return Ok(())
		}

		if let Some(packet) = self.reader.read_dec_packet_itl()? {
			self.resident_buffer.data.extend_from_slice(&packet);
		} else {
			self.fully_resident = true;
		}

		Ok(())
	}
}



enum StreamSource {
	Static(Cursor<&'static [u8]>),
	File(BufReader<File>),
}

impl io::Read for StreamSource {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		match self {
			StreamSource::Static(cur) => cur.read(buf),
			StreamSource::File(file) => file.read(buf),
		}
	}
}


impl io::Seek for StreamSource {
	fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
		match self {
			StreamSource::Static(cur) => cur.seek(pos),
			StreamSource::File(file) => file.seek(pos),
		}
	}
}