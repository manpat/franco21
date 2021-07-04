use crate::prelude::*;

// https://splice.com/blog/dynamic-game-audio-mix/
// https://www.youtube.com/watch?v=UuqcgQxpfO8

pub mod system;
pub mod file_stream;
pub mod buffer;
pub mod mixer;

pub use system::{AudioSystem, SoundAssetID};
pub use file_stream::FileStream;
pub use buffer::Buffer;
