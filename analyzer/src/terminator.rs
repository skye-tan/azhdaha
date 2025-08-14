#![allow(clippy::missing_docs_in_private_items)]

use ariadne::{Fmt as _, Label};

use repr::mir;

use crate::{
    DIAGNOSIS_REPORT_COLOR,
    linear::{LinearAnalyzer, LinearLocal, LinearStatus},
    report::ReportSpan,
};

impl LinearAnalyzer<'_> {
    pub(crate) fn process_terminator(
        &mut self,
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

                self.report.set_message("Memory leakage after return");

                self.report.add_label(
                    Label::new(ReportSpan::new(terminator.span))
                        .with_message(format!(
                            "Function returns in here without {} moving its value",
                            format!("`{}`", linear_local.name).fg(DIAGNOSIS_REPORT_COLOR)
                        ))
                        .with_color(DIAGNOSIS_REPORT_COLOR),
                );

                self.report.add_help(format!(
                    "Try to move {}'s value before reaching the return",
                    format!("`{}`", linear_local.name).fg(DIAGNOSIS_REPORT_COLOR)
                ));
            }
            None => {
                self.report.set_message("Memory leakage after return");

                self.report.add_note(format!(
                    "Function returns without {} moving its value",
                    format!("`{}`", linear_local.name).fg(DIAGNOSIS_REPORT_COLOR)
                ));

                self.report.add_help(format!(
                    "Try to move {}'s value before reaching the return",
                    format!("`{}`", linear_local.name).fg(DIAGNOSIS_REPORT_COLOR)
                ));
            }
        }

        Ok(true)
    }
}
