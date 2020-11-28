use colored::{Color, Colorize};
use pico_args::Arguments;
use crate::cmd::SubCommand;

#[derive(Debug,)]
pub struct Config {
    key: String,
}

impl Config {

    pub fn init(key: String) -> Self {
        Self { key }
    }
}

impl SubCommand for Config {

    fn new(key: String) -> Self {
        Self { key }
    }

    fn insert(key: String, val: String) -> Result<Self, pico_args::Error> {
        Ok(Self::default())
    }

    fn color() -> Color { Color::BrightBlue }

    fn with_args(key: String, args: &mut Arguments) -> Result<Self, pico_args::Error> {
        if args.clone().free()?.is_empty() {
            return Self::init(key).prompt_value();
        }
        println!("{}", format!("C: {:#?}", key).color(Config::color()));
        println!("{:#?}", args);
        Ok(Self { key })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::prompt_key().unwrap()
    }
}

impl ToString for Config {
    fn to_string(&self) -> String {
        "item".to_string()
    }
}
