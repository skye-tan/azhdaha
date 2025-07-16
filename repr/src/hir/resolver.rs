#![allow(clippy::missing_docs_in_private_items)]

use std::collections::HashMap;

use la_arena::{Arena, Idx};

use crate::hir::{Ident, Param, Ty};

pub type ResIdx<T> = Idx<T>;
pub type LabelIdx = Idx<String>;

#[derive(Debug, Clone)]
pub enum ResKind {
    Fn(Ty, Vec<Param>),
    Var(Ty),
}

#[derive(Debug, Clone)]
pub struct ResData {
    pub ident: Ident,
    pub kind: ResKind,
}

#[derive(Debug, Clone)]
pub struct Resolver<T> {
    pub arena: Arena<T>,
    pub map: HashMap<String, ResIdx<T>>,
}

impl<T> Resolver<T> {
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, data: T) -> anyhow::Result<ResIdx<T>> {
        let res = self.arena.alloc(data);

        self.map.insert(key, res);

        Ok(res)
    }

    pub fn lookup_res(&self, key: &str) -> Option<ResIdx<T>> {
        self.map.get(key).cloned()
    }

    pub fn get_item(&self, res: &ResIdx<T>) -> &T {
        &self.arena[*res]
    }
}

impl<T> Default for Resolver<T> {
    fn default() -> Self {
        Self::new()
    }
}
