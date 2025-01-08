// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::rc::Rc;

use crate::wasm_crate::utils::{define_wrapper, FromFFI, IntoFFI};

mod ffi {
    pub use crate::wasm_crate::bindgen::exports::nomic_foundation::slang::cst::{
        Cursor, NonterminalNode, TextRange,
    };
    pub use crate::wasm_crate::bindgen::exports::nomic_foundation::slang::parser::{
        Guest, GuestParseError, GuestParseOutput, GuestParser, NonterminalKind, ParseError,
        ParseErrorBorrow, ParseOutput, ParseOutputBorrow, Parser, ParserBorrow,
    };
}

mod rust {
    pub use crate::rust_crate::parser::{ParseError, ParseOutput, Parser};
}

impl ffi::Guest for crate::wasm_crate::World {
    type Parser = ParserWrapper;
    type ParseError = ParseErrorWrapper;
    type ParseOutput = ParseOutputWrapper;
}

//================================================
//
// resource parser
//
//================================================

define_wrapper! { Parser {
    fn root_kind() -> ffi::NonterminalKind {
        rust::Parser::ROOT_KIND._into_ffi()
    }

    fn create(language_version: String) -> Result<ffi::Parser, String> {
        semver::Version::parse(&language_version)
            .map_err(|_| format!("Invalid semantic version: '{language_version}'"))
            .and_then(|version| rust::Parser::create(version).map_err(|e| e.to_string()))
            .map(IntoFFI::_into_ffi)
    }

    fn language_version(&self) -> String {
        self._borrow_ffi().language_version().to_string()
    }

    fn parse(&self, kind: ffi::NonterminalKind, input: String) -> ffi::ParseOutput {
        self._borrow_ffi().parse(kind._from_ffi(), &input)._into_ffi()
    }
} }

//================================================
//
// resource parse-error
//
//================================================

define_wrapper! { ParseError {
    fn text_range(&self) -> ffi::TextRange {
        self._borrow_ffi().text_range()._into_ffi()
    }

    fn message(&self) -> String {
        self._borrow_ffi().message()
    }
} }

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
