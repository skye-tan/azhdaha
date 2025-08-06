#![allow(clippy::missing_docs_in_private_items)]

use anyhow::Context;
use ariadne::{Color, Fmt as _, Label, Report, ReportKind};

use repr::mir;

use crate::{LinearCtx, LinearLocal, LinearStatus};

const REPORT_COLOR: Color = Color::Rgb(255, 165, 0);

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

        self.process_terminator(body, linear_local, terminator)?;

        Ok(false)
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
        let debug_name = linear_local_decl
            .debug_name
            .as_ref()
            .context("Failed to retrieve debug name of local.")?;

        Report::build(
            ReportKind::Error,
            (&self.source_path, terminator.span.lo..terminator.span.hi),
        )
        .with_code(0)
        .with_message("Memory leakage after return")
        .with_label(
            Label::new((
                &self.source_path,
                linear_local_decl.span.lo..linear_local_decl.span.hi,
            ))
            .with_message(format!(
                "Variable {} was defined here as linear",
                format!("`{debug_name}`").fg(REPORT_COLOR)
            ))
            .with_color(REPORT_COLOR),
        )
        .with_label(
            Label::new((&self.source_path, terminator.span.lo..terminator.span.hi))
                .with_message(format!(
                    "Function returns here while {} might not have been moved",
                    format!("`{debug_name}`").fg(REPORT_COLOR)
                ))
                .with_color(REPORT_COLOR),
        )
        .with_note(format!(
            "Try to move {}'s value before reaching return",
            format!("`{debug_name}`").fg(REPORT_COLOR)
        ))
        .finish()
        .print((&self.source_path, &self.report_source))?;

        Ok(())
    }
}
