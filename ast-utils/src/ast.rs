use std::{fs::File, os::fd::AsRawFd};

use anyhow::Context;
use compile_commands::CompilationDatabase;
use tree_sitter::{Parser, Tree};

use crate::preprocess;

pub struct AstRepr {
    /// The source code which the tree has been generated from.
    pub source_code: Vec<u8>,
    /// The tree generated by the tree-sitter.
    pub tree: Tree,
}

impl AstRepr {
    pub fn create_dot_graph(&self, path: &str) -> anyhow::Result<()> {
        let file = File::create(path)?;

        self.tree.print_dot_graph(&file.as_raw_fd());

        Ok(())
    }

    pub fn construct(compile_commands: &CompilationDatabase) -> anyhow::Result<Vec<Self>> {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_c::LANGUAGE.into())
            .context("Failed to load C grammar.")?;

        let mut asts = vec![];

        for source_code in preprocess::preprocess(compile_commands)? {
            let Some(tree) = parser.parse(&source_code, None) else {
                log::warn!("Failed to parse using tree-sitter.");
                continue;
            };

            asts.push(AstRepr { source_code, tree });
        }

        Ok(asts)
    }
}
