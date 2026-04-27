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

mod semantic {
    use super::*;

    mod duplicate_mutability_keyword {
        use super::*;

        #[test]
        fn function_types() -> Result<()> {
            run("semantic/duplicate_mutability_keyword", "function_types")
        }

        #[test]
        fn functions() -> Result<()> {
            run("semantic/duplicate_mutability_keyword", "functions")
        }
    }
}
