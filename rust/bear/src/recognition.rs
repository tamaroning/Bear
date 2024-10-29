// SPDX-License-Identifier: GPL-3.0-or-later
use super::{config, semantic};
use intercept::Execution;
use std::convert::TryFrom;

/// Responsible for recognizing the semantic meaning of the executed commands.
///
/// The recognition logic is implemented in the `interpreters` module. Here we only handle
/// the errors and logging them to the console.
pub struct Recognition {
    interpreter: Box<dyn semantic::Interpreter>,
}

impl TryFrom<&config::Main> for Recognition {
    type Error = anyhow::Error;

    fn try_from(config: &config::Main) -> Result<Self, Self::Error> {
        let compilers_to_include = match &config.intercept {
            config::Intercept::Wrapper { executables, .. } => executables.clone(),
            _ => vec![],
        };
        let compilers_to_exclude = match &config.output {
            config::Output::Clang { compilers, .. } => compilers
                .into_iter()
                .filter(|compiler| compiler.ignore == config::Ignore::Always)
                .map(|compiler| compiler.path.clone())
                .collect(),
            _ => vec![],
        };
        let interpreter = semantic::interpreters::Builder::new()
            .compilers_to_recognize(compilers_to_include.as_slice())
            .compilers_to_exclude(compilers_to_exclude.as_slice())
            .build();

        Ok(Recognition {
            interpreter: Box::new(interpreter),
        })
    }
}

impl Recognition {
    pub fn apply(&self, execution: Execution) -> Option<semantic::Meaning> {
        match self.interpreter.recognize(&execution) {
            semantic::Recognition::Success(semantic::Meaning::Ignored) => {
                log::debug!("execution recognized, but ignored: {:?}", execution);
                None
            }
            semantic::Recognition::Success(semantic) => {
                log::debug!(
                    "execution recognized as compiler call, {:?} : {:?}",
                    semantic,
                    execution
                );
                Some(semantic)
            }
            semantic::Recognition::Error(reason) => {
                log::debug!(
                    "execution recognized with failure, {:?} : {:?}",
                    reason,
                    execution
                );
                None
            }
            semantic::Recognition::Unknown => {
                log::debug!("execution not recognized: {:?}", execution);
                None
            }
        }
    }
}
