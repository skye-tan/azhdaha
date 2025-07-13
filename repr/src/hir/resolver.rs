use std::collections::HashMap;

use anyhow::bail;
use la_arena::{Arena, Idx};

use crate::hir::{Ident, Param, Ty};

pub type ResIdx = Idx<ResData>;

#[derive(Debug, Clone)]
pub enum ResKind {
    Fn(Ty, Vec<Param>),
    Union,
    Struct,
    Local(Ty),
}

#[derive(Debug, Clone)]
pub struct ResData {
    pub ident: Ident,
    pub kind: ResKind,
}

#[derive(Debug, Clone)]
pub struct Resolver {
    pub arena: Arena<ResData>,
    pub map: HashMap<String, Idx<ResData>>,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, ident: Ident, kind: ResKind) -> anyhow::Result<ResIdx> {
        let key: String = ident.name.clone();
        let data = ResData { ident, kind };

        let idx = self.arena.alloc(data);

        if self.map.insert(key, idx).is_some() {
            bail!("Variable shadowing is not sepported.");
        }

        Ok(idx)
    }

    pub fn lookup_idx(&self, key: &str) -> Option<ResIdx> {
        self.map.get(key).cloned()
    }

    pub fn get_item(&self, res: &ResIdx) -> &ResData {
        &self.arena[*res]
    }
}

impl Default for Resolver {
    fn default() -> Self {
        Self::new()
    }
}
