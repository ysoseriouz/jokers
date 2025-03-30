use crate::{Error, params::*};
use serde::Deserialize;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Joke {
    Single(String),
    Twopart(String, String),
}

pub fn parse_joke(resp: &str, format: &Format) -> Result<Joke> {
    match format {
        Format::Json => parse_json(resp),
        #[cfg(feature = "yaml")]
        Format::Yaml => parse_yaml(resp),
        _ => parse_json(resp),
    }
}

fn parse_json(resp: &str) -> Result<Joke> {
    let joke_resp: JokeResponse = serde_json::from_str(resp)?;

    if joke_resp.error {
        Err(Error::ApiResponse("error:true".to_string()))
    } else {
        Ok(joke_resp.joke())
    }
}

#[cfg(feature = "yaml")]
fn parse_yaml(resp: &str) -> Result<Joke> {
    let joke_resp: JokeResponse = serde_yaml::from_str(resp)?;
    Ok(joke_resp.joke())
}

#[derive(Debug, Deserialize)]
struct JokeResponse {
    error: bool,
    #[serde(rename = "type")]
    joke_type: JokeType,
    // Single joke
    joke: Option<String>,
    // Two-part joke
    setup: Option<String>,
    delivery: Option<String>,
}

impl JokeResponse {
    pub fn joke(&self) -> Joke {
        match self.joke_type {
            JokeType::Single => Joke::Single(self.joke.clone().unwrap()),
            JokeType::Twopart => {
                Joke::Twopart(self.setup.clone().unwrap(), self.delivery.clone().unwrap())
            }
        }
    }
}

impl fmt::Display for Joke {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Single(joke) => write!(f, "{}", joke),
            Self::Twopart(setup, delivery) => write!(f, "{}\n{}", setup, delivery),
        }
    }
}
