use crate::schema::{
    EBNFDelimitedBy, EBNFDifference, EBNFRange, EBNFRepeat, EBNFSeparatedBy, Expression,
    ExpressionRef, Grammar, Production, EBNF,
};
use itertools::Itertools;
use semver::Version;
use std::{collections::HashMap, io::Write};

pub struct SpecProductionContext<'a> {
    pub grammar: &'a Grammar,
    pub productions_location: HashMap<String, String>,
}

pub fn write_production<T: Write>(
    w: &mut T,
    production: &Production,
    context: &SpecProductionContext,
) {
    write!(w, "<pre style=\"white-space: pre-wrap;\">").unwrap();
    write!(
        w,
        "<code id=\"{}\">",
        get_production_hash_link(&production.name)
    )
    .unwrap();

    let zero_version = Version::parse("0.0.0").unwrap();
    if production.versions.len() == 1 && production.versions.get(&zero_version).is_some() {
        write_version(w, &production, &production.versions[&zero_version], context);
    } else {
        for version in production.versions.keys().sorted() {
            write_token(w, TokenKind::comment, &format!("(* v{} *) ", version));
            write_version(w, &production, &production.versions[version], context);
        }
    }

    write!(w, "</code>").unwrap();
    write!(w, "</pre>").unwrap();

    writeln!(w).unwrap();
}

fn write_version<T: Write>(
    w: &mut T,
    production: &Production,
    expr: &Expression,
    context: &SpecProductionContext,
) {
    write_token(w, TokenKind::keyword, &get_production_name(production));
    write_token(w, TokenKind::operator, " = ");
    write_expression(w, expr, context);
    write_token(w, TokenKind::operator, ";");
    write!(w, "<br/>").unwrap();
}

fn write_expression<T: Write>(w: &mut T, expr: &Expression, context: &SpecProductionContext) {
    match &expr.ebnf {
        EBNF::End => {
            write_token(w, TokenKind::keyword, "$");
        }

        EBNF::ZeroOrMore(expr) => {
            write_token(w, TokenKind::operator, "{");
            write_expression(w, expr, context);
            write_token(w, TokenKind::operator, "}");
        }

        EBNF::OneOrMore(expr) => {
            write_token(w, TokenKind::constant, "1");
            write_token(w, TokenKind::operator, "…");
            write_expression(w, expr, context);
            write_token(w, TokenKind::operator, "}");
        }

        EBNF::Optional(expr) => {
            write_token(w, TokenKind::operator, "[");
            write_expression(w, expr, context);
            write_token(w, TokenKind::operator, "]");
        }

        EBNF::Repeat(EBNFRepeat { min, max, expr }) => {
            write_token(w, TokenKind::constant, &min.to_string());
            write_token(w, TokenKind::operator, "…");
            write_token(w, TokenKind::constant, &max.to_string());
            write_token(w, TokenKind::operator, "{");
            write_expression(w, expr, context);
            write_token(w, TokenKind::operator, "}");
        }

        EBNF::Not(sub_expr) => {
            write_token(w, TokenKind::operator, "¬");
            write_subexpression(w, expr, sub_expr, context);
        }

        EBNF::Choice(sub_exprs) => {
            let mut first = true;
            for sub_expr in sub_exprs {
                if first {
                    first = false;
                } else {
                    write_token(w, TokenKind::operator, " | ");
                }
                write_subexpression(w, expr, sub_expr, context);
            }
        }

        EBNF::SeparatedBy(EBNFSeparatedBy { expr, separator }) => {
            write_subexpression(w, expr, expr, context);
            write_token(w, TokenKind::operator, " { ");
            write_subexpression(w, expr, expr, context);
            write_token(w, TokenKind::string, &format_string_literal(separator));
            write_token(w, TokenKind::operator, " ");
            write_subexpression(w, expr, expr, context);
            write_token(w, TokenKind::operator, " }");
        }

        EBNF::DelimitedBy(EBNFDelimitedBy { open, expr, close }) => {
            write_token(w, TokenKind::string, &format_string_literal(open));
            write_token(w, TokenKind::operator, " ");
            write_subexpression(w, expr, expr, context);
            write_token(w, TokenKind::operator, " ");
            write_token(w, TokenKind::string, &format_string_literal(close));
        }

        EBNF::Sequence(sub_exprs) => {
            let mut first = true;
            for sub_expr in sub_exprs {
                if first {
                    first = false;
                } else {
                    write_token(w, TokenKind::operator, " ");
                }
                write_subexpression(w, expr, sub_expr, context);
            }
        }

        EBNF::Reference(name) => {
            let referenced = context.grammar.get_production(name);
            let location = context.productions_location.get(name).expect(name);

            write_token(
                w,
                TokenKind::keyword,
                &format!(
                    "<a href=\"{}#{}\">{}</a>",
                    location,
                    get_production_hash_link(name),
                    get_production_name(referenced.as_ref())
                ),
            );
        }

        EBNF::Terminal(string) => {
            write_token(w, TokenKind::string, &format_string_literal(string));
        }

        EBNF::Difference(EBNFDifference {
            minuend,
            subtrahend,
        }) => {
            write_subexpression(w, expr, minuend, context);
            write_token(w, TokenKind::operator, " - ");
            write_subexpression(w, expr, subtrahend, context);
        }

        EBNF::Range(EBNFRange { from, to }) => {
            write_token(
                w,
                TokenKind::string,
                &format_string_literal(&from.to_string()),
            );
            write_token(w, TokenKind::operator, "…");
            write_token(
                w,
                TokenKind::string,
                &format_string_literal(&to.to_string()),
            );
        }
    }
}

fn write_subexpression<T: Write>(
    w: &mut T,
    expr: &Expression,
    sub_expr: &ExpressionRef,
    context: &SpecProductionContext,
) {
    if expr.precedence() < sub_expr.precedence() {
        write_token(w, TokenKind::operator, "(");
        write_expression(w, sub_expr, context);
        write_token(w, TokenKind::operator, ")");
    } else {
        write_expression(w, sub_expr, context);
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
enum TokenKind {
    comment,
    keyword,
    constant,
    operator,
    string,
}

fn write_token<T: Write>(w: &mut T, kind: TokenKind, value: &str) {
    write!(
        w,
        "<span style=\"color: var(--md-code-hl-{:?}-color);\">{}</span>",
        kind, value,
    )
    .unwrap();
}

fn format_string_literal(value: &str) -> String {
    let delimiter = if value.len() == 1 {
        if value.contains("'") && !value.contains('"') {
            '"'
        } else {
            '\''
        }
    } else {
        if value.contains('"') && !value.contains("'") {
            '\''
        } else {
            '"'
        }
    };

    let formatted = value
        .chars()
        .map(|c| {
            if c == delimiter || c == '\\' {
                return format!("\\{}", c);
            } else if c.is_ascii_graphic() {
                return c.to_string();
            } else {
                return c.escape_unicode().to_string();
            }
        })
        .join("");

    return format!("{}{}{}", delimiter, formatted, delimiter);
}

fn get_production_name(production: &Production) -> String {
    if production.is_token() {
        format!("«{}»", production.name)
    } else {
        production.name.clone()
    }
}

fn get_production_hash_link(name: &str) -> String {
    return format!("{}Production", name);
}
