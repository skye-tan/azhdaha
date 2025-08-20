#![allow(clippy::missing_docs_in_private_items)]

use ariadne::{Fmt as _, Label, Report, ReportBuilder, ReportKind};
use log::error;

use repr::mir;

use crate::{
    DIAGNOSIS_REPORT_COLOR,
    linear::{LinearCtx, LinearLocal},
    report::ReportSpan,
};

impl LinearCtx<'_> {
    pub(crate) fn dfs_with_stack(
        &self,
        body: &mir::Body,
        linear_local: LinearLocal,
        bb: mir::BasicBlock,
    ) -> Option<Report<'_, ReportSpan>> {
        let report_builder = Report::build(ReportKind::Error, ReportSpan::new(body.span))
            .with_label(
                Label::new(ReportSpan::new(linear_local.span))
                    .with_message(format!(
                        "Variable {} is defined in here as linear",
                        format!("`{}`", linear_local.name).fg(DIAGNOSIS_REPORT_COLOR)
                    ))
                    .with_color(DIAGNOSIS_REPORT_COLOR),
            );

        let node_count = body.basic_blocks.len();
        let mut visited_edges = vec![vec![false; node_count]; node_count];

        let mut bb_stack = vec![(report_builder, linear_local, bb)];

        while let Some((mut report_builder, mut linear_local, bb)) = bb_stack.pop() {
            let index = bb.get_id();

            let bb_data = &body.basic_blocks[bb.into_inner()];

            match self.process_bb(body, &mut report_builder, &mut linear_local, bb_data) {
                Ok(should_be_reported) => {
                    if should_be_reported {
                        return Some(report_builder.finish());
                    }
                }
                Err(error) => {
                    error!("Failed to check linear bounds - {error:?}")
                }
            }

            if linear_local.is_altered {
                continue;
            }

            let Some(terminator) = &bb_data.terminator else {
                continue;
            };

            match &terminator.kind {
                mir::TerminatorKind::Goto { bb } => {
                    if !visited_edges[index][bb.get_id()] {
                        bb_stack.push((report_builder, linear_local, *bb));
                        visited_edges[index][bb.get_id()] = true;
                    }
                }
                mir::TerminatorKind::SwitchInt { targets, .. } => {
                    if !visited_edges[index][targets[0].get_id()] {
                        bb_stack.push((report_builder.clone(), linear_local.clone(), targets[0]));
                        visited_edges[index][targets[0].get_id()] = true;
                    }

                    if !visited_edges[index][targets[1].get_id()] {
                        bb_stack.push((report_builder, linear_local, targets[1]));
                        visited_edges[index][targets[1].get_id()] = true;
                    }
                }
                mir::TerminatorKind::Return => continue,
            }
        }

        None
    }

    fn process_bb(
        &self,
        body: &mir::Body,
        report_builder: &mut ReportBuilder<'_, ReportSpan>,
        linear_local: &mut LinearLocal,
        bb_data: &mir::BasicBlockData,
    ) -> anyhow::Result<bool> {
        for statement in &bb_data.statements {
            if self.process_statement(body, report_builder, linear_local, statement)? {
                return Ok(true);
            }
        }

        self.process_terminator(report_builder, linear_local, &bb_data.terminator)
    }
}
