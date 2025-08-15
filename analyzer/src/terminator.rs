#![allow(clippy::missing_docs_in_private_items)]

use ariadne::{Fmt as _, Label, ReportBuilder};

use repr::mir;

use crate::{
    DIAGNOSIS_REPORT_COLOR, LinearCtx,
    linear::{LinearLocal, LinearStatus},
    report::ReportSpan,
};

impl LinearCtx<'_> {
    pub(crate) fn process_terminator(
        &self,
        report_builder: &mut ReportBuilder<'_, ReportSpan>,
        linear_local: &mut LinearLocal,
        terminator: &Option<mir::Terminator>,
    ) -> anyhow::Result<bool> {
        match linear_local.status {
            LinearStatus::Free | LinearStatus::Unknown => return Ok(false),
            LinearStatus::Owner => (),
        }

        match terminator {
            Some(terminator) => {
                if !matches!(&terminator.kind, mir::TerminatorKind::Return) {
                    return Ok(false);
                }

                report_builder.set_message("Memory leakage after return");

                report_builder.add_label(
                    Label::new(ReportSpan::new(terminator.span))
                        .with_message(format!(
                            "Function returns in here without {} moving its value",
                            format!("`{}`", linear_local.name).fg(DIAGNOSIS_REPORT_COLOR)
                        ))
                        .with_color(DIAGNOSIS_REPORT_COLOR),
                );
            }
            None => {
                report_builder.set_message("Memory leakage after return");

                report_builder.add_note(format!(
                    "Function returns without {} moving its value",
                    format!("`{}`", linear_local.name).fg(DIAGNOSIS_REPORT_COLOR)
                ));
            }
        }

        report_builder.add_help(format!(
            "Try to move {}'s value before reaching the return",
            format!("`{}`", linear_local.name).fg(DIAGNOSIS_REPORT_COLOR)
        ));

        Ok(true)
    }
}
