use tree_sitter::TreeCursor;

mod datatype;
pub use datatype::*;

impl Stmt {
    fn construct(source_code: &[u8], mut cursor: TreeCursor) -> anyhow::Result<Self> {
        Ok(Stmt {
            kind: StmtKind::Decl(DeclStmt {
                ty: Ty {
                    kind: {
                        cursor.goto_first_child();
                        match cursor.node().kind() {
                            "primitive_type" => TyKind::PrimTy({
                                match std::str::from_utf8(
                                    &source_code
                                        [cursor.node().start_byte()..cursor.node().end_byte()],
                                )? {
                                    "int" => PrimTyKind::Int,
                                    "float" => PrimTyKind::Float,
                                    "double" => PrimTyKind::Double,
                                    "char" => PrimTyKind::Char,
                                    _ => todo!(),
                                }
                            }),
                            _ => todo!(),
                        }
                    },
                    span: Span { lo: 0, len: 0 },
                },
                ident: Ident {
                    name: {
                        cursor.goto_next_sibling();
                        cursor.goto_first_child();
                        std::str::from_utf8(
                            &source_code[cursor.node().start_byte()..cursor.node().end_byte()],
                        )?
                        .to_string()
                    },
                    span: Span { lo: 0, len: 0 },
                },
                init: Some(Expr {
                    kind: {
                        cursor.goto_next_sibling();
                        cursor.goto_next_sibling();
                        match cursor.node().kind() {
                            "number_literal" => ExprKind::Lit(Lit {
                                kind: LitKind::Int(
                                    std::str::from_utf8(
                                        &source_code
                                            [cursor.node().start_byte()..cursor.node().end_byte()],
                                    )?
                                    .to_string()
                                    .parse()?,
                                    LitIntType::Signed,
                                ),
                                span: Span { lo: 0, len: 0 },
                            }),
                            _ => todo!(),
                        }
                    },
                    span: Span { lo: 0, len: 0 },
                }),
                span: Span { lo: 0, len: 0 },
            }),
            span: Span { lo: 0, len: 0 },
        })
    }
}

pub fn construct_hir(source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Stmt> {
    let mut is_traversed = false;
    loop {
        if is_traversed {
            if cursor.goto_next_sibling() {
                is_traversed = false;
            } else {
                if !cursor.goto_parent() {
                    break;
                }
            }
        } else {
            let node = cursor.node();
            if node.kind() == "declaration" {
                return Stmt::construct(source_code, cursor.clone());
            }
            if !cursor.goto_first_child() {
                is_traversed = true;
            }
        }
    }
    todo!()
}
