use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::schema::*;

impl Production {
    fn ebnf_name(&self) -> String {
        if self.is_token {
            format!("«{}»", self.name)
        } else {
            self.name.clone()
        }
    }
}

pub fn generate(grammar: &Grammar, output_path: &PathBuf) {
    let mut w = File::create(output_path).expect("Unable to create file");

    let mut first = true;
    for p in grammar.productions.iter().flat_map(|(_, p)| p) {
        if let Some(expr) = &p.expr {
            if first {
                first = false;
            } else {
                writeln!(w).unwrap();
            }
            write!(w, "{} = ", p.ebnf_name()).unwrap();
            expr.generate_ebnf(grammar, &mut w);
        }
        for (version, expr) in &p.versions {
            if first {
                first = false;
            } else {
                writeln!(w).unwrap();
            }
            write!(w, "/* {} */ {} = ", version, p.ebnf_name()).unwrap();
            expr.generate_ebnf(grammar, &mut w);
        }
        writeln!(w, ";").unwrap();
    }
}

impl Expression {
    fn precedence_ebnf(&self) -> u8 {
        match self.ebnf {
            EBNF::End
            | EBNF::Repeat(..)
            | EBNF::Terminal(..)
            | EBNF::Reference(..)
            | EBNF::Range { .. } => 0,
            EBNF::Not(..) => 1,
            EBNF::Difference { .. } => 2,
            EBNF::Sequence(..) => 3,
            EBNF::Choice(..) => 4,
        }
    }

    fn generate_ebnf_subexpression<T: Write>(
        &self,
        grammar: &Grammar,
        w: &mut T,
        expr: &ExpressionRef,
    ) {
        if self.precedence_ebnf() < expr.precedence_ebnf() {
            write!(w, "( ").unwrap();
            expr.generate_ebnf(grammar, w);
            write!(w, ") ").unwrap();
        } else {
            expr.generate_ebnf(grammar, w);
        }
    }

    fn generate_ebnf<T: Write>(&self, grammar: &Grammar, w: &mut T) {
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
                    expr.generate_ebnf(grammar, w);
                    if let Some(separator) = separator {
                        write!(w, "/ ").unwrap();
                        separator.generate_ebnf(grammar, w);
                    }
                    write!(w, "}} ").unwrap();
                }
                (0, Some(1)) => {
                    write!(w, "[ ").unwrap();
                    expr.generate_ebnf(grammar, w);
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
                    expr.generate_ebnf(grammar, w);
                    if let Some(separator) = separator {
                        write!(w, "/ ").unwrap();
                        separator.generate_ebnf(grammar, w);
                    }
                    write!(w, "}} ").unwrap();
                }
            },

            EBNF::Not(expr) => {
                write!(w, "¬").unwrap();
                self.generate_ebnf_subexpression(grammar, w, expr);
            }

            EBNF::Choice(exprs) => {
                let mut first = true;
                for expr in exprs {
                    if first {
                        first = false;
                    } else {
                        write!(w, "| ").unwrap();
                    }
                    self.generate_ebnf_subexpression(grammar, w, expr);
                }
            }

            EBNF::Sequence(exprs) => {
                for expr in exprs {
                    self.generate_ebnf_subexpression(grammar, w, expr);
                }
            }

            EBNF::Reference(name) => {
                if let Some(production) = grammar.get_production(name) {
                    write!(w, "{} ", production.ebnf_name()).unwrap();
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
                self.generate_ebnf_subexpression(grammar, w, minuend);
                write!(w, "- ").unwrap();
                self.generate_ebnf_subexpression(grammar, w, subtrahend);
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
