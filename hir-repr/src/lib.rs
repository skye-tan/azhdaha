//! The HIR – "High-Level Intermediate Representation" – is the primary IR used for representation of the
//! abstract syntax tree (AST) that is generated after parsing, macro expansion, and name resolution.
//!
//! This implementation has been modeled after rustc's HIR.
//!

use tree_sitter::TreeCursor;

/// Contains constant values used to generate the HIR.
mod constant;
/// Contains implementation of the [`Constructable`] trait for datatypes.
mod construct;
/// Contains datatypes used to represent the HIR.
mod datatype;

use construct::Constructable;

pub fn construct_hir(
    source_code: &[u8],
    cursor: &mut TreeCursor,
) -> anyhow::Result<datatype::Expr> {
    let mut is_traversed = false;
    loop {
        if is_traversed {
            if cursor.goto_next_sibling() {
                is_traversed = false;
            } else if !cursor.goto_parent() {
                break;
            }
        } else {
            let node = cursor.node();
            if node.kind() == "compound_statement" {
                return datatype::Expr::construct(source_code, cursor);
            }
            if !cursor.goto_first_child() {
                is_traversed = true;
            }
        }
    }
    todo!()
}
