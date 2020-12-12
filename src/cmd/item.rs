use super::{
    Record, Cmd,
};
use clap::{ArgMatches, FromArgMatches};

#[derive(Default, Debug)]
pub struct Item {
    name: String,
}

impl Cmd for Item{

    fn cmd() -> clap::App<'static> {
        clap::App::new("item")
            .about("items")
            .subcommands(vec![
                clap::App::new("new")
            ])
            .args(&vec![
                clap::Arg::new("help")
            ])
    }

    fn run(&self) {

    }

    fn print_help() {
        println!("Item help")
    }

}

impl FromArgMatches for Item {
    fn from_arg_matches(_matches: &ArgMatches) -> Self {
        Self::print_help();
        println!("Item");
        Self::default()
    }
}
