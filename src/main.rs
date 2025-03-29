use jokers::JokeBuilder;
use jokers::params::*;

fn main() {
    let builder = JokeBuilder::default().add_category(Category::Dark);
    println!("{}", builder.url());
}
