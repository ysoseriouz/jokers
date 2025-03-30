mod builder;
mod error;
mod joke;

pub mod params;

pub use builder::JokeBuilder;
pub use error::Error;
pub use joke::{Joke, Result};
