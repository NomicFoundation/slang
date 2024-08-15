use crate::wit::utils::enum_to_enum;

mod ffi {
    pub use crate::wit::interface::exports::nomic_foundation::slang::kinds::{
        EdgeLabel, NonterminalKind, TerminalKind,
    };
}

mod rust {
    pub use crate::kinds::{EdgeLabel, NonterminalKind, TerminalKind};
}

//================================================
//
// enum nonterminal-kind
//
//================================================

enum_to_enum!(NonterminalKind);

//================================================
//
// enum terminal-kind
//
//================================================

enum_to_enum!(TerminalKind);

//================================================
//
// enum edge-label
//
//================================================

enum_to_enum!(EdgeLabel);
