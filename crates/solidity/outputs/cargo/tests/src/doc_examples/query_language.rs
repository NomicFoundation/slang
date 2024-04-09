use semver::Version;
use slang_solidity::kinds::RuleKind;
use slang_solidity::language::Language;
use slang_solidity::query::{Query, QueryResultIterator};

fn assert_matches(query: &Query, kind: RuleKind, source: &str) -> QueryResultIterator {
    let language = Language::new(Version::new(0, 8, 12)).unwrap();
    let cursor = language.parse(kind, source).create_tree_cursor();

    let tree = cursor.node();
    assert!(
        cursor.clone().query(vec![query.clone()]).count() > 0,
        "The following query didn't match: `{query}`, source: `{source}`, tree: {tree:#?}"
    );

    cursor.query(vec![query.clone()])
}

#[test]
fn query_syntax() {
    let query = Query::parse(
        &"
	// --8<-- [start:query-syntax-1]
	[MultiplicativeExpression [Expression] [Asterisk] [Expression]]
	// --8<-- [end:query-syntax-1]
	"
        .replace("// --8<-- [start:query-syntax-1]", "")
        .replace("// --8<-- [end:query-syntax-1]", ""),
    )
    .unwrap();

    assert_matches(&query, RuleKind::MultiplicativeExpression, "1*2");

    let query = Query::parse(
        &"
    // --8<-- [start:query-syntax-2]
	[MultiplicativeExpression [left_operand:Expression] [Asterisk] [right_operand:Expression]]
    // --8<-- [end:query-syntax-2]
    "
        .replace("// --8<-- [start:query-syntax-2]", "")
        .replace("// --8<-- [end:query-syntax-2]", ""),
    )
    .unwrap();

    assert_matches(&query, RuleKind::MultiplicativeExpression, "1*2");

    let query = Query::parse(
        &r#"
    // --8<-- [start:query-syntax-3]
	[MultiplicativeExpression [left_operand: _] [operator: "*"] [right_operand: _]]
    // --8<-- [end:query-syntax-3]
    "#
        .replace("// --8<-- [start:query-syntax-3]", "")
        .replace("// --8<-- [end:query-syntax-3]", ""),
    )
    .unwrap();

    assert_matches(&query, RuleKind::MultiplicativeExpression, "1*2");

    let query = Query::parse(
        &"
    // --8<-- [start:query-syntax-4]
    [MultiplicativeExpression [left_operand:_] [_] ...]
    // --8<-- [end:query-syntax-4]
    "
        .replace("// --8<-- [start:query-syntax-4]", "")
        .replace("// --8<-- [end:query-syntax-4]", ""),
    )
    .unwrap();
    assert_matches(&query, RuleKind::MultiplicativeExpression, "1*2");

    let query = Query::parse(
        &"
    // --8<-- [start:query-syntax-5]
    [MultiplicativeExpression ... [Expression [StringExpression]] ...]
    // --8<-- [end:query-syntax-5]
    "
        .replace("// --8<-- [start:query-syntax-5]", "")
        .replace("// --8<-- [end:query-syntax-5]", ""),
    )
    .unwrap();
    assert_matches(&query, RuleKind::MultiplicativeExpression, "1 * 'abc'");
    assert_matches(&query, RuleKind::MultiplicativeExpression, "'abc' * 1");
}

#[test]
fn capturing_nodes() {
    let query = Query::parse(
        &"
    // --8<-- [start:capturing-nodes-1]
	[StructDefinition ... @struct_name [name:Identifier] ...]
    // --8<-- [end:capturing-nodes-1]
    "
        .replace("// --8<-- [start:capturing-nodes-1]", "")
        .replace("// --8<-- [end:capturing-nodes-1]", ""),
    )
    .unwrap();

    assert_matches(&query, RuleKind::StructDefinition, "struct Abc {}");

    let query = Query::parse(
        &"
    // --8<-- [start:capturing-nodes-2]
	[ContractDefinition
		...
		@contract_name [name: Identifier]
		...
		[members: ContractMembers
			...
			[ContractMember
				[EventDefinition
					...
					@event_name [name: Identifier]
					...
				]
			]
			...
		]
		...
	]
    // --8<-- [end:capturing-nodes-2]
    "
        .replace("// --8<-- [start:capturing-nodes-2]", "")
        .replace("// --8<-- [end:capturing-nodes-2]", ""),
    )
    .unwrap();

    assert_matches(
        &query,
        RuleKind::ContractDefinition,
        "contract A { event A(); }",
    );
}

#[test]
fn quantification() {
    let query = Query::parse(
        &"
    // --8<-- [start:quantification-1]
	[SourceUnit ... ([leading_trivia: _])+]
    // --8<-- [end:quantification-1]
    "
        .replace("// --8<-- [start:quantification-1]", "")
        .replace("// --8<-- [end:quantification-1]", ""),
    )
    .unwrap();

    assert_matches(
        &query,
        RuleKind::SourceUnit,
        "// comment 1\n// comment 2\n/* comment 3 */",
    );

    let query = Query::parse(
        &"
    // --8<-- [start:quantification-2]
	[ContractDefinition
		...
		(@docline [SingleLineNatSpecComment])+
		...
		@name [name:_]
		...
	]
    // --8<-- [end:quantification-2]
    "
        .replace("// --8<-- [start:quantification-2]", "")
        .replace("// --8<-- [end:quantification-2]", ""),
    )
    .unwrap();

    assert_matches(
        &query,
        RuleKind::SourceUnit,
        "
		/// A doc comment
		contract A {}
		",
    );

    let query = Query::parse(
        &"
    // --8<-- [start:quantification-3]
	[FunctionCallExpression
		...
		[arguments: ArgumentsDeclaration
			[variant: PositionalArgumentsDeclaration
				...
				[arguments: PositionalArguments
					...
					(@arg [Expression [variant: StringExpression]])?
					...
				]
				...
			]
			...
		]
		...
	]
    // --8<-- [end:quantification-3]
    "
        .replace("// --8<-- [start:quantification-3]", "")
        .replace("// --8<-- [end:quantification-3]", ""),
    )
    .unwrap();

    let iter = assert_matches(
        &query,
        RuleKind::FunctionCallExpression,
        "
		call(1, 'abc', 3)
		",
    );

    let results: Vec<_> = iter.collect();

    results[3].bindings.get("arg").unwrap();
}

#[test]
fn alternations() {
    let query = Query::parse(
        &"
    // --8<-- [start:alternations-1]
	[FunctionCallExpression
		[operand: Expression
			(@function [variant: Identifier]
			| @method [variant: MemberAccessExpression])
		]
		...
	]
    // --8<-- [end:alternations-1]
    "
        .replace("// --8<-- [start:alternations-1]", "")
        .replace("// --8<-- [end:alternations-1]", ""),
    )
    .unwrap();

    let results: Vec<_> =
        assert_matches(&query, RuleKind::FunctionCallExpression, "call(1)").collect();
    results.first().unwrap().bindings.get("function").unwrap();
    let results: Vec<_> =
        assert_matches(&query, RuleKind::FunctionCallExpression, "a.call(1)").collect();
    results.first().unwrap().bindings.get("method").unwrap();

    let query = Query::parse(
        &r#"
    // --8<-- [start:alternations-2]
	@keyword (
		["break"]
	  | ["delete"]
	  | ["else"]
	  | ["for"]
	  | ["function"]
	  | ["if"]
	  | ["return"]
	  | ["try"]
	  | ["while"]
	)
    // --8<-- [end:alternations-2]
    "#
        .replace("// --8<-- [start:alternations-2]", "")
        .replace("// --8<-- [end:alternations-2]", ""),
    )
    .unwrap();

    let iter = assert_matches(&query, RuleKind::IfStatement, "if (true) { break; }");

    let results: Vec<_> = iter.collect();
    assert_eq!(results.len(), 2);
    assert_eq!(
        results[0].bindings.get("keyword").unwrap()[0]
            .node()
            .unparse(),
        "if"
    );
    assert_eq!(
        results[1].bindings.get("keyword").unwrap()[0]
            .node()
            .unparse(),
        "break"
    );
}
