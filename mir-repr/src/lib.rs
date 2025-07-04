//! The MIR - "Mid-level Intermediate Representation" is a radically simplified form constructed from HIR.
//! This representation is used for generating CFG - "Control Flow Graph" - of the source code.
//!
//! This implementation has been modeled after rustc's MIR representation.
//!

mod datatypes;

pub use datatypes::*;

// impl MirCtx {
//     pub fn new() -> Self {
//         Self { bodies: vec![] }
//     }

//     pub fn build_mir(&mut self) -> Body {
//         todo!()
//     }
// }
