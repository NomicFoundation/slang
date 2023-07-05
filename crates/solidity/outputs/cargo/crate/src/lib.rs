mod generated;

mod _supress_binary_dependencies {
    // Below are dependencies used by the CLI "main.rs", but not the API `lib.rs`.
    // However, we need to add a fake usage to suppress Cargo warnings about unused dependencies.
    // This is a known issue, and we should remove this hack once there is a better solution from Cargo.
    // https://github.com/rust-lang/cargo/issues/1982
    #[cfg(feature = "cli")]
    mod cli {
        use anyhow as _;
        use clap as _;
        use serde_json as _;
    }

    // Make sure codegen runs before building for tests.
    #[cfg(test)]
    mod tests {
        use solidity_cargo_build as _;
    }
}

pub use public_api::*;

mod public_api {
    /*
     * __SLANG_PUBLIC_API_SYNC__ (please keep in sync across all other instances)
     */

    pub mod language {
        pub use crate::generated::language::{Error, Language};
    }

    pub mod syntax {
        pub mod nodes {
            pub use crate::generated::cst::{Node, RuleNode, TokenNode};
            pub use crate::generated::kinds::{RuleKind, TokenKind};
            pub use crate::generated::text_index::{TextIndex, TextRange, TextRangeExtensions};
        }

        pub mod parser {
            pub use crate::generated::kinds::ProductionKind;
            pub use crate::generated::parse_output::{ParseError, ParseOutput};
        }

        pub mod visitors {
            pub use crate::generated::cst_visitor::{
                Visitable, Visitor, VisitorEntryResponse, VisitorExitResponse,
            };
        }
    }
}
