use lost_cities_game_lib::Config;

pub struct Bot<'c> {
    config: &'c Config,
}

impl<'c> Bot<'c> {
    pub fn new(config: &'c Config) -> Self {
        Self { config }
    }
}
