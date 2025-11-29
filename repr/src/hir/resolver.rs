#![allow(clippy::missing_docs_in_private_items)]

use std::{collections::HashMap, fmt::Debug};

use azhdaha_errors::bail;
use la_arena::{Arena, Idx};

use crate::hir::*;

pub type Label = Idx<()>;
pub type Symbol = Idx<SymbolKind>;

#[derive(Debug)]
pub enum SymbolKind {
    Var(VarDecl),
    Func(FuncDecl),
    Param(ParamDecl),
    TyDef(Ty),
    EnumVariant { value: i32, span: Span },
}

#[derive(Debug)]
pub struct FieldsData {
    pub by_index: Vec<Ty>,
    pub by_name: HashMap<String, Vec<usize>>,
}

#[derive(Debug)]
pub enum CompoundTypeData {
    Struct { fields: FieldsData },
    Union { fields: FieldsData },
    Enum,
    DeclaredOnly,
}

impl SymbolKind {
    pub(crate) fn ty(&self) -> azhdaha_errors::Result<Ty> {
        Ok(match self {
            SymbolKind::Var(var_decl) => var_decl.ty.clone(),
            SymbolKind::Func(func_decl) => Ty {
                kind: TyKind::Func {
                    sig: Box::new(func_decl.sig.clone()),
                },
                is_linear: false,
                quals: vec![],
                span: func_decl.span,
            },
            SymbolKind::Param(param_decl) => param_decl.ty.clone(),
            SymbolKind::TyDef(ty_def) => {
                bail!(ty_def.span, "Symbol is not a expression position symbol.")
            }
            &SymbolKind::EnumVariant { value: _, span } => Ty {
                kind: TyKind::PrimTy(PrimTyKind::Int(4)),
                is_linear: false,
                quals: vec![],
                span,
            },
        })
    }
}

#[derive(Debug)]
pub struct Resolver<T> {
    pub arena: Arena<T>,
    pub map: HashMap<String, Idx<T>>,
}

impl<T: Debug> Resolver<T> {
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
            map: HashMap::new(),
        }
    }

    pub fn insert_symbol(&mut self, name: String, data: T) -> Idx<T> {
        let res = self.arena.alloc(data);

        self.map.insert(name, res);

        res
    }

    /// Insert a symbol that is not resolvable by name (is only equal to self).
    pub fn insert_unnamed_symbol(&mut self, data: T) -> Idx<T> {
        self.arena.alloc(data)
    }

    pub fn get_res_by_name(&self, name: &str) -> Option<Idx<T>> {
        self.map.get(name).cloned()
    }

    #[track_caller]
    pub fn get_data_by_res(&self, res: &Idx<T>) -> &T {
        &self.arena[*res]
    }

    #[track_caller]
    pub fn get_data_by_res_mut(&mut self, res: &Idx<T>) -> &mut T {
        &mut self.arena[*res]
    }

    pub fn open_new_scope(&self) -> HashMap<String, Idx<T>> {
        self.map.clone()
    }

    pub fn restore_prev_scope(&mut self, saved_map: HashMap<String, Idx<T>>) {
        self.map = saved_map;
    }
}

impl<T: Debug> Default for Resolver<T> {
    fn default() -> Self {
        Self::new()
    }
}
