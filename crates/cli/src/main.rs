pub mod project;

use clap::{command, crate_version, Command};
use project::build;

const BUILD_DATE: &str = env!("BUILD_DATE");

fn main() {
    let mut cmd = Command::new("lvt")
        .bin_name("lvt")
        .subcommand_required(true)
        .subcommands([
            command!("version").alias("v").about("Shows the version"),
            command!("build").alias("b").about("Build a project"),
        ]);
    let matches = cmd.get_matches_mut();
    match matches.subcommand() {
        Some(("version", _)) => {
            println!("leviathan-cli version {} {BUILD_DATE}", crate_version!());
        }
        Some(("build", matches)) => {
            build(matches).unwrap_or_else(|err| err.format(&mut cmd).exit())
        }
        _ => unreachable!("clap should ensure we don't get here"),
    };
}