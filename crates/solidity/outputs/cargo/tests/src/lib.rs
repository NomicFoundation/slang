#![cfg(test)]

use metaslang_bindings as _;

mod backend;
mod bindings;
mod compilation;
mod cst;

mod generated;
mod multi_part_file;
mod resolver;
mod trivia;
mod utils;
