// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

mod contract_definition;
mod exponentiation_expression;
mod interface_definition;
mod multiline_comment;
mod source_unit;

use std::rc::Rc;

use anyhow::{bail, Result};
use solidity_rust_lib::generated::{
    cst,
    parse::{ParserType, Parsers},
};

pub const TEST_VERSIONS: [&str; 3] = ["0.4.11", "0.6.0", "0.8.17"];

pub fn get_parser<'a>(
    parsers: Parsers<'a>,
    parser_name: &str,
) -> Result<ParserType<'a, Rc<cst::Node>>> {
    return Ok(match parser_name {
        "contract_definition" => parsers.contract_definition,
        "exponentiation_expression" => parsers.exponentiation_expression,
        "interface_definition" => parsers.interface_definition,
        "multiline_comment" => parsers.multiline_comment,
        "source_unit" => parsers.source_unit,
        _ => bail!("Unrecognized parser_name: {parser_name}"),
    });
}
