#![doc = include_str!("../README.md")]

use std::{
    env,
    error::Error,
    path::{Path, PathBuf},
    process::Command,
};

/// Finds the `xfile` starting in the current working directory.
fn find_xfile() -> Result<PathBuf, Box<dyn Error>> {
    let cwd = env::current_dir()?;
    let mut current = cwd.as_path();
    loop {
        let xfile = current.join("xfile");
        if xfile.is_file() {
            break Ok(xfile);
        }
        if let Some(parent) = current.parent() {
            current = parent;
        } else {
            break Err("Unable to find `xfile`.".into());
        }
    }
}

/// Parses a command according to the POSIX standard.
fn parse_command(src: &str) -> Result<Command, Box<dyn Error>> {
    let words = shell_words::split(src)?;
    let mut program = words[0].as_str();
    let mut args = &words[1..];
    if cfg!(target_os = "windows") {
        if program == "/usr/bin/env" {
            program = &args[0];
            args = &args[1..];
        } else {
            program = Path::new(program).file_name().unwrap().to_str().unwrap();
        }
    }
    let mut cmd = Command::new(program);
    cmd.args(args);
    Ok(cmd)
}

/// Entrypoint of the `x` executable.
fn main() -> Result<(), Box<dyn Error>> {
    let xfile = find_xfile()?;
    let src = std::fs::read_to_string(&xfile)?;
    let args = std::env::args_os().collect::<Vec<_>>();
    let mut cmd = if let Some(src) = src.strip_prefix("#!") {
        if cfg!(target_os = "windows") {
            let (command, _) = src.split_once("\n").unwrap();
            parse_command(command)?
        } else {
            std::process::Command::new(&xfile)
        }
    } else if let Some(src) = src.strip_prefix("!") {
        parse_command(src)?
    } else if let Some(src) = src.strip_prefix(">") {
        let mut cmd = parse_command(src)?;
        cmd.current_dir(xfile.parent().unwrap());
        cmd
    } else {
        return Err("Invalid `xfile`.".into());
    };
    cmd.args(&args[1..]);
    let mut child = cmd.spawn()?;
    let status = child.wait()?;
    std::process::exit(status.code().unwrap())
}
