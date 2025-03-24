//! A compiler frontend tool for C programming language which analyzes the sources code
//! in order to detect memory leakage by applying linear type system principles.

use std::{fs::File, io::Write, os::fd::AsRawFd};

use anyhow::Context;
use log::trace;
use tree_sitter::{Parser, Tree};

/// Contains functions used for preprocessing source code.
mod preprocess;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args = cli_utils::parse_args();

    let source_codes = preprocess::expand(&args.compile_commands)?;

    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_c::LANGUAGE.into())
        .context("Failed to load C grammar.")?;

    let trees: Vec<Tree> = source_codes
        .iter()
        .map(|source_code| parser.parse(source_code, None).unwrap())
        .collect();

    {
        let mut f = File::create("./test").unwrap();
        trees[0].print_dot_graph(&f.as_raw_fd());
        f.flush().unwrap();
    }

    trace!(
        "{:#?}",
        hir_repr::construct_hir(&source_codes[0], &mut trees[0].walk()).unwrap()
    );

    Ok(())
}
