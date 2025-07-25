#![allow(clippy::missing_docs_in_private_items)]

use crate::{
    hir::{Span, Ty},
    mir::{MirCtx, datatypes::*},
};

impl MirCtx<'_> {
    pub(crate) fn alloc_bb(&mut self) -> BasicBlock {
        self.body.basic_blocks.alloc(BasicBlockData::default())
    }

    pub(crate) fn retrieve_bb(&mut self, bb: BasicBlock) -> &mut BasicBlockData {
        &mut self.body.basic_blocks[bb]
    }

    pub(crate) fn alloc_local(&mut self, debug_name: Option<String>, ty: &Ty, span: Span) -> Local {
        self.body.local_decls.alloc(LocalDecl {
            debug_name,
            ty: ty.clone(),
            span,
        })
    }
}
