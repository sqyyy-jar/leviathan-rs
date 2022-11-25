use clap::{arg, Command};
use leviathan_compiler::{resolver::CompileTask, rt};
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
            let source_result = leviathan_parser::source_parser::parse(&source);
            if source_result.is_err() {
                println!("{:#?}", source_result.unwrap_err());
                return;
            }
            let structure_result = leviathan_parser::structure_parser::parse(source_result.unwrap().0);
            if structure_result.is_err() {
                println!("{:#?}", structure_result.unwrap_err());
                return;
            }
            let mut compile_task = CompileTask::new();
            rt::load_rt(&mut compile_task);
            compile_task.load_structure(structure_result.unwrap()).unwrap();
            compile_task.validate().unwrap();
            println!("Dump: {:#?}", compile_task.root);
            //println!("{:#?}", structure_result.unwrap());
        }
        _ => {
            unreachable!();
        }
    }
}
