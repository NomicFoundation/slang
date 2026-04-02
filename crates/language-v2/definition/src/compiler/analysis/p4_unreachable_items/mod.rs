use std::collections::HashSet;
use std::rc::Rc;

use crate::compiler::analysis::Analysis;
use crate::model::{Identifier, SpannedItem};

pub(crate) fn run(analysis: &mut Analysis) {
    let language = Rc::clone(&analysis.language);

    let mut queue = vec![&*language.root_item];

    // Seed the queue with all trivia item names so they are considered visited:
    for item in language.items() {
        if let SpannedItem::Trivia { item } = item {
            queue.push(&item.name);
        }
    }

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

#[derive(thiserror::Error, Debug)]
enum Errors<'err> {
    #[error("Item '{0}' is not reachable from grammar root.")]
    UnreachableItem(&'err Identifier),
}
