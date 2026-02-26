use std::rc::Rc;

use anyhow::Result;
use semver::Version;
use slang_solidity::bindings;
use slang_solidity::parser::Parser;

use crate::compilation::resolver::TestsPathResolver;

const TEST_VERSION: Version = Version::new(0, 8, 26);

#[test]
fn test_struct_incomplete_dot_access() -> Result<()> {
    let source = r#"
contract Test {
    struct Data {
        uint120 value;
        uint120 count;
    }

    Data public data;

    function update() public {
        data.
    }
}
"#;

    let parser = Parser::create(TEST_VERSION.clone())?;
    let mut builder = bindings::create_with_resolver(&TEST_VERSION, Rc::new(TestsPathResolver {}));

    let parse_output = parser.parse_file_contents(source);
    builder.add_user_file("test.sol", parse_output.create_tree_cursor());

    let _graph = builder.build();

    Ok(())
}

#[test]
fn test_nested_struct_incomplete_dot_access() -> Result<()> {
    let source = r#"
contract Test {
    struct Inner {
        uint120 value;
    }

    struct Outer {
        Inner inner;
        uint120 count;
    }

    Outer public data;

    function f() public {
        data.inner.
    }
}
"#;

    let parser = Parser::create(TEST_VERSION.clone())?;
    let mut builder = bindings::create_with_resolver(&TEST_VERSION, Rc::new(TestsPathResolver {}));

    let parse_output = parser.parse_file_contents(source);
    builder.add_user_file("test.sol", parse_output.create_tree_cursor());

    let _graph = builder.build();

    Ok(())
}

#[test]
fn test_struct_array_member_incomplete_dot_access() -> Result<()> {
    let source = r#"
contract Test {
    struct Container {
        int256[] items;
    }

    Container private container;

    function f() public {
        container.items.
    }
}
"#;

    let parser = Parser::create(TEST_VERSION.clone())?;
    let mut builder = bindings::create_with_resolver(&TEST_VERSION, Rc::new(TestsPathResolver {}));

    let parse_output = parser.parse_file_contents(source);
    builder.add_user_file("test.sol", parse_output.create_tree_cursor());

    let _graph = builder.build();

    Ok(())
}

#[test]
fn test_imported_contract_type_variable() -> Result<()> {
    let foo_source = r#"
contract Foo {
    string public name = "Foo";
}
"#;

    let example_source = r#"
import "./Foo.sol";
contract Example {
    Foo public foo = new Foo();
}
"#;

    let parser = Parser::create(TEST_VERSION.clone())?;
    let mut builder = bindings::create_with_resolver(&TEST_VERSION, Rc::new(TestsPathResolver {}));

    let foo_output = parser.parse_file_contents(foo_source);
    builder.add_user_file("Foo.sol", foo_output.create_tree_cursor());

    let example_output = parser.parse_file_contents(example_source);
    builder.add_user_file("Example.sol", example_output.create_tree_cursor());

    let _graph = builder.build();

    Ok(())
}

#[test]
fn test_project_wide_compilation_with_incomplete_expression() -> Result<()> {
    let foo_source = r#"
contract Foo {
    string public name = "Foo";
}
"#;

    let example_source = r#"
import "./Foo.sol";
contract Example {
    Foo public foo = new Foo();
}
"#;

    let incomplete_source = r#"
contract Incomplete {
    struct Data {
        uint120 value;
    }

    Data public data;

    function f() public {
        data.
    }
}
"#;

    let parser = Parser::create(TEST_VERSION.clone())?;
    let mut builder = bindings::create_with_resolver(&TEST_VERSION, Rc::new(TestsPathResolver {}));

    let foo_output = parser.parse_file_contents(foo_source);
    builder.add_user_file("Foo.sol", foo_output.create_tree_cursor());

    let example_output = parser.parse_file_contents(example_source);
    builder.add_user_file("Example.sol", example_output.create_tree_cursor());

    let incomplete_output = parser.parse_file_contents(incomplete_source);
    builder.add_user_file("Incomplete.sol", incomplete_output.create_tree_cursor());

    let _graph = builder.build();

    Ok(())
}
