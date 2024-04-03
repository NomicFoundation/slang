// -*- coding: utf-8 -*-
// ------------------------------------------------------------------------------------------------
// Copyright © 2021, tree-sitter authors.
// Licensed under either of Apache License, Version 2.0, or MIT license, at your option.
// Please see the LICENSE-APACHE or LICENSE-MIT files in this distribution for license details.
// ------------------------------------------------------------------------------------------------
#![allow(clippy::needless_raw_string_hashes)]
#![allow(clippy::redundant_pattern_matching)]
#![allow(clippy::similar_names)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::unnecessary_mut_passed)]
#![allow(clippy::useless_conversion)]
#![allow(clippy::manual_string_new)]

#[cfg(feature = "cli")]
use {anyhow as _, clap as _, colored as _, tree_sitter_config as _, tree_sitter_loader as _};
use {
    log as _, regex as _, serde as _, serde_json as _, smallvec as _, string_interner as _,
    thiserror as _,
};

mod execution;
mod functions;
mod graph;
mod lazy_execution;
mod parse_errors;
mod parser;
mod variables;
