#![allow(clippy::missing_docs_in_private_items)]

use std::fmt;

use ariadne::{Cache, Source};

use repr::hir::Span;

pub(crate) struct ReportSpan(Span);

impl ReportSpan {
    pub(crate) fn new(span: Span) -> Self {
        Self(span)
    }
}

impl ariadne::Span for ReportSpan {
    type SourceId = ();

    fn source(&self) -> &Self::SourceId {
        &()
    }

    fn start(&self) -> usize {
        self.0.lo
    }

    fn end(&self) -> usize {
        self.0.hi
    }
}

pub(crate) struct ReportCache<'linear> {
    pub(crate) source_path: String,
    pub(crate) report_source: &'linear Source<&'linear str>,
}

impl<'linear> ReportCache<'linear> {
    pub fn new(source_path: String, report_source: &'linear Source<&'linear str>) -> Self {
        Self {
            source_path,
            report_source,
        }
    }
}

impl<'linear, Id> Cache<Id> for ReportCache<'linear> {
    type Storage = &'linear str;

    fn fetch(&mut self, _id: &Id) -> Result<&Source<Self::Storage>, impl std::fmt::Debug> {
        Ok::<&Source<&str>, fmt::Error>(self.report_source)
    }

    fn display<'a>(&self, _id: &'a Id) -> Option<impl fmt::Display + 'a> {
        Some(self.source_path.clone())
    }
}
