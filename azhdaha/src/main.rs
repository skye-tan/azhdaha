#![allow(clippy::missing_docs_in_private_items)]
use cli::Cli;

fn main() -> anyhow::Result<()> {
    let _cli = Cli::read();
    Ok(())
}
