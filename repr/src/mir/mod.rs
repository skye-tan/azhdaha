//! The MIR - "Mid-level Intermediate Representation" is a radically simplified form constructed from HIR.
//! This representation is used for generating CFG - "Control Flow Graph" - of the source code.
//!

use std::collections::HashMap;

use anyhow::Context;
use la_arena::{Arena, RawIdx};

use crate::hir::{
    self, Span,
    resolver::{CompoundTypeData, Label, Resolver, Symbol, SymbolKind},
};

/// Contains methods needed to manage arenas and resolvers.
mod allocation;
/// Contains methods to minimize the mir, reducing the time of furthur analyzing.
mod optimization;

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
/// Contains datatypes used to represent the initializer lists.
mod initializer_tree;

pub use datatypes::*;
pub use initializer_tree::InitializerTree;

pub const RETURN_LOCAL: Local = Local::from_raw(RawIdx::from_u32(0));

#[derive(Debug, Clone)]
pub struct MirCtx<'mir> {
    pub label_resolver: &'mir Resolver<()>,

    pub body: Body<'mir>,

    pub bb_map: HashMap<Label, BasicBlock>,
    pub local_map: HashMap<Symbol, Local>,
}

impl<'mir> MirCtx<'mir> {
    pub fn new(
        symbol_resolver: &'mir Resolver<SymbolKind>,
        label_resolver: &'mir Resolver<()>,
        type_tag_resolver: &'mir Resolver<CompoundTypeData>,
        span: Span,
    ) -> Self {
        Self {
            label_resolver,

            body: Body {
                symbol_resolver,
                type_tag_resolver,
                local_decls: Arena::new(),
                basic_blocks: Arena::new(),
                span,
            },

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

        self.alloc_real_local(
            func_dec.storage.clone(),
            func_dec.sig.ret_ty.clone(),
            func_dec.ident.clone(),
            false,
            func_def.body.span,
        );

        for param in &func_dec.sig.params {
            if let Some(ident) = &param.ident {
                let symbol = *func_def
                    .arguments_symbols
                    .get(&ident.name)
                    .context(format!(
                        "Parameter {} have not been inserted into resolver.",
                        ident.name
                    ))?;

                let local = self.alloc_real_local(
                    param.storage.clone(),
                    param.ty.clone(),
                    ident.clone(),
                    true,
                    param.span,
                );

                self.local_map.insert(symbol, local);
            }
        }

        let mut bb = self.alloc_bb();
        self.lower_to_bb(&func_def.body, &mut bb);

        self.body.optimize();

        Ok(self.body)
    }

    /// # Panics
    /// Initializer of the [`hir::VarDecl`] should not be empty.
    pub fn lower_static_to_mir(mut self, decl: &'mir hir::VarDecl) -> anyhow::Result<Body<'mir>> {
        let ret = self.alloc_real_local(
            decl.storage.clone(),
            decl.ty.clone(),
            decl.ident.clone(),
            false,
            decl.span,
        );

        let ret = Place {
            local: ret,
            projections: vec![],
            span: decl.span,
        };

        let mut bb = self.alloc_bb();

        let init = decl
            .init
            .as_ref()
            .expect("Initializer should not be empty.");

        let rvalue = self.lower_to_rvalue(init, &mut bb, decl.span);
        self.retrieve_bb(bb).statements.push(Statement {
            kind: StatementKind::Assign(ret, rvalue),
            span: decl.span,
        });

        self.retrieve_bb(bb).terminator = Some(Terminator {
            kind: TerminatorKind::Return,
            span: decl.span,
        });

        Ok(self.body)
    }
}
