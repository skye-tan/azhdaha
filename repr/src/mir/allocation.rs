#![allow(clippy::missing_docs_in_private_items)]

use crate::{
    hir::{Ident, Span, Storage, Ty},
    mir::{MirCtx, datatypes::*},
};

impl MirCtx<'_> {
    pub(crate) fn alloc_bb(&mut self) -> BasicBlock {
        self.body
            .basic_blocks
            .alloc(BasicBlockData::default())
            .into()
    }

    pub(crate) fn retrieve_bb(&mut self, bb: BasicBlock) -> &mut BasicBlockData {
        &mut self.body.basic_blocks[bb.into_inner()]
    }

    pub(crate) fn alloc_real_local(
        &mut self,
        storage: Option<Storage>,
        ty: Ty,
        ident: Ident,
        is_arg: bool,
        span: Span,
    ) -> Local {
        self.body.local_decls.alloc(LocalDecl {
            kind: LocalKind::Real {
                storage,
                ty,
                ident,
                is_arg,
            },
            span,
        })
    }

    pub(crate) fn alloc_temp_local(&mut self, span: Span) -> Local {
        self.body.local_decls.alloc(LocalDecl {
            kind: LocalKind::Temp,
            span,
        })
    }
}
