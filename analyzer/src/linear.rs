#![allow(clippy::missing_docs_in_private_items)]

use anyhow::Context;
use ariadne::{Fmt as _, Label, Report, ReportBuilder, ReportKind, Source};
use log::error;

use repr::mir;

use crate::{
    DIAGNOSIS_REPORT_COLOR,
    report::{ReportCache, ReportSpan},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LinearLocal {
    pub(crate) local: mir::Local,
    pub(crate) status: LinearStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LinearStatus {
    Owner,
    Free,
}

pub(crate) struct LinearAnalyzer<'linear> {
    pub(crate) report: ReportBuilder<'linear, ReportSpan>,
}

impl LinearAnalyzer<'_> {
    pub(crate) fn new(span: ReportSpan) -> Self {
        Self {
            report: Report::build(ReportKind::Error, span),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LinearCtx<'linear> {
    pub(crate) source_path: String,
    pub(crate) report_source: Source<&'linear str>,
}

impl<'linear> LinearCtx<'linear> {
    pub fn new(source_path: &str, source_code: &'linear [u8]) -> anyhow::Result<Self> {
        Ok(Self {
            source_path: source_path.to_owned(),
            report_source: Source::from(
                str::from_utf8(source_code)
                    .context("UTF-8 validity for the source code failed.")?,
            ),
        })
    }

    pub fn analyze(&self, body: &mut mir::Body) {
        let mut linear_locals: Vec<LinearLocal> = body
            .local_decls
            .iter()
            .filter_map(|(local, local_decl)| {
                if local_decl.ty.is_linear {
                    return Some(LinearLocal {
                        local,
                        status: LinearStatus::Free,
                    });
                }

                None
            })
            .collect();

        // TODO: Must be removed in the future.
        if linear_locals.is_empty() {
            let local = body.local_decls.alloc(mir::LocalDecl {
                debug_name: Some("dummy".to_owned()),
                storage: None,
                ty: repr::hir::Ty {
                    kind: repr::hir::TyKind::PrimTy(repr::hir::PrimTyKind::Void),
                    is_linear: true,
                    quals: vec![],
                    span: repr::hir::Span { lo: 0, hi: 0 },
                },
                span: repr::hir::Span { lo: 0, hi: 0 },
            });

            linear_locals.push(LinearLocal {
                local,
                status: LinearStatus::Free,
            });
        }

        for linear_local in linear_locals {
            let linear_local_decl = &body.local_decls[linear_local.local];

            let Some(linear_local_name) = &linear_local_decl.debug_name else {
                continue;
            };

            for (bb, _) in body.basic_blocks.iter() {
                let mut linear_analyzer = LinearAnalyzer::new(ReportSpan::new(body.span));

                linear_analyzer.report.add_label(
                    Label::new(ReportSpan::new(linear_local_decl.span))
                        .with_message(format!(
                            "Variable {} is defined in here as linear",
                            format!("`{linear_local_name}`").fg(DIAGNOSIS_REPORT_COLOR)
                        ))
                        .with_color(DIAGNOSIS_REPORT_COLOR),
                );

                if linear_analyzer.dfs_with_stack(body, linear_local.clone(), bb)
                    && let Err(error) = linear_analyzer.report.finish().print(ReportCache::new(
                        self.source_path.clone(),
                        &self.report_source,
                    ))
                {
                    error!("Failed to print the linear analyzer's report - {error:?}");
                }
            }
        }
    }
}
