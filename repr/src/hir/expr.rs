#![allow(clippy::missing_docs_in_private_items)]

use azhdaha_errors::{Context, bail};
use itertools::Either;
use log::trace;

use crate::hir::{initializer_tree::InitializerTree, resolver::SymbolKind, *};

use super::{constants, resolver::Symbol};

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub ty: Ty,
    pub span: Span,
}

impl Expr {
    fn take(&mut self) -> Expr {
        Expr {
            kind: self.kind.take(),
            ty: self.ty.clone(),
            span: self.span,
        }
    }
}

#[derive(Debug)]
pub enum ReturnSemantic {
    /// For x += 1 and ++x
    AfterAssign,
    /// For x++
    BeforeAssign,
}

#[derive(Debug)]
pub enum DesignatorKind {
    Subscript { value: i128 },
    Field { name: String },
}

#[derive(Debug)]
pub struct Designator {
    pub kind: DesignatorKind,
    pub span: Span,
}

#[derive(Debug)]
pub struct InitializerItem {
    pub designators: Option<Vec<Designator>>,
    pub value: ExprOrList,
}

#[derive(Debug)]
pub enum ExprOrList {
    Expr(Expr),
    List(Vec<InitializerItem>),
}

#[derive(Debug)]
pub enum ExprKind {
    Lit(Lit),
    Local(Symbol),
    Call(Box<Expr>, Vec<Expr>),
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Unary(UnOp, Box<Expr>),
    Assign(Box<Expr>, Box<Expr>),
    AssignWithBinOp(Box<Expr>, Box<Expr>, BinOp, Ty, ReturnSemantic),
    Field(Box<Expr>, usize),
    PtrOffset(Box<Expr>, Box<Expr>),
    PtrDiff(Box<Expr>, Box<Expr>),
    AssignPtrOffset(Box<Expr>, Box<Expr>, ReturnSemantic),
    Cast(Box<Expr>),
    InitializerList(Box<InitializerTree>),
    Comma(Vec<Expr>),
    Sizeof(Sizeof),
    VaArg(Box<Expr>, Ty),
    OffsetOf,
    Cond(Box<Expr>, Box<Expr>, Box<Expr>),
    GnuBlock(Block),
    Empty,
}

impl ExprKind {
    fn take(&mut self) -> ExprKind {
        std::mem::replace(self, ExprKind::Empty)
    }
}

#[derive(Debug)]
pub enum BuiltinMacro {
    OffsetOf,
    VaStart,
    VaArg,
    VaEnd,
    AtomicLoad,
    AtomicStore,
}

#[derive(Debug)]
pub struct Sizeof {
    pub kind: SizeofKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum SizeofKind {
    Ty(Ty),
    Expr(Box<Expr>),
}

#[derive(Debug, Clone)]
pub struct Lit {
    pub kind: LitKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum LitKind {
    Str(String),
    Char(char),
    Int(i128),
    Float(f64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Or,
    And,
    BitOr,
    BitXor,
    BitAnd,
    Eq,
    Lt,
    Le,
    Ne,
    Ge,
    Gt,
    Shl,
    Shr,
}

impl BinOp {
    const COMPARISONS: &[Self] = &[
        BinOp::Eq,
        BinOp::Le,
        BinOp::Ge,
        BinOp::Gt,
        BinOp::Lt,
        BinOp::Ne,
    ];
    const SHORT_CIRCUITS: &[Self] = &[BinOp::And, BinOp::Or];

    fn to_un_op(self) -> Option<UnOp> {
        match self {
            BinOp::Add => Some(UnOp::Pos),
            BinOp::Sub => Some(UnOp::Neg),
            BinOp::Mul => Some(UnOp::Deref),
            BinOp::And => Some(UnOp::AddrOf),
            BinOp::Div
            | BinOp::Rem
            | BinOp::Or
            | BinOp::BitOr
            | BinOp::BitXor
            | BinOp::BitAnd
            | BinOp::Eq
            | BinOp::Lt
            | BinOp::Le
            | BinOp::Ne
            | BinOp::Ge
            | BinOp::Gt
            | BinOp::Shl
            | BinOp::Shr => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    Not,
    Neg,
    Com,
    Pos,
    AddrOf,
    Deref,
}

impl HirCtx<'_> {
    pub(crate) fn lower_to_expr_with_expected_type(
        &mut self,
        node: Node,
        ty: Ty,
    ) -> azhdaha_errors::Result<Expr> {
        let expr = self.lower_to_expr_with_maybe_expected_type(node, Some(ty.clone()))?;

        Ok(Expr {
            span: expr.span,
            kind: ExprKind::Cast(Box::new(expr)),
            ty,
        })
    }

    pub(crate) fn lower_to_cond_expr(&mut self, node: Node) -> azhdaha_errors::Result<Expr> {
        let mut expr = self.lower_to_expr(node)?;
        self.condify(&mut expr);
        Ok(expr)
    }

    pub(crate) fn condify(&mut self, expr: &mut Expr) {
        self.array_to_pointer_decay_if_array(expr);
        self.pointer_to_address_decay_if_pointer(expr);
    }

    pub(crate) fn lower_to_expr(&mut self, node: Node) -> azhdaha_errors::Result<Expr> {
        self.lower_to_expr_with_maybe_expected_type(node, None)
    }

    pub(crate) fn lower_to_expr_with_maybe_expected_type(
        &mut self,
        node: Node,
        expected_ty: Option<Ty>,
    ) -> azhdaha_errors::Result<Expr> {
        trace!("[HIR/Expr] Lowering '{}'", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        let (kind, ty) = self.lower_to_expr_kind(node, expected_ty)?;

        Ok(Expr { kind, ty, span })
    }

    fn lower_to_builtin_macro(&self, path_node: Node<'_>) -> Option<BuiltinMacro> {
        match path_node.utf8_text(self.source_code).unwrap() {
            "__builtin_offsetof" => Some(BuiltinMacro::OffsetOf),
            "__builtin_c23_va_start" => Some(BuiltinMacro::VaStart),
            "__builtin_va_arg" => Some(BuiltinMacro::VaArg),
            "__builtin_va_end" => Some(BuiltinMacro::VaEnd),
            "__atomic_load_n" => Some(BuiltinMacro::AtomicLoad),
            "__atomic_store_n" => Some(BuiltinMacro::AtomicStore),
            _ => None,
        }
    }

    /// This function is for when parser parsed a type as an expression, e.g. for typedefs in sizeof.
    pub(crate) fn lower_to_expr_or_type(
        &mut self,
        node: Node,
    ) -> azhdaha_errors::Result<Either<Expr, Ty>> {
        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        Ok(match node.kind() {
            constants::IDENTIFIER => {
                let ident = self.lower_to_ident(node)?;

                let symbol = self
                    .symbol_resolver
                    .get_res_by_name(&ident.name)
                    .with_context(span, || {
                        format!("Use of undefined identifier '{}'.", &ident.name)
                    })?;

                if let SymbolKind::TyDef(ty) = &self.symbol_resolver.arena[symbol] {
                    Either::Right(ty.clone())
                } else {
                    Either::Left(self.lower_to_expr(node)?)
                }
            }
            constants::PARENTHESIZED_EXPRESSION => {
                let child = node.child(1).unwrap();
                self.lower_to_expr_or_type(child)?
            }
            _ => Either::Left(self.lower_to_expr(node)?),
        })
    }

    fn lower_un_op(
        &mut self,
        mut expr: Expr,
        un_op: UnOp,
        span: Span,
    ) -> azhdaha_errors::Result<(ExprKind, Ty)> {
        let ty = match un_op {
            UnOp::Not => {
                self.condify(&mut expr);
                Ty {
                    kind: TyKind::PrimTy(PrimTyKind::Int(4)),
                    is_linear: false,
                    quals: vec![],
                    span,
                }
            }
            UnOp::Neg | UnOp::Com | UnOp::Pos => expr.ty.clone(),
            UnOp::AddrOf => {
                if expr.ty.kind.is_array() {
                    let it = self.array_to_pointer_decay(expr);
                    return Ok((it.kind, it.ty));
                }
                Ty {
                    kind: TyKind::Ptr {
                        kind: Box::new(expr.ty.kind.clone()),
                        quals: vec![],
                    },
                    is_linear: false,
                    quals: vec![],
                    span,
                }
            }
            UnOp::Deref => {
                self.array_to_pointer_decay_if_array(&mut expr);
                let TyKind::Ptr { kind, quals: _ } = &expr.ty.kind else {
                    bail!(span, "Type error: dereference of non-ptr type");
                };
                if kind.is_fn() {
                    // dereference of function pointers is no op.
                    return Ok((expr.kind, expr.ty));
                }
                Ty {
                    kind: *kind.clone(),
                    is_linear: false,
                    quals: vec![],
                    span,
                }
            }
        };

        // Ignore [`UnOp::Pos`] because it has no effects.
        Ok(match un_op {
            UnOp::Pos => (expr.kind, expr.ty),
            _ => (ExprKind::Unary(un_op, Box::new(expr)), ty),
        })
    }

    fn lower_bin_op(
        &mut self,
        mut lhs: Expr,
        mut rhs: Expr,
        bin_op: BinOp,
        span: Span,
        is_assignment: bool,
    ) -> azhdaha_errors::Result<(ExprKind, Ty)> {
        if is_assignment && lhs.ty.kind.is_array() {
            bail!(span, "Type error - can not run binop on arrays.");
        } else {
            self.array_to_pointer_decay_if_array(&mut lhs);
        }
        self.array_to_pointer_decay_if_array(&mut rhs);

        if BinOp::SHORT_CIRCUITS.contains(&bin_op) {
            self.condify(&mut lhs);
            self.condify(&mut rhs);
        }
        if BinOp::COMPARISONS.contains(&bin_op) {
            self.pointer_to_address_decay_if_pointer(&mut lhs);
            self.pointer_to_address_decay_if_pointer(&mut rhs);
        }

        'check_pointers: {
            if bin_op == BinOp::Add {
                let lhs_is_ptr = lhs.ty.kind.is_ptr();
                let rhs_is_ptr = rhs.ty.kind.is_ptr();

                match (lhs_is_ptr, rhs_is_ptr) {
                    (true, true) => bail!(span, "Type error: adding two pointers"),
                    (true, false) => (),
                    (false, true) => {
                        std::mem::swap(&mut lhs, &mut rhs);
                    }
                    (false, false) => break 'check_pointers,
                }

                let ty = lhs.ty.clone();
                return Ok((ExprKind::PtrOffset(Box::new(lhs), Box::new(rhs)), ty));
            }
            if bin_op == BinOp::Sub {
                let lhs_is_ptr = lhs.ty.kind.is_ptr();
                let rhs_is_ptr = rhs.ty.kind.is_ptr();

                match (lhs_is_ptr, rhs_is_ptr) {
                    (true, true) => {
                        let ty = Ty {
                            kind: TyKind::PrimTy(PrimTyKind::Int(8)),
                            is_linear: false,
                            quals: vec![],
                            span,
                        };
                        return Ok((ExprKind::PtrDiff(Box::new(lhs), Box::new(rhs)), ty));
                    }
                    (true, false) => lhs.ty.clone(),
                    (false, true) => {
                        if is_assignment {
                            bail!(span, "Type error - can not rotate arguments in assignment.");
                        }
                        std::mem::swap(&mut lhs, &mut rhs);
                        lhs.ty.clone()
                    }
                    (false, false) => break 'check_pointers,
                };

                let ty = lhs.ty.clone();
                let rhs = Expr {
                    ty: rhs.ty.clone(),
                    span: rhs.span,
                    kind: ExprKind::Unary(UnOp::Neg, Box::new(rhs)),
                };

                return Ok((ExprKind::PtrOffset(Box::new(lhs), Box::new(rhs)), ty));
            }
        }

        let TyKind::PrimTy(lhs_ty) = lhs.ty.kind else {
            bail!(span, "Type error - can not use binop on type {}", lhs.ty);
        };
        let TyKind::PrimTy(rhs_ty) = rhs.ty.kind else {
            bail!(span, "Type error - can not use binop on type {}", rhs.ty);
        };

        let max_ty_kind = lhs_ty.max(rhs_ty);
        if max_ty_kind == PrimTyKind::Void {
            bail!(span, "Type error - can not use binop on void.");
        }
        let max_ty = || Ty {
            kind: TyKind::PrimTy(max_ty_kind),
            is_linear: false,
            quals: vec![],
            span,
        };

        if lhs_ty != max_ty_kind && !is_assignment {
            lhs = Expr {
                kind: ExprKind::Cast(Box::new(lhs)),
                ty: max_ty(),
                span,
            };
        }
        if rhs_ty != max_ty_kind {
            rhs = Expr {
                kind: ExprKind::Cast(Box::new(rhs)),
                ty: max_ty(),
                span,
            };
        }

        let ty = if BinOp::COMPARISONS.contains(&bin_op) || BinOp::SHORT_CIRCUITS.contains(&bin_op)
        {
            Ty {
                kind: TyKind::PrimTy(PrimTyKind::Int(4)),
                is_linear: false,
                quals: vec![],
                span,
            }
        } else {
            rhs.ty.clone()
        };
        Ok((ExprKind::Binary(bin_op, Box::new(lhs), Box::new(rhs)), ty))
    }

    fn pointer_to_address_decay_if_pointer(&mut self, expr: &mut Expr) {
        self.function_to_pointer_decay_if_function(expr);
        if !expr.ty.kind.is_ptr() {
            return;
        }

        let ty = Ty {
            kind: TyKind::PrimTy(PrimTyKind::Int(8)),
            is_linear: false,
            quals: vec![],
            span: expr.span,
        };

        *expr = Expr {
            span: expr.span,
            kind: ExprKind::Cast(Box::new(expr.take())),
            ty,
        };
    }

    fn function_to_pointer_decay_if_function(&mut self, expr: &mut Expr) {
        if expr.ty.kind.is_fn() {
            let ty = Ty {
                kind: TyKind::Ptr {
                    kind: Box::new(expr.ty.kind.clone()),
                    quals: vec![],
                },
                is_linear: false,
                quals: vec![],
                span: expr.span,
            };

            *expr = Expr {
                span: expr.span,
                kind: ExprKind::Cast(Box::new(expr.take())),
                ty,
            };
        }
    }

    fn array_to_pointer_decay_if_array(&mut self, expr: &mut Expr) {
        if !expr.ty.kind.is_array() {
            return;
        }
        *expr = self.array_to_pointer_decay(expr.take());
    }

    fn array_to_pointer_decay(&mut self, expr: Expr) -> Expr {
        let TyKind::Array { kind, size: _ } = &expr.ty.kind else {
            panic!("Expr is not array");
        };

        let ty = Ty {
            kind: TyKind::Ptr {
                kind: kind.clone(),
                quals: vec![],
            },
            is_linear: false,
            quals: vec![],
            span: expr.span,
        };

        Expr {
            span: expr.span,
            kind: ExprKind::Cast(Box::new(expr)),
            ty,
        }
    }

    fn lower_to_expr_kind(
        &mut self,
        node: Node,
        expected_ty: Option<Ty>,
    ) -> azhdaha_errors::Result<(ExprKind, Ty)> {
        trace!("[HIR/ExprKind] Lowering '{}'", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        Ok(match node.kind() {
            constants::IDENTIFIER => {
                let ident = self.lower_to_ident(node)?;

                let symbol = self
                    .symbol_resolver
                    .get_res_by_name(&ident.name)
                    .with_context(span, || {
                        format!("Use of undefined identifier '{}'.", &ident.name)
                    })?;

                let ty = self.symbol_resolver.arena[symbol]
                    .ty()
                    .context(span, "Identifier used here as expression.")?;

                (ExprKind::Local(symbol), ty)
            }
            constants::CALL_EXPRESSION => {
                let mut cursor = node.walk();
                cursor.goto_first_child();

                let path_node = cursor.node();

                let mut argument_nodes = vec![];

                cursor.goto_next_sibling();
                cursor.goto_first_child();
                cursor.goto_next_sibling();

                while cursor.node().kind() != ")" {
                    argument_nodes.push(cursor.node());
                    cursor.goto_next_sibling();
                    cursor.goto_next_sibling();
                }

                drop(cursor); // Make sure no one use it after this.

                if let Some(builtin) = self.lower_to_builtin_macro(path_node) {
                    return Ok(match builtin {
                        BuiltinMacro::OffsetOf => (
                            ExprKind::OffsetOf,
                            Ty {
                                kind: TyKind::PrimTy(PrimTyKind::Int(4)),
                                is_linear: false,
                                quals: vec![],
                                span,
                            },
                        ),
                        _ => (
                            ExprKind::Empty,
                            Ty {
                                kind: TyKind::PrimTy(PrimTyKind::Int(4)),
                                is_linear: false,
                                quals: vec![],
                                span,
                            },
                        ),
                    });
                }

                let path = match self.lower_to_expr_or_type(path_node)? {
                    Either::Left(path) => path,
                    Either::Right(casted_ty) => match argument_nodes.as_slice() {
                        [node] => {
                            let expr = self.lower_to_expr(*node)?;
                            return Ok((ExprKind::Cast(Box::new(expr)), casted_ty));
                        }
                        _ => bail!(span, "I bet no one use comma operator with cast."),
                    },
                };

                let sig = match &path.ty.kind {
                    TyKind::Func { sig } => sig,
                    TyKind::Ptr { kind, quals: _ } => match &**kind {
                        TyKind::Func { sig } => sig,
                        _ => bail!(
                            span,
                            "Type error: invalid call to pointer of non function type {}.",
                            path.ty,
                        ),
                    },
                    _ => bail!(
                        span,
                        "Type error: invalid call to non function type {}.",
                        path.ty,
                    ),
                };

                let mut arguments = vec![];

                for node in argument_nodes {
                    if let Some(param) = sig.params.get(arguments.len()) {
                        arguments
                            .push(self.lower_to_expr_with_expected_type(node, param.ty.clone())?);
                    } else if sig.variadic_param {
                        let mut expr = self.lower_to_expr(node)?;
                        match &expr.ty.kind {
                            TyKind::PrimTy(prim_ty_kind) => {
                                let target = match prim_ty_kind {
                                    PrimTyKind::Bool => PrimTyKind::Int(4),
                                    PrimTyKind::Char => PrimTyKind::Int(4),
                                    PrimTyKind::Int(bytes) => PrimTyKind::Int(4.max(*bytes)),
                                    PrimTyKind::Float(bytes) => PrimTyKind::Float(8.max(*bytes)),
                                    PrimTyKind::Void => {
                                        bail!(
                                            span,
                                            "Type error - can not pass void to variadic functino."
                                        )
                                    }
                                };
                                if target != *prim_ty_kind {
                                    expr = Expr {
                                        kind: ExprKind::Cast(Box::new(expr)),
                                        ty: Ty {
                                            kind: TyKind::PrimTy(target),
                                            is_linear: false,
                                            quals: vec![],
                                            span,
                                        },
                                        span,
                                    }
                                }
                            }
                            TyKind::Ptr { .. } | TyKind::Struct(_) | TyKind::Union(_) => (),
                            TyKind::Array { .. } => {
                                expr = self.array_to_pointer_decay(expr);
                            }
                            _ => bail!(
                                span,
                                "Type error - can not pass {} as variadic argument.",
                                &expr.ty
                            ),
                        }
                        arguments.push(expr);
                    } else {
                        bail!(span, "Type error - too many arguments to call {sig:?}");
                    }
                }

                let ty = sig.ret_ty.clone();

                (ExprKind::Call(Box::new(path), arguments), ty)
            }
            constants::BINARY_EXPRESSION => {
                let lhs = self.lower_to_expr_or_type(node.child(0).unwrap())?;

                let bin_op = self
                    .lower_to_bin_op(node.child(1).unwrap())?
                    .expect("Assignment isn't valid here");

                let rhs = self.lower_to_expr(node.child(2).unwrap())?;

                match lhs {
                    Either::Left(lhs) => self.lower_bin_op(lhs, rhs, bin_op, span, false)?,
                    Either::Right(casted_ty) => {
                        let un_op = bin_op
                            .to_un_op()
                            .context(span, "Can not apply binary operation to type")?;
                        let (kind, ty) = self.lower_un_op(rhs, un_op, span)?;
                        (ExprKind::Cast(Box::new(Expr { kind, ty, span })), casted_ty)
                    }
                }
            }
            constants::UPDATE_EXPRESSION => {
                let (lhs, bin_op, return_semantic) =
                    if let Ok(bin_op) = self.lower_to_bin_op(node.child(1).unwrap()) {
                        let lhs = self.lower_to_expr(node.child(0).unwrap())?;

                        (lhs, bin_op, ReturnSemantic::BeforeAssign)
                    } else {
                        let bin_op = self.lower_to_bin_op(node.child(0).unwrap())?;

                        let lhs = self.lower_to_expr(node.child(1).unwrap())?;

                        (lhs, bin_op, ReturnSemantic::AfterAssign)
                    };

                let rhs = Expr {
                    kind: ExprKind::Lit(Lit {
                        kind: LitKind::Int(1),
                        span,
                    }),
                    span,
                    ty: Ty {
                        kind: TyKind::PrimTy(PrimTyKind::Int(1)),
                        is_linear: false,
                        quals: vec![],
                        span,
                    },
                };

                let ty = lhs.ty.clone();
                let bin_op = bin_op.expect("Assignment isn't valid operator for update?");

                let (kind, binop_ty) = self.lower_bin_op(lhs, rhs, bin_op, span, true)?;
                match kind {
                    ExprKind::Binary(bin_op, lhs, rhs) => (
                        ExprKind::AssignWithBinOp(lhs, rhs, bin_op, binop_ty, return_semantic),
                        ty,
                    ),
                    ExprKind::PtrOffset(lhs, rhs) => {
                        (ExprKind::AssignPtrOffset(lhs, rhs, return_semantic), ty)
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
            constants::UNARY_EXPRESSION | constants::POINTER_EXPRESSION => {
                let un_op = self.lower_to_un_op(node.child(0).unwrap())?;
                let expr = self.lower_to_expr(node.child(1).unwrap())?;

                self.lower_un_op(expr, un_op, span)?
            }
            constants::PARENTHESIZED_EXPRESSION => {
                let child = node.child(1).unwrap();
                if child.kind() == constants::COMPOUND_STATEMENT {
                    let block = self.lower_to_block(child)?;
                    let StmtKind::Expr(last_expr) = &block.stmts.last().unwrap().kind else {
                        bail!(span, "Invalid gnu statement block");
                    };
                    let ty = last_expr.ty.clone();
                    (ExprKind::GnuBlock(block), ty)
                } else {
                    self.lower_to_expr_kind(child, expected_ty)?
                }
            }
            constants::ASSIGNMENT_EXPRESSION => {
                let lhs = self.lower_to_expr(node.child(0).unwrap())?;

                let bin_op = self.lower_to_bin_op(node.child(1).unwrap())?;

                let ty = lhs.ty.clone();

                (
                    match bin_op {
                        None => {
                            let rhs = self.lower_to_expr_with_expected_type(
                                node.child(2).unwrap(),
                                lhs.ty.clone(),
                            )?;
                            ExprKind::Assign(Box::new(lhs), Box::new(rhs))
                        }
                        Some(bin_op) => {
                            let rhs = self.lower_to_expr(node.child(2).unwrap())?;
                            let (kind, binop_ty) =
                                self.lower_bin_op(lhs, rhs, bin_op, span, true)?;
                            match kind {
                                ExprKind::Binary(bin_op, lhs, rhs) => ExprKind::AssignWithBinOp(
                                    lhs,
                                    rhs,
                                    bin_op,
                                    binop_ty,
                                    ReturnSemantic::AfterAssign,
                                ),
                                ExprKind::PtrOffset(lhs, rhs) => {
                                    ExprKind::AssignPtrOffset(lhs, rhs, ReturnSemantic::AfterAssign)
                                }
                                _ => {
                                    unreachable!();
                                }
                            }
                        }
                    },
                    ty,
                )
            }
            constants::FIELD_EXPRESSION => {
                let mut target =
                    self.lower_to_expr(node.child_by_field_name("argument").unwrap())?;

                let field = self.lower_to_ident(node.child_by_field_name("field").unwrap())?;

                if node.child(1).unwrap().kind() == "->" {
                    let (kind, ty) = self.lower_un_op(target, UnOp::Deref, span)?;
                    target = Expr { kind, ty, span };
                }

                let fields = target
                    .ty
                    .kind
                    .fields(&self.type_tag_resolver, span)
                    .context(span, "Failed to get fields of target expression.")?;

                let Some(field_data) = fields.by_name.get(&field.name) else {
                    bail!(
                        span,
                        "Unresolved field {}. Available fields are {:?}.",
                        field.name,
                        fields
                    );
                };
                let mut result = target;

                for &field_index in field_data {
                    let fields = result
                        .ty
                        .kind
                        .fields(&self.type_tag_resolver, span)
                        .context(span, "Failed to get fields of unnamed compound type.")?;
                    result = Expr {
                        span: result.span,
                        kind: ExprKind::Field(Box::new(result), field_index),
                        ty: fields.by_index[field_index].clone(),
                    };
                }

                (result.kind, result.ty)
            }
            constants::SUBSCRIPT_EXPRESSION => {
                let target = self.lower_to_expr(node.child(0).unwrap())?;

                let index = self.lower_to_expr(node.child(2).unwrap())?;

                let a_plus_i = {
                    let (kind, ty) = self.lower_bin_op(target, index, BinOp::Add, span, false)?;
                    Expr { kind, ty, span }
                };
                self.lower_un_op(a_plus_i, UnOp::Deref, span)?
            }
            constants::CAST_EXPRESSION | constants::COMPOUND_LITERAL => {
                let cast_node = node.child(1).unwrap();

                let decl_node = cast_node.child_by_field_name("declarator");

                let ty = self.lower_to_ty(cast_node, decl_node)?;

                let target = self.lower_to_expr(node.child(3).unwrap())?;

                (ExprKind::Cast(Box::new(target)), ty)
            }
            constants::INITIALIZER_LIST => {
                let list = self.lower_to_expr_or_list(node)?;
                let Some(expected_ty) = expected_ty else {
                    bail!(span, "Initializer lists should have expected type.");
                };
                let tree = self.lower_to_initializer_tree(&expected_ty.kind, list, span);

                let ty = Ty {
                    kind: TyKind::InitializerList,
                    is_linear: false,
                    quals: vec![],
                    span,
                };

                (ExprKind::InitializerList(Box::new(tree)), ty)
            }
            constants::COMMA_EXPRESSION => {
                let mut exprs = vec![];

                let mut cursor = node.walk();
                cursor.goto_first_child();

                loop {
                    exprs.push(self.lower_to_expr(cursor.node())?);

                    cursor.goto_next_sibling();
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }

                let ty = exprs.last().unwrap().ty.clone();

                (ExprKind::Comma(exprs), ty)
            }
            constants::CONDITIONAL_EXPRESSION => {
                let cond_expr = self.lower_to_cond_expr(node.child(0).unwrap())?;

                let mut body_expr = self.lower_to_expr(node.child(2).unwrap())?;
                let mut else_expr = self.lower_to_expr(node.child(4).unwrap())?;

                self.array_to_pointer_decay_if_array(&mut body_expr);
                self.array_to_pointer_decay_if_array(&mut else_expr);
                self.function_to_pointer_decay_if_function(&mut body_expr);
                self.function_to_pointer_decay_if_function(&mut else_expr);

                let ty = match (&body_expr.ty.kind, &else_expr.ty.kind) {
                    (TyKind::PrimTy(prim_l), TyKind::PrimTy(prim_r)) => {
                        TyKind::PrimTy(*prim_l.max(prim_r))
                    }
                    (TyKind::Struct(idx_l), TyKind::Struct(idx_r)) => {
                        if idx_l != idx_r {
                            bail!(span, "Incompatible structs in ternary.");
                        }
                        TyKind::Struct(*idx_l)
                    }
                    (TyKind::Union(idx_l), TyKind::Union(idx_r)) => {
                        if idx_l != idx_r {
                            bail!(span, "Incompatible unions in ternary.");
                        }
                        TyKind::Union(*idx_l)
                    }
                    (TyKind::Ptr { .. }, TyKind::Ptr { .. }) => body_expr.ty.kind.clone(),
                    // This is only allowed for 0 (null pointer constant) but we will do it
                    // for all ints. Who cares?
                    (ptr @ TyKind::Ptr { .. }, TyKind::PrimTy(_))
                    | (TyKind::PrimTy(_), ptr @ TyKind::Ptr { .. }) => ptr.clone(),
                    (TyKind::Array { .. }, TyKind::Array { .. }) => {
                        bail!(span, "Array is invalid in ternary.")
                    }
                    (TyKind::Func { .. }, TyKind::Func { .. }) => body_expr.ty.kind.clone(),
                    (TyKind::InitializerList, TyKind::InitializerList) => {
                        bail!(span, "Initializer list is invalid in ternary.")
                    }
                    _ => bail!(
                        span,
                        "Incompatible types in ternary {} vs {}.",
                        body_expr.ty,
                        else_expr.ty
                    ),
                };

                let ty = Ty {
                    kind: ty,
                    is_linear: false,
                    quals: vec![],
                    span,
                };

                (
                    ExprKind::Cond(
                        Box::new(cond_expr),
                        Box::new(Expr {
                            kind: ExprKind::Cast(Box::new(body_expr)),
                            ty: ty.clone(),
                            span,
                        }),
                        Box::new(Expr {
                            kind: ExprKind::Cast(Box::new(else_expr)),
                            ty: ty.clone(),
                            span,
                        }),
                    ),
                    ty,
                )
            }
            constants::SIZEOF_EXPRESSION => (
                ExprKind::Sizeof(self.lower_to_sizeof(node)?),
                Ty {
                    kind: TyKind::PrimTy(PrimTyKind::Int(8)),
                    is_linear: false,
                    quals: vec![],
                    span,
                },
            ),
            constants::VA_ARG_EXPRESSION => {
                let arg_ty_node = node.child_by_field_name("type").unwrap();
                let arg_ty =
                    self.lower_to_ty(arg_ty_node, arg_ty_node.child_by_field_name("declarator"))?;
                let va_list = self.lower_to_expr(node.child_by_field_name("value").unwrap())?;
                (ExprKind::VaArg(Box::new(va_list), arg_ty.clone()), arg_ty)
            }
            constants::SEMICOLON => (
                ExprKind::Empty,
                Ty {
                    kind: TyKind::PrimTy(PrimTyKind::Void),
                    is_linear: false,
                    quals: vec![],
                    span,
                },
            ),
            constants::CONCATENATED_STRING => {
                let mut result = "".to_owned();
                for node in node.children(&mut node.walk()) {
                    let LitKind::Str(part) = self.lower_to_lit_kind(node)? else {
                        bail!(span, "Invalid literal in concatenated string.");
                    };
                    result.push_str(&part);
                }
                (
                    ExprKind::Lit(Lit {
                        kind: LitKind::Str(result),
                        span,
                    }),
                    Ty {
                        kind: TyKind::Ptr {
                            kind: Box::new(TyKind::PrimTy(PrimTyKind::Char)),
                            quals: vec![],
                        },
                        is_linear: false,
                        quals: vec![],
                        span,
                    },
                )
            }
            kind if kind.contains(constants::LITERAL) => {
                let lit = self.lower_to_lit(node)?;
                let kind = match lit.kind {
                    LitKind::Str(_) => TyKind::Ptr {
                        kind: Box::new(TyKind::PrimTy(PrimTyKind::Char)),
                        quals: vec![],
                    },
                    LitKind::Char(_) => TyKind::PrimTy(PrimTyKind::Char),
                    LitKind::Int(_) => TyKind::PrimTy(PrimTyKind::Int(8)),
                    LitKind::Float(_) => TyKind::PrimTy(PrimTyKind::Float(8)),
                };
                (
                    ExprKind::Lit(lit),
                    Ty {
                        kind,
                        is_linear: false,
                        quals: vec![],
                        span,
                    },
                )
            }
            kind => bail!(span, "Cannot lower '{kind}' to 'ExprKind'."),
        })
    }

    fn lower_to_expr_or_list(&mut self, node: Node) -> azhdaha_errors::Result<ExprOrList> {
        Ok(match node.kind() {
            constants::INITIALIZER_LIST => {
                let mut elements = vec![];

                let mut cursor = node.walk();
                cursor.goto_first_child();
                cursor.goto_next_sibling();

                loop {
                    let node = cursor.node();
                    if node.kind() == "}" {
                        break;
                    }
                    elements.push(if node.kind() == constants::INITIALIZER_PAIR {
                        let value = node.child_by_field_name("value").unwrap();
                        let mut designators = vec![];
                        for designator in
                            node.children_by_field_name("designator", &mut node.walk())
                        {
                            designators.push(self.lower_to_designator(designator)?);
                        }
                        InitializerItem {
                            designators: Some(designators),
                            value: self.lower_to_expr_or_list(value)?,
                        }
                    } else {
                        InitializerItem {
                            designators: None,
                            value: self.lower_to_expr_or_list(node)?,
                        }
                    });

                    cursor.goto_next_sibling();
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }

                ExprOrList::List(elements)
            }
            _ => ExprOrList::Expr(self.lower_to_expr(node)?),
        })
    }

    pub(crate) fn lower_to_sizeof(&mut self, node: Node) -> azhdaha_errors::Result<Sizeof> {
        trace!("[HIR/SizeOf] Lowering '{}'", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        let kind = self.lower_to_sizeof_kind(node)?;

        Ok(Sizeof { kind, span })
    }

    fn lower_to_sizeof_kind(&mut self, node: Node) -> azhdaha_errors::Result<SizeofKind> {
        trace!("[HIR/SizeofKind] Lowering '{}'", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        let sizeof_kind = 'size_of: {
            if let Some(child) = node.child_by_field_name("value") {
                break 'size_of match self.lower_to_expr_or_type(child)? {
                    Either::Left(expr) => SizeofKind::Expr(Box::new(expr)),
                    Either::Right(ty) => SizeofKind::Ty(ty),
                };
            }

            if let Some(child) = node.child_by_field_name("type") {
                break 'size_of SizeofKind::Ty(
                    self.lower_to_ty(child, child.child_by_field_name("declarator"))?,
                );
            }

            bail!(span, "Cannot lower '{}' to 'SizeofKind'.", node.to_sexp());
        };

        Ok(sizeof_kind)
    }

    pub(crate) fn lower_to_lit(&self, node: Node) -> azhdaha_errors::Result<Lit> {
        trace!("[HIR/Lit] Lowering '{}'", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        Ok(Lit {
            kind: self.lower_to_lit_kind(node).with_context(span, || {
                format!(
                    "In lowering {:?} to literal",
                    node.utf8_text(self.source_code)
                )
            })?,
            span,
        })
    }

    fn lower_to_lit_kind(&self, node: Node) -> azhdaha_errors::Result<LitKind> {
        trace!("[HIR/LitKind] Lowering '{}'", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        Ok(match node.kind() {
            constants::STRING_LITERAL => {
                let text = &self.source_code[node.start_byte()..node.end_byte()];
                let Some((first_quote, _)) = text.iter().enumerate().find(|x| *x.1 == b'"') else {
                    bail!(span, "Could not found \" in string literal.");
                };
                let Some((last_quote, _)) = text.iter().enumerate().rev().find(|x| *x.1 == b'"')
                else {
                    bail!(span, "Could not found \" in string literal.");
                };
                let text = &text[first_quote + 1..last_quote];
                let Ok(text) = std::str::from_utf8(text) else {
                    bail!(span, "Invalid utf8 in string literal");
                };
                let Ok(text) = unescaper::unescape(text) else {
                    bail!(span, "Could not escape the string {text}");
                };
                LitKind::Str(text)
            }
            constants::CHAR_LITERAL => {
                let text = &self.source_code[node.start_byte()..node.end_byte()];
                let Some((first_quote, _)) = text.iter().enumerate().find(|x| *x.1 == b'\'') else {
                    bail!(span, "Could not found ' in char literal.");
                };
                let Some((last_quote, _)) = text.iter().enumerate().rev().find(|x| *x.1 == b'\'')
                else {
                    bail!(span, "Could not found ' in char literal.");
                };
                let text = &text[first_quote + 1..last_quote];
                let Ok(text) = std::str::from_utf8(text) else {
                    bail!(span, "Invalid utf8 in char literal");
                };
                let Ok(text) = unescaper::unescape(text) else {
                    bail!(span, "Could not escape the char {text}");
                };
                LitKind::Char(text.as_bytes()[0] as char)
            }
            constants::NUMBER_LITERAL => {
                let Ok(literal) =
                    std::str::from_utf8(&self.source_code[node.start_byte()..node.end_byte()])
                else {
                    bail!(span, "Invalid ut8 in number literal");
                };
                let literal = literal.to_lowercase();

                let literal = if let Some(literal) = literal.strip_suffix("llu") {
                    literal
                } else if let Some(literal) = literal.strip_suffix("lu") {
                    literal
                } else if let Some(literal) = literal.strip_suffix("u") {
                    literal
                } else if let Some(literal) = literal.strip_suffix("ll") {
                    literal
                } else if let Some(literal) = literal.strip_suffix("l") {
                    literal
                } else {
                    &literal
                };

                if let Some(stripped_literal) = literal.strip_prefix("0x") {
                    let Ok(int) = i128::from_str_radix(stripped_literal, 16) else {
                        bail!(span, "Invalid hex literal");
                    };
                    LitKind::Int(int)
                } else if let Some(stripped_literal) = literal.strip_prefix("0b") {
                    let Ok(int) = i128::from_str_radix(stripped_literal, 2) else {
                        bail!(span, "Invalid binary literal");
                    };
                    LitKind::Int(int)
                } else if let Some(stripped_literal) = literal.strip_prefix("0")
                    && !stripped_literal.starts_with(".")
                {
                    if stripped_literal.is_empty() {
                        LitKind::Int(0)
                    } else {
                        let Ok(int) = i128::from_str_radix(stripped_literal, 8) else {
                            bail!(span, "Invalid base 8 literal");
                        };
                        LitKind::Int(int)
                    }
                } else if let Ok(value) = literal.parse() {
                    LitKind::Int(value)
                } else {
                    let Ok(float) = literal.parse() else {
                        bail!(span, "Invalid float literal");
                    };
                    LitKind::Float(float)
                }
            }
            kind => bail!(span, "Cannot lower '{kind}' to 'Lit'."),
        })
    }

    // TODO: this function is garbage. Break it into two. One for assignments and one for normal operators.
    pub(crate) fn lower_to_bin_op(&self, node: Node) -> azhdaha_errors::Result<Option<BinOp>> {
        trace!("[HIR/BinOp] Lowering '{}'", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        Ok(Some(match node.kind() {
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
            constants::ASSIGN => return Ok(None),
            kind => bail!(span, "Cannot lower '{kind}' to 'BinOp'."),
        }))
    }

    fn lower_to_un_op(&mut self, node: Node) -> azhdaha_errors::Result<UnOp> {
        trace!("[HIR/UnOp] Lowering '{}'", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        Ok(match node.kind() {
            constants::NOT => UnOp::Not,
            constants::NEG => UnOp::Neg,
            constants::COM => UnOp::Com,
            constants::POS => UnOp::Pos,
            constants::ADDR_OF => UnOp::AddrOf,
            constants::DEREF => UnOp::Deref,
            kind => bail!(span, "Cannot lower '{kind}' to 'UnOp'."),
        })
    }

    fn lower_to_designator(&mut self, node: Node<'_>) -> azhdaha_errors::Result<Designator> {
        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        let kind = match node.kind() {
            constants::SUBSCRIPT_DESIGNATOR => DesignatorKind::Subscript {
                value: self.const_eval_enum_value(node.child(1).unwrap())? as i128,
            },
            constants::FIELD_DESIGNATOR => DesignatorKind::Field {
                name: self.lower_to_ident(node.child(1).unwrap())?.name,
            },
            kind => {
                bail!(span, "Cannot lower '{kind}' to 'Designator'")
            }
        };
        Ok(Designator { kind, span })
    }
}
