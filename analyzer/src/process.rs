#![allow(clippy::missing_docs_in_private_items)]

use anyhow::Context;
use ariadne::{Fmt as _, Label};

use repr::mir;

use crate::{
    DIAGNOSIS_REPORT_COLOR,
    linear::{LinearAnalyzer, LinearLocal, LinearStatus},
    report::ReportSpan,
};

impl LinearAnalyzer<'_> {
    pub(crate) fn process_bb(
        &mut self,
        body: &mir::Body,
        linear_local: &mut LinearLocal,
        bb_data: &mir::BasicBlockData,
    ) -> anyhow::Result<()> {
        for statement in &bb_data.statements {
            self.process_statement(body, linear_local, statement)?;
        }

        let Some(terminator) = &bb_data.terminator else {
            return Ok(());
        };

        self.process_terminator(body, linear_local, terminator)
    }

    pub(crate) fn process_statement(
        &mut self,
        _body: &mir::Body,
        _linear_local: &mut LinearLocal,
        _statement: &mir::Statement,
    ) -> anyhow::Result<()> {
        todo!()
    }

    pub(crate) fn process_terminator(
        &mut self,
        body: &mir::Body,
        linear_local: &mut LinearLocal,
        terminator: &mir::Terminator,
    ) -> anyhow::Result<()> {
        if !matches!(&terminator.kind, mir::TerminatorKind::Return) {
            return Ok(());
        }

        if linear_local.status == LinearStatus::Free {
            return Ok(());
        }

        let linear_local_decl = &body.local_decls[linear_local.local];
        let linear_local_name = linear_local_decl
            .debug_name
            .as_ref()
            .context("Failed to retrieve name of the local.")?;

        self.report.set_message("Memory leakage after return");

        self.report.add_label(
            Label::new(ReportSpan::new(terminator.span))
                .with_message(format!(
                    "Function returns here while {} might not have moved its value",
                    format!("`{linear_local_name}`").fg(DIAGNOSIS_REPORT_COLOR)
                ))
                .with_color(DIAGNOSIS_REPORT_COLOR),
        );

        self.report.add_help(format!(
            "Try to move {}'s value before reaching return",
            format!("`{linear_local_name}`").fg(DIAGNOSIS_REPORT_COLOR)
        ));

        Err(anyhow::Error::msg("Memory leakage after return"))
    }
}
