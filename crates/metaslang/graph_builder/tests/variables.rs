// -*- coding: utf-8 -*-
// ------------------------------------------------------------------------------------------------
// Copyright © 2022, tree-sitter authors.
// Licensed under either of Apache License, Version 2.0, or MIT license, at your option.
// Please see the LICENSE-APACHE or LICENSE-MIT files in this distribution for license details.
// ------------------------------------------------------------------------------------------------

use metaslang_graph_builder::Variables;
use {
    env_logger as _, indoc as _, log as _, metaslang_cst as _, once_cell as _, regex as _,
    semver as _, serde as _, serde_json as _, smallvec as _, stack_graphs as _,
    string_interner as _, strum as _, strum_macros as _, thiserror as _,
};

#[test]
fn can_create_nested_variables() {
    fn f<'a>(v: &'a Variables<'a>) -> Variables<'a> {
        let mut w = Variables::nested(v);
        w.add("bar".into(), 2.into()).expect("Failed to set bar");
        w
    }
    let mut v = Variables::new();
    v.add("foo".into(), 1.into()).expect("Failed to set foo");
    let w = f(&v);
    w.get(&"foo".into()).expect("Failed to get foo");
}
