use clap::{arg, Command};
use std::fs;

fn cli() -> Command {
    Command::new("lvt")
        .about("A CLI for the Leviathan programming language")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("run")
                .about("Run a Leviathan file")
                .arg(arg!(<FILE> "The file to run"))
                .arg_required_else_help(true),
        )
}

fn main() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("run", sub_matches)) => {
            let file_path = sub_matches.get_one::<String>("FILE").expect("required");
            let source = fs::read_to_string(file_path);
            if let Err(err) = source {
                println!("Could not load file: {}", err);
                return;
            }
            let source = source.unwrap();
            println!(
                "{:#?}",
                leviathan_parser::source_parser::Parser::parse(&source)
            );
        }
        _ => {
            unreachable!();
        }
    }
}
