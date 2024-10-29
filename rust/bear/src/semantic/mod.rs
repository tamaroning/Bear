// SPDX-License-Identifier: GPL-3.0-or-later

pub mod interpreters;

use intercept::Execution;
use std::path::PathBuf;

/// Represents an executed command semantic.
#[derive(Debug, PartialEq)]
pub enum Meaning {
    /// This is a compiler call.
    Compiler {
        compiler: PathBuf,
        working_dir: PathBuf,
        passes: Vec<CompilerPass>,
    },
    /// This is something else we recognised, but not interested to fully specify.
    Ignored,
}

/// Represents a compiler call pass.
#[derive(Debug, PartialEq)]
pub enum CompilerPass {
    Preprocess,
    Compile {
        source: PathBuf,
        output: Option<PathBuf>,
        flags: Vec<String>,
    },
}

/// This abstraction is representing a tool which semantic we are aware of.
///
/// A single tool has a potential to recognize a command execution and
/// identify the semantic of that command. This abstraction is also can
/// represent a set of interpreters, and the recognition process can be
/// distributed amongst the interpreters.
pub trait Interpreter: Send {
    fn recognize(&self, _: &Execution) -> Recognition<Meaning>;
}

/// Represents a semantic recognition result.
#[derive(Debug, PartialEq)]
pub enum Recognition<T> {
    Success(T),
    Error(String),
    Unknown,
}
