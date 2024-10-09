use std::path::Path;

use anyhow::Result;
use infra_utils::paths::PathExtensions;
use semver::Version;
use slang_solidity::cst::{NonterminalKind, Query, QueryMatchIterator};
use slang_solidity::parser::{ParseOutput, Parser};

fn parse_doc_input_file<T: AsRef<Path>>(path: T) -> Result<ParseOutput> {
    let input_path = Path::repo_path("documentation/public/user-guide/inputs").join(path.as_ref());

    let source = input_path.read_to_string()?;

    let parser = Parser::create(Version::new(0, 8, 0))?;

    Ok(parser.parse(NonterminalKind::SourceUnit, source.trim()))
}

#[test]
fn using_queries() -> Result<()> {
    #[allow(unused_variables, clippy::items_after_statements)]
    {
        let parse_output = parse_doc_input_file("using-the-cursor.sol")?;
        // --8<-- [start:creating-a-query]
        use slang_solidity::cst::Query;

        // Any `Cursor` can be used to create a query.
        let cursor = parse_output.create_tree_cursor();

        let query = Query::parse("[ContractDefinition]").unwrap();
        let result: QueryMatchIterator = cursor.query(vec![query]);
        // --8<-- [end:creating-a-query]
    }

    {
        let parse_output = parse_doc_input_file("using-the-cursor.sol")?;
        let cursor = parse_output.create_tree_cursor();
        // --8<-- [start:visiting-contracts]
        let mut found = vec![];

        let query = Query::parse("@contract [ContractDefinition]").unwrap();

        for r#match in cursor.query(vec![query]) {
            let captures = r#match.captures;
            let cursors = captures.get("contract").unwrap();

            let cursor = cursors.first().unwrap();

            found.push(cursor.node().unparse().trim().to_owned());
        }

        assert_eq!(
            found,
            ["contract Foo {}", "contract Bar {}", "contract Baz {}"]
        );
        // --8<-- [end:visiting-contracts]
    }

    {
        let parse_output = parse_doc_input_file("multiple-data-types.sol")?;
        let cursor = parse_output.create_tree_cursor();
        // --8<-- [start:multiple-patterns]
        let mut names = vec![];

        let struct_def = Query::parse("[StructDefinition @name [Identifier]]").unwrap();
        let enum_def = Query::parse("[EnumDefinition @name [Identifier]]").unwrap();

        for r#match in cursor.query(vec![struct_def, enum_def]) {
            let index = r#match.query_number;
            let captures = r#match.captures;
            let cursors = captures.get("name").unwrap();

            let cursor = cursors.first().unwrap();

            names.push((index, cursor.node().unparse()));
        }

        assert_eq!(
            names,
            &[
                (0, "Foo".to_string()),
                (1, "Bar".to_string()),
                (0, "Baz".to_string()),
                (1, "Qux".to_string())
            ]
        );
        // --8<-- [end:multiple-patterns]
    }

    {
        let parse_output = parse_doc_input_file("typed-tuple.sol")?;
        let cursor = parse_output.create_tree_cursor();
        // --8<-- [start:matching-on-label]

        let mut names = vec![];

        let query = Query::parse("[TypedTupleMember @type type_name:[_]]").unwrap();

        for r#match in cursor.query(vec![query]) {
            let captures = r#match.captures;
            let cursors = captures.get("type").unwrap();

            let cursor = cursors.first().unwrap();

            names.push(cursor.node().unparse());
        }

        assert_eq!(names, &["uint", " uint16", " uint64", " uint256"]);
        // --8<-- [end:matching-on-label]
    }

    {
        // Matching on node's literal value
        let parse_output = parse_doc_input_file("typed-tuple.sol")?;
        let cursor = parse_output.create_tree_cursor();
        // --8<-- [start:matching-on-literal-value]

        let mut names = vec![];

        let query = Query::parse(r#"[ElementaryType @uint_keyword variant:["uint"]]"#).unwrap();

        for r#match in cursor.query(vec![query]) {
            let captures = r#match.captures;
            let cursors = captures.get("uint_keyword").unwrap();

            let cursor = cursors.first().unwrap();

            names.push(cursor.node().unparse());
        }

        assert_eq!(names, &["uint"]);
        // --8<-- [end:matching-on-literal-value]
    }

    Ok(())
}

#[test]
fn tx_origin_query() -> Result<()> {
    let parse_output = parse_doc_input_file("tx-origin.sol")?;
    let cursor = parse_output.create_tree_cursor();
    // --8<-- [start:tx-origin]
    let query = Query::parse(
        r#"@txorigin [MemberAccessExpression
                [Expression @start ["tx"]]
                ["origin"]
            ]"#,
    )
    .unwrap();

    let mut results = vec![];

    for r#match in cursor.query(vec![query]) {
        let captures = r#match.captures;
        let cursors = captures.get("txorigin").unwrap();

        let cursor = cursors.first().unwrap();

        results.push((cursor.text_offset().utf8, cursor.node().unparse()));
    }

    assert_eq!(results, &[(375usize, "tx.origin".to_string())]);
    // --8<-- [end:tx-origin]

    Ok(())
}
