use bot_v1::bot::Bot;
use lost_cities_game_lib::Config;

fn main() {
    let config = Config::default();
    let bot = Bot::new(&config);
    println!("Hello, world!");
}
