//! Facilities for rendering errors beatifully.

use std::ops::Range;

use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet, renderer::DecorStyle};

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

impl Span {
    pub const DUMMY: Self = Self { lo: 0, hi: 0 };

    /// Convert span to a range.
    fn to_range(self) -> Range<usize> {
        self.lo..self.hi
    }
}

#[derive(Debug)]
pub struct Error {
    pub primary_message: (String, Span),
    pub additional_messages: Vec<(String, Span)>,
}

impl Error {
    #[allow(clippy::print_stderr)]
    pub fn report(&self, source: &str) {
        let mut section = Snippet::source(source).annotation(
            AnnotationKind::Primary
                .span(self.primary_message.1.to_range())
                .label(&self.primary_message.0),
        );

        for (message, span) in &self.additional_messages {
            section =
                section.annotation(AnnotationKind::Context.span(span.to_range()).label(message));
        }
        let element = Level::ERROR
            .primary_title(&self.primary_message.0)
            .element(section);

        let report = &[element];

        let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
        eprintln!("{}", renderer.render(report));
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Context: Sized {
    type OkResult;

    fn context(self, span: Span, message: &str) -> Result<Self::OkResult> {
        self.with_context(span, || message.to_owned())
    }
    fn with_context(self, span: Span, message: impl FnOnce() -> String) -> Result<Self::OkResult>;
}

impl<T> Context for Result<T> {
    type OkResult = T;

    fn with_context(
        mut self,
        span: Span,
        message: impl FnOnce() -> String,
    ) -> Result<Self::OkResult> {
        if let Err(error) = &mut self {
            error.additional_messages.push((message(), span));
        }
        self
    }
}

impl<T> Context for Option<T> {
    type OkResult = T;

    fn with_context(self, span: Span, message: impl FnOnce() -> String) -> Result<Self::OkResult> {
        match self {
            Some(value) => Ok(value),
            None => Err(Error {
                primary_message: (message(), span),
                additional_messages: vec![],
            }),
        }
    }
}

#[macro_export]
macro_rules! bail {
    ($span:expr, $($arg:tt)*) => {
        return Err($crate::Error {
            primary_message: (format!($($arg)*), $span),
            additional_messages: vec![],
        })
    };
}
