use std::path::PathBuf;

use solar::ast::{self, SourceUnit};
use solar::interface::Session;
use solar::parse::Parser;

use crate::dataset::SolidityProject;

pub fn run(project: &SolidityProject) {
    go(project, (), |_: &SourceUnit<'_>, (): ()| ());
}

pub fn test(project: &SolidityProject) -> usize {
    go(project, 0, |unit: &SourceUnit<'_>, prev: usize| {
        prev + unit.count_contracts()
    })
}

fn go<T: Copy, F>(project: &SolidityProject, initial: T, fold: F) -> T
where
    F: Fn(&SourceUnit<'_>, T) -> T,
{
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

    let mut result = initial;
    // Enter the context and parse the file.
    let _ = sess.enter(|| -> solar::interface::Result<()> {
        // Use one arena for the entire parsing
        let arena = ast::Arena::new();

        for fname in project.sources.keys() {
            let mut parser = Parser::from_file(&sess, &arena, &PathBuf::from(fname))?;

            let unit = parser.parse_file().map_err(|e| e.emit())?;
            result = fold(&unit, result);
        }

        Ok(())
    });

    // There shouldn't be any error (or panic)
    sess.emitted_errors().unwrap().unwrap();
    result
}
