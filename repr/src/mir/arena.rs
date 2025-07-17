#![allow(clippy::missing_docs_in_private_items)]

use crate::{
    hir::{Span, Ty},
    mir::datatypes::*,
};

impl<'mir> MirCtx<'mir> {
    pub(crate) fn alloc_bb(&mut self) -> BasicBlock {
        self.body.basic_blocks.alloc(BasicBlockData::default())
    }

    pub(crate) fn retrive_bb(&mut self, bb: BasicBlock) -> &mut BasicBlockData {
        &mut self.body.basic_blocks[bb]
    }

    pub(crate) fn alloc_local(
        &mut self,
        debug_ident: Option<String>,
        ty: &Ty,
        span: Span,
    ) -> Local {
        self.body.local_decls.alloc(LocalDecl {
            debug_ident,
            ty: ty.clone(),
            span,
        })
    }
}
