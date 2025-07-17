use std::fs;

use anyhow::Context;
use clap::{Parser, ValueHint};
use compile_commands::CompilationDatabase;

/// A cli to interact with azhdaha.
#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    /// Path to compile-commands which must be formatted as json.
    #[arg(value_parser = parse_compile_commands, value_hint = ValueHint::FilePath)]
    pub compile_commands: CompilationDatabase,
    /// Determines whether the source code should be annotated before analyzing or not.
    #[arg(long)]
    pub annotate: bool,
    /// Determines whether the dot-graph of the source code should be generated or not.
    #[arg(long)]
    pub dot_graph: bool,
}

/// Parse compile-commands into a [`CompilationDatabase`].
///
/// # Errors
///
/// Return [`Err`] if the file does not exist or parsing fails.
///
fn parse_compile_commands(path_to_file: &str) -> anyhow::Result<CompilationDatabase> {
    serde_json::from_str::<CompilationDatabase>(
        &fs::read_to_string(path_to_file).context("Failed to read compile-commands.")?,
    )
    .context("Failed to parse compile-commands.")
}

pub fn parse_args() -> Args {
    Args::parse()
}
