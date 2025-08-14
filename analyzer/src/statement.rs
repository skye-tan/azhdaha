#![allow(clippy::missing_docs_in_private_items)]

use anyhow::bail;
use ariadne::{Fmt as _, Label};

use repr::{
    hir::{self, Span, resolver},
    mir,
};

use crate::{
    DIAGNOSIS_REPORT_COLOR,
    linear::{LinearAnalyzer, LinearLocal, LinearStatus},
    report::ReportSpan,
};

impl LinearAnalyzer<'_> {
    pub(crate) fn process_statement(
        &mut self,
        body: &mir::Body,
        linear_local: &mut LinearLocal,
        statement: &mir::Statement,
    ) -> anyhow::Result<bool> {
        match &statement.kind {
            mir::StatementKind::Assign(lhs, rhs) => {
                let mut is_accessed = false;
                let mut rhs_is_linear = false;

                match rhs {
                    mir::Rvalue::Use(operand) => {
                        if let mir::Operand::Place(place) = operand {
                            if linear_local.local == place.local {
                                is_accessed = true;
                            }

                            if body.local_decls[place.local].is_linear() {
                                rhs_is_linear = true;
                            }
                        }
                    }
                    mir::Rvalue::BinaryOp(_, left_operand, right_operand) => {
                        if let mir::Operand::Place(place) = left_operand {
                            if linear_local.local == place.local {
                                is_accessed = true;
                            }

                            if body.local_decls[place.local].is_linear() {
                                rhs_is_linear = true;
                            }
                        } else if let mir::Operand::Place(place) = right_operand {
                            if linear_local.local == place.local {
                                is_accessed = true;
                            }

                            if body.local_decls[place.local].is_linear() {
                                rhs_is_linear = true;
                            }
                        }
                    }
                    mir::Rvalue::UnaryOp(_, operand) => {
                        if let mir::Operand::Place(place) = operand {
                            if linear_local.local == place.local {
                                is_accessed = true;
                            }

                            if body.local_decls[place.local].is_linear() {
                                rhs_is_linear = true;
                            }
                        }
                    }
                    mir::Rvalue::Call(func, func_params) => {
                        let (func_name, func_sig, decl_span) = match func {
                            mir::Operand::Place(_) => unreachable!(),
                            mir::Operand::Const(_const) => match _const {
                                mir::Const::Symbol(symbol) => {
                                    let symbol_kind = body.symbol_resolver.get_data_by_res(symbol);

                                    match symbol_kind {
                                        resolver::SymbolKind::Func(func_decl) => {
                                            (&func_decl.ident.name, &func_decl.sig, func_decl.span)
                                        }
                                        resolver::SymbolKind::Var(local_decl) => {
                                            let mut ty_kind = &local_decl.ty.kind;

                                            loop {
                                                match ty_kind {
                                                    hir::TyKind::Ptr { kind, .. } => {
                                                        ty_kind = kind.as_ref();
                                                    }
                                                    hir::TyKind::Array { kind, .. } => {
                                                        ty_kind = kind.as_ref();
                                                    }
                                                    hir::TyKind::Func { sig } => {
                                                        break (
                                                            &local_decl.ident.name,
                                                            sig.as_ref(),
                                                            local_decl.span,
                                                        );
                                                    }
                                                    _ => unreachable!(),
                                                };
                                            }
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                                _ => unreachable!(),
                            },
                        };

                        if self.process_func_call(
                            body,
                            linear_local,
                            func_name,
                            func_sig,
                            func_params,
                            decl_span,
                        )? {
                            return Ok(true);
                        };

                        let lhs_decl = &body.local_decls[lhs.local];

                        if lhs.local == linear_local.local {
                            if func_sig.ret_ty.is_linear {
                                match linear_local.status {
                                    LinearStatus::Owner => {
                                        self.report.set_message("Overwriting owned value");

                                        self.report.add_label(
                                        Label::new(ReportSpan::new(statement.span))
                                            .with_message(format!(
                                                "Current owned value of {} is overwritten in here",
                                                format!("`{}`", linear_local.name)
                                                    .fg(DIAGNOSIS_REPORT_COLOR)
                                            ))
                                            .with_color(DIAGNOSIS_REPORT_COLOR),
                                        );

                                        self.report.add_help(format!(
                                            "Try to move {}'s value before reaching this statement",
                                            format!("`{}`", linear_local.name)
                                                .fg(DIAGNOSIS_REPORT_COLOR)
                                        ));

                                        return Ok(true);
                                    }
                                    LinearStatus::Free | LinearStatus::Unknown => {
                                        linear_local.status = LinearStatus::Owner;

                                        self.report.add_label(
                                            Label::new(ReportSpan::new(statement.span))
                                                .with_message(format!(
                                                    "A new value is moved to {} in here",
                                                    format!("`{}`", linear_local.name)
                                                        .fg(DIAGNOSIS_REPORT_COLOR),
                                                ))
                                                .with_color(DIAGNOSIS_REPORT_COLOR),
                                        );
                                    }
                                }
                            } else {
                                self.report
                                    .set_message("Assignment of non-linear to linear");

                                self.report.add_label(
                                    Label::new(ReportSpan::new(decl_span))
                                        .with_message(format!(
                                            "Function {} is defined in here which does not return a linear value",
                                            format!("`{func_name}`").fg(DIAGNOSIS_REPORT_COLOR),
                                        ))
                                        .with_color(DIAGNOSIS_REPORT_COLOR),
                                );

                                self.report.add_label(
                                    Label::new(ReportSpan::new(statement.span))
                                        .with_message(format!(
                                            "Cannot store a non-linear value in {} which is defined as linear",
                                            format!("`{}`", linear_local.name)
                                                .fg(DIAGNOSIS_REPORT_COLOR),
                                        ))
                                        .with_color(DIAGNOSIS_REPORT_COLOR),
                                );

                                self.report.add_help(
                                    "Try to store the returned value in a non-linear variable",
                                );

                                return Ok(true);
                            }

                            return Ok(false);
                        }

                        if lhs_decl.is_linear() && !func_sig.ret_ty.is_linear {
                            bail!(
                                "Not supported yet - Stored non-linear in linear after function call."
                            );
                        } else if !lhs_decl.is_linear() && func_sig.ret_ty.is_linear {
                            bail!(
                                "Not supported yet - Stored linear in non-linear after function call."
                            );
                        }

                        return Ok(false);
                    }
                    mir::Rvalue::Empty => (),
                }

                if is_accessed {
                    let lhs_decl = &body.local_decls[lhs.local];

                    let lhs_name = match &lhs_decl.kind {
                        mir::LocalKind::Real { ident, .. } => ident.name.clone(),
                        mir::LocalKind::Temp => bail!("Cannot retrieve name of the local."),
                    };

                    match (lhs_decl.is_linear(), &linear_local.status) {
                        (true, LinearStatus::Owner | LinearStatus::Unknown) => {
                            linear_local.status = LinearStatus::Free;

                            self.report.add_label(
                                Label::new(ReportSpan::new(statement.span))
                                    .with_message(format!(
                                        "{}'s value is moved in here",
                                        format!("`{}`", linear_local.name)
                                            .fg(DIAGNOSIS_REPORT_COLOR),
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
                                        "Cannot move {}'s invalid value to {}",
                                        format!("`{}`", linear_local.name)
                                            .fg(DIAGNOSIS_REPORT_COLOR),
                                        format!("`{lhs_name}`").fg(DIAGNOSIS_REPORT_COLOR)
                                    ))
                                    .with_color(DIAGNOSIS_REPORT_COLOR),
                            );

                            self.report.add_help(format!(
                                "Try to move a value to {} before reaching this statement",
                                format!("`{}`", linear_local.name).fg(DIAGNOSIS_REPORT_COLOR)
                            ));

                            return Ok(true);
                        }
                        (false, LinearStatus::Owner | LinearStatus::Unknown) => (),
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
                                        "Cannot lend {}'s invalid value to {}",
                                        format!("`{}`", linear_local.name)
                                            .fg(DIAGNOSIS_REPORT_COLOR),
                                        format!("`{lhs_name}`").fg(DIAGNOSIS_REPORT_COLOR)
                                    ))
                                    .with_color(DIAGNOSIS_REPORT_COLOR),
                            );

                            self.report.add_help(format!(
                                "Try to move a value to {} before reaching this statement",
                                format!("`{}`", linear_local.name).fg(DIAGNOSIS_REPORT_COLOR)
                            ));

                            return Ok(true);
                        }
                    }
                }

                // TODO: Better reports when a non-linear is assigned to a linear.
                if lhs.local == linear_local.local {
                    if !rhs_is_linear {
                        self.report
                            .set_message("Assignment of non-linear to linear");

                        self.report.add_label(
                            Label::new(ReportSpan::new(statement.span))
                                .with_message(format!(
                                    "Cannot store a non-linear value in {} which is defined as linear",
                                    format!("`{}`",linear_local.name)
                                        .fg(DIAGNOSIS_REPORT_COLOR),
                                ))
                                .with_color(DIAGNOSIS_REPORT_COLOR),
                        );

                        self.report
                            .add_help("Try to store the value in a non-linear variable");

                        return Ok(true);
                    }

                    match linear_local.status {
                        LinearStatus::Owner => {
                            self.report.set_message("Overwriting owned value");

                            self.report.add_label(
                                Label::new(ReportSpan::new(statement.span))
                                    .with_message(format!(
                                        "Current owned value of {} is overwritten in here",
                                        format!("`{}`", linear_local.name)
                                            .fg(DIAGNOSIS_REPORT_COLOR)
                                    ))
                                    .with_color(DIAGNOSIS_REPORT_COLOR),
                            );

                            self.report.add_help(format!(
                                "Try to move {}'s value before reaching this statement",
                                format!("`{}`", linear_local.name).fg(DIAGNOSIS_REPORT_COLOR)
                            ));

                            return Ok(true);
                        }
                        LinearStatus::Free | LinearStatus::Unknown => {
                            linear_local.status = LinearStatus::Owner;

                            self.report.add_label(
                                Label::new(ReportSpan::new(statement.span))
                                    .with_message(format!(
                                        "A new value is moved to {} in here",
                                        format!("`{}`", linear_local.name)
                                            .fg(DIAGNOSIS_REPORT_COLOR),
                                    ))
                                    .with_color(DIAGNOSIS_REPORT_COLOR),
                            );
                        }
                    }
                }
            }
            mir::StatementKind::Call(func, func_params) => {
                let (func_name, func_sig, decl_span) = match func {
                    mir::Operand::Place(_) => unreachable!(),
                    mir::Operand::Const(_const) => match _const {
                        mir::Const::Symbol(symbol) => {
                            let symbol_kind = body.symbol_resolver.get_data_by_res(symbol);

                            match symbol_kind {
                                resolver::SymbolKind::Func(func_decl) => {
                                    (&func_decl.ident.name, &func_decl.sig, func_decl.span)
                                }
                                resolver::SymbolKind::Var(local_decl) => {
                                    let mut ty_kind = &local_decl.ty.kind;

                                    loop {
                                        match ty_kind {
                                            hir::TyKind::Ptr { kind, .. } => {
                                                ty_kind = kind.as_ref();
                                            }
                                            hir::TyKind::Array { kind, .. } => {
                                                ty_kind = kind.as_ref();
                                            }
                                            hir::TyKind::Func { sig } => {
                                                break (
                                                    &local_decl.ident.name,
                                                    sig.as_ref(),
                                                    local_decl.span,
                                                );
                                            }
                                            _ => unreachable!(),
                                        };
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                        _ => unreachable!(),
                    },
                };

                if self.process_func_call(
                    body,
                    linear_local,
                    func_name,
                    func_sig,
                    func_params,
                    decl_span,
                )? {
                    return Ok(true);
                };

                if func_sig.ret_ty.is_linear {
                    bail!("Not supported yet - Ignored linear result after function call.")
                }
            }
        }

        Ok(false)
    }

    fn process_func_call(
        &mut self,
        body: &mir::Body,
        linear_local: &mut LinearLocal,
        func_name: &str,
        func_sig: &hir::FuncSig,
        func_params: &[mir::Operand],
        decl_span: Span,
    ) -> anyhow::Result<bool> {
        if func_params.len() != func_sig.params.len() {
            bail!("Invalid number of arguments for the function call.");
        }

        for (param_operand, func_param_decl) in func_params.iter().zip(func_sig.params.iter()) {
            let mir::Operand::Place(param_place) = param_operand else {
                continue;
            };

            if !func_param_decl.ty.is_linear {
                continue;
            }

            if linear_local.local != param_place.local {
                if !body.local_decls[param_place.local].is_linear() {
                    bail!("Not supported yet - Passed non-linear as linear to function.");
                }

                continue;
            }

            match linear_local.status {
                LinearStatus::Owner | LinearStatus::Unknown => {
                    linear_local.status = LinearStatus::Free;

                    self.report.add_label(
                        Label::new(ReportSpan::new(param_place.span))
                            .with_message(format!(
                                "{}'s value is moved in here",
                                format!("`{}`", linear_local.name).fg(DIAGNOSIS_REPORT_COLOR),
                            ))
                            .with_color(DIAGNOSIS_REPORT_COLOR),
                    );

                    continue;
                }
                LinearStatus::Free => (),
            }

            let func_param_name = func_param_decl
                .ident
                .clone()
                .map(|ident| ident.name)
                .unwrap_or_default();

            self.report.set_message("Use of moved value");

            self.report.add_label(
                Label::new(ReportSpan::new(decl_span))
                    .with_message(format!(
                        "Function {} is defined in here which captures parameter {} as linear",
                        format!("`{func_name}`").fg(DIAGNOSIS_REPORT_COLOR),
                        format!("`{func_param_name}`",).fg(DIAGNOSIS_REPORT_COLOR)
                    ))
                    .with_color(DIAGNOSIS_REPORT_COLOR),
            );

            self.report.add_label(
                Label::new(ReportSpan::new(param_place.span))
                    .with_message(format!(
                        "Cannot move and pass {}'s invalid value",
                        format!("`{}`", linear_local.name).fg(DIAGNOSIS_REPORT_COLOR),
                    ))
                    .with_color(DIAGNOSIS_REPORT_COLOR),
            );

            self.report.add_help(format!(
                "Try to move a value to {} before reaching this statement",
                format!("`{}`", linear_local.name).fg(DIAGNOSIS_REPORT_COLOR)
            ));

            return Ok(true);
        }

        Ok(false)
    }
}
