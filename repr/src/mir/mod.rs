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

use crate::hir::{self, Lit, LitKind, PrimTyKind, Span, Ty, TyKind, resolver::Resolver};

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
                    self.map.insert(resolver_idx, local);
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
            hir::ExprKind::Lit(lit) => {
                let place = self.lower_lit(lit);

                bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(
                        place.clone(),
                        Rvalue::Use(Operand::Const(lit.clone())),
                    ),
                    span,
                });

                Some(place)
            }
            hir::ExprKind::Ret(expr) => {
                let local = self.lower_expr(expr, bb_data);

                if let Some(local) = local {
                    bb_data.statements.push(Statement {
                        kind: StatementKind::Assign(
                            Place {
                                local: Local::from_raw(RawIdx::from_u32(0)),
                                projections: vec![],
                            },
                            Rvalue::Use(Operand::Place(local)),
                        ),
                        span,
                    });
                }

                bb_data.terminator = Some(Terminator {
                    kind: TerminatorKind::Return,
                    span,
                });

                _ = self.add_basic_block(bb_data);

                None
            }
            hir::ExprKind::Path(path) => {
                return self.map.get(&path.res).map(|local| Place {
                    local: *local,
                    projections: vec![],
                });
            }
            hir::ExprKind::Call(expr, exprs) => todo!(),
            hir::ExprKind::Binary(bin_op, left_expr, right_expr) => {
                let left_local = self.lower_expr(left_expr, bb_data).unwrap();
                let right_local = self.lower_expr(right_expr, bb_data).unwrap();

                let ty = self.body.borrow().local_decls[left_local.local].ty.clone();

                let local = self.add_local(&ty, span);

                let lh_place = Place {
                    local,
                    projections: vec![],
                };

                bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(
                        lh_place.clone(),
                        Rvalue::BinaryOp(
                            *bin_op,
                            Box::new(Operand::Place(left_local)),
                            Box::new(Operand::Place(right_local)),
                        ),
                    ),
                    span,
                });

                return Some(lh_place);
            }
            hir::ExprKind::Unary(un_op, expr) => todo!(),
            hir::ExprKind::If(expr, expr1, expr2) => todo!(),
            hir::ExprKind::Loop(loop_source, expr) => todo!(),
            hir::ExprKind::Break => todo!(),
            hir::ExprKind::Continue => todo!(),
            hir::ExprKind::Assign(lhs_expr, rhs_expr) => {
                let lhs_local = self.lower_expr(lhs_expr, bb_data).unwrap();
                let rhs_local = self.lower_expr(rhs_expr, bb_data).unwrap();

                bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(lhs_local, Rvalue::Use(Operand::Place(rhs_local))),
                    span,
                });

                return None;
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
                                Rvalue::Use(Operand::Place(init_local)),
                            ),
                            span,
                        });
                    }
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

    // pub(crate) fn lower_to_rvalue(&mut self, expr: &'mir hir::Expr) -> Rvalue {
    //     match &expr.kind {
    //         hir::ExprKind::Lit(lit) => todo!(),
    //         hir::ExprKind::Path(path) => todo!(),
    //         hir::ExprKind::Call(expr, exprs) => todo!(),
    //         hir::ExprKind::Binary(bin_op, expr, expr1) => todo!(),
    //         hir::ExprKind::Unary(un_op, expr) => todo!(),
    //         hir::ExprKind::Assign(expr, expr1) => todo!(),
    //         hir::ExprKind::AssignOp(bin_op, expr, expr1) => todo!(),
    //         hir::ExprKind::Field(expr, ident) => todo!(),
    //         hir::ExprKind::Index(expr, expr1, span) => todo!(),
    //         hir::ExprKind::Cast(expr, ty) => todo!(),
    //         hir::ExprKind::Array(exprs) => todo!(),
    //         hir::ExprKind::AddrOf(expr) => todo!(),
    //         hir::ExprKind::Comma(exprs) => todo!(),
    //         hir::ExprKind::Sizeof(sizeof) => todo!(),
    //     }
    // }

    pub(crate) fn lower_lit(&self, lit: &Lit) -> Place {
        let kind = match &lit.kind {
            LitKind::Str(_) => todo!(),
            LitKind::Char(_) => TyKind::PrimTy(PrimTyKind::Char),
            LitKind::Int(_) => TyKind::PrimTy(PrimTyKind::Int),
            LitKind::Float(_) => TyKind::PrimTy(PrimTyKind::Float),
        };

        let ty = Ty {
            kind,
            span: lit.span,
        };

        let local = self.add_local(&ty, lit.span);

        Place {
            local,
            projections: vec![],
        }
    }
}
