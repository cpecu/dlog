use crate::{
    models::attrib::Attrib,
    cmd::Cmd,
};
use clap::{ArgMatches, FromArgMatches};
use colored::{Colorize, Color};

#[derive(Debug)]
pub enum AttribCmd {
    New(Option<Attrib>),
    Help,
    List,
}

impl Default for AttribCmd {
    fn default() -> Self {
        Self::Help
    }
}


impl Cmd for AttribCmd {

    fn cmd() -> clap::App<'static> {
        clap::App::new("attrib")
            .about("attribs")
            .subcommands(vec![
                clap::App::new("new"),
                Self::help_cmd(),
            ])
            .args(&vec![
                clap::Arg::new("name")
            ])
    }

    fn run(&self) {
        println!("{}", format!("Running attrib cmd...")
            .color(Color::BrightRed))

    }

    fn print_help() {
        let help = format!("
            ATTRIB: The attribute command allows for the look-\n
                    up or tagging of facts/items/records by u-\n
                    ser defined or automatically defined...\n
        ").color(Color::BrightRed);
        println!("> {}", help)
    }

    fn help_cmd() -> clap::App<'static> {
        clap::App::new("fact_help")
            .about("Prints help command for fact")
            .long_flag("help")
            .short_flag('h')
            .long_about("Prints the help information")
    }

}

impl FromArgMatches for AttribCmd {
    fn from_arg_matches(_matches: &ArgMatches) -> Self {
        Self::print_help();
        Self::default()
    }
}

impl clap::Subcommand for AttribCmd {
    fn from_subcommand(sub: Option<(&str, &ArgMatches)>)
        -> Option<Self>
    {
        let (sub, args) = sub.unwrap();
        if sub == "attrib" {
            Some(Self::from_arg_matches(args))
        } else {
            None
        }
    }

    fn augment_subcommands(app: clap::App<'_>) -> clap::App<'_>
    {
        app
    }
}
