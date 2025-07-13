#![allow(clippy::missing_docs_in_private_items)]

use std::mem;

use anyhow::{Context, bail};
use log::trace;

use crate::hir::{constants, datatypes::*};

impl LoweringCtx<'_> {
    fn lower_to_bin_op(&mut self) -> anyhow::Result<BinOp> {
        let node = self.cursor.node();
        trace!("Construct [BinOp] from node: {}", node.kind());

        Ok(match node.kind() {
            constants::ADD | constants::ASSIGN_ADD | constants::INC => BinOp::Add,
            constants::SUB | constants::ASSIGN_SUB | constants::DEC => BinOp::Sub,
            constants::MUL | constants::ASSIGN_MUL => BinOp::Mul,
            constants::DIV | constants::ASSIGN_DIV => BinOp::Div,
            constants::REM | constants::ASSIGN_REM => BinOp::Rem,
            constants::AND => BinOp::And,
            constants::OR => BinOp::Or,
            constants::BIT_XOR | constants::ASSIGN_BIT_XOR => BinOp::BitXor,
            constants::BIT_AND | constants::ASSIGN_BIT_AND => BinOp::BitAnd,
            constants::BIT_OR | constants::ASSIGN_BIT_OR => BinOp::BitOr,
            constants::SHL | constants::ASSIGN_SHL => BinOp::Shl,
            constants::SHR | constants::ASSIGN_SHR => BinOp::Shr,
            constants::EQ => BinOp::Eq,
            constants::LT => BinOp::Lt,
            constants::LE => BinOp::Le,
            constants::NE => BinOp::Ne,
            constants::GE => BinOp::Ge,
            constants::GT => BinOp::Gt,
            constants::ASSIGN => BinOp::Assign,
            kind => bail!("Unsupported [BinOp] node: {kind}"),
        })
    }

    fn lower_to_un_op(&mut self) -> anyhow::Result<UnOp> {
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

    fn lower_to_sizeof_kind(&mut self) -> anyhow::Result<SizeofKind> {
        let node = self.cursor.node();
        trace!("Construct [SizeofKind] from node: {}", node.kind());

        self.cursor.goto_first_child();
        self.cursor.goto_next_sibling();

        let sizeof_kind = match self.cursor.node().kind() {
            constants::PARENTHESIZED_EXPRESSION => {
                SizeofKind::Expr(Box::new(self.lower_to_expr()?))
            }
            _ => {
                self.cursor.goto_next_sibling();

                SizeofKind::Ty(self.lower_to_ty()?)
            }
        };

        self.cursor.goto_parent();

        Ok(sizeof_kind)
    }

    fn lower_to_sizeof(&mut self) -> anyhow::Result<Sizeof> {
        let node = self.cursor.node();
        trace!("Construct [SizeOf] from node: {}", node.kind());

        Ok(Sizeof {
            kind: self.lower_to_sizeof_kind()?,
            span: Span {
                lo: node.start_byte(),
                hi: node.end_byte(),
            },
        })
    }

    fn lower_to_expr_kind(&mut self) -> anyhow::Result<ExprKind> {
        let node = self.cursor.node();
        trace!("Construct [ExprKind] from node: {}", node.kind());

        Ok(match node.kind() {
            kind if kind.contains("literal") => ExprKind::Lit(self.lower_to_lit()?),
            constants::COMPOUND_STATEMENT => ExprKind::Block(self.lower_to_block()?),
            constants::RETURN_STATEMENT => {
                self.cursor.goto_first_child();
                self.cursor.goto_next_sibling();

                let expr = self.lower_to_expr()?;

                self.cursor.goto_parent();

                ExprKind::Ret(Box::new(expr))
            }
            constants::IDENTIFIER => {
                let ident = self.lower_to_ident()?;

                let idx = self
                    .resolver
                    .lookup_idx(&ident.name)
                    .context("Use of unidentified variable.")?;

                ExprKind::Local(idx)
            }
            constants::CALL_EXPRESSION => {
                self.cursor.goto_first_child();

                let path = self.lower_to_expr()?;

                self.cursor.goto_next_sibling();
                self.cursor.goto_first_child();
                self.cursor.goto_next_sibling();

                let mut arguments = vec![];

                while self.cursor.node().kind() != ")" {
                    arguments.push(self.lower_to_expr()?);

                    self.cursor.goto_next_sibling();
                    self.cursor.goto_next_sibling();
                }

                self.cursor.goto_parent();
                self.cursor.goto_parent();

                ExprKind::Call(Box::new(path), arguments)
            }
            constants::EXPRESSION_STATEMENT => {
                self.cursor.goto_first_child();

                let expr_kind = self.lower_to_expr_kind()?;

                self.cursor.goto_parent();

                expr_kind
            }
            constants::BINARY_EXPRESSION => {
                self.cursor.goto_first_child();

                let lhs = self.lower_to_expr()?;

                self.cursor.goto_next_sibling();

                let bin_op = self.lower_to_bin_op()?;

                self.cursor.goto_next_sibling();

                let rhs = self.lower_to_expr()?;

                self.cursor.goto_parent();

                ExprKind::Binary(bin_op, Box::new(lhs), Box::new(rhs))
            }
            constants::UPDATE_EXPRESSION => {
                self.cursor.goto_first_child();

                let lhs = self.lower_to_expr()?;

                self.cursor.goto_next_sibling();

                let bin_op = self.lower_to_bin_op()?;
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

                let un_op = self.lower_to_un_op()?;

                self.cursor.goto_next_sibling();

                let expr = self.lower_to_expr()?;

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

                let expr_kind = self.lower_to_expr_kind()?;

                self.cursor.goto_parent();

                expr_kind
            }
            constants::IF_STATEMENT => {
                self.cursor.goto_first_child();
                self.cursor.goto_next_sibling();

                let condition = self.lower_to_expr()?;

                self.cursor.goto_next_sibling();

                let body = self.lower_to_expr()?;

                let else_clause = if self.cursor.goto_next_sibling() {
                    self.cursor.goto_first_child();
                    self.cursor.goto_next_sibling();

                    let x = self.lower_to_expr()?;

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

                let condition = self.lower_to_expr()?;

                self.cursor.goto_next_sibling();

                let body = self.lower_to_expr()?;

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

                let body = self.lower_to_expr()?;

                self.cursor.goto_next_sibling();
                self.cursor.goto_next_sibling();

                let condition = self.lower_to_expr()?;

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
                        let span = body.span;

                        (
                            vec![Stmt {
                                kind: StmtKind::Semi(body),
                                span,
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

                let initialization = self.lower_to_stmt()?;

                self.cursor.goto_next_sibling();

                let condition = self.lower_to_expr()?;

                self.cursor.goto_next_sibling();
                self.cursor.goto_next_sibling();

                let update_stmt = Stmt {
                    kind: StmtKind::Semi(self.lower_to_expr()?),
                    span: Span {
                        lo: self.cursor.node().start_byte(),
                        hi: self.cursor.node().end_byte(),
                    },
                };

                self.cursor.goto_next_sibling();
                self.cursor.goto_next_sibling();

                let mut body = self.lower_to_expr()?;

                self.cursor.goto_parent();

                let resolver = mem::replace(&mut self.resolver, pre_resolver);

                match &mut body.kind {
                    ExprKind::Block(block) => {
                        block.stmts.push(update_stmt);
                    }
                    _ => {
                        let span = body.span;

                        let mut block = Block {
                            stmts: vec![Stmt {
                                kind: StmtKind::Semi(body),
                                span,
                            }],
                            resolver: resolver.clone(),
                            span,
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

                let lhs = self.lower_to_expr()?;

                self.cursor.goto_next_sibling();

                let bin_op = self.lower_to_bin_op()?;

                self.cursor.goto_next_sibling();

                let rhs = self.lower_to_expr()?;

                self.cursor.goto_parent();

                match bin_op {
                    BinOp::Assign => ExprKind::Assign(Box::new(lhs), Box::new(rhs)),
                    _ => ExprKind::AssignOp(bin_op, Box::new(lhs), Box::new(rhs)),
                }
            }
            constants::FIELD_EXPRESSION => {
                self.cursor.goto_first_child();

                let target = self.lower_to_expr()?;

                self.cursor.goto_next_sibling();
                self.cursor.goto_next_sibling();

                let field = self.lower_to_ident()?;

                self.cursor.goto_parent();

                ExprKind::Field(Box::new(target), field)
            }
            constants::SUBSCRIPT_EXPRESSION => {
                self.cursor.goto_first_child();

                let target = self.lower_to_expr()?;

                self.cursor.goto_next_sibling();
                self.cursor.goto_next_sibling();

                let index = self.lower_to_expr()?;

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

                let ty = self.lower_to_ty()?;

                self.cursor.goto_next_sibling();
                self.cursor.goto_next_sibling();

                let target = self.lower_to_expr()?;

                self.cursor.goto_parent();

                ExprKind::Cast(Box::new(target), ty)
            }
            constants::INITIALIZER_LIST => {
                self.cursor.goto_first_child();
                self.cursor.goto_next_sibling();

                let mut elements = vec![];

                loop {
                    elements.push(self.lower_to_expr()?);

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
                    exprs.push(self.lower_to_expr()?);

                    self.cursor.goto_next_sibling();
                    if !self.cursor.goto_next_sibling() {
                        break;
                    }
                }

                self.cursor.goto_parent();

                ExprKind::Comma(exprs)
            }
            constants::SIZEOF_EXPRESSION => ExprKind::Sizeof(self.lower_to_sizeof()?),
            kind => bail!("Unsupported [ExprKind] node: {kind}"),
        })
    }

    pub(crate) fn lower_to_expr(&mut self) -> anyhow::Result<Expr> {
        let node = self.cursor.node();
        trace!("Construct [Expr] from node: {}", node.kind());

        Ok(Expr {
            kind: self.lower_to_expr_kind()?,
            span: Span {
                lo: node.start_byte(),
                hi: node.end_byte(),
            },
        })
    }
}
