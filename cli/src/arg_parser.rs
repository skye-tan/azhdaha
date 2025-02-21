use std::path::{Path, PathBuf};

use anyhow::bail;
use clap::{Parser, ValueEnum, ValueHint};

/// Commands supported by the cli.
#[derive(ValueEnum, Debug, Clone)]
enum Command {
    /// Analyze the source code for memory leakage.
    Analyze,
    /// Annotate the source code for linear types.
    Annotate,
}

/// A cli to interact with azhdaha.
#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    /// Command to be executed.
    command: Command,
    /// Path to the source code which can be a directory or a single file.
    #[arg(short, long, value_parser = parse_path, value_hint = ValueHint::FilePath)]
    path: PathBuf,
}

impl Cli {
    pub fn read() -> Self {
        Self::parse()
    }
}

/// Parse input string into [`PathBuf`].
///
/// # Errors
///
/// Will return [`Err`] if path does not exist.
///
fn parse_path(path: &str) -> anyhow::Result<PathBuf> {
    let path = Path::new(path);

    if !path.exists() {
        bail!("Path does not exist.");
    }

    Ok(path.to_path_buf())
}
