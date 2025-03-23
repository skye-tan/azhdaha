use anyhow::Context;
use tree_sitter::TreeCursor;

use crate::datatype::{
    Block, DeclStmt, Expr, ExprKind, Ident, Lit, LitKind, PrimTyKind, Span, Stmt, StmtKind, Ty,
    TyKind,
};

pub trait Constructable {
    fn construct(source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self>
    where
        Self: Sized;
}

impl Constructable for PrimTyKind {
    fn construct(source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self> {
        let node = cursor.node();

        Ok(
            match std::str::from_utf8(&source_code[node.start_byte()..node.end_byte()])? {
                "int" => PrimTyKind::Int,
                "float" => PrimTyKind::Float,
                "double" => PrimTyKind::Double,
                "char" => PrimTyKind::Char,
                _ => todo!(),
            },
        )
    }
}

impl Constructable for TyKind {
    fn construct(source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self> {
        let node = cursor.node();

        Ok(match node.kind() {
            "primitive_type" => TyKind::PrimTy(PrimTyKind::construct(source_code, cursor)?),
            _ => todo!(),
        })
    }
}

impl Constructable for Ty {
    fn construct(source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self> {
        let node = cursor.node();

        Ok(Self {
            kind: TyKind::construct(source_code, cursor)?,
            span: Span {
                lo: node.start_byte(),
                hi: node.end_byte(),
                ctxt: node.kind().to_owned(),
            },
        })
    }
}

impl Constructable for Ident {
    fn construct(source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self> {
        let node = cursor.node();

        Ok(Self {
            name: std::str::from_utf8(
                &source_code[cursor.node().start_byte()..cursor.node().end_byte()],
            )?
            .to_string(),
            span: Span {
                lo: node.start_byte(),
                hi: node.end_byte(),
                ctxt: node.kind().to_owned(),
            },
        })
    }
}

impl Constructable for DeclStmt {
    fn construct(source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self> {
        let node = cursor.node();

        cursor.goto_first_child();
        let ty = Ty::construct(source_code, cursor)?;

        cursor.goto_next_sibling();
        cursor.goto_first_child();
        let ident = Ident::construct(source_code, cursor)?;

        cursor.goto_next_sibling();
        cursor.goto_next_sibling();
        let init = Expr::construct(source_code, cursor).map_or(None, |expr| Some(expr));

        cursor.goto_parent();
        cursor.goto_parent();

        Ok(Self {
            ty,
            ident,
            init,
            span: Span {
                lo: node.start_byte(),
                hi: node.end_byte(),
                ctxt: node.kind().to_owned(),
            },
        })
    }
}

impl Constructable for StmtKind {
    fn construct(source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self> {
        let node = cursor.node();

        Ok({
            match node.kind() {
                "declaration" => Self::Decl(DeclStmt::construct(source_code, cursor)?),
                _ => todo!(),
            }
        })
    }
}

impl Constructable for Stmt {
    fn construct(source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self> {
        let node = cursor.node();

        Ok(Self {
            kind: StmtKind::construct(source_code, cursor)?,
            span: Span {
                lo: node.start_byte(),
                hi: node.end_byte(),
                ctxt: node.kind().to_owned(),
            },
        })
    }
}

impl Constructable for Block {
    fn construct(source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self> {
        let node = cursor.node();
        cursor.goto_first_child();
        cursor.goto_next_sibling();

        let mut stmts = vec![];

        while cursor.node().kind() != "}" {
            stmts.push(Stmt::construct(source_code, cursor)?);
            cursor.goto_next_sibling();
        }

        cursor.goto_descendant(node.id());

        Ok(Self {
            stmts,
            span: Span {
                lo: node.start_byte(),
                hi: node.end_byte(),
                ctxt: node.kind().to_owned(),
            },
        })
    }
}

impl Constructable for LitKind {
    fn construct(source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self> {
        let node = cursor.node();

        Ok(match node.kind() {
            "string_literal" => {
                let node = node.child(1).context("")?;
                Self::Str(
                    std::str::from_utf8(&source_code[node.start_byte()..node.end_byte()])?
                        .to_owned(),
                )
            }
            "char_literal" => Self::Char(source_code[node.start_byte() + 1] as char),
            "number_literal" => {
                let literal =
                    std::str::from_utf8(&source_code[node.start_byte()..node.end_byte()])?;

                if let Ok(value) = literal.parse() {
                    Self::Int(value)
                } else {
                    Self::Float(literal.parse()?)
                }
            }
            _ => todo!(),
        })
    }
}

impl Constructable for Lit {
    fn construct(source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self> {
        let node = cursor.node();

        Ok(Self {
            kind: LitKind::construct(source_code, cursor)?,
            span: Span {
                lo: node.start_byte(),
                hi: node.end_byte(),
                ctxt: node.kind().to_owned(),
            },
        })
    }
}

impl Constructable for ExprKind {
    fn construct(source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self> {
        let node = cursor.node();

        Ok(match node.kind() {
            kind if kind.contains("literal") => Self::Lit(Lit::construct(source_code, cursor)?),
            "compound_statement" => Self::Block(Block::construct(source_code, cursor)?),
            _ => todo!(),
        })
    }
}

impl Constructable for Expr {
    fn construct(source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self> {
        let node = cursor.node();

        Ok(Self {
            kind: ExprKind::construct(source_code, cursor)?,
            span: Span {
                lo: node.start_byte(),
                hi: node.end_byte(),
                ctxt: node.kind().to_owned(),
            },
        })
    }
}
