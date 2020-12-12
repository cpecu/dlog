use clap::{ArgMatches, FromArgMatches, Subcommand};
use colored::{Color, Colorize, Style, Styles};
use super::Cmd;

#[derive(Debug)]
pub enum RecordCmd {
    New(Option<Record>),
    List
}

impl Default for RecordCmd {
    fn default() -> Self {
        RecordCmd::New(None)
    }
}

impl Cmd for RecordCmd {

    fn cmd() -> clap::App<'static> {
        clap::App::new("record")
            .about("items")
            .subcommands(vec![
                clap::App::new("new")
            ])
            .args(&vec![
                clap::Arg::new("help")
            ])

    }

    fn run(&self) {
        println!("{}", format!("Running record cmd...")
            .color(Color::BrightGreen))
    }

    fn print_help() {
        let help = format!("
            RECORD: Define a grouping of items to keep track \n
                    of in different records, and assign diff-\n
                    erent actions and attributes based on... \n
        ").color(Color::BrightGreen);
        println!("> {}", help)
    }
}

impl FromArgMatches for RecordCmd {
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        Self::default()
    }
}

// NOTE Don't think this is supposed to be implemented yet
//      until functionality is finished for clap 3.0 but
//      implementing anyways
impl Subcommand for RecordCmd {
    fn from_subcommand(sub: Option<(&str, &ArgMatches)>) -> Option<Self> {
        if let Some((sub, args)) = sub {
            if sub == "record" {
                Some(Self::from_arg_matches(args))
            } else {
                None
            }
        } else { None }
    }

    fn augment_subcommands(app: clap::App<'_>) -> clap::App<'_>
    {
        app
    }
}

#[derive(Debug)]
pub struct Record {
    pub name: String,
}

impl Default for Record {
    fn default() -> Self {
        Self { name: "Uncategorized".into() }
    }
}

impl FromArgMatches for Record {
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        Self::default()
    }
}
