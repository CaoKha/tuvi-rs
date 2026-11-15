//! Dev commands for the Thiên Hạc workspace.
//!
//! - `cargo up`: run `topcoat dev` for the web app, from any folder.

use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
};

use clap::{Parser, Subcommand};

/// Dev commands for the Thiên Hạc workspace.
#[derive(Parser)]
#[command(bin_name = "cargo tuvi", after_help = "Shortcut: `cargo up`.")]
struct Cli {
    #[command(subcommand)]
    command: Task,
}

#[derive(Subcommand)]
enum Task {
    /// Run `topcoat dev`: it rebuilds the app and reloads the page on save.
    Up,
}

fn main() -> ExitCode {
    let result = match Cli::parse().command {
        Task::Up => up(),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

fn up() -> Result<(), String> {
    step("Starting topcoat dev (Ctrl-C to stop)");
    let mut dev = Command::new("topcoat");
    dev.arg("dev")
        .current_dir(workspace_root().join("crates/tuvi-web"));
    exec(dev)
        .map_err(|error| format!("{error}\nInstall the Topcoat CLI: cargo install topcoat-cli"))
}

/// The workspace root. This crate is at `<root>/crates/tuvi-cli`.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn step(message: &str) {
    eprintln!("\x1b[1;32m==>\x1b[0m {message}");
}

/// Replaces this process with `command`, so Ctrl-C goes straight to it.
#[cfg(unix)]
fn exec(mut command: Command) -> Result<(), String> {
    use std::os::unix::process::CommandExt;
    let name = display(&command);
    // `exec` returns only on failure.
    let error = command.exec();
    Err(format!("cannot run `{name}`: {error}"))
}

#[cfg(not(unix))]
fn exec(mut command: Command) -> Result<(), String> {
    let name = display(&command);
    let status = command
        .status()
        .map_err(|error| format!("cannot run `{name}`: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("`{name}` failed ({status})"))
    }
}

fn display(command: &Command) -> String {
    std::iter::once(command.get_program())
        .chain(command.get_args())
        .map(OsStr::to_string_lossy)
        .collect::<Vec<_>>()
        .join(" ")
}
