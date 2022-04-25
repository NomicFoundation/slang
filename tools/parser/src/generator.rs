use std::{
    io::Write,
    process::{Command, Stdio},
};

use anyhow::Result;
use proc_macro2::Ident;
use quote::quote;

use super::ebnf_impl::*;

pub fn generate(productions: Vec<Production>) {
    let rules = productions
        .iter()
        .map(|p| {
            let name = Ident::new(p.name.as_str(), proc_macro2::Span::call_site());
            quote!(
                let #name = choice((
                    none_of("'").repeated().padded_by(just('\'')),
                    none_of("\"").repeated().padded_by(just('"')),
                ))
                .ignored();
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
