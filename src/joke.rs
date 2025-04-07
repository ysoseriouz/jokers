use crate::{Error, params::*};
use serde::Deserialize;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Deserialize)]
pub struct Joke {
    pub category: Category,
    #[serde(rename = "type")]
    pub joke_type: JokeType,
    // Single joke
    #[serde(default = "missing_joke")]
    pub joke: String,
    // Two-part joke
    #[serde(default = "missing_joke")]
    pub setup: String,
    #[serde(default = "missing_joke")]
    pub delivery: String,
    pub safe: bool,
}

pub fn parse_joke(resp: &str, format: &Format, amount: u8) -> Result<Vec<Joke>> {
    match format {
        Format::Json => parse::<JsonBackend>(resp, amount),
        #[cfg(feature = "yaml")]
        Format::Yaml => parse::<YamlBackend>(resp, amount),
        _ => parse::<JsonBackend>(resp, amount),
    }
}

trait Backend: Sized {
    fn from_str<T: for<'de> Deserialize<'de>>(s: &str) -> Result<T>;
}

struct JsonBackend;

impl Backend for JsonBackend {
    fn from_str<T: for<'de> Deserialize<'de>>(s: &str) -> Result<T> {
        serde_json::from_str(s).map_err(Error::from)
    }
}

#[cfg(feature = "yaml")]
struct YamlBackend;

#[cfg(feature = "yaml")]
impl Backend for YamlBackend {
    fn from_str<T: for<'de> Deserialize<'de>>(s: &str) -> Result<T> {
        serde_yaml::from_str(s).map_err(Error::from)
    }
}

fn parse<B: Backend>(resp: &str, amount: u8) -> Result<Vec<Joke>> {
    if amount == 1 {
        let joke_resp: JokeResponse = B::from_str(resp)?;
        joke_resp.is_ok()?;

        Ok(joke_resp.into())
    } else {
        let joke_resp: MultiJokeResponse = B::from_str(resp)?;
        joke_resp.is_ok()?;

        Ok(joke_resp.into())
    }
}

#[derive(Debug, Deserialize)]
struct JokeResponse {
    error: bool,
    #[serde(flatten)]
    joke: Joke,
}

#[derive(Debug, Deserialize)]
struct MultiJokeResponse {
    error: bool,
    #[allow(dead_code)]
    amount: u8,
    jokes: Vec<Joke>,
}

trait ApiOk {
    fn is_ok(&self) -> Result<()>;
}

impl ApiOk for JokeResponse {
    fn is_ok(&self) -> Result<()> {
        if self.error {
            Err(Error::ApiResponse("error:true".to_string()))
        } else {
            Ok(())
        }
    }
}

impl ApiOk for MultiJokeResponse {
    fn is_ok(&self) -> Result<()> {
        if self.error {
            Err(Error::ApiResponse("error:true".to_string()))
        } else {
            Ok(())
        }
    }
}

impl From<JokeResponse> for Vec<Joke> {
    fn from(item: JokeResponse) -> Self {
        vec![item.joke]
    }
}

impl From<MultiJokeResponse> for Vec<Joke> {
    fn from(item: MultiJokeResponse) -> Self {
        item.jokes
    }
}

fn missing_joke() -> String {
    "<missing joke>".to_string()
}

impl fmt::Display for Joke {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.joke_type {
            JokeType::Single => write!(f, "{}", self.joke),
            JokeType::Twopart => write!(f, "{}\n{}", self.setup, self.delivery),
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
