use std::collections::HashMap;

use anyhow::bail;
use la_arena::{Arena, Idx};

use crate::hir::{FnSig, Ty};

pub type ResolverIdx = Idx<ResolverData>;

#[derive(Debug, Clone)]
pub enum ResolverData {
    Fn(FnSig),
    Union,
    Struct,
    Local(Ty),
}

#[derive(Debug, Clone)]
pub struct Resolver {
    pub arena: Arena<ResolverData>,
    pub map: HashMap<String, Idx<ResolverData>>,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, item: ResolverData) -> anyhow::Result<()> {
        let idx = self.arena.alloc(item);

        if self.map.insert(key, idx).is_some() {
            bail!("Variable shadowing is not sepported.");
        }

        Ok(())
    }

    pub fn lookup_idx(&self, key: &str) -> Option<Idx<ResolverData>> {
        self.map.get(key).cloned()
    }

    pub fn get_item(&self, idx: Idx<ResolverData>) -> &ResolverData {
        &self.arena[idx]
    }
}

impl Default for Resolver {
    fn default() -> Self {
        Self::new()
    }
}
