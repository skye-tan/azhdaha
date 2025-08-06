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
    ) -> anyhow::Result<bool> {
        for statement in &bb_data.statements {
            if self.process_statement(body, linear_local, statement)? {
                return Ok(true);
            }
        }

        let Some(terminator) = &bb_data.terminator else {
            return Ok(false);
        };

        self.process_terminator(body, linear_local, terminator)
    }

    pub(crate) fn process_statement(
        &mut self,
        body: &mir::Body,
        linear_local: &mut LinearLocal,
        statement: &mir::Statement,
    ) -> anyhow::Result<bool> {
        let linear_local_decl = &body.local_decls[linear_local.local];
        let linear_local_name = linear_local_decl
            .debug_name
            .as_ref()
            .context("Cannot retrieve name of the local.")?;

        match &statement.kind {
            mir::StatementKind::Assign(lhs, rhs) => {
                let mut is_accessed = false;

                match rhs {
                    mir::Rvalue::Use(operand) => {
                        if let mir::Operand::Place(place) = operand
                            && place.local == linear_local.local
                        {
                            is_accessed = true;
                        }
                    }
                    mir::Rvalue::BinaryOp(_, left_operand, right_operand) => {
                        if let mir::Operand::Place(place) = left_operand
                            && place.local == linear_local.local
                        {
                            is_accessed = true;
                        } else if let mir::Operand::Place(place) = right_operand
                            && place.local == linear_local.local
                        {
                            is_accessed = true;
                        }
                    }
                    mir::Rvalue::UnaryOp(_, operand) => {
                        if let mir::Operand::Place(place) = operand
                            && place.local == linear_local.local
                        {
                            is_accessed = true;
                        }
                    }
                    mir::Rvalue::Call(_, operands) => {
                        for operand in operands {
                            if let mir::Operand::Place(place) = operand
                                && place.local == linear_local.local
                            {
                                is_accessed = true;
                                break;
                            }
                        }
                    }
                    mir::Rvalue::Empty => (),
                }

                if is_accessed {
                    let lhs_decl = &body.local_decls[lhs.local];
                    let lhs_name = lhs_decl
                        .debug_name
                        .as_ref()
                        .context("Cannot retrieve name of the local.")?;

                    match (lhs_decl.ty.is_linear, &linear_local.status) {
                        (true, LinearStatus::Owner) => {
                            linear_local.status = LinearStatus::Free;

                            self.report.add_label(
                                Label::new(ReportSpan::new(statement.span))
                                    .with_message(format!(
                                        "{}'s value is moved in here  ",
                                        format!("`{linear_local_name}`").fg(DIAGNOSIS_REPORT_COLOR),
                                    ))
                                    .with_color(DIAGNOSIS_REPORT_COLOR),
                            );
                        }
                        (true, LinearStatus::Free) => {
                            self.report.set_message("Use of moved value");

                            self.report.add_label(
                                Label::new(ReportSpan::new(lhs_decl.span))
                                    .with_message(format!(
                                        "Variable {} is defined in here as linear",
                                        format!("`{lhs_name}`").fg(DIAGNOSIS_REPORT_COLOR)
                                    ))
                                    .with_color(DIAGNOSIS_REPORT_COLOR),
                            );

                            self.report.add_label(
                                Label::new(ReportSpan::new(statement.span))
                                    .with_message(format!(
                                        "Cannot move {}'s invalid value to {} ",
                                        format!("`{linear_local_name}`").fg(DIAGNOSIS_REPORT_COLOR),
                                        format!("`{lhs_name}`").fg(DIAGNOSIS_REPORT_COLOR)
                                    ))
                                    .with_color(DIAGNOSIS_REPORT_COLOR),
                            );

                            self.report.add_help(format!(
                                "Try to move a value to {} before reaching this statement",
                                format!("`{linear_local_name}`").fg(DIAGNOSIS_REPORT_COLOR)
                            ));

                            return Ok(true);
                        }
                        (false, LinearStatus::Owner) => {
                            self.report.add_label(
                                Label::new(ReportSpan::new(statement.span))
                                    .with_message(format!(
                                        "{}'s value is burrowed in here  ",
                                        format!("`{linear_local_name}`").fg(DIAGNOSIS_REPORT_COLOR),
                                    ))
                                    .with_color(DIAGNOSIS_REPORT_COLOR),
                            );
                        }
                        (false, LinearStatus::Free) => {
                            self.report.set_message("Use of moved value");

                            self.report.add_label(
                                Label::new(ReportSpan::new(lhs_decl.span))
                                    .with_message(format!(
                                        "Variable {} is defined in here as linear",
                                        format!("`{lhs_name}`").fg(DIAGNOSIS_REPORT_COLOR)
                                    ))
                                    .with_color(DIAGNOSIS_REPORT_COLOR),
                            );

                            self.report.add_label(
                                Label::new(ReportSpan::new(statement.span))
                                    .with_message(format!(
                                        "Cannot lend {}'s invalid value to {} ",
                                        format!("`{linear_local_name}`").fg(DIAGNOSIS_REPORT_COLOR),
                                        format!("`{lhs_name}`").fg(DIAGNOSIS_REPORT_COLOR)
                                    ))
                                    .with_color(DIAGNOSIS_REPORT_COLOR),
                            );

                            self.report.add_help(format!(
                                "Try to move a value to {} before reaching this statement",
                                format!("`{linear_local_name}`").fg(DIAGNOSIS_REPORT_COLOR)
                            ));

                            return Ok(true);
                        }
                    }
                }

                if lhs.local == linear_local.local {
                    match linear_local.status {
                        LinearStatus::Owner => {
                            self.report.set_message("Overwriting owned value");

                            self.report.add_label(
                                Label::new(ReportSpan::new(statement.span))
                                    .with_message(format!(
                                        "Current owned value of {} is overwritten in here",
                                        format!("`{linear_local_name}`").fg(DIAGNOSIS_REPORT_COLOR)
                                    ))
                                    .with_color(DIAGNOSIS_REPORT_COLOR),
                            );

                            self.report.add_help(format!(
                                "Try to move {}'s value before reaching this statement",
                                format!("`{linear_local_name}`").fg(DIAGNOSIS_REPORT_COLOR)
                            ));

                            return Ok(true);
                        }
                        LinearStatus::Free => linear_local.status = LinearStatus::Owner,
                    }
                }
            }
            mir::StatementKind::Call(_operand, _operands) => (),
        }

        Ok(false)
    }

    pub(crate) fn process_terminator(
        &mut self,
        body: &mir::Body,
        linear_local: &mut LinearLocal,
        terminator: &mir::Terminator,
    ) -> anyhow::Result<bool> {
        if !matches!(&terminator.kind, mir::TerminatorKind::Return) {
            return Ok(false);
        }

        if linear_local.status == LinearStatus::Free {
            return Ok(false);
        }

        let linear_local_decl = &body.local_decls[linear_local.local];
        let linear_local_name = linear_local_decl
            .debug_name
            .as_ref()
            .context("Cannot retrieve name of the local.")?;

        self.report.set_message("Memory leakage after return");

        self.report.add_label(
            Label::new(ReportSpan::new(terminator.span))
                .with_message(format!(
                    "Function returns in here while {} might not have moved its value",
                    format!("`{linear_local_name}`").fg(DIAGNOSIS_REPORT_COLOR)
                ))
                .with_color(DIAGNOSIS_REPORT_COLOR),
        );

        self.report.add_help(format!(
            "Try to move {}'s value before reaching the return",
            format!("`{linear_local_name}`").fg(DIAGNOSIS_REPORT_COLOR)
        ));

        Ok(true)
    }
}
