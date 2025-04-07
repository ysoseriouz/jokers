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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_joke_json() {
        let json = r#"
        {
            "error": false,
            "category": "Programming",
            "type": "single",
            "joke": "test joke",
            "safe": true
        }
        "#;
        let result = parse_joke(json, &Format::Json, 1);
        assert!(result.is_ok());

        let jokes = result.unwrap();
        assert_eq!(jokes.len(), 1);
        assert_eq!(jokes[0].category, Category::Programming);
        assert_eq!(jokes[0].joke_type, JokeType::Single);
        assert_eq!(jokes[0].joke, "test joke");
    }

    #[test]
    fn test_parse_multi_joke_json() {
        let json = r#"
        {
            "error": false,
            "amount": 2,
            "jokes": [
                {
                    "category": "Programming",
                    "type": "single",
                    "joke": "joke1",
                    "safe": true
                },
                {
                    "category": "Misc",
                    "type": "twopart",
                    "setup": "joke2 setup",
                    "delivery": "joke2 delivery",
                    "safe": true
                }
            ]
        }
        "#;

        let result = parse_joke(json, &Format::Json, 2);
        assert!(result.is_ok());

        let jokes = result.unwrap();
        assert_eq!(jokes.len(), 2);

        assert_eq!(jokes[0].category, Category::Programming);
        assert_eq!(jokes[0].joke_type, JokeType::Single);
        assert_eq!(jokes[0].joke, "joke1");
        assert_eq!(jokes[1].category, Category::Misc);
        assert_eq!(jokes[1].joke_type, JokeType::Twopart);
        assert_eq!(jokes[1].setup, "joke2 setup");
        assert_eq!(jokes[1].delivery, "joke2 delivery");
    }

    #[test]
    fn test_parse_error_response() {
        let json = r#"
        {
            "error": true,
            "category": "Programming",
            "type": "single",
            "joke": "This won't be returned",
            "safe": true
        }
        "#;

        let result = parse_joke(json, &Format::Json, 1);
        assert!(result.is_err());

        if let Err(Error::ApiResponse(msg)) = result {
            assert_eq!(msg, "error:true");
        } else {
            panic!("Expected ApiResponse error");
        }
    }
}
