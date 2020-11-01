#![deny(rust_2018_idioms, future_incompatible)]

pub mod types;
pub mod importer;

pub use self::types::*;
pub use self::importer::*;

pub const DEFAULT_COLOR_DATA_NAME: &'static str = "Col";

pub type ToyResult<T> = Result<T, failure::Error>;
