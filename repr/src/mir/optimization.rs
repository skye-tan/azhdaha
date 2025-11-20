use std::collections::{HashMap, hash_map};

use crate::mir::{BasicBlock, Body, TerminatorKind};

impl Body<'_> {
    pub(crate) fn optimize(&mut self) {
        self.flat_unneeded_gotos();
        self.remove_unneeded_basic_blocks();
    }

    fn flat_unneeded_gotos(&mut self) {
        let mut bb_map = HashMap::new();
        for (bb, _) in self.basic_blocks.iter() {
            let bb = BasicBlock(bb);
            bb_map.insert(bb, self.find_most_inner_goto(bb));
        }
        self.transform_bbs_with_map(bb_map);
    }

    fn find_most_inner_goto(&self, bb: BasicBlock) -> BasicBlock {
        let bb_data = &self.basic_blocks[bb.0];
        if !bb_data.statements.is_empty() {
            return bb;
        }
        if let Some(TerminatorKind::Goto { bb }) = bb_data
            .terminator
            .as_ref()
            .map(|terminator| &terminator.kind)
        {
            self.find_most_inner_goto(*bb)
        } else {
            bb
        }
    }

    fn remove_unneeded_basic_blocks(&mut self) {
        let prev_basic_blocks = std::mem::take(&mut self.basic_blocks);
        let mut bb_map = HashMap::new();

        let mut mark_used = |bb: BasicBlock| match bb_map.entry(bb) {
            hash_map::Entry::Occupied(_) => (),
            hash_map::Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(BasicBlock(
                    self.basic_blocks
                        .alloc(prev_basic_blocks[bb.into_inner()].clone()),
                ));
            }
        };

        mark_used(BasicBlock(prev_basic_blocks.iter().next().unwrap().0));

        for (_, bb_data) in prev_basic_blocks.iter() {
            match &bb_data.terminator {
                Some(terminator) => match &terminator.kind {
                    TerminatorKind::Goto { bb } => mark_used(*bb),
                    TerminatorKind::SwitchInt { discr: _, targets } => {
                        mark_used(targets[0]);
                        mark_used(targets[1]);
                    }
                    TerminatorKind::Return => (),
                },
                None => (),
            }
        }

        self.transform_bbs_with_map(bb_map);
    }

    fn transform_bbs_with_map(&mut self, bb_map: HashMap<BasicBlock, BasicBlock>) {
        for (_, bb_data) in self.basic_blocks.iter_mut() {
            match &mut bb_data.terminator {
                Some(terminator) => match &mut terminator.kind {
                    TerminatorKind::Goto { bb } => {
                        *bb = bb_map[bb];
                    }
                    TerminatorKind::SwitchInt { discr: _, targets } => {
                        targets[0] = bb_map[&targets[0]];
                        targets[1] = bb_map[&targets[1]];
                    }
                    TerminatorKind::Return => (),
                },
                None => (),
            }
        }
    }
}
