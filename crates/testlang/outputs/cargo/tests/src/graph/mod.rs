use semver::Version;
use slang_testlang::graph_builder::{ExecutionConfig, File, Functions, NoCancellation, Variables};
use slang_testlang::kinds::NonTerminalKind;
use slang_testlang::language::Language;

#[test]
fn builds_a_graph() {
    let version = Version::parse("1.0.0").unwrap();
    let language = Language::new(version).unwrap();

    let msgb_source = r#"
        @tree [Tree] {
            node @tree.def
            attr (@tree.def) is_tree
        }

        @tree_node [TreeNode] {
            node @tree_node.def
        }

        @tree [Tree ... @root node: [TreeNode] ...] {
            edge @root.def -> @tree.def
        }

        @parent [TreeNode ... members: [_ ... [_ @child variant: [TreeNode]] ...] ...] {
            edge @child.def -> @parent.def
        }
    "#;

    let msgb = File::from_str(msgb_source);
    assert!(msgb.is_ok());

    let mut msgb = msgb.unwrap();
    assert!(msgb.check().is_ok());

    let source = "tree $t1 [A [B C]];";
    let parse_output = language.parse(NonTerminalKind::SourceUnit, source);

    assert!(parse_output.is_valid());
    let tree = parse_output.create_tree_cursor();

    let functions = Functions::stdlib();
    let variables = Variables::new();
    let config = ExecutionConfig::new(&functions, &variables);

    let graph = msgb.execute(&tree, &config, &NoCancellation).unwrap();

    assert_eq!(
        graph.pretty_print().to_string(),
        "node 0\n  is_tree: #true\nnode 1\nedge 1 -> 0\nnode 2\nedge 2 -> 1\n"
    );
}
