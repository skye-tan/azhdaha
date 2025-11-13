#![allow(clippy::missing_docs_in_private_items)]

use std::fmt::Display;

use anyhow::{Context, bail};
use la_arena::Idx;
use log::trace;

use crate::hir::{
    resolver::{CompoundTypeData, SymbolKind},
    *,
};

use super::constants;

#[derive(Debug, Clone)]
pub struct Ty {
    pub kind: TyKind,
    pub is_linear: bool,
    pub quals: Vec<TyQual>,
    pub span: Span,
}

impl Display for Ty {
    fn fmt(&self, fm: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fm, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum TyKind {
    PrimTy(PrimTyKind),
    Struct(Idx<CompoundTypeData>),
    Union(Idx<CompoundTypeData>),
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

impl TyKind {
    pub fn is_ptr(&self) -> bool {
        matches!(self, TyKind::Ptr { .. })
    }

    pub fn is_array(&self) -> bool {
        matches!(self, TyKind::Array { .. })
    }

    pub fn is_fn(&self) -> bool {
        matches!(self, TyKind::Func { .. })
    }
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
    pub(crate) fn lower_to_ty(
        &mut self,
        node: Node,
        decl_node: Option<Node>,
    ) -> anyhow::Result<Ty> {
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

        let mut kind = self.lower_to_ty_kind(node, decl_node)?;

        let Some(mut decl_node) = decl_node else {
            return Ok(Ty {
                kind,
                is_linear,
                quals,
                span,
            });
        };

        if node.kind() == constants::FUNCTION_DEFINITION {
            return Ok(Ty {
                kind,
                is_linear,
                quals,
                span,
            });
        }

        while decl_node.kind() != constants::FUNCTION_DECLARATOR
            && let Some(node) = decl_node.child_by_field_name("declarator")
        {
            decl_node = node;
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

    pub(crate) fn lower_to_ty_kind(
        &mut self,
        node: Node,
        decl_node: Option<Node>,
    ) -> anyhow::Result<TyKind> {
        trace!("[HIR/TyKind] Lowering '{}'", node.kind());

        let mut ty_node = node.child_by_field_name("type").unwrap();

        while let Some(child) = ty_node.child_by_field_name("type") {
            ty_node = child;
        }

        let mut ty_kind = match ty_node.kind() {
            constants::TYPE_DESCRIPTOR => {
                self.lower_to_ty_kind(ty_node.child(0).unwrap(), decl_node)?
            }
            constants::TYPE_IDENTIFIER => {
                let ident = self.lower_to_ident(ty_node)?;

                let symbol = self
                    .symbol_resolver
                    .get_res_by_name(&ident.name)
                    .context(format!("Use of undefined identifier '{}'.", &ident.name))?;

                let symbol_kind = self.symbol_resolver.get_data_by_res(&symbol);

                match symbol_kind {
                    SymbolKind::TyDef(ty) => ty.kind.clone(),
                    _ => bail!("Use of invalid type identifier '{}'.", &ident.name),
                }
            }
            constants::PRIMITIVE_TYPE => TyKind::PrimTy(self.lower_to_prim_ty_kind(ty_node)?),
            constants::SIZED_TYPE_SPECIFIER => TyKind::PrimTy(PrimTyKind::Int),
            constants::UNION_SPECIFIER
            | constants::STRUCT_SPECIFIER
            | constants::ENUM_SPECIFIER => {
                let idx = self.lower_struct_or_union_or_enum(ty_node)?;
                match ty_node.kind() {
                    constants::STRUCT_SPECIFIER => TyKind::Struct(idx),
                    constants::UNION_SPECIFIER => TyKind::Union(idx),
                    constants::ENUM_SPECIFIER => TyKind::PrimTy(PrimTyKind::Int),
                    _ => unreachable!(),
                }
            }
            kind => bail!("Cannot lower '{kind}' to 'TyKind'."),
        };

        let Some(mut decl_node) = decl_node else {
            return Ok(ty_kind);
        };

        loop {
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
                constants::FUNCTION_DECLARATOR
                | constants::PARAMETER_DECLARATION
                | constants::FIELD_IDENTIFIER
                | constants::TYPE_IDENTIFIER
                | constants::IDENTIFIER => {
                    break;
                }
                constants::INIT_DECLARATOR => (),
                kind => bail!("Cannot lower '{kind}' to 'TyKind' <todo: wrong message>."),
            }

            match decl_node.child_by_field_name("declarator") {
                Some(node) => decl_node = node,
                None => break,
            }
        }

        Ok(ty_kind)
    }

    pub(crate) fn lower_enum(&mut self, node: Node<'_>) -> anyhow::Result<Idx<CompoundTypeData>> {
        let idx = if let Some(name) = node.child_by_field_name("name") {
            let ident = self.lower_to_ident(name)?;
            match self.type_tag_resolver.get_res_by_name(&ident.name) {
                Some(idx) => idx,
                None => self
                    .type_tag_resolver
                    .insert_symbol(ident.name.clone(), CompoundTypeData::Enum),
            }
        } else {
            self.type_tag_resolver
                .insert_unnamed_symbol(CompoundTypeData::Enum)
        };
        if let Some(body) = node.child_by_field_name("body") {
            let mut value = 0;
            for child in body.children(&mut body.walk()) {
                if child.kind() == "{" || child.kind() == "}" || child.kind() == "," {
                    continue;
                }
                let ident = self.lower_to_ident(child.child_by_field_name("name").unwrap())?;
                if let Some(value_node) = child.child_by_field_name("value") {
                    value = self.const_eval_enum_value(value_node)?;
                }
                self.symbol_resolver.insert_symbol(
                    ident.name.clone(),
                    SymbolKind::EnumVariant {
                        value,
                        span: ident.span,
                    },
                );
                value += 1;
            }
        }
        Ok(idx)
    }

    fn const_eval_enum_value(&self, node: Node<'_>) -> anyhow::Result<i32> {
        match node.kind() {
            constants::NUMBER_LITERAL => {
                let lit = self.lower_to_lit(node)?;
                let LitKind::Int(value) = lit.kind else {
                    bail!("Invalid literal {lit:?} for enum value.");
                };
                Ok(value as i32)
            }
            kind => bail!("Cannot const eval node of type '{kind}'"),
        }
    }

    pub(crate) fn lower_struct_or_union_or_enum(
        &mut self,
        ty_node: Node<'_>,
    ) -> anyhow::Result<Idx<CompoundTypeData>> {
        if ty_node.kind() == constants::ENUM_SPECIFIER {
            return self.lower_enum(ty_node);
        }
        let (mut idx, ident) = if let Some(name) = ty_node.child_by_field_name("name") {
            let ident = self.lower_to_ident(name)?;
            let idx = match self.type_tag_resolver.get_res_by_name(&ident.name) {
                Some(idx) => idx,
                None => self
                    .type_tag_resolver
                    .insert_symbol(ident.name.clone(), CompoundTypeData::DeclaredOnly),
            };
            (idx, Some(ident))
        } else {
            (
                self.type_tag_resolver
                    .insert_unnamed_symbol(CompoundTypeData::DeclaredOnly),
                None,
            )
        };
        let data = if let Some(body) = ty_node.child_by_field_name("body") {
            let fields = self.lower_fields_in_specifier(body);
            Some(match ty_node.kind() {
                constants::STRUCT_SPECIFIER => CompoundTypeData::Struct { fields },
                constants::UNION_SPECIFIER => CompoundTypeData::Union { fields },
                _ => unreachable!(),
            })
        } else {
            None
        };

        if let Some(data) = data {
            let value = self.type_tag_resolver.get_data_by_res_mut(&idx);
            if !matches!(*value, CompoundTypeData::DeclaredOnly) {
                idx = self
                    .type_tag_resolver
                    .insert_symbol(ident.unwrap().name, data);
            } else {
                *value = data;
            }
        }

        Ok(idx)
    }

    fn lower_to_prim_ty_kind(&mut self, node: Node) -> anyhow::Result<PrimTyKind> {
        trace!("[HIR/PrimTyKind] Lowering '{}'", node.kind());

        Ok(
            match std::str::from_utf8(&self.source_code[node.start_byte()..node.end_byte()])? {
                kind if kind.contains(constants::INT) => PrimTyKind::Int,
                constants::SIZE => PrimTyKind::Int,
                constants::BOOL => PrimTyKind::Bool,
                constants::CHAR => PrimTyKind::Char,
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
