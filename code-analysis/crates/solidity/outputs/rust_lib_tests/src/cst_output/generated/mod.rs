// This file is generated via `cargo build`. Please don't edit by hand.

mod contract_definition;
mod source_unit;

use std::rc::Rc;

use anyhow::{bail, Result};
use solidity_rust_lib::generated::{
    cst,
    parse::{ParserType, Parsers},
};

pub const BREAKING_VERSIONS: [&str; 3] = ["0.4.10", "0.6.0", "0.8.13"];

pub fn get_parser<'a>(
    parsers: Parsers<'a>,
    parser_name: &str,
) -> Result<ParserType<'a, Rc<cst::Node>>> {
    return Ok(match parser_name {
        "contract_definition" => parsers.contract_definition,
        "source_unit" => parsers.source_unit,
        _ => bail!("Unrecognized parser_name: {parser_name}"),
    });
}
