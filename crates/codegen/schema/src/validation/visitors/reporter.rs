use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

use anyhow::Result;
use infra_utils::errors::InfraErrors;
use infra_utils::paths::PathExtensions;

use crate::validation::visitors::location::{Location, LocationRef};
use crate::yaml::{NodeRef, Parser};

pub struct Reporter {
    errors: Vec<(LocationRef, String)>,
}

impl Reporter {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn report(&mut self, location: &LocationRef, error: impl Error) {
        self.errors.push((location.to_owned(), error.to_string()));
    }

    pub fn into_result(self) -> Result<()> {
        let mut errors = InfraErrors::new();
        let mut cst_cache = HashMap::<PathBuf, NodeRef>::new();

        for (location, message) in self.errors {
            let (file_path, path) = location.flatten();

            let cst = cst_cache.entry(file_path.to_owned()).or_insert_with(|| {
                let source = file_path.read_to_string().unwrap();

                Parser::run_parser(&file_path, &source)
                    .unwrap_or_else(|_| panic!("File cannot be parsed: {file_path:?}"))
            });

            let mut current_node = cst.as_ref();
            for location in path.into_iter().rev() {
                current_node = match location.as_ref() {
                    Location::Root { .. } => {
                        unreachable!("Should already be flattened.")
                    }
                    Location::Field { field, .. } => current_node.field(field),
                    Location::Index { index, .. } => current_node.index(*index),
                };
            }

            let range = current_node.range().to_owned();
            errors.push(file_path, range, message);
        }

        errors.into_result()
    }
}
