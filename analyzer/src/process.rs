#![allow(clippy::missing_docs_in_private_items)]

use repr::mir;

use crate::{LinearCtx, LinearLocal};

impl LinearCtx<'_> {
    pub(crate) fn process_bb(
        &self,
        body: &mir::Body,
        linear_local: &mut LinearLocal,
        bb_data: &mir::BasicBlockData,
    ) -> anyhow::Result<bool> {
        for statement in &bb_data.statements {
            self.process_statement(body, linear_local, statement)?;
        }

        let Some(terminator) = &bb_data.terminator else {
            return Ok(false);
        };

        self.process_terminator(body, linear_local, terminator)
    }

    pub(crate) fn process_statement(
        &self,
        _body: &mir::Body,
        _linear_local: &mut LinearLocal,
        _statement: &mir::Statement,
    ) -> anyhow::Result<bool> {
        todo!()
    }

    pub(crate) fn process_terminator(
        &self,
        _body: &mir::Body,
        _linear_local: &mut LinearLocal,
        _terminator: &mir::Terminator,
    ) -> anyhow::Result<bool> {
        todo!()
    }
}
