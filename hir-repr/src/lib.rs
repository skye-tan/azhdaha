use tree_sitter::TreeCursor;

mod construct;
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
