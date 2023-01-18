use std::fmt::Write;

use codegen_schema::types::{
    grammar::Grammar,
    productions::{Production, ProductionKind, ProductionVersioning},
};

use super::expression::ExpressionEBNFPrivateExtensions;

pub trait ProductionEBNFGeneratorExtensions {
    fn generate_ebnf(&self, grammar: &Grammar) -> Vec<String>;
}

pub(crate) trait ProductionEBNFPrivateExtensions {
    fn ebnf_display_name(&self) -> String;
}

impl ProductionEBNFGeneratorExtensions for Production {
    fn generate_ebnf(&self, grammar: &Grammar) -> Vec<String> {
        match &self.versioning {
            ProductionVersioning::Unversioned(expression) => {
                let mut w = String::new();
                write!(w, "{} = ", self.ebnf_display_name()).unwrap();
                expression.generate_ebnf(grammar, &mut w);
                write!(w, ";").unwrap();
                vec![w]
            }
            ProductionVersioning::Versioned(versions) => versions
                .iter()
                .map(|(version, expr)| {
                    let mut w = String::new();
                    write!(w, "(* {} *) {} = ", version, self.ebnf_display_name()).unwrap();
                    expr.generate_ebnf(grammar, &mut w);
                    write!(w, ";").unwrap();
                    w
                })
                .collect(),
        }
    }
}

impl ProductionEBNFPrivateExtensions for Production {
    fn ebnf_display_name(&self) -> String {
        if self.kind == ProductionKind::Token {
            format!("«{}»", self.name)
        } else {
            self.name.clone()
        }
    }
}
