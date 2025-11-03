use lost_cities_game_lib::Config;
use v1::bot::Bot;

fn main() {
    let config = Config::default();
    let bot = Bot::new(&config);
    println!("Hello, world!");
}
