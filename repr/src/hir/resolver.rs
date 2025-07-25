#![allow(clippy::missing_docs_in_private_items)]

use std::collections::HashMap;

use la_arena::{Arena, Idx};

use crate::hir::*;

pub type Label = Idx<()>;
pub type Symbol = Idx<SymbolKind>;

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Local(LocalDecl),
    Func(FuncDecl),
    Param(ParamDecl),
}

#[derive(Debug, Clone)]
pub struct Resolver<T> {
    pub arena: Arena<T>,
    pub map: HashMap<String, Idx<T>>,
}

impl<T> Resolver<T> {
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

    pub fn get_res_by_name(&self, name: &str) -> Option<Idx<T>> {
        self.map.get(name).cloned()
    }

    pub fn get_data_by_res(&self, res: &Idx<T>) -> &T {
        &self.arena[*res]
    }
}

impl<T> Default for Resolver<T> {
    fn default() -> Self {
        Self::new()
    }
}
