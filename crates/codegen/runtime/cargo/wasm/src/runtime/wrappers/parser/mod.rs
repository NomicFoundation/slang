use std::rc::Rc;

use crate::wasm_crate::utils::{define_wrapper, FromFFI, IntoFFI};

mod ffi {
    pub use crate::wasm_crate::bindgen::exports::nomic_foundation::slang::cst::{
        Cursor, NonterminalNode,
    };
    pub use crate::wasm_crate::bindgen::exports::nomic_foundation::slang::parser::{
        Guest, GuestParseOutput, GuestParser, NonterminalKind, ParseError, ParseOutput,
        ParseOutputBorrow, Parser, ParserBorrow,
    };
}

mod rust {
    pub use crate::rust_crate::parser::{ParseError, ParseOutput, Parser};
}

impl ffi::Guest for crate::wasm_crate::World {
    type Parser = ParserWrapper;
    type ParseOutput = ParseOutputWrapper;
}

//================================================
//
// resource parser
//
//================================================

define_wrapper! { Parser {
    fn create(language_version: String) -> Result<ffi::Parser, String> {
        semver::Version::parse(&language_version)
            .map_err(|_| format!("Invalid semantic version: '{language_version}'"))
            .and_then(|version| rust::Parser::create(version).map_err(|e| e.to_string()))
            .map(IntoFFI::_into_ffi)
    }

    fn language_version(&self) -> String {
        self._borrow_ffi().language_version().to_string()
    }

    fn parse_file_contents(&self, input: String) -> ffi::ParseOutput {
        self._borrow_ffi().parse_file_contents(&input)._into_ffi()
    }

    fn parse_nonterminal(&self, kind: ffi::NonterminalKind, input: String) -> ffi::ParseOutput {
        self._borrow_ffi().parse_nonterminal(kind._from_ffi(), &input)._into_ffi()
    }
} }

//================================================
//
// record parse-error
//
//================================================

impl IntoFFI<ffi::ParseError> for rust::ParseError {
    #[inline]
    fn _into_ffi(self) -> ffi::ParseError {
        ffi::ParseError {
            message: self.message(),
            text_range: self.text_range()._into_ffi(),
        }
    }
}

//================================================
//
// resource parse-output
//
//================================================

define_wrapper! { ParseOutput {
    fn tree(&self) -> ffi::NonterminalNode {
        Rc::clone(self._borrow_ffi().tree())._into_ffi()
    }

    fn errors(&self) -> Vec<ffi::ParseError> {
        self._borrow_ffi().errors().iter().map(|e| e.clone()._into_ffi()).collect()
    }

    fn is_valid(&self) -> bool {
        self._borrow_ffi().is_valid()
    }

    fn create_tree_cursor(&self) -> ffi::Cursor {
        self._borrow_ffi().create_tree_cursor()._into_ffi()
    }
} }
