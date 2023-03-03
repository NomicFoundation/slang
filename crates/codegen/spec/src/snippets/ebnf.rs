use std::fmt::Write;

use codegen_ebnf::ebnf_writer::{EBNFWriter, TokenKind};
use codegen_schema::types::{grammar::Grammar, production::Production};
use inflector::Inflector;

use crate::grammar::GrammarSpecPrivateExtensions;

pub struct HTMLWriter<'a> {
    w: String,
    first_line: bool,
    grammar: &'a Grammar,
}

impl<'a> HTMLWriter<'a> {
    pub fn new(grammar: &'a Grammar) -> Self {
        return Self {
            w: String::new(),
            first_line: true,
            grammar,
        };
    }

    pub fn to_string(self) -> String {
        return self.w;
    }
}

impl EBNFWriter for HTMLWriter<'_> {
    fn write_prelude(&mut self) {
        writeln!(self.w, "<pre style=\"white-space: nowrap\">").unwrap();
        writeln!(self.w, "  <code>").unwrap();
    }

    fn write_postlude(&mut self) {
        writeln!(self.w, "  </code>").unwrap();
        writeln!(self.w, "</pre>").unwrap();
    }

    fn write_line_start(&mut self) {
        if self.first_line {
            self.first_line = false;
        } else {
            writeln!(self.w).unwrap();
        }
    }

    fn write_line_end(&mut self) {
        writeln!(self.w, "    <br/>").unwrap();
    }

    fn write_global_definition(&mut self, name: &str) {
        self.write_keyword(&format!(
            "<span id=\"{id}\">{display_name}</span>",
            id = global_id(name),
            display_name = self.display_name(name),
        ));
    }

    fn write_local_definition(&mut self, parent_name: &str, name: &str) {
        self.write_keyword(&format!(
            "<span id=\"{id}\">{display_name}</span>",
            id = local_id(parent_name, name),
            display_name = self.display_name(name),
        ));
    }

    fn write_global_reference(&mut self, name: &str) {
        let (section, topic) = self.grammar.locate_production(name);
        self.write_keyword(&format!(
            "<a class=\"slang-global-ebnf-link\" href=\"../../{section}/{topic}/#{id}\">{display_name}</a>",
            section = section.path,
            topic = topic.path,
            id = global_id(name),
            display_name = self.display_name(name),
        ));
    }

    fn write_local_reference(&mut self, parent_name: &str, name: &str) {
        self.write_keyword(&format!(
            "<a href=\"#{id}\">{display_name}</a>",
            id = local_id(parent_name, name),
            display_name = self.display_name(name),
        ));
    }

    fn write_token(&mut self, kind: TokenKind, value: &str) {
        writeln!(
            self.w,
            "    <span style=\"color: var(--md-code-hl-{kind}-color);\">{value}</span>",
            kind = format!("{:?}", kind).to_lowercase(),
        )
        .unwrap();
    }
}

impl HTMLWriter<'_> {
    fn display_name(&self, name: &str) -> String {
        if let Some(production) = self.grammar.productions.get(name) {
            if let Production::Scanner { .. } = production.as_ref() {
                return format!("«{name}»");
            }
        }

        return name.to_owned();
    }
}

fn global_id(name: &str) -> String {
    return format!("{name}-production", name = name.to_kebab_case());
}

fn local_id(parent_name: &str, name: &str) -> String {
    return format!(
        "{parent_name}-{name}-production",
        parent_name = parent_name.to_kebab_case(),
        name = name.to_kebab_case()
    );
}
