use std::path::PathBuf;

use solar::ast::{self};
use solar::interface::Session;
use solar::parse::Parser;

use crate::dataset::SolidityProject;

pub fn run(project: &SolidityProject) {
    go(project, false);
}

pub fn test(project: &SolidityProject) -> usize {
    go(project, true)
}

fn go(project: &SolidityProject, count: bool) -> usize {
    // From Solar's docs: Create a new session with a buffer emitter.
    // This is required to capture the emitted diagnostics and to return them at the end.
    let sess = Session::builder()
        .with_buffer_emitter(solar::interface::ColorChoice::Auto)
        .build();

    let source_map = sess.source_map();

    for (fname, source) in &project.sources {
        let _ = source_map
            .new_source_file(PathBuf::from(fname), source)
            .unwrap();
    }

    let mut result = 0;
    // Enter the context and parse the file.
    let _ = sess.enter(|| -> solar::interface::Result<()> {
        // Use one arena for the entire parsing
        let arena = ast::Arena::new();

        for fname in project.sources.keys() {
            let mut parser = Parser::from_file(&sess, &arena, &PathBuf::from(fname))?;

            let unit = parser.parse_file().map_err(|e| e.emit())?;

            if count {
                result += unit.count_contracts();
            }
        }

        Ok(())
    });

    // There shouldn't be any error (or panic)
    sess.emitted_errors().unwrap().unwrap();
    result
}
