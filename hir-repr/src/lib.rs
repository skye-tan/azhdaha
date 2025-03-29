//! The HIR – "High-Level Intermediate Representation" – is the primary IR used for representation of the
//! abstract syntax tree (AST) that is generated after parsing, macro expansion, and name resolution.
//!
//! This implementation has been modeled after rustc's HIR.
//!

/// Contains constant values used to generate the HIR.
mod constant;
/// Contains implementation of the [`Constructable`] trait for datatypes.
mod construct;
/// Contains datatypes used to represent the HIR.
mod datatype;

pub use datatype::*;
