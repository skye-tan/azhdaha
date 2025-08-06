#![allow(clippy::missing_docs_in_private_items)]

use log::trace;

use repr::mir;

use crate::linear::{LinearAnalyzer, LinearLocal};

impl LinearAnalyzer<'_> {
    pub(crate) fn dfs_with_stack(
        &mut self,
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

            if let Err(error) = self.process_bb(body, &mut linear_local, bb_data) {
                trace!("{error:?}");
                return;
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
