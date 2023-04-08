use std::{
    fs::{create_dir_all, read_dir, read_to_string, File, ReadDir},
    path::PathBuf,
    process::exit,
};

use clap::{
    error::{ErrorKind, Result},
    ArgMatches, Error,
};
use leviathan_compiler::{
    compiler::{CompileTask, Status},
    parser::{ast::build_ast, tokenizer::tokenize},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub package: PackageConfig,
}

#[derive(Deserialize)]
pub struct PackageConfig {
    pub name: String,
    pub version: String,
    #[serde(rename = "binary")]
    pub binary_path: Option<String>,
}

struct SourceFile {
    main: bool,
    module_name: String,
    path: PathBuf,
}

enum LoadError {
    Parse(leviathan_compiler::parser::error::Error),
    Compile(leviathan_compiler::compiler::error::Error),
}

pub fn build(matches: &ArgMatches) -> Result<()> {
    let config = read_to_string("build.lvt.toml");
    if let Err(err) = config {
        if let std::io::ErrorKind::NotFound = err.kind() {
            return Err(Error::raw(ErrorKind::Io, "Could not find a build.lvt.toml"));
        }
        return Err(err.into());
    }
    let config = config.unwrap();
    let config = toml::from_str(&config);
    if let Err(err) = config {
        return Err(Error::raw(ErrorKind::Format, err));
    }
    let config: Config = config.unwrap();
    let source_dir = read_dir("src");
    if let Err(err) = source_dir {
        if let std::io::ErrorKind::NotFound = err.kind() {
            return Err(Error::raw(ErrorKind::Io, "Could not find src directory"));
        }
        return Err(err.into());
    }
    let mut source_files = Vec::with_capacity(0);
    let mut main_found = false;
    collect_dir(&mut source_files, source_dir.unwrap(), &mut main_found)?;
    if !main_found {
        return Err(Error::raw(
            ErrorKind::MissingRequiredArgument,
            "No main module was found",
        ));
    }
    let mut task = CompileTask::default();
    let mut errors = Vec::with_capacity(0);
    for source_file in source_files {
        let source = read_to_string(&source_file.path)?;
        let Some(path) = source_file.path.to_str() else {
            return Err(Error::raw(ErrorKind::InvalidUtf8, "File with invalid name"));
        };
        let result = tokenize(source_file.module_name, path.to_string(), source);
        if let Err(err) = result {
            errors.push(LoadError::Parse(err));
            continue;
        }
        let ast = build_ast(result.unwrap());
        if let Err(err) = ast {
            errors.push(LoadError::Parse(err));
            continue;
        }
        let result = task.include(ast.unwrap(), source_file.main);
        if let Err(err) = result {
            errors.push(LoadError::Compile(err));
            task.status = Status::Open;
            continue;
        }
    }
    if !errors.is_empty() {
        for err in errors {
            match err {
                LoadError::Parse(err) => err.report(),
                LoadError::Compile(err) => err.report(),
            }
        }
        exit(1);
    }
    if let Err(err) = task.compile() {
        err.abort();
    };
    if let Err(err) = task.filter() {
        err.abort();
    };
    create_dir_all("out")?;
    let mut binary = File::create(
        config
            .package
            .binary_path
            .unwrap_or_else(|| format!("out/{}.bin", config.package.name)),
    )?;
    let mut offset_out = if !matches.get_flag("no-offsets") {
        Some(File::create(format!("out/{}.map", config.package.name))?)
    } else {
        None
    };
    if let Err(err) = task.assemble(&mut binary, offset_out.as_mut()) {
        err.abort();
    };
    Ok(())
}

fn collect_dir(
    source_files: &mut Vec<SourceFile>,
    source_dir: ReadDir,
    main_found: &mut bool,
) -> Result<()> {
    for entry in source_dir {
        let entry = entry?;
        let type_ = entry.file_type()?;
        if type_.is_file() {
            let os_file_name = entry.file_name();
            let Some(file_name) = os_file_name.to_str() else {
                return Err(Error::raw(ErrorKind::InvalidUtf8, "File with invalid name"));
            };
            let Some(stripped_name) = file_name.strip_suffix(".lvt") else {
                continue;
            };
            if stripped_name.is_empty() {
                return Err(Error::raw(ErrorKind::Io, "File has empty name"));
            }
            for c in stripped_name.chars() {
                match c {
                    '#' | ';' | '"' | '\\' | '(' | ')' | '[' | ']' | '{' | '}' => {
                        return Err(Error::raw(
                            ErrorKind::Io,
                            format!("File with invalid character '{c}' in name"),
                        ));
                    }
                    c if c.is_whitespace() => {
                        return Err(Error::raw(ErrorKind::Io, "File with whitespace in name"));
                    }
                    _ => {}
                }
            }
            let path = entry.path();
            let main = stripped_name == "main";
            if main {
                *main_found = true;
            }
            source_files.push(SourceFile {
                main,
                module_name: stripped_name.to_string(),
                path,
            });
            continue;
        }
        if type_.is_dir() {
            let source_dir = read_dir(entry.path())?;
            collect_dir(source_files, source_dir, main_found)?;
            continue;
        }
    }
    Ok(())
}
