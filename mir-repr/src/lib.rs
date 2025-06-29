//! The MIR - "Mid-level Intermediate Representation" is a radically simplified form constructed from HIR.
//! This representation is used for generating CFG - "Control Flow Graph" - of the source code.
//!
//! This implementation has been modeled after rustc's MIR representation.
//!

#![allow(clippy::missing_docs_in_private_items)]
#![allow(dead_code)]

mod basic_block;
mod operand;
mod statement;
mod terminator;
mod types;
