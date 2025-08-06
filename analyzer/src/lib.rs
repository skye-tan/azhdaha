//! The analyzer which applies the "Linear Type" rules to the MIR representation of the
//! input source code and reports the possible memory leakages.
//!

use ariadne::Color;

/// Contains methods needed to perform DFS on the MIR.
mod dfs;
/// Contains linear datatypes' definitions.
mod linear;
/// Contains methods needed to process MIR's components.
mod process;
/// Contains custom implementation of [`ariadne::Cache`] and [`ariadne::Span`].
mod report;

pub use linear::LinearCtx;

/// The color used to generate reports.
pub(crate) const DIAGNOSIS_REPORT_COLOR: Color = Color::Rgb(255, 165, 0);
