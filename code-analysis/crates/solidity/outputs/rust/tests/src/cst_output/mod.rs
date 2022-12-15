mod generated;

use std::{collections::HashMap, io::Write, ops::Range};

use anyhow::Result;
use codegen_utils::context::CodegenContext;
use semver::Version;
use serde::Serialize;
use solidity_rust_lib::generated::{
    cst,
    kinds::{Rule, Token},
    language::Language,
    lex,
};

use crate::cst_output::generated::TEST_VERSIONS;

pub fn run(parser_name: &str, test_name: &str) -> Result<()> {
    return CodegenContext::with_context(|codegen| {
        let test_dir = codegen
            .repo_root
            .join("code-analysis/crates/solidity/test_data/cst_output")
            .join(parser_name)
            .join(test_name);

        let input = &std::fs::read_to_string(&test_dir.join("input.sol"))?;

        let mut last_output: String = String::new();

        for version in TEST_VERSIONS {
            let current_output = run_parser(version, parser_name, input)?;

            if current_output == last_output {
                // Skip versions that don't change its output.
                continue;
            }

            let output_path = test_dir.join(format!("generated/{version}.yml"));
            codegen.write_file(&output_path, &current_output)?;

            last_output = current_output;
        }

        return Ok(());
    });
}

fn run_parser(version: &str, parser_name: &str, input: &str) -> Result<String> {
    let version = Version::parse(version)?;
    let parser_output = Language::new(version)
        .parse(parser_name, &input)
        .expect(format!("No such parser: {}", parser_name).as_str());
    let mut result = Vec::new();

    // Manually serializing errors for now, as serde_yaml doesn't support multi-line strings.
    if parser_output.error_count() == 0 {
        writeln!(&mut result, "errors: []")?;
    } else {
        writeln!(&mut result, "errors:")?;
        for report in parser_output.errors_as_strings(input, /* with_colour */ false) {
            writeln!(&mut result, "  - |")?;
            for line in report.lines() {
                writeln!(&mut result, "    {line}")?;
            }
        }
    }

    writeln!(&mut result)?;

    {
        let mut root_node = HashMap::new();
        root_node.insert(
            "root",
            parser_output.parse_tree().and_then(|node| {
                let metadata = TestNodeMetadata::from_cst(node.as_ref());
                Some(TestNode::from_metadata(metadata, input))
            }),
        );

        let root_node = serde_yaml::to_string(&root_node)?;
        writeln!(&mut result, "{root_node}")?;
    }

    return Ok(String::from_utf8(result)?);
}

#[derive(Serialize, PartialEq)]
enum TestNodeKind {
    RuleKind(Rule),
    RuleGroup,
    TokenKind(Token),
    TokenFragment,
}

#[derive(Serialize)]
struct TestNode {
    // A separate enum to render the kind as a yaml TAG!
    kind: TestNodeKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    contents: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<TestNode>>,
}

impl TestNode {
    pub fn from_metadata(metadata: TestNodeMetadata, input: &str) -> Self {
        let contents = metadata
            .range
            .and_then(|range| Some(Self::render_contents(input, range)));

        let children = metadata.children.and_then(|children| {
            Some(
                children
                    .into_iter()
                    .map(|child| Self::from_metadata(child, input))
                    .collect(),
            )
        });

        return Self {
            kind: metadata.kind,
            contents,
            children,
        };
    }

    fn render_contents(input: &str, range: Range<usize>) -> String {
        let mut contents = input[range].to_string();

        let max_length = 80;
        if contents.len() > max_length {
            let half_length = max_length / 2;
            contents = format!(
                "{}...{}",
                &contents[..half_length],
                &contents[(contents.len() - half_length)..]
            );
        }

        return contents.replace("\r", "\\r").replace("\n", "\\n");
    }
}

struct TestNodeMetadata {
    kind: TestNodeKind,
    children: Option<Vec<TestNodeMetadata>>,
    range: Option<Range<usize>>,
}

impl TestNodeMetadata {
    pub fn from_cst(node: &cst::Node) -> Self {
        let kind = match node {
            cst::Node::Rule { kind, .. } => TestNodeKind::RuleKind(*kind),
            cst::Node::Token { kind, .. } => TestNodeKind::TokenKind(*kind),
            cst::Node::Group { .. } => TestNodeKind::RuleGroup,
        };

        let children = match node {
            cst::Node::Rule { children, .. } | cst::Node::Group { children } => {
                children.iter().map(|child| Self::from_cst(child)).collect()
            }
            cst::Node::Token {
                lex_node, trivia, ..
            } => Self::reorder_token_trivia(lex_node, trivia),
        };

        let range = Self::calculate_cst_range(&children);

        // If children contains a single fragment (no kind), absorb into parent for brevity
        let children = if children.len() == 1 && children[0].kind == TestNodeKind::TokenFragment {
            None
        } else {
            Some(children)
        };

        return Self {
            kind,
            children,
            range,
        };
    }

    pub fn from_lex(node: &lex::Node) -> Self {
        let kind = match node {
            lex::Node::Chars(_) | lex::Node::Choice(_, _) | lex::Node::Sequence(_) => {
                TestNodeKind::TokenFragment
            }
            lex::Node::Named(kind, _) => TestNodeKind::TokenKind(*kind),
        };

        let children = match node {
            lex::Node::Chars(_) => None,
            lex::Node::Choice(_, child) | lex::Node::Named(_, child) => {
                Some(vec![Self::from_lex(child)])
            }
            lex::Node::Sequence(children) => {
                if children.is_empty() {
                    None
                } else {
                    Some(children.iter().map(|child| Self::from_lex(child)).collect())
                }
            }
        };

        return Self {
            kind,
            children,
            range: Some(node.range()),
        };
    }

    fn reorder_token_trivia(
        lex_node: &lex::Node,
        trivia: &Vec<std::rc::Rc<cst::Node>>,
    ) -> Vec<Self> {
        let mut leading = vec![];
        let mut trailing = vec![];

        for child in trivia {
            match child.as_ref() {
                cst::Node::Rule { kind, children } => {
                    for child in children {
                        match child.as_ref() {
                            cst::Node::Token { kind, trivia, .. } => {
                                assert!(
                                    trivia.is_empty(),
                                    "Unexpected trivia children: {trivia:?}"
                                );
                                match kind {
                                    Token::Whitespace | Token::EndOfLine | Token::Linefeed => {
                                        continue; // skip
                                    }
                                    Token::SingleLineComment | Token::MultilineComment => {
                                        // include
                                    }
                                    _ => {
                                        unreachable!("Unexpected trivia token kind: {child:?}")
                                    }
                                }
                            }
                            _ => unreachable!("Unexpected trivia child: {child:?}"),
                        };

                        match kind {
                            Rule::LeadingTrivia => leading.push(Self::from_cst(child)),
                            Rule::TrailingTrivia => trailing.push(Self::from_cst(child)),
                            _ => unreachable!("Unexpected trivia rule rule: {child:?}"),
                        }
                    }
                }
                _ => unreachable!("Unexpected trivia node: {trivia:?}"),
            }
        }

        let mut children = vec![];

        children.extend(leading);
        children.push(Self::from_lex(lex_node));
        children.extend(trailing);

        return children;
    }

    fn calculate_cst_range(children: &Vec<Self>) -> Option<Range<usize>> {
        let ranges: Vec<&Range<usize>> = children
            .iter()
            .filter_map(|child| child.range.as_ref())
            .collect();

        if ranges.is_empty() {
            return None;
        }

        let first = ranges.first().unwrap();
        let last = ranges.last().unwrap();
        return Some(first.start..last.end);
    }
}
