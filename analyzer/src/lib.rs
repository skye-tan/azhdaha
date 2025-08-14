//! The analyzer which applies the "Linear Type" rules to the MIR representation of the
//! input source code and reports the possible memory leakages.
//!

use ariadne::Color;

/// Contains methods needed to perform DFS on the MIR.
mod dfs;
/// Contains linear datatypes' definitions.
mod linear;
/// Contains custom implementation of [`ariadne::Cache`] and [`ariadne::Span`].
mod report;
/// Contains methods needed to process MIR's [`repr::mir::Statement`].
mod statement;
/// Contains methods needed to process MIR's [`repr::mir::Terminator`].
mod terminator;

pub use linear::LinearCtx;

/// The color used to generate reports.
pub(crate) const DIAGNOSIS_REPORT_COLOR: Color = Color::Rgb(255, 165, 0);
