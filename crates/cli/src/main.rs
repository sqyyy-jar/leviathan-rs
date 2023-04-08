pub mod disasm;
pub mod project;

use std::path::PathBuf;

use clap::{arg, command, crate_version, value_parser, Command};
use disasm::disasm;
use project::build;

const BUILD_DATE: &str = env!("BUILD_DATE");

fn main() {
    let mut cmd = Command::new("lvt")
        .bin_name("lvt")
        .subcommand_required(true)
        .subcommands([
            command!("version").alias("v").about("Shows the version"),
            command!("build")
                .alias("b")
                .about("Build a project")
                .arg(arg!(--"no-offsets").required(false)),
            command!("disasm")
                .alias("d")
                .about("Disassemble a binary")
                .args([
                    arg!(<FILE>)
                        .value_parser(value_parser!(PathBuf))
                        .required(true),
                    arg!(<OFFSETS>)
                        .value_parser(value_parser!(PathBuf))
                        .required(false),
                ]),
        ]);
    let matches = cmd.get_matches_mut();
    match matches.subcommand() {
        Some(("version", _)) => {
            println!("leviathan-cli version {} {BUILD_DATE}", crate_version!());
        }
        Some(("build", matches)) => {
            build(matches).unwrap_or_else(|err| err.format(&mut cmd).exit())
        }
        Some(("disasm", matches)) => {
            disasm(matches).unwrap_or_else(|err| err.format(&mut cmd).exit())
        }
        _ => unreachable!("clap should ensure we don't get here"),
    };
}
