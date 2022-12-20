mod nodes;
mod tree_view;

use std::{self, fmt::Write};

use anyhow::Result;
use solidity_rust_lib::generated::language::ParserOutput;

pub trait ParserOutputTestSnapshotExtensions {
    fn to_test_snapshot(&self, source_id: &str, source: &str) -> Result<String>;
}

impl ParserOutputTestSnapshotExtensions for ParserOutput {
    fn to_test_snapshot(&self, source_id: &str, source: &str) -> Result<String> {
        let mut snapshot = String::new();

        if self.error_count() == 0 {
            writeln!(&mut snapshot, "Errors: 0")?;
            writeln!(&mut snapshot)?;
        } else {
            for report in self.errors_as_strings(source_id, source, /* with_colour */ false) {
                writeln!(&mut snapshot, "{report}")?;
                writeln!(&mut snapshot)?;
            }
        }

        if let Some(parse_tree) = self.parse_tree() {
            writeln!(
                &mut snapshot,
                "{}",
                nodes::TestNode::from_cst(&parse_tree).to_tree_view(source)?
            )?;
        } else {
            writeln!(&mut snapshot, "Tree: null")?;
        }

        return Ok(snapshot);
    }
}
