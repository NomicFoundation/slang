use std::fmt::Write;

use codegen_schema::{Grammar, Production, ProductionVersions};

use super::expression::ExpressionEBNFPrivateExtensions;

pub trait ProductionEBNFGeneratorExtensions {
    fn generate_ebnf(&self, grammar: &Grammar) -> Vec<String>;
}

pub(crate) trait ProductionEBNFPrivateExtensions {
    fn ebnf_display_name(&self) -> String;
}

impl ProductionEBNFGeneratorExtensions for Production {
    fn generate_ebnf(&self, grammar: &Grammar) -> Vec<String> {
        match &self.versions {
            ProductionVersions::Unversioned(expression) => {
                let mut w = String::new();
                write!(w, "{} = ", self.ebnf_display_name()).unwrap();
                expression.generate_ebnf(grammar, &mut w);
                write!(w, ";").unwrap();
                vec![w]
            }
            ProductionVersions::Versioned(versions) => versions
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
        if self.is_token() {
            format!("«{}»", self.name)
        } else {
            self.name.clone()
        }
    }
}
