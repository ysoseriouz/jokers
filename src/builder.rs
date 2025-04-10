use crate::*;

pub struct JokeBuilder {
    base_url: String,
    categories: Selector<Category>,
    flags: Selector<Flag>,
    format: Format,
    joke_type: Selector<JokeType>,
    amount: u8,
}

impl JokeBuilder {
    pub fn new() -> Self {
        let mut categories = Selector::new(&[Category::Any]);
        categories.select(Category::Any);

        Self {
            base_url: String::from("https://v2.jokeapi.dev/joke/"),
            categories,
            flags: Selector::new(&[]),
            format: Format::Json,
            joke_type: Selector::new(&[JokeType::Single, JokeType::Twopart]),
            amount: 1,
        }
    }

    pub fn url(&self) -> String {
        format!(
            "{}{}?blacklistFlags={}&format={}&type={}&amount={}",
            self.base_url, self.categories, self.flags, self.format, self.joke_type, self.amount
        )
    }

    pub fn add_category(mut self, category: Category) -> Self {
        self.categories.select(category);
        self
    }

    pub fn add_flag(mut self, flag: Flag) -> Self {
        self.flags.select(flag);
        self
    }

    pub fn format(mut self, format: Format) -> Self {
        self.format = format;
        self
    }

    pub fn joke_type(mut self, jt: JokeType) -> Self {
        self.joke_type.select(jt);
        self
    }

    pub fn amount(mut self, amount: u8) -> Self {
        self.amount = amount;
        self
    }

    #[cfg(feature = "async")]
    pub async fn fetch(&self) -> Result<Vec<Joke>> {
        use crate::joke::parse_joke;

        let resp = reqwest::Client::new()
            .get(self.url())
            .send()
            .await?
            .text()
            .await?;

        parse_joke(&resp, &self.format, self.amount)
    }

    #[cfg(feature = "blocking")]
    pub fn get(&self) -> Result<Vec<Joke>> {
        use crate::joke::parse_joke;

        let resp = reqwest::blocking::Client::new()
            .get(self.url())
            .send()?
            .text()?;

        parse_joke(&resp, &self.format, self.amount)
    }
}

impl Default for JokeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_joke_builder_url() {
        let builder = JokeBuilder::default();
        assert_eq!(
            builder.url(),
            "https://v2.jokeapi.dev/joke/Any?blacklistFlags=&format=json&type=&amount=1"
        );

        let builder = JokeBuilder::new()
            .add_category(Category::Dark)
            .add_flag(Flag::Nsfw);
        assert_eq!(
            builder.url(),
            "https://v2.jokeapi.dev/joke/Dark?blacklistFlags=nsfw&format=json&type=&amount=1"
        );

        let builder = JokeBuilder::new()
            .add_category(Category::Dark)
            .add_flag(Flag::Nsfw)
            .format(Format::Txt);
        assert_eq!(
            builder.url(),
            "https://v2.jokeapi.dev/joke/Dark?blacklistFlags=nsfw&format=txt&type=&amount=1"
        );

        let builder = JokeBuilder::new()
            .add_category(Category::Dark)
            .add_flag(Flag::Nsfw)
            .joke_type(JokeType::Single)
            .format(Format::Txt);
        assert_eq!(
            builder.url(),
            "https://v2.jokeapi.dev/joke/Dark?blacklistFlags=nsfw&format=txt&type=single&amount=1"
        );

        let builder = JokeBuilder::new()
            .add_category(Category::Dark)
            .add_flag(Flag::Nsfw)
            .format(Format::Txt)
            .amount(10);
        assert_eq!(
            builder.url(),
            "https://v2.jokeapi.dev/joke/Dark?blacklistFlags=nsfw&format=txt&type=&amount=10"
        );
    }
}
