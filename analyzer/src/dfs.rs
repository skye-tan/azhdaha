#![allow(clippy::missing_docs_in_private_items)]

use log::error;

use repr::mir;

use crate::linear::{LinearAnalyzer, LinearLocal};

impl LinearAnalyzer<'_> {
    pub(crate) fn dfs_with_stack(
        &mut self,
        body: &mir::Body,
        mut linear_local: LinearLocal,
        bb: mir::BasicBlock,
    ) -> bool {
        let mut visited = vec![false; body.basic_blocks.len()];
        let mut bb_stack = vec![bb];

        while let Some(bb) = bb_stack.pop() {
            let index = bb.get_id();

            if visited[index] {
                continue;
            }

            visited[index] = true;

            let bb_data = &body.basic_blocks[bb.into_inner()];

            match self.process_bb(body, &mut linear_local, bb_data) {
                Ok(should_be_reported) => {
                    if should_be_reported {
                        return true;
                    }
                }
                Err(error) => error!("Failed to finish linear analyzing - {error:?}"),
            }

            let Some(terminator) = &bb_data.terminator else {
                continue;
            };

            match &terminator.kind {
                mir::TerminatorKind::Goto { bb } => bb_stack.push(*bb),
                mir::TerminatorKind::SwitchInt { targets, .. } => {
                    bb_stack.push(targets[0]);
                    bb_stack.push(targets[1]);
                }
                mir::TerminatorKind::Return => continue,
            }
        }

        false
    }
}
