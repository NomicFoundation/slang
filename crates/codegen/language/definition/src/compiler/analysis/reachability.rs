use crate::{
    compiler::{analysis::Analysis, versions::VersionSet},
    spanned::TriviaParser,
    Identifier,
};
use std::collections::HashSet;

impl Analysis {
    pub fn analyze_reachability(&mut self) {
        self.check_unused_versions();
        self.check_unreachabable_items();
    }

    fn check_unused_versions(&mut self) {
        for (name, metadata) in &self.metadata {
            if name == &*self.language.root_item {
                continue;
            }

            let unused_in = metadata.defined_in.difference(&metadata.used_in);

            if !unused_in.is_empty() {
                self.errors
                    .add(&metadata.name, &Errors::UnusedVersion(name, &unused_in));
            }
        }
    }

    fn check_unreachabable_items(&mut self) {
        let language = self.language.clone();

        let mut queue = vec![&*language.root_item];

        collect_tokens(&language.leading_trivia, &mut queue);
        collect_tokens(&language.trailing_trivia, &mut queue);

        let mut visited = queue.iter().cloned().collect::<HashSet<_>>();

        while let Some(name) = queue.pop() {
            for referenced_item in &self.metadata[name].referenced_items {
                if visited.insert(referenced_item) {
                    queue.push(referenced_item);
                }
            }
        }

        for metadata in self.metadata.values() {
            if !metadata.defined_in.is_empty() && !visited.contains(&*metadata.name) {
                self.errors
                    .add(&metadata.name, &Errors::Unreachable(&*metadata.name));
            }
        }
    }
}

fn collect_tokens<'l>(parser: &'l TriviaParser, tokens: &mut Vec<&'l Identifier>) {
    match parser {
        TriviaParser::Sequence { parsers } | TriviaParser::Choice { parsers } => {
            for parser in parsers {
                collect_tokens(parser, tokens);
            }
        }
        TriviaParser::ZeroOrMore { parser } | TriviaParser::Optional { parser } => {
            collect_tokens(parser, tokens);
        }
        TriviaParser::Token { token } => {
            tokens.push(token);
        }
    };
}

#[derive(thiserror::Error, Debug)]
enum Errors<'err> {
    #[error("Item '{0}' is not used in versions: {1}")]
    UnusedVersion(&'err Identifier, &'err VersionSet),
    #[error("Item '{0}' is not reachable from grammar root.")]
    Unreachable(&'err Identifier),
}
