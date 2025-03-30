# Jokers

API wrapper for https://v2.jokeapi.dev/

## Example

```rust
use jokers::JokeBuilder;
use jokers::params::Category;

#[tokio::main]
async fn main() {
    let builder = JokeBuilder::default().add_category(Category::Dark);
    println!("Call: {}", builder.url());

    match builder.fetch().await {
        Ok(joke) => println!("{}", joke),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```
