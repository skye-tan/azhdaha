//! A compiler frontend tool for C programming language which analyzes the sources code
//! in order to detect memory leakage by applying linear type system principles.

use anyhow::Context;
use tree_sitter::Parser;

/// Contains functions used for preprocessing source code.
mod preprocessor;

fn main() -> anyhow::Result<()> {
    let args = cli_utils::parse_args();

    let source_codes = preprocessor::expand(&args.compile_commands)?;

    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_c::LANGUAGE.into())
        .context("Failed to load C grammar.")?;

    let _trees: Vec<_> = source_codes
        .iter()
        .map(|source_code| parser.parse(source_code, None).unwrap())
        .collect();

    Ok(())
}
