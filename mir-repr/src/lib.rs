//! The MIR - "Mid-level Intermediate Representation" is a radically simplified form constructed from HIR.
//! This representation is used for generating CFG - "Control Flow Graph" - of the source code.
//!
//! This implementation has been modeled after rustc's MIR representation.
//!

mod datatypes;
mod display;

pub use datatypes::*;

use std::{cell::RefCell, collections::HashMap};

use la_arena::{Arena, RawIdx};

use hir_repr::{Resolver, Span, Ty};

impl<'mir> MirCtx<'mir> {
    pub fn new(resolver: &'mir Resolver, span: Span) -> Self {
        Self {
            body: RefCell::new(Body {
                basic_blocks: Arena::new(),
                local_decls: Arena::new(),
                span,
            }),
            resolver,
            map: HashMap::new(),
        }
    }

    fn add_basic_block(&self, bb_data: &BasicBlockData) -> BasicBlock {
        self.body.borrow_mut().basic_blocks.alloc(bb_data.clone())
    }

    fn add_local(&self, ty: &Ty, span: Span) -> Local {
        self.body.borrow_mut().local_decls.alloc(LocalDecl {
            ty: ty.clone(),
            span,
        })
    }

    pub fn lower(mut self, item: &'mir hir_repr::Fn) -> Option<Body> {
        self.add_local(&item.sig.ty, item.body.span);

        for param in &item.sig.params {
            match &param.ident {
                Some(ident) => {
                    let resolver_idx = item.resolver.lookup_idx(&ident.name).unwrap();
                    let local = self.add_local(&param.ty, param.ty.span);
                    self.map.insert(resolver_idx, local);
                }
                None => return None,
            }
        }

        let mut bb_data = BasicBlockData::default();

        self.lower_expr(&item.body, &mut bb_data);

        Some(self.body.into_inner())
    }

    fn lower_expr(
        &mut self,
        expr: &'mir hir_repr::Expr,
        bb_data: &mut BasicBlockData,
    ) -> Option<Local> {
        let span = expr.span;

        match &expr.kind {
            hir_repr::ExprKind::Block(block) => {
                let pre_resolver = self.resolver;
                self.resolver = &block.resolver;

                for stmt in &block.stmts {
                    self.lower_stmt(stmt, bb_data);
                }

                self.resolver = pre_resolver;

                return None;
            }
            hir_repr::ExprKind::Lit(lit) => todo!(),
            hir_repr::ExprKind::Ret(expr) => {
                let local = self.lower_expr(expr, bb_data);

                if let Some(local) = local {
                    bb_data.statements.push(Statement {
                        kind: StatementKind::Assign(
                            Place {
                                local: Local::from_raw(RawIdx::from_u32(0)),
                                projections: vec![],
                            },
                            Rvalue::Use(Operand::Place(Place {
                                local,
                                projections: vec![],
                            })),
                        ),
                        span,
                    });
                }

                bb_data.terminator = Some(Terminator {
                    kind: TerminatorKind::Return,
                    span,
                });

                _ = self.add_basic_block(bb_data);

                return None;
            }
            hir_repr::ExprKind::Path(path) => {
                let local = self.map.get(&path.res);

                return local.cloned();
            }
            hir_repr::ExprKind::Call(expr, exprs) => todo!(),
            hir_repr::ExprKind::Binary(bin_op, left_expr, right_expr) => {
                let left_local = self.lower_expr(left_expr, bb_data).unwrap();
                let right_local = self.lower_expr(right_expr, bb_data).unwrap();

                let ty = self.body.borrow().local_decls[left_local].ty.clone();

                let local = self.add_local(&ty, span);

                bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(
                        Place {
                            local: local,
                            projections: vec![],
                        },
                        Rvalue::BinaryOp(
                            *bin_op,
                            Box::new(Operand::Place(Place {
                                local: left_local,
                                projections: vec![],
                            })),
                            Box::new(Operand::Place(Place {
                                local: right_local,
                                projections: vec![],
                            })),
                        ),
                    ),
                    span,
                });

                return Some(local);
            }
            hir_repr::ExprKind::Unary(un_op, expr) => todo!(),
            hir_repr::ExprKind::If(expr, expr1, expr2) => todo!(),
            hir_repr::ExprKind::Loop(loop_source, expr) => todo!(),
            hir_repr::ExprKind::Break => todo!(),
            hir_repr::ExprKind::Continue => todo!(),
            hir_repr::ExprKind::Assign(lhs_expr, rhs_expr) => {
                let lhs_local = self.lower_expr(lhs_expr, bb_data).unwrap();
                let rhs_local = self.lower_expr(rhs_expr, bb_data).unwrap();

                bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(
                        Place {
                            local: lhs_local,
                            projections: vec![],
                        },
                        Rvalue::Use(Operand::Place(Place {
                            local: rhs_local,
                            projections: vec![],
                        })),
                    ),
                    span,
                });

                return None;
            }
            hir_repr::ExprKind::AssignOp(bin_op, expr, expr1) => todo!(),
            hir_repr::ExprKind::Field(expr, ident) => todo!(),
            hir_repr::ExprKind::Index(expr, expr1, span) => todo!(),
            hir_repr::ExprKind::Cast(expr, ty) => todo!(),
            hir_repr::ExprKind::Array(exprs) => todo!(),
            hir_repr::ExprKind::AddrOf(expr) => todo!(),
            hir_repr::ExprKind::Comma(exprs) => todo!(),
            hir_repr::ExprKind::Sizeof(sizeof) => todo!(),
        }
    }

    fn lower_stmt(&mut self, stmt: &'mir hir_repr::Stmt, bb_data: &mut BasicBlockData) {
        let span = stmt.span;

        match &stmt.kind {
            hir_repr::StmtKind::Decl(decl_stmt) => {
                let resolver_idx = self.resolver.lookup_idx(&decl_stmt.ident.name).unwrap();
                let local = self.add_local(&decl_stmt.ty, decl_stmt.span);
                self.map.insert(resolver_idx, local);

                if let Some(init) = &decl_stmt.init {
                    let init_local = self.lower_expr(init, bb_data);

                    if let Some(init_local) = init_local {
                        bb_data.statements.push(Statement {
                            kind: StatementKind::Assign(
                                Place {
                                    local,
                                    projections: vec![],
                                },
                                Rvalue::Use(Operand::Place(Place {
                                    local: init_local,
                                    projections: vec![],
                                })),
                            ),
                            span,
                        });
                    }
                }
            }
            hir_repr::StmtKind::Expr(expr) => todo!(),
            hir_repr::StmtKind::Semi(expr) => {
                self.lower_expr(expr, bb_data);
            }
        }
    }
}
