use std::collections::HashSet;
use std::rc::Rc;

use crate::compiler::analysis::Analysis;
use crate::model::{Identifier, SpannedTriviaParser};

pub(crate) fn run(analysis: &mut Analysis) {
    let language = Rc::clone(&analysis.language);

    let mut queue = vec![&*language.root_item];

    collect_trivia(&language.leading_trivia, &mut queue);
    collect_trivia(&language.trailing_trivia, &mut queue);

    let mut visited = queue.iter().copied().collect::<HashSet<_>>();

    while let Some(name) = queue.pop() {
        for referenced_item in &analysis.metadata[name].referenced_items {
            if visited.insert(referenced_item) {
                queue.push(referenced_item);
            }
        }
    }

    for metadata in analysis.metadata.values() {
        if !metadata.defined_in.is_empty() && !visited.contains(&*metadata.name) {
            analysis
                .errors
                .add(&metadata.name, &Errors::UnreachableItem(&metadata.name));
        }
    }
}

fn collect_trivia<'l>(parser: &'l SpannedTriviaParser, acc: &mut Vec<&'l Identifier>) {
    match parser {
        SpannedTriviaParser::Sequence { parsers } | SpannedTriviaParser::Choice { parsers } => {
            for parser in parsers {
                collect_trivia(parser, acc);
            }
        }
        SpannedTriviaParser::OneOrMore { parser }
        | SpannedTriviaParser::ZeroOrMore { parser }
        | SpannedTriviaParser::Optional { parser } => {
            collect_trivia(parser, acc);
        }
        SpannedTriviaParser::Trivia { reference } => {
            acc.push(reference);
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors<'err> {
    #[error("Item '{0}' is not reachable from grammar root.")]
    UnreachableItem(&'err Identifier),
}
