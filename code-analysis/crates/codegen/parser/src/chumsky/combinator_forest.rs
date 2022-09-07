use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::{collections::HashMap, rc::Rc};

use codegen_ebnf::ProductionEBNFExtensions;
use codegen_schema::*;

use super::combinator_node::CombinatorNode;
use super::combinator_tree::{self, CombinatorTree, CombinatorTreeRef};
use super::{boilerplate, rustfmt};

use proc_macro2::Ident;
use quote::{format_ident, quote};

#[derive(Clone, Debug)]
pub struct CombinatorForest {
    pub grammar: GrammarRef,
    pub trees_by_production_name: HashMap<String, CombinatorTreeRef>,
}

pub type CombinatorForestRef = Rc<CombinatorForest>;

impl CombinatorForest {
    pub fn from_grammar(grammar: GrammarRef) -> CombinatorForestRef {
        let mut trees_by_production_name = HashMap::new();
        let mut expression_trees = Vec::new();

        for production in grammar.productions.iter().map(|(_, v)| v).flatten() {
            let tree = CombinatorTree::from_production(&grammar, &production);
            if let CombinatorNode::Expression { members, .. } = tree.root_node.as_ref() {
                expression_trees.push((tree.clone(), members.clone()));
            }
            trees_by_production_name.insert(production.name.clone(), tree);
        }

        for (tree, mut members) in expression_trees {
            members.reverse();
            let mut next_sibling = None;
            for production in members {
                let mut member_tree = trees_by_production_name
                    .get_mut(&production.name)
                    .expect("TODO: production not found");
                let member_tree =
                    Rc::get_mut(&mut member_tree).expect("There cannot be any other references");
                member_tree.convert_to_expression_member(tree.production.clone(), next_sibling);
                next_sibling = Some(production);
            }
        }

        Rc::new(CombinatorForest {
            grammar,
            trees_by_production_name,
        })
    }

    pub fn generate(&self, output_dir: PathBuf) {
        // Compute a topological ordering of productions, ignoring cycles

        fn visit_production(
            forest: &CombinatorForest,
            name: &String,
            ordering: &mut BTreeMap<String, usize>,
        ) -> usize {
            let mut order = 0;
            ordering.insert(name.clone(), 0);
            let identifiers = forest
                .trees_by_production_name
                .get(name)
                .expect("TODO: production lookup")
                .referenced_identifiers();
            for name in identifiers {
                let child_order = if let Some(child_order) = ordering.get(&name) {
                    *child_order
                } else {
                    visit_production(forest, &name, ordering)
                };
                if child_order > order {
                    order = child_order;
                }
            }
            order += 1;
            ordering.insert(name.clone(), order);
            order
        }

        let mut ordering = BTreeMap::new();
        visit_production(self, &self.grammar.manifest.root_production, &mut ordering);

        let mut ordered_productions = ordering.keys().cloned().collect::<Vec<String>>();
        ordered_productions.sort_by(|a, b| (&ordering[a]).cmp(&ordering[b]));

        // Detect recursively-referenced productions

        let mut recursive_production_names = BTreeSet::new();

        for (name, order) in &ordering {
            let tree = self
                .trees_by_production_name
                .get(name)
                .expect("TODO: unknown production");
            let identifiers = tree.referenced_identifiers();
            for name in &identifiers {
                if ordering[name] >= *order {
                    recursive_production_names.insert(name.clone());
                }
            }
        }

        // Collect and merge the code for each production

        let mut cst_rule_kinds = BTreeSet::<Ident>::new();
        let mut cst_token_kinds = BTreeSet::<Ident>::new();
        let mut cst_token_part_kinds = BTreeSet::<Ident>::new();

        let mut cst_parser_field_decl_fragments = vec![];
        let mut cst_parser_impl_predecl_fragments = vec![];
        let mut cst_parser_impl_fragments = vec![];
        let mut cst_parser_impl_field_init_fragments = vec![];

        let mut ast_fragments = vec![];
        let mut ast_impl_fragments = vec![];

        let mut ast_parser_field_decl_fragments = vec![];
        let mut ast_parser_impl_predecl_fragments = vec![];
        let mut ast_parser_impl_fragments = vec![];
        let mut ast_parser_impl_field_init_fragments = vec![];

        for name in ordered_productions {
            let production = self.grammar.get_production(&name);
            let combinator_tree = self
                .trees_by_production_name
                .get(&name)
                .expect("TODO: missing production");
            let is_recursive = recursive_production_names.contains(&name);
            let combinator_tree::CodeForTree {
                cst_rule_kinds: tree_cst_rule_kinds,
                cst_token_kinds: tree_cst_token_kinds,
                cst_token_part_kinds: tree_cst_token_part_kinds,
                cst_parser_field_decl_fragment,
                cst_parser_impl_predecl_fragment,
                cst_parser_impl_fragment,
                cst_parser_impl_field_init_fragment,
                ast_fragment,
                ast_impl_fragment,
                ast_parser_field_decl_fragment,
                ast_parser_impl_field_init_fragment,
                ast_parser_impl_predecl_fragment,
                ast_parser_impl_fragment,
            } = combinator_tree.to_generated_code(self, is_recursive);

            cst_parser_field_decl_fragments.push(cst_parser_field_decl_fragment);
            cst_parser_impl_field_init_fragments.push(cst_parser_impl_field_init_fragment);
            cst_parser_impl_predecl_fragments.push(cst_parser_impl_predecl_fragment.to_string());

            ast_parser_field_decl_fragments.push(ast_parser_field_decl_fragment);
            ast_parser_impl_field_init_fragments.push(ast_parser_impl_field_init_fragment);
            ast_parser_impl_predecl_fragments.push(ast_parser_impl_predecl_fragment.to_string());

            let ebnf_comment = production
                .generate_ebnf(&self.grammar)
                .iter()
                .map(|s| format!("// {}", s))
                .collect::<Vec<_>>()
                .join("\n");
            ast_parser_impl_fragments.push(format!(
                "{}\n{}",
                ebnf_comment,
                ast_parser_impl_fragment.to_string()
            ));
            cst_parser_impl_fragments.push(format!(
                "{}\n{}",
                ebnf_comment,
                cst_parser_impl_fragment.to_string()
            ));

            cst_rule_kinds.extend(tree_cst_rule_kinds);
            cst_token_kinds.extend(tree_cst_token_kinds);
            cst_token_part_kinds.extend(tree_cst_token_part_kinds);

            let ebnf_doc_comment = production
                .generate_ebnf(&self.grammar)
                .iter()
                .map(|s| format!("/// {}", s))
                .collect::<Vec<_>>()
                .join("\n");
            ast_fragments.push(format!(
                "{}\n{}",
                ebnf_doc_comment,
                ast_fragment.to_string()
            ));

            ast_impl_fragments.push(ast_impl_fragment.to_string());
        }

        // Format and write the code

        rustfmt::format_and_write_source(
            &output_dir.join("mod.rs"),
            boilerplate::r#mod().to_string(),
        );

        let cst_rule_kinds = cst_rule_kinds
            .iter()
            .map(|s| format_ident!("{}", s))
            .collect::<Vec<_>>();
        let cst_token_kinds = cst_token_kinds
            .iter()
            .map(|s| format_ident!("{}", s))
            .collect::<Vec<_>>();
        let cst_token_part_kinds = cst_token_part_kinds
            .iter()
            .map(|s| format_ident!("{}", s))
            .collect::<Vec<_>>();
        rustfmt::format_and_write_source(
            &output_dir.join("cst.rs"),
            vec![
                boilerplate::cst().to_string(),
                quote!(
                    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
                    pub enum RuleKind { #(#cst_rule_kinds),* }
                )
                .to_string(),
                quote!(
                    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
                    pub enum TokenKind { #(#cst_token_kinds),* }
                )
                .to_string(),
                quote!(
                    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
                    pub enum TokenPartKind { #(#cst_token_part_kinds),* }
                )
                .to_string(),
            ]
            .join("\n\n"),
        );

        rustfmt::format_and_write_source(
            &output_dir.join("cst_parser.rs"),
            vec![
                boilerplate::cst_parser().to_string(),
                quote!(
                    #[allow(dead_code)]
                    pub struct Parsers { #(#cst_parser_field_decl_fragments),* }
                )
                .to_string(),
            ]
            .join("\n\n"),
        );

        rustfmt::format_and_write_source(
            &output_dir.join("cst_parser_impl.rs"),
            vec![
                boilerplate::cst_parser_impl().to_string(),
                "impl Parsers { pub fn new() -> Self {".to_owned(),
                cst_parser_impl_predecl_fragments.join("\n\n"),
                cst_parser_impl_fragments.join("\n\n"),
                quote!(
                    Self {
                        #(#cst_parser_impl_field_init_fragments),*
                    }
                )
                .to_string(),
                "}}".to_owned(),
            ]
            .join("\n\n"),
        );

        rustfmt::format_and_write_source(
            &output_dir.join("ast.rs"),
            vec![boilerplate::ast().to_string(), ast_fragments.join("\n\n")].join("\n\n"),
        );

        rustfmt::format_and_write_source(
            &output_dir.join("ast_impl.rs"),
            vec![
                boilerplate::ast_impl().to_string(),
                ast_impl_fragments.join("\n\n"),
            ]
            .join("\n\n"),
        );

        rustfmt::format_and_write_source(
            &output_dir.join("ast_parser.rs"),
            vec![
                boilerplate::ast_parser().to_string(),
                quote!(
                    #[allow(dead_code)]
                    pub struct Parsers { #(#ast_parser_field_decl_fragments),* }
                )
                .to_string(),
            ]
            .join("\n\n"),
        );

        rustfmt::format_and_write_source(
            &output_dir.join("ast_parser_impl.rs"),
            vec![
                boilerplate::ast_parser_impl().to_string(),
                "impl Parsers { pub fn new() -> Self {".to_owned(),
                ast_parser_impl_predecl_fragments.join("\n\n"),
                ast_parser_impl_fragments.join("\n\n"),
                quote!(
                    Self {
                        #(#ast_parser_impl_field_init_fragments),*
                    }
                )
                .to_string(),
                "}}".to_owned(),
            ]
            .join("\n\n"),
        );
    }
}
