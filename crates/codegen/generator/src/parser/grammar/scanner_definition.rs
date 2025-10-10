use std::collections::BTreeSet;
use std::fmt::Debug;
use std::rc::Rc;

use language_definition::model::{self, Identifier};
use proc_macro2::TokenStream;

use crate::parser::grammar::{GrammarVisitor, Visitable};

pub trait ScannerDefinition: Debug {
    /// A unique identifier for this scanner.
    fn name(&self) -> &Identifier;
    /// Quotes the matching Rust scanner code.
    fn to_scanner_code(&self) -> TokenStream;
    /// A set of literals that this scanner can match.
    ///
    /// If the scanner matches more than just (a union of) literals, this method should return `None`.
    fn literals(&self) -> Option<BTreeSet<String>>;
    /// For which language version the scanner is defined.
    fn version_specifier(&self) -> Option<&model::VersionSpecifier> {
        None
    }
}

pub type ScannerDefinitionRef = Rc<dyn ScannerDefinition>;

impl Visitable for ScannerDefinitionRef {
    fn accept_visitor<V: GrammarVisitor>(&self, visitor: &mut V) {
        visitor.scanner_definition_enter(self);
    }
}
