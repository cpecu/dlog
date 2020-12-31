use clap::{ArgMatches, FromArgMatches};
use super::Cmd;
use colored::{Color, Colorize};

#[derive(Debug)]
pub enum UserCmd {
    Help
}

impl Default for UserCmd {
    fn default() -> Self {
        Self::Help
    }
}

impl Cmd for UserCmd {

    fn run(&self) {
        println!("{}", format!("Running User cmd...")
            .color(Color::BrightCyan))
    }

    fn cmd() -> clap::App<'static> {
        clap::App::new("user")
            .about("users")
            .subcommands(vec![
                Self::help_cmd(),
            ])
            .args(&vec![
                clap::Arg::new("help")
                    .about("Prints help for the User command")
            ])
    }

    fn print_help() {
        let help = format!("User
        ").color(Color::BrightCyan);
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

impl FromArgMatches for UserCmd {
    fn from_arg_matches(_matches: &ArgMatches) -> Self {
        Self::print_help();
        Self::default()
    }
}

impl clap::Subcommand for UserCmd {
    fn from_subcommand(sub: Option<(&str, &ArgMatches)>)
        -> Option<Self>
    {
        let (sub, args) = sub.unwrap();
        if sub == "user" {
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
