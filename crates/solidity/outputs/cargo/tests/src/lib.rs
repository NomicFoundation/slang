#![cfg(test)]

use metaslang_bindings as _;

mod backend;
mod binder;
mod binding_resolver;
mod binding_rules;
mod bindings_output;
mod compilation;
mod cst_output;
mod generated;
mod multi_part_file;
mod resolver;
mod trivia;
