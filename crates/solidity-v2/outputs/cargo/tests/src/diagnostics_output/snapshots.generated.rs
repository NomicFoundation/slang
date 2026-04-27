// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::diagnostics_output::runner::run;

mod compilation {
    use super::*;

    #[test]
    fn missing_file() -> Result<()> {
        run("compilation", "missing_file")
    }

    #[test]
    fn unresolved_import() -> Result<()> {
        run("compilation", "unresolved_import")
    }
}
