//! The MIR - "Mid-level Intermediate Representation" is a radically simplified form constructed from HIR.
//! This representation is used for generating CFG - "Control Flow Graph" - of the source code.
//!

use std::collections::HashMap;

use anyhow::Context;
use la_arena::{Arena, RawIdx};

use crate::hir::{
    self, Span,
    resolver::{Label, Resolver, Symbol, SymbolKind},
};

/// Contains methods needed to manage arenas and resolvers.
mod arena;
/// Contains methods needed to lower HIR to MIR's [`BasicBlock`].
mod basic_block;
/// Contains methods needed to lower HIR to MIR's [`Operand`].
mod operand;
/// Contains methods needed to lower HIR to MIR's [`Place`].
mod place;
/// Contains methods needed to lower HIR to MIR's [`Rvalue`].
mod rvalue;

/// Contains datatypes used to represent the MIR.
mod datatypes;

pub use datatypes::*;

pub const RETURN_LOCAL: Local = Local::from_raw(RawIdx::from_u32(0));

#[derive(Debug, Clone)]
pub struct MirCtx<'mir> {
    pub label_resolver: &'mir Resolver<()>,

    pub body: Body<'mir>,
    pub has_inner_symbol_resolver: bool,

    pub bb_map: HashMap<Label, BasicBlock>,
    pub local_map: HashMap<Symbol, Local>,
}

impl<'mir> MirCtx<'mir> {
    pub fn new(
        symbol_resolver: &'mir Resolver<SymbolKind>,
        label_resolver: &'mir Resolver<()>,
        span: Span,
    ) -> Self {
        Self {
            label_resolver,

            body: Body {
                symbol_resolver,
                local_decls: Arena::new(),
                basic_blocks: Arena::new(),
                span,
            },
            has_inner_symbol_resolver: false,

            local_map: HashMap::new(),
            bb_map: HashMap::new(),
        }
    }

    pub fn lower_to_mir(mut self, func_def: &'mir hir::FuncDef) -> anyhow::Result<Body<'mir>> {
        let symbol_kind = self.body.symbol_resolver.get_data_by_res(&func_def.symbol);

        let func_dec = match symbol_kind {
            SymbolKind::Func(func_sig) => func_sig,
            _ => unreachable!(),
        };

        self.alloc_local(
            None,
            func_dec.storage.clone(),
            &func_dec.sig.ret_ty,
            func_def.body.span,
        );

        for param in &func_dec.sig.params {
            if let Some(ident) = &param.ident {
                let symbol = self
                    .body
                    .symbol_resolver
                    .get_res_by_name(&ident.name)
                    .context(format!(
                        "Parameter {} have not been inserted into resolver.",
                        ident.name
                    ))?;

                let local = self.alloc_local(
                    Some(ident.name.clone()),
                    param.storage.clone(),
                    &param.ty,
                    param.span,
                );

                self.local_map.insert(symbol, local);
            }
        }

        let mut bb = self.alloc_bb();
        self.lower_to_bb(&func_def.body, &mut bb);

        Ok(self.body)
    }
}
