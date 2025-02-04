// -*- coding: utf-8 -*-
// ------------------------------------------------------------------------------------------------
// Copyright © 2021, tree-sitter authors.
// Licensed under either of Apache License, Version 2.0, or MIT license, at your option.
// Please see the LICENSE-APACHE or LICENSE-MIT files in this distribution for license details.
// ------------------------------------------------------------------------------------------------

use indoc::indoc;
use metaslang_graph_builder::ast::File;
use metaslang_graph_builder::functions::Functions;
use metaslang_graph_builder::{
    ExecutionConfig, ExecutionError, Identifier, NoCancellation, Variables,
};
use {log as _, regex as _, serde_json as _, smallvec as _, string_interner as _, thiserror as _};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
    serde::Serialize,
    strum_macros::IntoStaticStr,
    strum_macros::EnumString,
)]
pub enum DummyKind {
    #[default]
    Module,
}
impl metaslang_cst::kinds::TerminalKindExtensions for DummyKind {}
impl metaslang_cst::kinds::NonterminalKindExtensions for DummyKind {}
impl metaslang_cst::kinds::EdgeLabelExtensions for DummyKind {}
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct KindTypes;
impl metaslang_cst::kinds::KindTypes for KindTypes {
    type NonterminalKind = DummyKind;
    type TerminalKind = DummyKind;
    type EdgeLabel = DummyKind;
}

fn init_log() {
    let _ = env_logger::builder()
        .is_test(true)
        .format_level(false)
        .format_target(false)
        .format_timestamp(None)
        .try_init(); // try, because earlier test may have already initialized it
}

fn execute(dsl_source: &str) -> Result<String, ExecutionError> {
    init_log();
    let file = File::from_str(dsl_source).expect("Cannot parse file");
    let functions = Functions::stdlib();
    let mut globals = Variables::new();
    globals
        .add(Identifier::from("filename"), "test.py".into())
        .map_err(|_| ExecutionError::DuplicateVariable("filename".into()))?;
    let config = ExecutionConfig::new(&functions, &globals);
    let tree =
        metaslang_cst::nodes::Node::<KindTypes>::terminal(DummyKind::Module, "pass".to_owned());
    let cursor = tree.create_cursor(metaslang_cst::text_index::TextIndex::ZERO);
    let graph = file.execute(&cursor, &config, &NoCancellation)?;
    let result = graph.pretty_print().to_string();
    Ok(result)
}

fn check_execution(dsl_source: &str, expected_graph: &str) {
    match execute(dsl_source) {
        Ok(actual_graph) => assert_eq!(actual_graph, expected_graph),
        Err(e) => panic!("Could not execute file: {e}"),
    }
}

fn fail_execution(dsl_source: &str) {
    assert!(
        execute(dsl_source).is_err(),
        "Execution succeeded unexpectedly"
    );
}

#[test]
fn can_eq_equal_bools() {
    check_execution(
        indoc! {r#"
          [ Module ]
          {
            node n
            attr (n) eq = (eq #true #true)
          }
        "#},
        indoc! {r#"
          node 0
            eq: #true
        "#},
    );
}

#[test]
fn can_eq_nonequal_bools() {
    check_execution(
        indoc! {r#"
          [ Module ]
          {
            node n
            attr (n) eq = (eq #true #false)
          }
        "#},
        indoc! {r#"
          node 0
            eq: #false
        "#},
    );
}

#[test]
fn cannot_eq_bool_and_string() {
    fail_execution(indoc! {r#"
          [ Module ]
          {
            node n
            attr (n) eq = (eq #true "false")
          }
        "#});
}

#[test]
fn can_format_string_null_and_escaped_braces() {
    check_execution(
        indoc! {r#"
          [ Module ]
          {
            node n
            attr (n) str = (format "{} : {{ {} }}" "foo" #null)
          }
        "#},
        indoc! {r#"
          node 0
            str: "foo : { #null }"
        "#},
    );
}

#[test]
fn cannot_format_with_missing_parameter() {
    fail_execution(indoc! {r#"
          [ Module ]
          {
            node n
            attr (n) str = (format "{} : {{ {} }}" "foo")
          }
        "#});
}

#[test]
fn cannot_format_with_extra_parameter() {
    fail_execution(indoc! {r#"
          [ Module ]
          {
            node n
            attr (n) str = (format "{} : {{ {} }}" "foo" #null 42)
          }
        "#});
}

#[test]
fn cannot_format_with_unexpected_opening_brace() {
    fail_execution(indoc! {r#"
          [ Module ]
          {
            node n
            attr (n) str = (format "{} : { {} }}" "foo" #null)
          }
        "#});
}

#[test]
fn cannot_format_with_unexpected_closing_brace() {
    fail_execution(indoc! {r#"
          [ Module ]
          {
            node n
            attr (n) str = (format "{} : {{ {} }" "foo" #null)
          }
        "#});
}

#[test]
fn can_concat_lists() {
    check_execution(
        indoc! {r#"
          [ Module ]
          {
            node n
            attr (n) xs = (concat [1, 2] [] [3, 4, 5])
          }
        "#},
        indoc! {r#"
          node 0
            xs: [1, 2, 3, 4, 5]
        "#},
    );
}

#[test]
fn can_join_list_with_separator() {
    check_execution(
        indoc! {r#"
          [ Module ]
          {
            node n
            attr (n) str = (join [1, 2, 3] ".")
          }
        "#},
        indoc! {r#"
          node 0
            str: "1.2.3"
        "#},
    );
}

#[test]
fn can_join_list_without_separator() {
    check_execution(
        indoc! {r#"
          [ Module ]
          {
            node n
            attr (n) str = (join [1, 2, 3])
          }
        "#},
        indoc! {r#"
          node 0
            str: "123"
        "#},
    );
}
