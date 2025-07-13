//! The MIR - "Mid-level Intermediate Representation" is a radically simplified form constructed from HIR.
//! This representation is used for generating CFG - "Control Flow Graph" - of the source code.
//!
//! This implementation has been modeled after rustc's MIR representation.
//!

/// Contains datatypes used to represent the MIR.
mod datatypes;

pub use datatypes::*;

use std::{cell::RefCell, collections::HashMap};

use la_arena::{Arena, RawIdx};

use crate::hir::{self, PrimTyKind, Span, Ty, TyKind, resolver::Resolver};

impl<'mir> MirCtx<'mir> {
    pub fn new(resolver: &'mir Resolver, span: Span) -> Self {
        Self {
            body: RefCell::new(Body {
                basic_blocks: Arena::new(),
                local_decls: Arena::new(),
                span,
            }),
            resolver,
            local_map: HashMap::new(),
        }
    }

    pub(crate) fn add_basic_block(&self, bb_data: &BasicBlockData) -> BasicBlock {
        self.body.borrow_mut().basic_blocks.alloc(bb_data.clone())
    }

    pub(crate) fn add_local(&self, ty: &Ty, span: Span) -> Local {
        self.body.borrow_mut().local_decls.alloc(LocalDecl {
            ty: ty.clone(),
            span,
        })
    }

    pub fn lower(mut self, item: &'mir hir::Fn) -> Option<Body> {
        self.add_local(&item.sig.ty, item.body.span);

        for param in &item.sig.params {
            match &param.ident {
                Some(ident) => {
                    let resolver_idx = item.resolver.lookup_idx(&ident.name).unwrap();
                    let local = self.add_local(&param.ty, param.ty.span);
                    self.local_map.insert(resolver_idx, local);
                }
                None => return None,
            }
        }

        let mut bb_data = BasicBlockData::default();

        self.lower_expr(&item.body, &mut bb_data);

        Some(self.body.into_inner())
    }

    pub(crate) fn lower_expr(
        &mut self,
        expr: &'mir hir::Expr,
        bb_data: &mut BasicBlockData,
    ) -> Option<Place> {
        let span = expr.span;

        match &expr.kind {
            hir::ExprKind::Block(block) => {
                let pre_resolver = self.resolver;
                self.resolver = &block.resolver;

                for stmt in &block.stmts {
                    self.lower_stmt(stmt, bb_data);
                }

                self.resolver = pre_resolver;

                None
            }
            hir::ExprKind::Lit(lit) => todo!(),
            hir::ExprKind::Ret(expr) => {
                let rvalue = self.lower_to_rvalue(expr, bb_data);

                bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(
                        Place {
                            local: Local::from_raw(RawIdx::from_u32(0)),
                            projections: vec![],
                        },
                        rvalue,
                    ),
                    span,
                });

                bb_data.terminator = Some(Terminator {
                    kind: TerminatorKind::Return,
                    span,
                });

                _ = self.add_basic_block(bb_data);

                None
            }
            hir::ExprKind::Path(path) => todo!(),
            hir::ExprKind::Call(expr, exprs) => todo!(),
            hir::ExprKind::Binary(bin_op, left_expr, right_expr) => None,
            hir::ExprKind::Unary(un_op, expr) => None,
            hir::ExprKind::If(expr, expr1, expr2) => todo!(),
            hir::ExprKind::Loop(loop_source, expr) => todo!(),
            hir::ExprKind::Break => todo!(),
            hir::ExprKind::Continue => todo!(),
            hir::ExprKind::Assign(lhs_expr, rhs_expr) => {
                let place = self.lower_to_place(lhs_expr);

                let rvalue = self.lower_to_rvalue(rhs_expr, bb_data);

                bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(place.clone(), rvalue),
                    span,
                });

                Some(place)
            }
            hir::ExprKind::AssignOp(bin_op, expr, expr1) => todo!(),
            hir::ExprKind::Field(expr, ident) => todo!(),
            hir::ExprKind::Index(expr, expr1, span) => todo!(),
            hir::ExprKind::Cast(expr, ty) => todo!(),
            hir::ExprKind::Array(exprs) => todo!(),
            hir::ExprKind::AddrOf(expr) => todo!(),
            hir::ExprKind::Comma(exprs) => todo!(),
            hir::ExprKind::Sizeof(sizeof) => todo!(),
        }
    }

    pub(crate) fn lower_stmt(&mut self, stmt: &'mir hir::Stmt, bb_data: &mut BasicBlockData) {
        let span = stmt.span;

        match &stmt.kind {
            hir::StmtKind::Decl(decl_stmt) => {
                let init_rvalue = decl_stmt
                    .init
                    .as_ref()
                    .map(|init_expr| self.lower_to_rvalue(init_expr, bb_data));

                let resolver_idx = self.resolver.lookup_idx(&decl_stmt.ident.name).unwrap();
                let local = self.add_local(&decl_stmt.ty, decl_stmt.span);
                self.local_map.insert(resolver_idx, local);

                if let Some(init_rvalue) = init_rvalue {
                    bb_data.statements.push(Statement {
                        kind: StatementKind::Assign(
                            Place {
                                local,
                                projections: vec![],
                            },
                            init_rvalue,
                        ),
                        span,
                    });
                }
            }
            hir::StmtKind::Expr(expr) => {
                _ = self.lower_expr(expr, bb_data);
            }
            hir::StmtKind::Semi(expr) => {
                _ = self.lower_expr(expr, bb_data);
            }
        }
    }

    pub(crate) fn lower_to_rvalue(
        &mut self,
        expr: &'mir hir::Expr,
        bb_data: &mut BasicBlockData,
    ) -> Rvalue {
        let span = expr.span;

        match &expr.kind {
            hir::ExprKind::Path(_) | hir::ExprKind::Lit(_) => {
                Rvalue::Use(self.lower_to_operand(expr, bb_data))
            }
            hir::ExprKind::Binary(bin_op, left_expr, right_expr) => {
                let left_operand = self.lower_to_operand(left_expr, bb_data);
                let right_operand = self.lower_to_operand(right_expr, bb_data);

                Rvalue::BinaryOp(*bin_op, left_operand, right_operand)
            }
            hir::ExprKind::Unary(un_op, expr) => {
                let operand = self.lower_to_operand(expr, bb_data);

                Rvalue::UnaryOp(*un_op, operand)
            }
            hir::ExprKind::Assign(lhs_expr, rhs_expr) => {
                let place = self.lower_to_place(lhs_expr);

                let rvalue = self.lower_to_rvalue(rhs_expr, bb_data);

                bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(place.clone(), rvalue),
                    span,
                });

                Rvalue::Use(Operand::Place(place))
            }
            hir::ExprKind::Call(expr, exprs) => {
                let operand = self.lower_to_operand(expr, bb_data);

                let arguments = exprs
                    .iter()
                    .map(|expr| self.lower_to_operand(expr, bb_data))
                    .collect();

                Rvalue::Call(operand, arguments)
            }
            _ => unreachable!(),
        }
    }

    pub(crate) fn lower_to_operand(
        &mut self,
        expr: &'mir hir::Expr,
        bb_data: &mut BasicBlockData,
    ) -> Operand {
        let span = expr.span;

        match &expr.kind {
            hir::ExprKind::Lit(lit) => Operand::Const(Const::Lit(lit.clone())),
            hir::ExprKind::Path(path) => {
                let Some(local) = self.local_map.get(&path.res) else {
                    return Operand::Const(Const::Function(path.res));
                };

                Operand::Place(Place {
                    local: *local,
                    projections: vec![],
                })
            }
            hir::ExprKind::Binary(bin_op, left_expr, right_expr) => {
                let left_operand = self.lower_to_operand(left_expr, bb_data);
                let right_operand = self.lower_to_operand(right_expr, bb_data);

                let local = self.add_local(
                    &Ty {
                        kind: TyKind::PrimTy(PrimTyKind::Int),
                        span,
                    },
                    span,
                );

                let place = Place {
                    local,
                    projections: vec![],
                };

                bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(
                        place.clone(),
                        Rvalue::BinaryOp(*bin_op, left_operand, right_operand),
                    ),
                    span,
                });

                Operand::Place(place)
            }
            hir::ExprKind::Unary(un_op, expr) => {
                let operand = self.lower_to_operand(expr, bb_data);

                let local = self.add_local(
                    &Ty {
                        kind: TyKind::PrimTy(PrimTyKind::Int),
                        span,
                    },
                    span,
                );

                let place = Place {
                    local,
                    projections: vec![],
                };

                bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(place.clone(), Rvalue::UnaryOp(*un_op, operand)),
                    span,
                });

                Operand::Place(place)
            }
            _ => unreachable!(),
        }
    }

    pub(crate) fn lower_to_place(&mut self, expr: &'mir hir::Expr) -> Place {
        match &expr.kind {
            hir::ExprKind::Path(path) => {
                let local = self.local_map.get(&path.res).unwrap();

                Place {
                    local: *local,
                    projections: vec![],
                }
            }
            _ => unreachable!(),
        }
    }
}
