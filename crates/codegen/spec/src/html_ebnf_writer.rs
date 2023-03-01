use std::{collections::HashMap, io::Write};

use codegen_ebnf::ebnf_writer::{EBNFWriter, TokenKind};

use codegen_schema::types::{
    grammar::Grammar,
    production::{Production, ProductionRef},
};

pub struct SpecProductionContext<'a> {
    pub grammar: &'a Grammar,
    pub productions_location: HashMap<String, String>,
}

pub struct HTMLEBNFWriter<'a, T: Write> {
    pub w: T,
    pub context: &'a SpecProductionContext<'a>,
}

impl<T: Write> EBNFWriter for HTMLEBNFWriter<'_, T> {
    fn write_prelude(&mut self) {
        write!(self.w, "<pre style=\"white-space: pre-wrap;\"><code>").unwrap();
    }

    fn write_postlude(&mut self) {
        write!(self.w, "</code></pre>").unwrap();
        writeln!(self.w).unwrap();
    }

    fn write_line_break(&mut self) {
        write!(self.w, "<br/>").unwrap();
    }

    fn write_token(&mut self, kind: TokenKind, value: &str) {
        write!(
            self.w,
            "<span style=\"color: var(--md-code-hl-{:?}-color);\">{}</span>",
            kind, value,
        )
        .unwrap();
    }

    fn write_production_definition(&mut self, production_name: &str) {
        let link = production_hash_link(production_name);

        let display_name =
            production_display_name(&self.context.grammar.productions, production_name);

        self.write_keyword(&format!("<span id=\"{}\">{}</span>", link, display_name));
    }

    fn write_production_reference(&mut self, production_name: &str) {
        let link = production_hash_link(production_name);

        let display_name =
            production_display_name(&self.context.grammar.productions, production_name);

        let location = self
            .context
            .productions_location
            .get(production_name)
            .cloned()
            .unwrap();

        self.write_keyword(&format!(
            "<a href=\"{}#{}\">{}</a>",
            location, link, display_name
        ));
    }

    fn write_local_production_reference(&mut self, parent_name: &str, production_name: &str) {
        let link = production_hash_link(production_name);

        let display_name =
            production_display_name(&self.context.grammar.productions, production_name);

        let location = self
            .context
            .productions_location
            .get(parent_name)
            .cloned()
            .unwrap();

        self.write_keyword(&format!(
            "<a href=\"{}#{}\">{}</a>",
            location, link, display_name
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

pub fn production_hash_link(name: &str) -> String {
    format!("{}Production", name)
}
