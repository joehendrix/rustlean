use std::fs::{self, DirEntry};
use std::io;
use std::process::exit;
use std::path::{Path, PathBuf};

use cargo::core::external as cargo;
use clap;
use serde_json;

fn infer_file(entry: &DirEntry) -> Option<(String, PathBuf)> {
    let path = entry.path();
    path.file_stem()
        .and_then(|p| p.to_str())
        .map(|p| (p.to_owned(), path.clone()))
}

fn infer_subdirectory(entry: &DirEntry, main: &str) -> Option<(String, PathBuf)> {
    let path = entry.path();
    let main = path.join(main);
    let name = path.file_name().and_then(|n| n.to_str());
    match (name, main.exists()) {
        (Some(name), true) => Some((name.to_owned(), main)),
        _ => None,
    }
}

fn infer_any(entry: &DirEntry, extension: &str, main: &str) -> Option<(String, PathBuf)> {
    if entry.path().extension().and_then(|p| p.to_str()) == Some(extension) {
        infer_file(entry)
    } else if entry.file_type().map(|t| t.is_dir()).ok() == Some(true) {
        infer_subdirectory(entry, main)
    } else {
        None
    }
}

fn is_not_dotfile(entry: &DirEntry) -> bool {
    entry.file_name().to_str().map(|s| s.starts_with('.')) == Some(false)
}

fn infer_from_directory(directory: &Path, extension: &str, main: &str) -> Vec<(String, PathBuf)> {
    let entries = match fs::read_dir(directory) {
        Err(_) => return Vec::new(),
        Ok(dir) => dir,
    };

    entries
        .filter_map(|e| e.ok())
        .filter(is_not_dotfile)
        .filter_map(|d| infer_any(&d, extension, main))
        .collect()
}



fn inferred_bins(package_root: &Path, package_name: &str) -> Vec<(String, PathBuf)> {
    let main = package_root.join("src").join("main.lean");
    let mut result = Vec::new();
    if main.exists() {
        result.push((package_name.to_string(), main));
    }
    result.extend(infer_from_directory(&package_root.join("src").join("bin"), "lean", "main.lean"));

    result
}

fn option_arg(name: &'static str) -> clap::Arg<'static,'static> {
    clap::Arg::with_name(name).takes_value(true)
}

fn build_subcommand() -> clap::App<'static, 'static> {
    clap::SubCommand::with_name("build")
        .about("Build Lean package")
        .arg(clap::Arg::with_name("file").help("File to build"))
        .arg(option_arg("crate-name").long("crate-name").help("Crate name"))
        .arg(option_arg("crate-type").long("crate-type"))
        .arg(option_arg("edition").long("edition").help("Rust edition"))
        .arg(option_arg("emit").long("emit"))
        .arg(option_arg("error-format").long("error-format").help("Format name"))
        .arg(option_arg("json").long("json"))
        .arg(option_arg("out-dir").long("out-dir"))
        .arg(option_arg("option").short("C").number_of_values(1).multiple(true))
        .arg(option_arg("link").short("L"))
        .arg(option_arg("features").short("Z").help("Enable specific features"))

}

fn handle_build(args: &clap::ArgMatches<'_>) -> io::Result<()> {
    eprintln!("Parsed args: {:?}", args);
    Ok(())
}


fn target_subcommand() -> clap::App<'static, 'static> {
    clap::SubCommand::with_name("targets")
        .about("Infer build targets")
}

fn handle_targets() -> io::Result<()> {
    let cmd : cargo::TargetRequest = serde_json::from_reader(io::stdin())?;

    let mut targets = Vec::new();

    for (name, src_path) in inferred_bins(Path::new(&cmd.package_root), &cmd.package_name) {
        targets.push(cargo::ExtTarget {
            kind: cargo::ExtTargetKind::Bin,
            name,
            src_path: src_path.into_os_string(),
        })
    }



    let warnings = vec![];
    let errors = vec![];

    let res = cargo::TargetResult::Success { targets, warnings, errors };
    serde_json::to_writer(io::stdout(), &res)?;
    Ok(())
}

fn handle_unknown(cmd: &str) -> io::Result<()> {
    eprintln!("Unknown command: {}", cmd);
    exit(-1)
}

fn handle_missing() -> io::Result<()> {
    eprintln!("Missing command");
    exit(-1)
}


fn main() -> io::Result<()> {
    eprintln!("Args: {:?}", std::env::args());
    let m = clap::App::new("cargobuild-lean")
        .subcommand(build_subcommand())
        .subcommand(target_subcommand()).get_matches();
    match m.subcommand() {
        ("build", Some(matches)) => handle_build(matches),
        ("targets", Some(_matches)) => handle_targets(),
        ("", _) => handle_missing(),
        (cmd, _) => handle_unknown(cmd),
    }
}
