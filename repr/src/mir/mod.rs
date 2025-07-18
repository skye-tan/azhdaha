//! The MIR - "Mid-level Intermediate Representation" is a radically simplified form constructed from HIR.
//! This representation is used for generating CFG - "Control Flow Graph" - of the source code.
//!
//! This implementation has been modeled after rustc's MIR representation.
//!

use std::collections::HashMap;

use anyhow::Context;
use la_arena::Arena;

use crate::hir::{
    self, Span,
    resolver::{Resolver, SymbolKind},
};

/// Contains the methods needed to manage arenas and resolvers.
mod arena;
/// Contains the methods needed to lower HIR to MIR's [`BasicBlock`].
mod basic_block;
/// Contains the methods needed to lower HIR to MIR's [`Rvalue`], [`Operand`], and [`Place`].
mod operand;

/// Contains datatypes used to represent the MIR.
mod datatypes;

pub use datatypes::*;

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
                basic_blocks: Arena::new(),
                local_decls: Arena::new(),
                span,
            },
            local_map: HashMap::new(),
            bb_map: HashMap::new(),
        }
    }

    pub fn lower_to_mir(mut self, func: &'mir hir::Func) -> anyhow::Result<Body<'mir>> {
        let func_sig = match self.body.symbol_resolver.get_data_by_res(&func.sig) {
            SymbolKind::Func(func_sig) => func_sig,
            SymbolKind::Local(..) => unreachable!(),
        };

        self.alloc_local(None, &func_sig.ret_ty, func.body.span);

        for param in &func_sig.params {
            if let Some(ident) = &param.ident {
                let symbol = self
                    .body
                    .symbol_resolver
                    .get_res_by_name(&ident.name)
                    .context(format!(
                        "Parameter {} have not been inserted into resolver.",
                        ident.name
                    ))?;

                let local = self.alloc_local(Some(ident.name.clone()), &param.ty, param.span);

                self.local_map.insert(symbol, local);
            }
        }

        let bb = self.alloc_bb();
        self.lower_to_bb(&func.body, bb);

        Ok(self.body)
    }
}
