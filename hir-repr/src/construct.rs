use std::vec;

use anyhow::Context;
use log::trace;
use tree_sitter::TreeCursor;

use crate::{
    constant,
    datatype::{
        BinOp, BinOpKind, Block, DeclStmt, Expr, ExprKind, Ident, Lit, LitKind, LoopSource, Path,
        PrimTyKind, Span, Stmt, StmtKind, Ty, TyKind, UnOp,
    },
};

/// Must be implemented by datatypes which are construable from an ast node.
pub(crate) trait Constructable {
    /// The constructed type by the [`Constructable::construct`] method.
    type ConsType;

    /// Construct Self from source code and a the current node pointed by the cursor.
    fn construct(source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self::ConsType>;
}

impl Constructable for PrimTyKind {
    type ConsType = Self;

    fn construct(source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self::ConsType> {
        let node = cursor.node();
        trace!("Construct [PrimTyKind] from node: {}", node.kind());

        Ok(
            match std::str::from_utf8(&source_code[node.start_byte()..node.end_byte()])? {
                constant::INT => PrimTyKind::Int,
                constant::FLOAT => PrimTyKind::Float,
                constant::DOUBLE => PrimTyKind::Double,
                constant::CHAR => PrimTyKind::Char,
                _ => todo!(),
            },
        )
    }
}

impl Constructable for TyKind {
    type ConsType = Self;

    fn construct(source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self::ConsType> {
        let node = cursor.node();
        trace!("Construct [TyKind] from node: {}", node.kind());

        Ok(match node.kind() {
            constant::PRIMITIVE_TYPE => TyKind::PrimTy(PrimTyKind::construct(source_code, cursor)?),
            constant::TYPE_DESCRIPTOR => {
                cursor.goto_first_child();

                let ty_kind = TyKind::construct(source_code, cursor)?;

                cursor.goto_parent();

                ty_kind
            }
            _ => todo!(),
        })
    }
}

impl Constructable for Ty {
    type ConsType = Self;

    fn construct(source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self::ConsType> {
        let node = cursor.node();
        trace!("Construct [Ty] from node: {}", node.kind());

        Ok(Self {
            kind: TyKind::construct(source_code, cursor)?,
            span: Span {
                lo: node.start_byte(),
                hi: node.end_byte(),
            },
        })
    }
}

impl Constructable for Ident {
    type ConsType = Self;

    fn construct(source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self::ConsType> {
        let node = cursor.node();
        trace!("Construct [Ident] from node: {}", node.kind());

        Ok(Self {
            name: std::str::from_utf8(
                &source_code[cursor.node().start_byte()..cursor.node().end_byte()],
            )?
            .to_string(),
            span: Span {
                lo: node.start_byte(),
                hi: node.end_byte(),
            },
        })
    }
}

impl Constructable for DeclStmt {
    type ConsType = Vec<Self>;

    fn construct(source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self::ConsType> {
        let node = cursor.node();
        trace!("Construct [DeclStmt] from node: {}", node.kind());

        cursor.goto_first_child();

        let ty = Ty::construct(source_code, cursor)?;

        cursor.goto_next_sibling();

        fn process_decl(
            source_code: &[u8],
            cursor: &mut TreeCursor,
            ty: Ty,
        ) -> anyhow::Result<(Ty, Ident)> {
            let node = cursor.node();
            trace!("Process [DeclStmt] from node: {}", node.kind());

            Ok(match node.kind() {
                constant::ARRAY_DECLARATOR => {
                    cursor.goto_first_child();

                    let ident = Ident::construct(source_code, cursor)?;

                    cursor.goto_next_sibling();
                    cursor.goto_next_sibling();

                    let array_len = Expr::construct(source_code, cursor)?;

                    cursor.goto_parent();

                    let span = ty.span.clone();

                    (
                        Ty {
                            kind: TyKind::Array(Box::new(ty), Box::new(array_len)),
                            span,
                        },
                        ident,
                    )
                }
                constant::POINTER_DECLARATOR => {
                    cursor.goto_first_child();
                    cursor.goto_next_sibling();

                    let ident = Ident::construct(source_code, cursor)?;

                    cursor.goto_parent();

                    let span = ty.span.clone();

                    (
                        Ty {
                            kind: TyKind::Ptr(Box::new(ty)),
                            span,
                        },
                        ident,
                    )
                }
                _ => (ty, Ident::construct(source_code, cursor)?),
            })
        }

        let mut decl_stmts = vec![];

        while cursor.node().kind() != ";" {
            let ty = ty.clone();

            let (ty, ident, init) = match cursor.node().kind() {
                constant::INIT_DECLARATOR => {
                    cursor.goto_first_child();

                    let (ty, ident) = process_decl(source_code, cursor, ty)?;

                    cursor.goto_next_sibling();
                    cursor.goto_next_sibling();

                    let init = Expr::construct(source_code, cursor)?;

                    cursor.goto_parent();

                    (ty, ident, Some(init))
                }
                _ => {
                    let (ty, ident) = process_decl(source_code, cursor, ty)?;

                    (ty, ident, None)
                }
            };

            cursor.goto_next_sibling();
            cursor.goto_next_sibling();

            decl_stmts.push(Self {
                ty,
                ident,
                init,
                span: Span {
                    lo: node.start_byte(),
                    hi: node.end_byte(),
                },
            });
        }

        cursor.goto_parent();

        Ok(decl_stmts)
    }
}

impl Constructable for StmtKind {
    type ConsType = Vec<Self>;

    fn construct(source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self::ConsType> {
        let node = cursor.node();
        trace!("Construct [StmtKind] from node: {}", node.kind());

        Ok({
            match node.kind() {
                constant::DECLARATION => DeclStmt::construct(source_code, cursor)?
                    .into_iter()
                    .map(Self::Decl)
                    .collect(),
                constant::RETURN_STATEMENT
                | constant::EXPRESSION_STATEMENT
                | constant::BREAK_STATEMENT
                | constant::CONTINUE_STATEMENT => {
                    vec![Self::Semi(Expr::construct(source_code, cursor)?)]
                }
                constant::IF_STATEMENT | constant::WHILE_STATEMENT | constant::FOR_STATEMENT => {
                    vec![Self::Expr(Expr::construct(source_code, cursor)?)]
                }
                _ => todo!(),
            }
        })
    }
}

impl Constructable for Stmt {
    type ConsType = Vec<Self>;

    fn construct(source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self::ConsType> {
        let node = cursor.node();
        trace!("Construct [Stmt] from node: {}", node.kind());

        Ok(StmtKind::construct(source_code, cursor)?
            .into_iter()
            .map(|stmt_kind| Self {
                kind: stmt_kind,
                span: Span {
                    lo: node.start_byte(),
                    hi: node.end_byte(),
                },
            })
            .collect())
    }
}

impl Constructable for Block {
    type ConsType = Self;

    fn construct(source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self::ConsType> {
        let node = cursor.node();
        trace!("Construct [Block] from node: {}", node.kind());

        cursor.goto_first_child();
        cursor.goto_next_sibling();

        let mut stmts = vec![];

        while cursor.node().kind() != "}" {
            stmts.extend(Stmt::construct(source_code, cursor)?);

            cursor.goto_next_sibling();
        }

        cursor.goto_parent();

        Ok(Self {
            stmts,
            span: Span {
                lo: node.start_byte(),
                hi: node.end_byte(),
            },
        })
    }
}

impl Constructable for LitKind {
    type ConsType = Self;

    fn construct(source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self::ConsType> {
        let node = cursor.node();
        trace!("Construct [LitKind] from node: {}", node.kind());

        Ok(match node.kind() {
            constant::STRING_LITERAL => {
                let node = node.child(1).context("")?;
                Self::Str(
                    std::str::from_utf8(&source_code[node.start_byte()..node.end_byte()])?
                        .to_owned(),
                )
            }
            constant::CHAR_LITERAL => Self::Char(source_code[node.start_byte() + 1] as char),
            constant::NUMBER_LITERAL => {
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
    type ConsType = Self;

    fn construct(source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self::ConsType> {
        let node = cursor.node();
        trace!("Construct [Lit] from node: {}", node.kind());

        Ok(Self {
            kind: LitKind::construct(source_code, cursor)?,
            span: Span {
                lo: node.start_byte(),
                hi: node.end_byte(),
            },
        })
    }
}

impl Constructable for Path {
    type ConsType = Self;

    fn construct(source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self::ConsType> {
        let node = cursor.node();
        trace!("Construct [Path] from node: {}", node.kind());

        Ok(Self {
            res: Ident::construct(source_code, cursor)?,
            span: Span {
                lo: node.start_byte(),
                hi: node.end_byte(),
            },
        })
    }
}

impl Constructable for BinOpKind {
    type ConsType = Self;

    fn construct(_source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self::ConsType> {
        let node = cursor.node();
        trace!("Construct [BinOpKind] from node: {}", node.kind());

        Ok({
            match node.kind() {
                constant::ADD | constant::ASSIGN_ADD => Self::Add,
                constant::SUB | constant::ASSIGN_SUB => Self::Sub,
                constant::MUL | constant::ASSIGN_MUL => Self::Mul,
                constant::DIV | constant::ASSIGN_DIV => Self::Div,
                constant::REM | constant::ASSIGN_REM => Self::Rem,
                constant::AND => Self::And,
                constant::OR => Self::Or,
                constant::BIT_XOR | constant::ASSIGN_BIT_XOR => Self::BitXor,
                constant::BIT_AND | constant::ASSIGN_BIT_AND => Self::BitAnd,
                constant::BIT_OR | constant::ASSIGN_BIT_OR => Self::BitOr,
                constant::SHL | constant::ASSIGN_SHL => Self::Shl,
                constant::SHR | constant::ASSIGN_SHR => Self::Shr,
                constant::EQ => Self::Eq,
                constant::LT => Self::Lt,
                constant::LE => Self::Le,
                constant::NE => Self::Ne,
                constant::GE => Self::Ge,
                constant::GT => Self::Gt,
                constant::ASSIGN => Self::Assign,
                _ => unreachable!(),
            }
        })
    }
}

impl Constructable for BinOp {
    type ConsType = Self;

    fn construct(source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self::ConsType> {
        let node = cursor.node();
        trace!("Construct [BinOp] from node: {}", node.kind());

        Ok(Self {
            node: BinOpKind::construct(source_code, cursor)?,
            span: Span {
                lo: node.start_byte(),
                hi: node.end_byte(),
            },
        })
    }
}

impl Constructable for UnOp {
    type ConsType = Self;

    fn construct(_source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self::ConsType> {
        let node = cursor.node();
        trace!("Construct [UnOp] from node: {}", node.kind());

        Ok({
            match node.kind() {
                constant::NOT => Self::Not,
                constant::NEG => Self::Neg,
                constant::COM => Self::Com,
                constant::POS => Self::Pos,
                constant::ADDR_OF => Self::AddrOf,
                constant::DEREF => Self::Deref,
                _ => unreachable!(),
            }
        })
    }
}

impl Constructable for ExprKind {
    type ConsType = Self;

    fn construct(source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self::ConsType> {
        let node = cursor.node();
        trace!("Construct [ExprKind] from node: {}", node.kind());

        Ok(match node.kind() {
            kind if kind.contains("literal") => Self::Lit(Lit::construct(source_code, cursor)?),
            constant::COMPOUND_STATEMENT => Self::Block(Block::construct(source_code, cursor)?),
            constant::RETURN_STATEMENT => {
                cursor.goto_first_child();
                cursor.goto_next_sibling();

                let expr = Expr::construct(source_code, cursor)?;

                cursor.goto_parent();

                Self::Ret(Box::new(expr))
            }
            constant::IDENTIFIER => Self::Path(Path::construct(source_code, cursor)?),
            constant::CALL_EXPRESSION => {
                cursor.goto_first_child();

                let path = Expr::construct(source_code, cursor)?;

                cursor.goto_next_sibling();
                cursor.goto_first_child();
                cursor.goto_next_sibling();

                let mut arguments = vec![];

                while cursor.node().kind() != ")" {
                    arguments.push(Expr::construct(source_code, cursor)?);

                    cursor.goto_next_sibling();
                    cursor.goto_next_sibling();
                }

                cursor.goto_parent();
                cursor.goto_parent();

                Self::Call(Box::new(path), arguments)
            }
            constant::EXPRESSION_STATEMENT => {
                cursor.goto_first_child();

                let expr_kind = ExprKind::construct(source_code, cursor)?;

                cursor.goto_parent();

                expr_kind
            }
            constant::BINARY_EXPRESSION => {
                cursor.goto_first_child();

                let lhs = Expr::construct(source_code, cursor)?;

                cursor.goto_next_sibling();

                let bin_op = BinOp::construct(source_code, cursor)?;

                cursor.goto_next_sibling();

                let rhs = Expr::construct(source_code, cursor)?;

                cursor.goto_parent();

                Self::Binary(bin_op, Box::new(lhs), Box::new(rhs))
            }
            constant::UNARY_EXPRESSION | constant::POINTER_EXPRESSION => {
                cursor.goto_first_child();

                let un_op = UnOp::construct(source_code, cursor)?;

                cursor.goto_next_sibling();

                let expr = Expr::construct(source_code, cursor)?;

                cursor.goto_parent();

                // Ignore [`UnOp::Pos`] because it has no effects.
                match un_op {
                    UnOp::Pos => expr.kind,
                    _ => Self::Unary(un_op, Box::new(expr)),
                }
            }
            constant::PARENTHESIZED_EXPRESSION => {
                cursor.goto_first_child();
                cursor.goto_next_sibling();

                let expr_kind = ExprKind::construct(source_code, cursor)?;

                cursor.goto_parent();

                expr_kind
            }
            constant::IF_STATEMENT => {
                cursor.goto_first_child();
                cursor.goto_next_sibling();

                let condition = Expr::construct(source_code, cursor)?;

                cursor.goto_next_sibling();

                let body = Expr::construct(source_code, cursor)?;

                let else_clause = if cursor.goto_next_sibling() {
                    cursor.goto_first_child();
                    cursor.goto_next_sibling();

                    let x = Expr::construct(source_code, cursor)?;

                    cursor.goto_parent();

                    Some(Box::new(x))
                } else {
                    None
                };

                cursor.goto_parent();

                Self::If(Box::new(condition), Box::new(body), else_clause)
            }
            constant::WHILE_STATEMENT => {
                cursor.goto_first_child();
                cursor.goto_next_sibling();

                let condition = Expr::construct(source_code, cursor)?;

                cursor.goto_next_sibling();

                let body = Expr::construct(source_code, cursor)?;

                cursor.goto_parent();

                Self::Loop(
                    LoopSource::While,
                    Box::new(Expr {
                        kind: Self::If(
                            Box::new(condition),
                            Box::new(body),
                            Some(Box::new(Expr {
                                kind: Self::Break,
                                span: Span {
                                    lo: node.start_byte(),
                                    hi: node.end_byte(),
                                },
                            })),
                        ),
                        span: Span {
                            lo: node.start_byte(),
                            hi: node.end_byte(),
                        },
                    }),
                )
            }
            constant::FOR_STATEMENT => {
                cursor.goto_first_child();
                cursor.goto_next_sibling();
                cursor.goto_next_sibling();

                let initialization = Stmt::construct(source_code, cursor)?;

                cursor.goto_next_sibling();

                let condition = Expr::construct(source_code, cursor)?;

                cursor.goto_next_sibling();
                cursor.goto_next_sibling();

                let updatation = Expr::construct(source_code, cursor)?;

                cursor.goto_next_sibling();
                cursor.goto_next_sibling();

                let mut body = Expr::construct(source_code, cursor)?;

                cursor.goto_parent();

                let span = updatation.span.clone();
                body.kind = match body.kind {
                    Self::Block(mut block) => {
                        block.stmts.push(Stmt {
                            kind: StmtKind::Semi(updatation),
                            span,
                        });
                        Self::Block(block)
                    }
                    _ => unreachable!(),
                };

                let mut stmts = initialization;
                stmts.push(Stmt {
                    kind: StmtKind::Expr(Expr {
                        kind: Self::Loop(
                            LoopSource::For,
                            Box::new(Expr {
                                kind: Self::If(
                                    Box::new(condition),
                                    Box::new(body),
                                    Some(Box::new(Expr {
                                        kind: Self::Break,
                                        span: Span {
                                            lo: node.start_byte(),
                                            hi: node.end_byte(),
                                        },
                                    })),
                                ),
                                span: Span {
                                    lo: node.start_byte(),
                                    hi: node.end_byte(),
                                },
                            }),
                        ),
                        span: Span {
                            lo: node.start_byte(),
                            hi: node.end_byte(),
                        },
                    }),
                    span: Span {
                        lo: node.start_byte(),
                        hi: node.end_byte(),
                    },
                });

                Self::Block(Block {
                    stmts,
                    span: Span {
                        lo: node.start_byte(),
                        hi: node.end_byte(),
                    },
                })
            }
            constant::ASSIGNMENT_EXPRESSION => {
                cursor.goto_first_child();

                let lhs = Expr::construct(source_code, cursor)?;

                cursor.goto_next_sibling();

                let bin_op = BinOp::construct(source_code, cursor)?;

                cursor.goto_next_sibling();

                let rhs = Expr::construct(source_code, cursor)?;

                cursor.goto_parent();

                match bin_op.node {
                    BinOpKind::Assign => Self::Assign(Box::new(lhs), Box::new(rhs)),
                    _ => Self::AssignOp(bin_op, Box::new(lhs), Box::new(rhs)),
                }
            }
            constant::FIELD_EXPRESSION => {
                cursor.goto_first_child();

                let target = Expr::construct(source_code, cursor)?;

                cursor.goto_next_sibling();
                cursor.goto_next_sibling();

                let field = Ident::construct(source_code, cursor)?;

                cursor.goto_parent();

                Self::Field(Box::new(target), field)
            }
            constant::SUBSCRIPT_EXPRESSION => {
                cursor.goto_first_child();

                let target = Expr::construct(source_code, cursor)?;

                cursor.goto_next_sibling();
                cursor.goto_next_sibling();

                let index = Expr::construct(source_code, cursor)?;

                cursor.goto_parent();

                Self::Index(
                    Box::new(target),
                    Box::new(index),
                    Span {
                        lo: node.start_byte(),
                        hi: node.end_byte(),
                    },
                )
            }
            constant::BREAK_STATEMENT => Self::Break,
            constant::CONTINUE_STATEMENT => Self::Continue,
            constant::CAST_EXPRESSION => {
                cursor.goto_first_child();
                cursor.goto_next_sibling();

                let ty = Ty::construct(source_code, cursor)?;

                cursor.goto_next_sibling();
                cursor.goto_next_sibling();

                let target = Expr::construct(source_code, cursor)?;

                cursor.goto_parent();

                Self::Cast(Box::new(target), ty)
            }
            constant::INITIALIZER_LIST => {
                cursor.goto_first_child();
                cursor.goto_next_sibling();

                let mut elements = vec![];

                while cursor.node().kind() != "}" {
                    elements.push(Expr::construct(source_code, cursor)?);

                    cursor.goto_next_sibling();
                    cursor.goto_next_sibling();
                }

                cursor.goto_parent();

                Self::Array(elements)
            }
            constant::COMMA_EXPRESSION => {
                cursor.goto_first_child();

                let mut exprs = vec![];

                loop {
                    exprs.push(Expr::construct(source_code, cursor)?);

                    cursor.goto_next_sibling();
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }

                cursor.goto_parent();

                Self::Comma(exprs)
            }
            _ => todo!(),
        })
    }
}

impl Constructable for Expr {
    type ConsType = Self;

    fn construct(source_code: &[u8], cursor: &mut TreeCursor) -> anyhow::Result<Self::ConsType> {
        let node = cursor.node();
        trace!("Construct [Expr] from node: {}", node.kind());

        Ok(Self {
            kind: ExprKind::construct(source_code, cursor)?,
            span: Span {
                lo: node.start_byte(),
                hi: node.end_byte(),
            },
        })
    }
}
