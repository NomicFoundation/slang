use once_cell::sync::Lazy;
use regex::Regex;
use semver::Version;
use slang_solidity::kinds::NonterminalKind;
use slang_solidity::language::Language;
use slang_solidity::query::{Query, QueryMatchIterator};

static SNIPPET_MARKER_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new("// --8<-- \\[(start|end):([a-z0-9-]+)\\]").unwrap());

/// Convenience trait that allows removing inline Mkdoc snippet markers from a string via postfix call.
trait RemoveMkdocSnippetMarkers {
    fn remove_mkdoc_snippet_markers(&self) -> String;
}

impl RemoveMkdocSnippetMarkers for &str {
    fn remove_mkdoc_snippet_markers(&self) -> String {
        SNIPPET_MARKER_RE.replace_all(self, "").into_owned()
    }
}

fn assert_matches(query: &Query, kind: NonterminalKind, source: &str) -> QueryMatchIterator {
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
        .remove_mkdoc_snippet_markers(),
    )
    .unwrap();

    assert_matches(&query, NonterminalKind::MultiplicativeExpression, "1*2");

    let query = Query::parse(
        &"
    // --8<-- [start:query-syntax-2]
	[MultiplicativeExpression left_operand:[Expression] [Asterisk] right_operand:[Expression]]
    // --8<-- [end:query-syntax-2]
    "
        .remove_mkdoc_snippet_markers(),
    )
    .unwrap();

    assert_matches(&query, NonterminalKind::MultiplicativeExpression, "1*2");

    let query = Query::parse(
        &r#"
    // --8<-- [start:query-syntax-3]
	[MultiplicativeExpression left_operand:[_] operator:["*"] right_operand:[_]]
    // --8<-- [end:query-syntax-3]
    "#
        .remove_mkdoc_snippet_markers(),
    )
    .unwrap();

    assert_matches(&query, NonterminalKind::MultiplicativeExpression, "1*2");

    let query = Query::parse(
        &"
    // --8<-- [start:query-syntax-4]
    [MultiplicativeExpression left_operand:[_] [_]]
    // --8<-- [end:query-syntax-4]
    "
        .remove_mkdoc_snippet_markers(),
    )
    .unwrap();
    assert_matches(&query, NonterminalKind::MultiplicativeExpression, "1*2");

    let query = Query::parse(
        &"
    // --8<-- [start:query-syntax-5]
    [MultiplicativeExpression [Expression [StringExpression]]]
    // --8<-- [end:query-syntax-5]
    "
        .remove_mkdoc_snippet_markers(),
    )
    .unwrap();
    assert_matches(
        &query,
        NonterminalKind::MultiplicativeExpression,
        "1 * 'abc'",
    );
    assert_matches(
        &query,
        NonterminalKind::MultiplicativeExpression,
        "'abc' * 1",
    );
}

#[test]
fn capturing_nodes() {
    let query = Query::parse(
        &"
    // --8<-- [start:capturing-nodes-1]
	[StructDefinition @struct_name name:[Identifier]]
    // --8<-- [end:capturing-nodes-1]
    "
        .remove_mkdoc_snippet_markers(),
    )
    .unwrap();

    assert_matches(&query, NonterminalKind::StructDefinition, "struct Abc {}");

    let query = Query::parse(
        &"
    // --8<-- [start:capturing-nodes-2]
	[ContractDefinition
		@contract_name name:[Identifier]
		members:[ContractMembers
			[ContractMember
				[EventDefinition @event_name name:[Identifier]]
			]
		]
	]
    // --8<-- [end:capturing-nodes-2]
    "
        .remove_mkdoc_snippet_markers(),
    )
    .unwrap();

    assert_matches(
        &query,
        NonterminalKind::ContractDefinition,
        "contract A { event A(); }",
    );
}

#[test]
fn quantification() {
    let query = Query::parse(
        &"
    // --8<-- [start:quantification-1]
	[SourceUnit (leading_trivia:[_])+]
    // --8<-- [end:quantification-1]
    "
        .remove_mkdoc_snippet_markers(),
    )
    .unwrap();

    assert_matches(
        &query,
        NonterminalKind::SourceUnit,
        "// comment 1\n// comment 2\n/* comment 3 */",
    );

    let query = Query::parse(
        &"
    // --8<-- [start:quantification-2]
	[ContractDefinition
		(@docline [SingleLineNatSpecComment])+
		@name name:[_]
	]
    // --8<-- [end:quantification-2]
    "
        .remove_mkdoc_snippet_markers(),
    )
    .unwrap();

    assert_matches(
        &query,
        NonterminalKind::SourceUnit,
        "
		/// A doc comment
		contract A {}
		",
    );

    let query = Query::parse(
        &"
    // --8<-- [start:quantification-3]
	[FunctionCallExpression
		arguments:[ArgumentsDeclaration
			variant:[PositionalArgumentsDeclaration
				arguments:[PositionalArguments
					(@arg [Expression variant:[StringExpression]])?
				]
			]
		]
	]
    // --8<-- [end:quantification-3]
    "
        .remove_mkdoc_snippet_markers(),
    )
    .unwrap();

    let iter = assert_matches(
        &query,
        NonterminalKind::FunctionCallExpression,
        "
		call(1, 'abc', 3)
		",
    );

    let matches: Vec<_> = iter.collect();

    matches[0].captures.get("arg").unwrap();
}

#[test]
fn alternations() {
    let query = Query::parse(
        &"
    // --8<-- [start:alternations-1]
	[FunctionCallExpression
    .
		operand:[Expression
			(@function variant:[Identifier]
			| @method variant:[MemberAccessExpression])
		]
	]
    // --8<-- [end:alternations-1]
    "
        .remove_mkdoc_snippet_markers(),
    )
    .unwrap();

    let matches: Vec<_> =
        assert_matches(&query, NonterminalKind::FunctionCallExpression, "call(1)").collect();
    matches.first().unwrap().captures.get("function").unwrap();
    let matches: Vec<_> =
        assert_matches(&query, NonterminalKind::FunctionCallExpression, "a.call(1)").collect();
    matches.first().unwrap().captures.get("method").unwrap();

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
        .remove_mkdoc_snippet_markers(),
    )
    .unwrap();

    let iter = assert_matches(&query, NonterminalKind::IfStatement, "if (true) { break; }");

    let matches: Vec<_> = iter.collect();
    assert_eq!(matches.len(), 2);
    assert_eq!(
        matches[0].captures.get("keyword").unwrap()[0]
            .node()
            .unparse(),
        "if"
    );
    assert_eq!(
        matches[1].captures.get("keyword").unwrap()[0]
            .node()
            .unparse(),
        "break"
    );
}

#[test]
fn anchoring() {
    let query = Query::parse(
        &r#"
    // --8<-- [start:anchoring-1]
  [FunctionDefinition
    [ParametersDeclaration
      [Parameters . @first_param [Parameter]]
    ]
  ]
    // --8<-- [end:anchoring-1]
    "#
        .remove_mkdoc_snippet_markers(),
    )
    .unwrap();

    let iter = assert_matches(
        &query,
        NonterminalKind::FunctionDefinition,
        "function test(int x, int y);",
    );

    let matches: Vec<_> = iter.collect();
    assert_eq!(matches.len(), 1);
    assert_eq!(
        matches[0].captures.get("first_param").unwrap()[0]
            .node()
            .unparse(),
        "int x"
    );

    let query = Query::parse(
        &r#"
    // --8<-- [start:anchoring-2]
  [FunctionDefinition
    [ParametersDeclaration
      [Parameters @last_param [Parameter] .]
    ]
  ]
    // --8<-- [end:anchoring-2]
    "#
        .remove_mkdoc_snippet_markers(),
    )
    .unwrap();

    let iter = assert_matches(
        &query,
        NonterminalKind::FunctionDefinition,
        "function test(int x, int y);",
    );

    let matches: Vec<_> = iter.collect();
    assert_eq!(matches.len(), 1);
    assert_eq!(
        matches[0].captures.get("last_param").unwrap()[0]
            .node()
            .unparse(),
        " int y"
    );

    let query = Query::parse(
        &r#"
    // --8<-- [start:anchoring-3]
  [Statements
    @stmt1 [Statement] . @stmt2 [Statement]
  ]
    // --8<-- [end:anchoring-3]
    "#
        .remove_mkdoc_snippet_markers(),
    )
    .unwrap();

    let iter = assert_matches(&query, NonterminalKind::Statements, "int x; int y; x + y;");

    let matches: Vec<_> = iter.collect();
    assert_eq!(matches.len(), 2);
    assert_eq!(
        matches[0].captures.get("stmt1").unwrap()[0]
            .node()
            .unparse(),
        "int x;"
    );
    assert_eq!(
        matches[0].captures.get("stmt2").unwrap()[0]
            .node()
            .unparse(),
        " int y;"
    );
    assert_eq!(
        matches[1].captures.get("stmt1").unwrap()[0]
            .node()
            .unparse(),
        " int y;"
    );
    assert_eq!(
        matches[1].captures.get("stmt2").unwrap()[0]
            .node()
            .unparse(),
        " x + y;"
    );
}
