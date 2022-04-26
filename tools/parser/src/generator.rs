use std::{
    io::Write,
    process::{Command, Stdio},
};

use anyhow::Result;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::ebnf_impl::*;

fn generate_expression(expression: &Expression) -> TokenStream {
    match expression {
        Expression::End => quote!(todo()),
        Expression::Any => quote!(todo()),
        Expression::Repeat(RepeatCount::ZeroOrOne, expr) => {
            let expr = generate_expression(expr);
            quote!( #expr.or_not())
        }
        Expression::Repeat(RepeatCount::ZeroOrMore, expr) => {
            let expr = generate_expression(expr);
            quote!( #expr.repeated())
        }
        Expression::Repeat(RepeatCount::OneOrMore, expr) => {
            let expr = generate_expression(expr);
            quote!( #expr.repeated().at_least(1))
        }
        Expression::Choice(choices) => {
            let choices = choices.iter().map(generate_expression).collect::<Vec<_>>();
            quote!(choice((#(#choices),*)))
        }
        Expression::Sequence(exprs) => {
            let mut exprs = exprs.iter().map(generate_expression).collect::<Vec<_>>();
            let first = exprs.remove(0);
            quote!(#first #(.then(#exprs))*)
        }
        Expression::Difference(_, _) => quote!(todo()),
        Expression::Chars(chars) => quote!(just(#chars)),
        Expression::Identifier(ident) => {
            let id = format_ident!("{}", ident);
            quote!(#id)
        }
        Expression::CharSet(negated, elements) => {
            let chars = elements.iter().map(|element| match element {
                CharSetElement::Char(c) => quote!( c == #c ),
                CharSetElement::Range('a', 'z') => quote!(c.is_ascii_lowercase()),
                CharSetElement::Range('A', 'Z') => quote!(c.is_ascii_uppercase()),
                CharSetElement::Range('0', '9') => quote!(c.is_ascii_digit()),
                CharSetElement::Range(from, to) => {
                    quote!( (#from.as_u32() <= c.as_u32() && c.as_u32() <= #to.as_u32()) )
                }
            });
            if *negated {
                quote!(filter(|c| !(#(#chars)||*)))
            } else {
                quote!(filter(|c| #(#chars)||*))
            }
        }
    }
}

pub fn generate(productions: Vec<Production>) {
    let rules = productions
        .iter()
        .rev()
        .map(|p| {
            let name = format_ident!("{}", p.name);
            let expression = generate_expression(&p.expr);
            // let doc = format!("Production: {}", "blah ::= foo | bar*");
            quote!(
                // #[doc = #doc]
                let #name = #expression.ignored();
            )
        })
        .collect::<Vec<_>>();
    let src = quote!(
        pub fn parse() {
            #(#rules)*
        }
    );

    println!("{}", rustfmt(src.to_string()).unwrap());

    // println!("{:#?}", productions);
}

fn rustfmt(source: String) -> Result<String> {
    let mut child = Command::new("rustfmt")
        .arg("--edition")
        .arg("2021")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(source.as_bytes())
            .expect("Failed to write to stdin");
    });

    let output = child.wait_with_output().expect("Failed to read stdout");

    Ok(String::from_utf8(output.stdout)?)
}
