use semver::Version;

use crate::schema::*;

use std::fmt::Write;

impl Production {
    pub(crate) fn ebnf_display_name(&self) -> String {
        if self.is_token() {
            format!("«{}»", self.name)
        } else {
            self.name.clone()
        }
    }

    pub(crate) fn generate_ebnf(&self, grammar: &Grammar) -> Vec<String> {
        let zero_version = Version::parse("0.0.0").unwrap();

        if self.versions.len() == 1 && self.versions.get(&zero_version).is_some() {
            let mut w = String::new();
            write!(w, "{} = ", self.ebnf_display_name()).unwrap();
            self.versions[&zero_version].generate_ebnf(grammar, &mut w);
            write!(w, ";").unwrap();
            vec![w]
        } else {
            self.versions
                .iter()
                .map(|(version, expr)| {
                    let mut w = String::new();
                    write!(w, "(* {} *) {} = ", version, self.ebnf_display_name()).unwrap();
                    expr.generate_ebnf(grammar, &mut w);
                    write!(w, ";").unwrap();
                    w
                })
                .collect()
        }
    }
}
