use std::{collections::HashMap, io::Write};

use codegen_ebnf::ebnf_writer::{EBNFWriter, TokenKind};
use codegen_schema::types::{
    grammar::Grammar,
    production::{Production, ProductionRef},
};

pub struct CodeCommentEBNFWriter<'a, T: Write> {
    pub w: T,
    pub grammar: &'a Grammar,
}

impl<T: Write> EBNFWriter for CodeCommentEBNFWriter<'_, T> {
    fn write_line_start(&mut self) {
        write!(self.w, "// ").unwrap();
    }

    fn write_line_break(&mut self) {
        writeln!(self.w).unwrap();
    }

    fn write_token(&mut self, _kind: TokenKind, value: &str) {
        write!(self.w, "{}", value).unwrap();
    }

    fn write_production_definition(&mut self, production_name: &str) {
        self.write_keyword(&production_display_name(
            &self.grammar.productions,
            production_name,
        ));
    }

    fn write_production_reference(&mut self, production_name: &str) {
        self.write_keyword(&production_display_name(
            &self.grammar.productions,
            production_name,
        ));
    }
}

pub fn production_display_name(productions: &HashMap<String, ProductionRef>, name: &str) -> String {
    if productions.contains_key(name) && matches!(*productions[name], Production::Scanner { .. }) {
        format!("«{}»", name)
    } else {
        name.to_string()
    }
}
