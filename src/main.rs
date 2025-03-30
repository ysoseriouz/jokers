use jokers::JokeBuilder;
use jokers::params::*;

fn main() {
    let builder = JokeBuilder::default().add_category(Category::Dark);
    println!("Call: {}", builder.url());

    #[cfg(feature = "blocking")]
    match builder.get() {
        Ok(joke) => println!("{}", joke),
        Err(e) => eprintln!("Error: {}", e),
    }
}
