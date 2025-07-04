#![allow(clippy::missing_docs_in_private_items)]

use std::mem;

use anyhow::bail;
use log::trace;

use crate::{constants, datatypes::*};

impl LoweringCtx<'_> {
    fn lower_bin_op_kind(&mut self) -> anyhow::Result<BinOpKind> {
        let node = self.cursor.node();
        trace!("Construct [BinOpKind] from node: {}", node.kind());

        Ok(match node.kind() {
            constants::ADD | constants::ASSIGN_ADD | constants::INC => BinOpKind::Add,
            constants::SUB | constants::ASSIGN_SUB | constants::DEC => BinOpKind::Sub,
            constants::MUL | constants::ASSIGN_MUL => BinOpKind::Mul,
            constants::DIV | constants::ASSIGN_DIV => BinOpKind::Div,
            constants::REM | constants::ASSIGN_REM => BinOpKind::Rem,
            constants::AND => BinOpKind::And,
            constants::OR => BinOpKind::Or,
            constants::BIT_XOR | constants::ASSIGN_BIT_XOR => BinOpKind::BitXor,
            constants::BIT_AND | constants::ASSIGN_BIT_AND => BinOpKind::BitAnd,
            constants::BIT_OR | constants::ASSIGN_BIT_OR => BinOpKind::BitOr,
            constants::SHL | constants::ASSIGN_SHL => BinOpKind::Shl,
            constants::SHR | constants::ASSIGN_SHR => BinOpKind::Shr,
            constants::EQ => BinOpKind::Eq,
            constants::LT => BinOpKind::Lt,
            constants::LE => BinOpKind::Le,
            constants::NE => BinOpKind::Ne,
            constants::GE => BinOpKind::Ge,
            constants::GT => BinOpKind::Gt,
            constants::ASSIGN => BinOpKind::Assign,
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
            constants::NOT => UnOp::Not,
            constants::NEG => UnOp::Neg,
            constants::COM => UnOp::Com,
            constants::POS => UnOp::Pos,
            constants::ADDR_OF => UnOp::AddrOf,
            constants::DEREF => UnOp::Deref,
            kind => bail!("Unsupported [UnOp] node: {kind}"),
        })
    }

    fn lower_sizeof_kind(&mut self) -> anyhow::Result<SizeofKind> {
        let node = self.cursor.node();
        trace!("Construct [SizeofKind] from node: {}", node.kind());

        self.cursor.goto_first_child();
        self.cursor.goto_next_sibling();

        let sizeof_kind = match self.cursor.node().kind() {
            constants::PARENTHESIZED_EXPRESSION => SizeofKind::Expr(Box::new(self.lower_expr()?)),
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
            constants::COMPOUND_STATEMENT => ExprKind::Block(self.lower_block()?),
            constants::RETURN_STATEMENT => {
                self.cursor.goto_first_child();
                self.cursor.goto_next_sibling();

                let expr = self.lower_expr()?;

                self.cursor.goto_parent();

                ExprKind::Ret(Box::new(expr))
            }
            constants::IDENTIFIER => ExprKind::Path(self.lower_path()?),
            constants::CALL_EXPRESSION => {
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
            constants::EXPRESSION_STATEMENT => {
                self.cursor.goto_first_child();

                let expr_kind = self.lower_expr_kind()?;

                self.cursor.goto_parent();

                expr_kind
            }
            constants::BINARY_EXPRESSION => {
                self.cursor.goto_first_child();

                let lhs = self.lower_expr()?;

                self.cursor.goto_next_sibling();

                let bin_op = self.lower_bin_op()?;

                self.cursor.goto_next_sibling();

                let rhs = self.lower_expr()?;

                self.cursor.goto_parent();

                ExprKind::Binary(bin_op, Box::new(lhs), Box::new(rhs))
            }
            constants::UPDATE_EXPRESSION => {
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
            constants::UNARY_EXPRESSION | constants::POINTER_EXPRESSION => {
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
            constants::PARENTHESIZED_EXPRESSION => {
                self.cursor.goto_first_child();
                self.cursor.goto_next_sibling();

                let expr_kind = self.lower_expr_kind()?;

                self.cursor.goto_parent();

                expr_kind
            }
            constants::IF_STATEMENT => {
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
            constants::WHILE_STATEMENT => {
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
            constants::DO_STATEMENT => {
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

                let (mut stmts, resolver) = match body.kind {
                    ExprKind::Block(block) => (block.stmts, block.resolver),
                    _ => {
                        let span = body.span.clone();

                        (
                            vec![Stmt {
                                kind: StmtKind::Semi(body),
                                span: span.clone(),
                            }],
                            self.resolver.clone(),
                        )
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
                    resolver,
                    span: Span {
                        lo: node.start_byte(),
                        hi: node.end_byte(),
                    },
                })
            }
            constants::FOR_STATEMENT => {
                self.cursor.goto_first_child();
                self.cursor.goto_next_sibling();
                self.cursor.goto_next_sibling();

                let pre_resolver = self.resolver.clone();

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

                let resolver = mem::replace(&mut self.resolver, pre_resolver);

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
                            resolver: resolver.clone(),
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
                    resolver,
                    span: Span {
                        lo: node.start_byte(),
                        hi: node.end_byte(),
                    },
                })
            }
            constants::ASSIGNMENT_EXPRESSION => {
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
            constants::FIELD_EXPRESSION => {
                self.cursor.goto_first_child();

                let target = self.lower_expr()?;

                self.cursor.goto_next_sibling();
                self.cursor.goto_next_sibling();

                let field = self.lower_ident()?;

                self.cursor.goto_parent();

                ExprKind::Field(Box::new(target), field)
            }
            constants::SUBSCRIPT_EXPRESSION => {
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
            constants::BREAK_STATEMENT => ExprKind::Break,
            constants::CONTINUE_STATEMENT => ExprKind::Continue,
            constants::CAST_EXPRESSION => {
                self.cursor.goto_first_child();
                self.cursor.goto_next_sibling();

                let ty = self.lower_ty()?;

                self.cursor.goto_next_sibling();
                self.cursor.goto_next_sibling();

                let target = self.lower_expr()?;

                self.cursor.goto_parent();

                ExprKind::Cast(Box::new(target), ty)
            }
            constants::INITIALIZER_LIST => {
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
            constants::COMMA_EXPRESSION => {
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
            constants::SIZEOF_EXPRESSION => ExprKind::Sizeof(self.lower_sizeof()?),
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
