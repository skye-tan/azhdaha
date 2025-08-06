#![allow(clippy::missing_docs_in_private_items)]

use log::trace;

use repr::mir;

use crate::{LinearCtx, LinearLocal};

impl LinearCtx<'_> {
    pub(crate) fn dfs_with_stack(
        &self,
        body: &mir::Body,
        mut linear_local: LinearLocal,
        bb: mir::BasicBlock,
    ) {
        let mut visited = vec![false; body.basic_blocks.len()];
        let mut bb_stack = vec![bb];

        while let Some(bb) = bb_stack.pop() {
            let index = bb.into_raw().into_u32() as usize;

            if visited[index] {
                continue;
            }

            visited[index] = true;

            let bb_data = &body.basic_blocks[bb];

            match self.process_bb(body, &mut linear_local, bb_data) {
                Ok(altered_linear_state) => {
                    if altered_linear_state {
                        continue;
                    }
                }
                Err(error) => {
                    trace!("{error:?}");
                    return;
                }
            }

            let Some(terminator) = &bb_data.terminator else {
                continue;
            };

            match &terminator.kind {
                mir::TerminatorKind::Goto { bb } => bb_stack.push(*bb),
                mir::TerminatorKind::SwitchInt { targets, .. } => {
                    for bb in &targets.bbs {
                        bb_stack.push(*bb);
                    }
                }
                mir::TerminatorKind::Return => continue,
            }
        }
    }
}
