use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::schema::*;

pub fn generate(grammar: &Grammar, _output_path: &PathBuf) {
    let mut w = File::create(_output_path).expect("Unable to create file");

    let mut first = true;
    for p in grammar.productions.iter().flat_map(|(_, p)| p) {
        if let Some(expr) = &p.expr {
            if first {
                first = false;
            } else {
                writeln!(w).unwrap();
            }
            write!(w, "{} =", p.name).unwrap();
            expr.generate_ebnf(&mut w);
        }
        for (version, expr) in &p.versions {
            if first {
                first = false;
            } else {
                writeln!(w).unwrap();
            }
            write!(w, "/* {} */ {} =", version, p.name).unwrap();
            expr.generate_ebnf(&mut w);
        }
        writeln!(w, " ;").unwrap();
    }
}

impl Expression {
    fn precedence_ebnf(&self) -> u8 {
        match self.ebnf {
            EBNF::End
            | EBNF::Any
            | EBNF::ZeroOrMore(..)
            | EBNF::Optional(..)
            | EBNF::Terminal(..)
            | EBNF::Reference(..)
            | EBNF::Range { .. } => 0,
            EBNF::Not(..) => 1,
            EBNF::Difference { .. } => 2,
            EBNF::Sequence(..) | EBNF::OneOrMore(..) => 3,
            EBNF::Choice(..) => 4,
        }
    }

    fn generate_ebnf_subexpression<T: Write>(&self, w: &mut T, expr: &ExpressionRef) {
        if self.precedence_ebnf() < expr.precedence_ebnf() {
            write!(w, " (").unwrap();
            expr.generate_ebnf(w);
            write!(w, " )").unwrap();
        } else {
            expr.generate_ebnf(w);
        }
    }

    fn generate_ebnf<T: Write>(&self, w: &mut T) {
        match &self.ebnf {
            EBNF::End => write!(w, " $").unwrap(),

            EBNF::Any => write!(w, " .").unwrap(),

            EBNF::ZeroOrMore(expr) => {
                write!(w, " {{").unwrap();
                expr.generate_ebnf(w);
                write!(w, " }}").unwrap();
            }

            EBNF::OneOrMore(expr) => {
                expr.generate_ebnf(w);
                write!(w, " {{").unwrap();
                expr.generate_ebnf(w);
                write!(w, " }}").unwrap();
            }

            EBNF::Optional(expr) => {
                write!(w, " [").unwrap();
                expr.generate_ebnf(w);
                write!(w, " ]").unwrap();
            }

            EBNF::Not(expr) => {
                write!(w, " ¬").unwrap();
                self.generate_ebnf_subexpression(w, expr);
            }

            EBNF::Choice(exprs) => {
                let mut first = true;
                for expr in exprs {
                    if first {
                        first = false;
                    } else {
                        write!(w, " |").unwrap();
                    }
                    self.generate_ebnf_subexpression(w, expr);
                }
            }

            EBNF::Sequence(exprs) => {
                for expr in exprs {
                    self.generate_ebnf_subexpression(w, expr);
                }
            }

            EBNF::Reference(name) => {
                write!(w, " {}", name).unwrap();
            }

            EBNF::Terminal(string) => {
                write!(w, " '").unwrap();
                for c in string.chars() {
                    if c == '\'' || c == '\\' {
                        write!(w, "\\{}", c).unwrap();
                    } else if c.is_ascii_graphic() || c == '¬' || c == '…' {
                        write!(w, "{}", c).unwrap();
                    } else {
                        write!(w, "{}", c.escape_unicode().to_string()).unwrap();
                    }
                }
                write!(w, "'").unwrap();
            }

            EBNF::Difference(EBNFDifference {
                minuend,
                subtrahend,
            }) => {
                self.generate_ebnf_subexpression(w, minuend);
                write!(w, " -").unwrap();
                self.generate_ebnf_subexpression(w, subtrahend);
            }

            EBNF::Range(EBNFRange { from, to }) => {
                write!(w, " '").unwrap();
                if from.is_ascii_graphic() || *from == '¬' || *from == '…' {
                    write!(w, "{}", from).unwrap()
                } else {
                    write!(w, "{}", from.escape_unicode().to_string()).unwrap()
                }
                write!(w, "'…'").unwrap();
                if to.is_ascii_graphic() || *to == '¬' || *to == '…' {
                    write!(w, "{}", to).unwrap()
                } else {
                    write!(w, "{}", to.escape_unicode().to_string()).unwrap()
                }
                write!(w, "'").unwrap();
            }
        }
    }
}
