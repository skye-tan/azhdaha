//! A cli which parses the entered commands.

/// The module responsible for parsing the cli's commands.
mod arg_parser;

pub use arg_parser::parse_args;
