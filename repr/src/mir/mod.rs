//! The MIR - "Mid-level Intermediate Representation" is a radically simplified form constructed from HIR.
//! This representation is used for generating CFG - "Control Flow Graph" - of the source code.
//!
//! This implementation has been modeled after rustc's MIR representation.
//!

use std::collections::HashMap;

use la_arena::Arena;

use crate::hir::{
    self, Span,
    resolver::{ResData, Resolver},
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
        resolver: &'mir Resolver<ResData>,
        label_resolver: &'mir Resolver<()>,
        span: Span,
    ) -> Self {
        Self {
            body: Body {
                basic_blocks: Arena::new(),
                local_decls: Arena::new(),
                resolver,
                label_resolver,
                span,
            },
            local_map: HashMap::new(),
            bb_map: HashMap::new(),
        }
    }

    pub fn lower_to_mir(mut self, item: &'mir hir::Fn) -> anyhow::Result<Body<'mir>> {
        self.alloc_local(None, &item.sig.ty, item.body.span);

        for param in &item.sig.params {
            if let Some(res) = &param.res {
                let res_data = item.resolver.get_item(res);

                let local =
                    self.alloc_local(Some(res_data.ident.name.clone()), &param.ty, param.span);

                self.local_map.insert(*res, local);
            }
        }

        let bb = self.alloc_bb();
        self.lower_to_bb(&item.body, bb);

        Ok(self.body)
    }
}
