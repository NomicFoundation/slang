// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "StringLiteral";

#[test]
fn double_quote_unicode() -> Result<()> {
    run(T, "double_quote_unicode")
}

#[test]
fn escape_arbitrary_ascii() -> Result<()> {
    run(T, "escape_arbitrary_ascii")
}

#[test]
fn escape_arbitrary_unicode() -> Result<()> {
    run(T, "escape_arbitrary_unicode")
}

#[test]
fn escape_ascii() -> Result<()> {
    run(T, "escape_ascii")
}

#[test]
fn escape_cr_double_quote() -> Result<()> {
    run(T, "escape_cr_double_quote")
}

#[test]
fn escape_cr_single_quote() -> Result<()> {
    run(T, "escape_cr_single_quote")
}

#[test]
fn escape_crlf_double_quote() -> Result<()> {
    run(T, "escape_crlf_double_quote")
}

#[test]
fn escape_crlf_single_quote() -> Result<()> {
    run(T, "escape_crlf_single_quote")
}

#[test]
fn escape_hex() -> Result<()> {
    run(T, "escape_hex")
}

#[test]
fn escape_hex_invalid() -> Result<()> {
    run(T, "escape_hex_invalid")
}

#[test]
fn escape_lf_double_quote() -> Result<()> {
    run(T, "escape_lf_double_quote")
}

#[test]
fn escape_lf_single_quote() -> Result<()> {
    run(T, "escape_lf_single_quote")
}

#[test]
fn escape_unicode() -> Result<()> {
    run(T, "escape_unicode")
}

#[test]
fn escape_unicode_invalid() -> Result<()> {
    run(T, "escape_unicode_invalid")
}

#[test]
fn single_quote_unicode() -> Result<()> {
    run(T, "single_quote_unicode")
}

#[test]
fn tabs_double_quote() -> Result<()> {
    run(T, "tabs_double_quote")
}

#[test]
fn tabs_single_quote() -> Result<()> {
    run(T, "tabs_single_quote")
}
