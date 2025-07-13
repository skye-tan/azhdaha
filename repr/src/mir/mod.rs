//! The MIR - "Mid-level Intermediate Representation" is a radically simplified form constructed from HIR.
//! This representation is used for generating CFG - "Control Flow Graph" - of the source code.
//!
//! This implementation has been modeled after rustc's MIR representation.
//!

/// Contains datatypes used to represent the MIR.
mod datatypes;

pub use datatypes::*;

use std::{collections::HashMap, mem};

use la_arena::{Arena, RawIdx};

use crate::hir::{self, PrimTyKind, Span, Ty, TyKind, resolver::Resolver};

impl<'mir> MirCtx<'mir> {
    pub fn new(resolver: &'mir Resolver, span: Span) -> Self {
        Self {
            body: Body {
                basic_blocks: Arena::new(),
                local_decls: Arena::new(),
                resolver,
                span,
            },
            bb_data: BasicBlockData::default(),
            local_map: HashMap::new(),
        }
    }

    pub(crate) fn next_bb(&mut self) -> BasicBlock {
        let bb_data = mem::replace(&mut self.bb_data, BasicBlockData::default());

        self.body.basic_blocks.alloc(bb_data)
    }

    pub(crate) fn add_local(&mut self, debug_ident: Option<String>, ty: &Ty, span: Span) -> Local {
        self.body.local_decls.alloc(LocalDecl {
            debug_ident,
            ty: ty.clone(),
            span,
        })
    }

    pub fn lower(mut self, item: &'mir hir::Fn) -> anyhow::Result<Body<'mir>> {
        self.add_local(None, &item.sig.ty, item.body.span);

        for param in &item.sig.params {
            match &param.res {
                Some(res) => {
                    let res_data = item.resolver.get_item(res);

                    let local =
                        self.add_local(Some(res_data.ident.name.clone()), &param.ty, param.span);

                    self.local_map.insert(*res, local);
                }
                None => (),
            }
        }

        _ = self.lower_expr(&item.body);

        Ok(self.body)
    }

    pub(crate) fn lower_expr(&mut self, expr: &'mir hir::Expr) -> Option<Place> {
        let span = expr.span;

        match &expr.kind {
            hir::ExprKind::Block(block) => {
                let pre_resolver = self.body.resolver;
                self.body.resolver = &block.resolver;

                for stmt in &block.stmts {
                    self.lower_stmt(stmt);
                }

                self.body.resolver = pre_resolver;

                None
            }
            hir::ExprKind::Lit(lit) => todo!(),
            hir::ExprKind::Ret(expr) => {
                let rvalue = self.lower_to_rvalue(expr);

                self.bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(
                        Place {
                            local: Local::from_raw(RawIdx::from_u32(0)),
                            projections: vec![],
                        },
                        rvalue,
                    ),
                    span,
                });

                self.bb_data.terminator = Some(Terminator {
                    kind: TerminatorKind::Return,
                    span,
                });

                _ = self.next_bb();

                None
            }
            hir::ExprKind::Local(res) => todo!(),
            hir::ExprKind::Call(expr, exprs) => todo!(),
            hir::ExprKind::Binary(bin_op, left_expr, right_expr) => None,
            hir::ExprKind::Unary(un_op, expr) => None,
            hir::ExprKind::If(expr, expr1, expr2) => todo!(),
            hir::ExprKind::Loop(loop_source, expr) => todo!(),
            hir::ExprKind::Break => todo!(),
            hir::ExprKind::Continue => todo!(),
            hir::ExprKind::Assign(lhs_expr, rhs_expr) => {
                let place = self.lower_to_place(lhs_expr);

                let rvalue = self.lower_to_rvalue(rhs_expr);

                self.bb_data.statements.push(Statement {
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

    pub(crate) fn lower_stmt(&mut self, stmt: &'mir hir::Stmt) {
        let span = stmt.span;

        match &stmt.kind {
            hir::StmtKind::Decl(decl_stmt) => {
                let init_rvalue = decl_stmt
                    .init
                    .as_ref()
                    .map(|init_expr| self.lower_to_rvalue(init_expr));

                let res_data = self.body.resolver.get_item(&decl_stmt.res);

                let local = self.add_local(
                    Some(res_data.ident.name.clone()),
                    &decl_stmt.ty,
                    decl_stmt.span,
                );

                self.local_map.insert(decl_stmt.res, local);

                if let Some(init_rvalue) = init_rvalue {
                    self.bb_data.statements.push(Statement {
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
                _ = self.lower_expr(expr);
            }
            hir::StmtKind::Semi(expr) => {
                _ = self.lower_expr(expr);
            }
        }
    }

    pub(crate) fn lower_to_rvalue(&mut self, expr: &'mir hir::Expr) -> Rvalue {
        let span = expr.span;

        match &expr.kind {
            hir::ExprKind::Local(_) | hir::ExprKind::Lit(_) => {
                Rvalue::Use(self.lower_to_operand(expr))
            }
            hir::ExprKind::Binary(bin_op, left_expr, right_expr) => {
                let left_operand = self.lower_to_operand(left_expr);
                let right_operand = self.lower_to_operand(right_expr);

                Rvalue::BinaryOp(*bin_op, left_operand, right_operand)
            }
            hir::ExprKind::Unary(un_op, expr) => {
                let operand = self.lower_to_operand(expr);

                Rvalue::UnaryOp(*un_op, operand)
            }
            hir::ExprKind::Assign(lhs_expr, rhs_expr) => {
                let place = self.lower_to_place(lhs_expr);

                let rvalue = self.lower_to_rvalue(rhs_expr);

                self.bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(place.clone(), rvalue),
                    span,
                });

                Rvalue::Use(Operand::Place(place))
            }
            hir::ExprKind::Call(expr, exprs) => {
                let operand = self.lower_to_operand(expr);

                let arguments = exprs
                    .iter()
                    .map(|expr| self.lower_to_operand(expr))
                    .collect();

                Rvalue::Call(operand, arguments)
            }
            _ => unreachable!(),
        }
    }

    pub(crate) fn lower_to_operand(&mut self, expr: &'mir hir::Expr) -> Operand {
        let span = expr.span;

        match &expr.kind {
            hir::ExprKind::Lit(lit) => Operand::Const(Const::Lit(lit.clone())),
            hir::ExprKind::Local(res) => {
                let Some(local) = self.local_map.get(res) else {
                    return Operand::Const(Const::Fn(*res));
                };

                Operand::Place(Place {
                    local: *local,
                    projections: vec![],
                })
            }
            hir::ExprKind::Binary(bin_op, left_expr, right_expr) => {
                let left_operand = self.lower_to_operand(left_expr);
                let right_operand = self.lower_to_operand(right_expr);

                let local = self.add_local(
                    None,
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

                self.bb_data.statements.push(Statement {
                    kind: StatementKind::Assign(
                        place.clone(),
                        Rvalue::BinaryOp(*bin_op, left_operand, right_operand),
                    ),
                    span,
                });

                Operand::Place(place)
            }
            hir::ExprKind::Unary(un_op, expr) => {
                let operand = self.lower_to_operand(expr);

                let local = self.add_local(
                    None,
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

                self.bb_data.statements.push(Statement {
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
            hir::ExprKind::Local(res) => {
                let local = self.local_map.get(res).unwrap();

                Place {
                    local: *local,
                    projections: vec![],
                }
            }
            _ => unreachable!(),
        }
    }
}
