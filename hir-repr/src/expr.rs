#![allow(clippy::missing_docs_in_private_items)]

use anyhow::bail;
use log::trace;

use crate::{constant, datatype::*};

impl LoweringCtx<'_> {
    fn lower_bin_op_kind(&mut self) -> anyhow::Result<BinOpKind> {
        let node = self.cursor.node();
        trace!("Construct [BinOpKind] from node: {}", node.kind());

        Ok(match node.kind() {
            constant::ADD | constant::ASSIGN_ADD | constant::INC => BinOpKind::Add,
            constant::SUB | constant::ASSIGN_SUB | constant::DEC => BinOpKind::Sub,
            constant::MUL | constant::ASSIGN_MUL => BinOpKind::Mul,
            constant::DIV | constant::ASSIGN_DIV => BinOpKind::Div,
            constant::REM | constant::ASSIGN_REM => BinOpKind::Rem,
            constant::AND => BinOpKind::And,
            constant::OR => BinOpKind::Or,
            constant::BIT_XOR | constant::ASSIGN_BIT_XOR => BinOpKind::BitXor,
            constant::BIT_AND | constant::ASSIGN_BIT_AND => BinOpKind::BitAnd,
            constant::BIT_OR | constant::ASSIGN_BIT_OR => BinOpKind::BitOr,
            constant::SHL | constant::ASSIGN_SHL => BinOpKind::Shl,
            constant::SHR | constant::ASSIGN_SHR => BinOpKind::Shr,
            constant::EQ => BinOpKind::Eq,
            constant::LT => BinOpKind::Lt,
            constant::LE => BinOpKind::Le,
            constant::NE => BinOpKind::Ne,
            constant::GE => BinOpKind::Ge,
            constant::GT => BinOpKind::Gt,
            constant::ASSIGN => BinOpKind::Assign,
            kind => bail!("Unsupported [BinOpKind] node: {kind}"),
        })
    }

    fn lower_bin_op(&mut self) -> anyhow::Result<BinOp> {
        let node = self.cursor.node();
        trace!("Construct [BinOp] from node: {}", node.kind());

        Ok(BinOp {
            node: self.lower_bin_op_kind()?,
            span: Span {
                lo: node.start_byte(),
                hi: node.end_byte(),
            },
        })
    }

    fn lower_un_op(&mut self) -> anyhow::Result<UnOp> {
        let node = self.cursor.node();
        trace!("Construct [UnOp] from node: {}", node.kind());

        Ok(match node.kind() {
            constant::NOT => UnOp::Not,
            constant::NEG => UnOp::Neg,
            constant::COM => UnOp::Com,
            constant::POS => UnOp::Pos,
            constant::ADDR_OF => UnOp::AddrOf,
            constant::DEREF => UnOp::Deref,
            kind => bail!("Unsupported [UnOp] node: {kind}"),
        })
    }

    fn lower_sizeof_kind(&mut self) -> anyhow::Result<SizeofKind> {
        let node = self.cursor.node();
        trace!("Construct [SizeofKind] from node: {}", node.kind());

        self.cursor.goto_first_child();
        self.cursor.goto_next_sibling();

        let sizeof_kind = match self.cursor.node().kind() {
            constant::PARENTHESIZED_EXPRESSION => SizeofKind::Expr(Box::new(self.lower_expr()?)),
            _ => {
                self.cursor.goto_next_sibling();

                SizeofKind::Ty(self.lower_ty()?)
            }
        };

        self.cursor.goto_parent();

        Ok(sizeof_kind)
    }

    fn lower_sizeof(&mut self) -> anyhow::Result<Sizeof> {
        let node = self.cursor.node();
        trace!("Construct [SizeOf] from node: {}", node.kind());

        Ok(Sizeof {
            kind: self.lower_sizeof_kind()?,
            span: Span {
                lo: node.start_byte(),
                hi: node.end_byte(),
            },
        })
    }

    fn lower_expr_kind(&mut self) -> anyhow::Result<ExprKind> {
        let node = self.cursor.node();
        trace!("Construct [ExprKind] from node: {}", node.kind());

        Ok(match node.kind() {
            kind if kind.contains("literal") => ExprKind::Lit(self.lower_lit()?),
            constant::COMPOUND_STATEMENT => ExprKind::Block(self.lower_block()?),
            constant::RETURN_STATEMENT => {
                self.cursor.goto_first_child();
                self.cursor.goto_next_sibling();

                let expr = self.lower_expr()?;

                self.cursor.goto_parent();

                ExprKind::Ret(Box::new(expr))
            }
            constant::IDENTIFIER => ExprKind::Path(self.lower_path()?),
            constant::CALL_EXPRESSION => {
                self.cursor.goto_first_child();

                let path = self.lower_expr()?;

                self.cursor.goto_next_sibling();
                self.cursor.goto_first_child();
                self.cursor.goto_next_sibling();

                let mut arguments = vec![];

                while self.cursor.node().kind() != ")" {
                    arguments.push(self.lower_expr()?);

                    self.cursor.goto_next_sibling();
                    self.cursor.goto_next_sibling();
                }

                self.cursor.goto_parent();
                self.cursor.goto_parent();

                ExprKind::Call(Box::new(path), arguments)
            }
            constant::EXPRESSION_STATEMENT => {
                self.cursor.goto_first_child();

                let expr_kind = self.lower_expr_kind()?;

                self.cursor.goto_parent();

                expr_kind
            }
            constant::BINARY_EXPRESSION => {
                self.cursor.goto_first_child();

                let lhs = self.lower_expr()?;

                self.cursor.goto_next_sibling();

                let bin_op = self.lower_bin_op()?;

                self.cursor.goto_next_sibling();

                let rhs = self.lower_expr()?;

                self.cursor.goto_parent();

                ExprKind::Binary(bin_op, Box::new(lhs), Box::new(rhs))
            }
            constant::UPDATE_EXPRESSION => {
                self.cursor.goto_first_child();

                let lhs = self.lower_expr()?;

                self.cursor.goto_next_sibling();

                let bin_op = self.lower_bin_op()?;
                let op_node = self.cursor.node();

                self.cursor.goto_parent();

                let rhs = Expr {
                    kind: ExprKind::Lit(Lit {
                        kind: LitKind::Int(1),
                        span: Span {
                            lo: op_node.start_byte(),
                            hi: op_node.end_byte(),
                        },
                    }),
                    span: Span {
                        lo: op_node.start_byte(),
                        hi: op_node.end_byte(),
                    },
                };

                ExprKind::Binary(bin_op, Box::new(lhs), Box::new(rhs))
            }
            constant::UNARY_EXPRESSION | constant::POINTER_EXPRESSION => {
                self.cursor.goto_first_child();

                let un_op = self.lower_un_op()?;

                self.cursor.goto_next_sibling();

                let expr = self.lower_expr()?;

                self.cursor.goto_parent();

                // Ignore [`UnOp::Pos`] because it has no effects.
                match un_op {
                    UnOp::Pos => expr.kind,
                    _ => ExprKind::Unary(un_op, Box::new(expr)),
                }
            }
            constant::PARENTHESIZED_EXPRESSION => {
                self.cursor.goto_first_child();
                self.cursor.goto_next_sibling();

                let expr_kind = self.lower_expr_kind()?;

                self.cursor.goto_parent();

                expr_kind
            }
            constant::IF_STATEMENT => {
                self.cursor.goto_first_child();
                self.cursor.goto_next_sibling();

                let condition = self.lower_expr()?;

                self.cursor.goto_next_sibling();

                let body = self.lower_expr()?;

                let else_clause = if self.cursor.goto_next_sibling() {
                    self.cursor.goto_first_child();
                    self.cursor.goto_next_sibling();

                    let x = self.lower_expr()?;

                    self.cursor.goto_parent();

                    Some(Box::new(x))
                } else {
                    None
                };

                self.cursor.goto_parent();

                ExprKind::If(Box::new(condition), Box::new(body), else_clause)
            }
            constant::WHILE_STATEMENT => {
                self.cursor.goto_first_child();
                self.cursor.goto_next_sibling();

                let condition = self.lower_expr()?;

                self.cursor.goto_next_sibling();

                let body = self.lower_expr()?;

                self.cursor.goto_parent();

                ExprKind::Loop(
                    LoopSource::While,
                    Box::new(Expr {
                        kind: ExprKind::If(
                            Box::new(condition),
                            Box::new(body),
                            Some(Box::new(Expr {
                                kind: ExprKind::Break,
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
            constant::DO_STATEMENT => {
                self.cursor.goto_first_child();
                self.cursor.goto_next_sibling();

                let body = self.lower_expr()?;

                self.cursor.goto_next_sibling();
                self.cursor.goto_next_sibling();

                let condition = self.lower_expr()?;

                self.cursor.goto_parent();

                let loop_expr = ExprKind::Loop(
                    LoopSource::DoWhile,
                    Box::new(Expr {
                        kind: ExprKind::If(
                            Box::new(condition),
                            Box::new(body.clone()),
                            Some(Box::new(Expr {
                                kind: ExprKind::Break,
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
                );

                let mut stmts = match body.kind {
                    ExprKind::Block(block) => block.stmts.clone(),
                    _ => {
                        let span = body.span.clone();

                        vec![Stmt {
                            kind: StmtKind::Semi(body),
                            span: span.clone(),
                        }]
                    }
                };

                stmts.push(Stmt {
                    kind: StmtKind::Expr(Expr {
                        kind: loop_expr,
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

                ExprKind::Block(Block {
                    stmts,
                    span: Span {
                        lo: node.start_byte(),
                        hi: node.end_byte(),
                    },
                })
            }
            constant::FOR_STATEMENT => {
                self.cursor.goto_first_child();
                self.cursor.goto_next_sibling();
                self.cursor.goto_next_sibling();

                let initialization = self.lower_stmt()?;

                self.cursor.goto_next_sibling();

                let condition = self.lower_expr()?;

                self.cursor.goto_next_sibling();
                self.cursor.goto_next_sibling();

                let update_stmt = Stmt {
                    kind: StmtKind::Semi(self.lower_expr()?),
                    span: Span {
                        lo: self.cursor.node().start_byte(),
                        hi: self.cursor.node().end_byte(),
                    },
                };

                self.cursor.goto_next_sibling();
                self.cursor.goto_next_sibling();

                let mut body = self.lower_expr()?;

                self.cursor.goto_parent();

                match &mut body.kind {
                    ExprKind::Block(block) => {
                        block.stmts.push(update_stmt);
                    }
                    _ => {
                        let span = body.span.clone();

                        let mut block = Block {
                            stmts: vec![Stmt {
                                kind: StmtKind::Semi(body),
                                span: span.clone(),
                            }],
                            span: span.clone(),
                        };

                        block.stmts.push(update_stmt);

                        body = Expr {
                            kind: ExprKind::Block(block),
                            span,
                        }
                    }
                }

                let loop_expr = ExprKind::Loop(
                    LoopSource::For,
                    Box::new(Expr {
                        kind: ExprKind::If(
                            Box::new(condition),
                            Box::new(body),
                            Some(Box::new(Expr {
                                kind: ExprKind::Break,
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
                );

                let mut stmts = initialization;
                stmts.push(Stmt {
                    kind: StmtKind::Expr(Expr {
                        kind: loop_expr,
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

                ExprKind::Block(Block {
                    stmts,
                    span: Span {
                        lo: node.start_byte(),
                        hi: node.end_byte(),
                    },
                })
            }
            constant::ASSIGNMENT_EXPRESSION => {
                self.cursor.goto_first_child();

                let lhs = self.lower_expr()?;

                self.cursor.goto_next_sibling();

                let bin_op = self.lower_bin_op()?;

                self.cursor.goto_next_sibling();

                let rhs = self.lower_expr()?;

                self.cursor.goto_parent();

                match bin_op.node {
                    BinOpKind::Assign => ExprKind::Assign(Box::new(lhs), Box::new(rhs)),
                    _ => ExprKind::AssignOp(bin_op, Box::new(lhs), Box::new(rhs)),
                }
            }
            constant::FIELD_EXPRESSION => {
                self.cursor.goto_first_child();

                let target = self.lower_expr()?;

                self.cursor.goto_next_sibling();
                self.cursor.goto_next_sibling();

                let field = self.lower_ident()?;

                self.cursor.goto_parent();

                ExprKind::Field(Box::new(target), field)
            }
            constant::SUBSCRIPT_EXPRESSION => {
                self.cursor.goto_first_child();

                let target = self.lower_expr()?;

                self.cursor.goto_next_sibling();
                self.cursor.goto_next_sibling();

                let index = self.lower_expr()?;

                self.cursor.goto_parent();

                ExprKind::Index(
                    Box::new(target),
                    Box::new(index),
                    Span {
                        lo: node.start_byte(),
                        hi: node.end_byte(),
                    },
                )
            }
            constant::BREAK_STATEMENT => ExprKind::Break,
            constant::CONTINUE_STATEMENT => ExprKind::Continue,
            constant::CAST_EXPRESSION => {
                self.cursor.goto_first_child();
                self.cursor.goto_next_sibling();

                let ty = self.lower_ty()?;

                self.cursor.goto_next_sibling();
                self.cursor.goto_next_sibling();

                let target = self.lower_expr()?;

                self.cursor.goto_parent();

                ExprKind::Cast(Box::new(target), ty)
            }
            constant::INITIALIZER_LIST => {
                self.cursor.goto_first_child();
                self.cursor.goto_next_sibling();

                let mut elements = vec![];

                loop {
                    elements.push(self.lower_expr()?);

                    self.cursor.goto_next_sibling();
                    if !self.cursor.goto_next_sibling() {
                        break;
                    }
                }

                self.cursor.goto_parent();

                ExprKind::Array(elements)
            }
            constant::COMMA_EXPRESSION => {
                self.cursor.goto_first_child();

                let mut exprs = vec![];

                loop {
                    exprs.push(self.lower_expr()?);

                    self.cursor.goto_next_sibling();
                    if !self.cursor.goto_next_sibling() {
                        break;
                    }
                }

                self.cursor.goto_parent();

                ExprKind::Comma(exprs)
            }
            constant::SIZEOF_EXPRESSION => ExprKind::Sizeof(self.lower_sizeof()?),
            kind => bail!("Unsupported [ExprKind] node: {kind}"),
        })
    }

    pub(crate) fn lower_expr(&mut self) -> anyhow::Result<Expr> {
        let node = self.cursor.node();
        trace!("Construct [Expr] from node: {}", node.kind());

        Ok(Expr {
            kind: self.lower_expr_kind()?,
            span: Span {
                lo: node.start_byte(),
                hi: node.end_byte(),
            },
        })
    }
}
