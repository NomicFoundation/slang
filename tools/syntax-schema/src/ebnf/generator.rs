use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use semver::Version;

use crate::schema::*;

impl Grammar {
    pub fn generate_ebnf(&self, output_path: &PathBuf) {
        let mut w = File::create(output_path).expect("Unable to create file");

        let zero_version = Version::parse("0.0.0").unwrap();
        let mut first = true;
        for p in self.productions.iter().flat_map(|(_, p)| p) {
            if p.versions.len() == 1 && p.versions.get(&zero_version).is_some() {
                if first {
                    first = false;
                } else {
                    writeln!(w).unwrap();
                }
                write!(w, "{} = ", p.print_name()).unwrap();
                p.versions[&zero_version].generate(self, &mut w);
            } else {
                for (version, expr) in &p.versions {
                    if first {
                        first = false;
                    } else {
                        writeln!(w).unwrap();
                    }
                    write!(w, "/* {} */ {} = ", version, p.print_name()).unwrap();
                    expr.generate(self, &mut w);
                }
            }
            writeln!(w, ";").unwrap();
        }
    }
}

trait EBNFProduction {
    fn print_name(&self) -> String;
}

impl EBNFProduction for Production {
    fn print_name(&self) -> String {
        if self.is_token {
            format!("«{}»", self.name)
        } else {
            self.name.clone()
        }
    }
}

trait EBNFExpression {
    fn generate<T: Write>(&self, grammar: &Grammar, w: &mut T);

    fn generate_subexpression<T: Write>(&self, grammar: &Grammar, w: &mut T, expr: &ExpressionRef);
}

impl EBNFExpression for Expression {
    fn generate_subexpression<T: Write>(&self, grammar: &Grammar, w: &mut T, expr: &ExpressionRef) {
        if self.precedence() < expr.precedence() {
            write!(w, "( ").unwrap();
            expr.generate(grammar, w);
            write!(w, ") ").unwrap();
        } else {
            expr.generate(grammar, w);
        }
    }

    fn generate<T: Write>(&self, grammar: &Grammar, w: &mut T) {
        fn write_char<T: Write>(w: &mut T, c: char) {
            if c == '\'' || c == '\\' {
                write!(w, "\\{}", c).unwrap();
            } else if c.is_ascii_graphic()
                || c == '¬'
                || c == '…'
                || c == '«'
                || c == '»'
                || c == '∞'
            {
                write!(w, "{}", c).unwrap();
            } else {
                write!(w, "{}", c.escape_unicode().to_string()).unwrap();
            }
        }

        match &self.ebnf {
            EBNF::End => write!(w, "$ ").unwrap(),

            EBNF::Repeat(EBNFRepeat {
                min,
                max,
                expr,
                separator,
            }) => match (min, max) {
                (0, None) => {
                    write!(w, "{{ ").unwrap();
                    expr.generate(grammar, w);
                    if let Some(separator) = separator {
                        write!(w, "/ ").unwrap();
                        separator.generate(grammar, w);
                    }
                    write!(w, "}} ").unwrap();
                }
                (0, Some(1)) => {
                    write!(w, "[ ").unwrap();
                    expr.generate(grammar, w);
                    write!(w, "] ").unwrap();
                }
                _ => {
                    if *min != 0 {
                        write!(w, "{}", min).unwrap();
                    }
                    write!(w, "…").unwrap();
                    if let Some(max) = max {
                        write!(w, "{}", max).unwrap();
                    }
                    write!(w, "*{{ ").unwrap();
                    expr.generate(grammar, w);
                    if let Some(separator) = separator {
                        write!(w, "/ ").unwrap();
                        separator.generate(grammar, w);
                    }
                    write!(w, "}} ").unwrap();
                }
            },

            EBNF::Not(expr) => {
                write!(w, "¬").unwrap();
                self.generate_subexpression(grammar, w, expr);
            }

            EBNF::Choice(exprs) => {
                let mut first = true;
                for expr in exprs {
                    if first {
                        first = false;
                    } else {
                        write!(w, "| ").unwrap();
                    }
                    self.generate_subexpression(grammar, w, expr);
                }
            }

            EBNF::Sequence(exprs) => {
                for expr in exprs {
                    self.generate_subexpression(grammar, w, expr);
                }
            }

            EBNF::Reference(name) => {
                if let Some(production) = grammar.get_production(name) {
                    write!(w, "{} ", production.print_name()).unwrap();
                } else {
                    panic!("Cannot find {} production", name)
                }
            }

            EBNF::Terminal(string) => {
                write!(w, "'").unwrap();
                for c in string.chars() {
                    write_char(w, c);
                }
                write!(w, "' ").unwrap();
            }

            EBNF::Difference(EBNFDifference {
                minuend,
                subtrahend,
            }) => {
                self.generate_subexpression(grammar, w, minuend);
                write!(w, "- ").unwrap();
                self.generate_subexpression(grammar, w, subtrahend);
            }

            EBNF::Range(EBNFRange { from, to }) => {
                write!(w, "'").unwrap();
                write_char(w, *from);
                write!(w, "'…'").unwrap();
                write_char(w, *to);
                write!(w, "' ").unwrap();
            }
        }
    }
}
