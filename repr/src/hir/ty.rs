#![allow(clippy::missing_docs_in_private_items)]

use anyhow::bail;
use log::trace;

use crate::hir::*;

use super::constants;

#[derive(Debug, Clone)]
pub struct Ty {
    pub kind: TyKind,
    pub is_linear: bool,
    pub quals: Vec<TyQual>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum TyKind {
    PrimTy(PrimTyKind),
    Ptr {
        kind: Box<TyKind>,
        quals: Vec<TyQual>,
    },
    Array {
        kind: Box<TyKind>,
        size: Option<Box<Expr>>,
    },
    Func {
        sig: Box<FuncSig>,
    },
}

#[derive(Debug, Clone)]
pub enum PrimTyKind {
    Bool,
    Char,
    Int,
    Float,
    Double,
    Void,
}

#[derive(Debug, Clone)]
pub enum TyQual {
    Const,
    ConstExpr,
    Volatile,
    Restrict,
    Atomic,
    NoReturn,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Storage {
    Extern,
    Static,
    Auto,
    Register,
    Inline,
    ThreadLocal,
}

impl HirCtx<'_> {
    pub(crate) fn lower_to_ty(&mut self, node: Node) -> anyhow::Result<Ty> {
        trace!("[HIR/Ty] Lowering '{}'", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        let mut is_linear = false;
        let mut quals = vec![];

        let mut cursor = node.walk();

        for child in node.children(&mut cursor) {
            match child.kind() {
                constants::TYPE_QUALIFIER => {
                    quals.push(self.lower_to_ty_qual(child.child(0).unwrap())?)
                }
                constants::LINEAR_TY_SPECIFIER => is_linear = true,
                _ => (),
            }
        }

        let mut kind = self.lower_to_ty_kind(node)?;

        if node.kind() == constants::FUNCTION_DEFINITION {
            return Ok(Ty {
                kind,
                is_linear,
                quals,
                span,
            });
        }

        let mut decl_node = node;

        while let Some(node) = decl_node.child_by_field_name("declarator") {
            decl_node = node;

            if decl_node.kind() == constants::FUNCTION_DECLARATOR {
                break;
            }
        }

        if decl_node.kind() != constants::FUNCTION_DECLARATOR {
            return Ok(Ty {
                kind,
                is_linear,
                quals,
                span,
            });
        }

        let func_sig = self.lower_to_func_sig(
            decl_node,
            Ty {
                kind,
                is_linear,
                quals,
                span,
            },
        )?;

        kind = TyKind::Func {
            sig: Box::new(func_sig),
        };

        is_linear = false;
        quals = vec![];

        while let Some(node) = decl_node.child_by_field_name("declarator") {
            decl_node = node;

            match decl_node.kind() {
                constants::POINTER_DECLARATOR | constants::ABSTRACT_POINTER_DECLARATOR => {
                    let mut quals = vec![];

                    let mut cursor = decl_node.walk();

                    for child in decl_node.children(&mut cursor) {
                        if child.kind() == constants::TYPE_QUALIFIER {
                            quals.push(self.lower_to_ty_qual(child.child(0).unwrap())?);
                        }
                    }

                    kind = TyKind::Ptr {
                        kind: Box::new(kind),
                        quals,
                    }
                }
                constants::ARRAY_DECLARATOR | constants::ABSTRACT_ARRAY_DECLARATOR => {
                    let size = if decl_node.child_count() == 4 {
                        Some(Box::new(self.lower_to_expr(decl_node.child(2).unwrap())?))
                    } else {
                        None
                    };

                    kind = TyKind::Array {
                        kind: Box::new(kind),
                        size,
                    }
                }
                constants::PARENTHESIZED_DECLARATOR => continue,
                _ => break,
            }
        }

        Ok(Ty {
            kind,
            is_linear,
            quals,
            span,
        })
    }

    fn lower_to_ty_kind(&mut self, node: Node) -> anyhow::Result<TyKind> {
        trace!("[HIR/TyKind] Lowering '{}'", node.kind());

        let mut ty_node = node.child_by_field_name("type").unwrap();

        while let Some(child) = ty_node.child_by_field_name("type") {
            ty_node = child;
        }

        let mut ty_kind = match ty_node.kind() {
            constants::PRIMITIVE_TYPE => TyKind::PrimTy(self.lower_to_prim_ty_kind(ty_node)?),
            constants::TYPE_DESCRIPTOR => self.lower_to_ty_kind(ty_node.child(0).unwrap())?,
            constants::TYPE_IDENTIFIER => todo!(),
            kind => bail!("Cannot lower '{kind}' to 'TyKind'."),
        };

        let mut decl_node = node;

        while let Some(node) = decl_node.child_by_field_name("declarator") {
            decl_node = node;

            match decl_node.kind() {
                constants::POINTER_DECLARATOR | constants::ABSTRACT_POINTER_DECLARATOR => {
                    let mut quals = vec![];

                    let mut cursor = decl_node.walk();

                    for child in decl_node.children(&mut cursor) {
                        if child.kind() == constants::TYPE_QUALIFIER {
                            quals.push(self.lower_to_ty_qual(child.child(0).unwrap())?);
                        }
                    }

                    ty_kind = TyKind::Ptr {
                        kind: Box::new(ty_kind),
                        quals,
                    }
                }
                constants::ARRAY_DECLARATOR | constants::ABSTRACT_ARRAY_DECLARATOR => {
                    let size = if decl_node.child_count() == 4 {
                        Some(Box::new(self.lower_to_expr(decl_node.child(2).unwrap())?))
                    } else {
                        None
                    };

                    ty_kind = TyKind::Array {
                        kind: Box::new(ty_kind),
                        size,
                    }
                }
                constants::INIT_DECLARATOR => continue,
                constants::FUNCTION_DECLARATOR
                | constants::PARAMETER_DECLARATION
                | constants::IDENTIFIER => {
                    break;
                }
                kind => bail!("Cannot lower '{kind}' to 'TyKind'."),
            }
        }

        Ok(ty_kind)
    }

    fn lower_to_prim_ty_kind(&mut self, node: Node) -> anyhow::Result<PrimTyKind> {
        trace!("[HIR/PrimTyKind] Lowering '{}'", node.kind());

        Ok(
            match std::str::from_utf8(&self.source_code[node.start_byte()..node.end_byte()])? {
                constants::BOOL => PrimTyKind::Bool,
                constants::CHAR => PrimTyKind::Char,
                constants::INT => PrimTyKind::Int,
                constants::FLOAT => PrimTyKind::Float,
                constants::DOUBLE => PrimTyKind::Double,
                constants::VOID => PrimTyKind::Void,
                kind => bail!("Cannot lower '{kind}' to 'PrimTyKind'."),
            },
        )
    }

    pub(crate) fn lower_to_storage(&mut self, node: Node) -> anyhow::Result<Storage> {
        trace!("[HIR/Storage] Lowering '{}'", node.kind());

        Ok(match node.kind() {
            constants::EXTERN => Storage::Extern,
            constants::STATIC => Storage::Static,
            constants::AUTO => Storage::Auto,
            constants::REGISTER => Storage::Register,
            constants::INLINE => Storage::Inline,
            constants::THREAD_LOCAL => Storage::ThreadLocal,
            kind => bail!("Cannot lower '{kind}' to 'Storage'."),
        })
    }

    fn lower_to_ty_qual(&mut self, node: Node) -> anyhow::Result<TyQual> {
        trace!("[HIR/TyQual] Lowering '{}'", node.kind());

        Ok(match node.kind() {
            constants::CONST => TyQual::Const,
            constants::CONSTEXPR => TyQual::ConstExpr,
            constants::VOLATILE => TyQual::Volatile,
            constants::RESTRICT => TyQual::Restrict,
            constants::ATOMIC => TyQual::Atomic,
            constants::NORETURN => TyQual::NoReturn,
            kind => bail!("Cannot lower '{kind}' to 'TyQual'."),
        })
    }
}
