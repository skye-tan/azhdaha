//! A compiler frontend tool for C programming language which analyzes the sources code
//! in order to detect memory leakage by applying linear type system principles.

use anyhow::Context;
use tree_sitter::{Parser, Tree};

/// Contains functions used for preprocessing source code.
mod preprocess;

// fn traverse_tree(cursor: &mut TreeCursor) {
//     let mut traversed = false;
//     loop {
//         if traversed {
//             if cursor.goto_next_sibling() {
//                 traversed = false;
//             } else {
//                 if !cursor.goto_parent() {
//                     break;
//                 }
//             }
//         } else {
//             let node = cursor.node();
//             println!("{}", node.kind());
//             if !cursor.goto_first_child() {
//                 traversed = true;
//             }
//         }
//     }
// }

fn main() -> anyhow::Result<()> {
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

    println!(
        "{:#?}",
        hir_repr::construct_hir(&source_codes[0], &mut trees[0].walk()).unwrap()
    );

    Ok(())
}
