// This file is generated automatically by infrastructure scripts (crates/codegen/runner/src/main.rs). Please don't edit by hand.

#![allow(clippy::too_many_lines)] // large match statements for all non-terminals
#![allow(clippy::unnecessary_wraps)] // using `Result` for all functions for error handling

use std::rc::Rc;

use crate::rust_crate::cst::{Edge, EdgeLabel, Node, NonterminalKind, NonterminalNode};

//
// Sequences:
//

pub fn select_sequence(node: &Rc<NonterminalNode>) -> Result<Vec<Option<Node>>> {
    Err(format!("Invoking AST selectors in stubs: {node:#?}"))
} //
  // Choices:
  //

pub fn select_choice(node: &Rc<NonterminalNode>) -> Result<Node> {
    Err(format!("Invoking AST selectors in stubs: {node:#?}"))
}

//
// Repeated:
//

pub fn select_repeated(node: &Rc<NonterminalNode>) -> Result<Vec<Node>> {
    Err(format!("Invoking AST selectors in stubs: {node:#?}"))
}

//
// Separated:
//

pub fn select_separated(node: &Rc<NonterminalNode>) -> Result<Vec<Vec<Node>>> {
    Err(format!("Invoking AST selectors in stubs: {node:#?}"))
}

//
// Common:
//

type Result<T> = std::result::Result<T, String>;

struct Helper {
    node: Rc<NonterminalNode>,
    index: usize,
}

impl Helper {
    fn new(node: &Rc<NonterminalNode>) -> Self {
        Self {
            node: Rc::clone(node),
            index: 0,
        }
    }

    fn select(&mut self, target_label: EdgeLabel) -> Result<Node> {
        match self.try_select(target_label) {
            Some(node) => {
                Ok(node)
            },
            None => {
                Err(format!("Missing child with label '{target_label}'. Creating AST types from incorrect/incomplete CST nodes is not supported yet."))
            }
        }
    }

    fn try_select(&mut self, target_label: EdgeLabel) -> Option<Node> {
        let edge = self.current()?;

        if edge.label == target_label {
            self.index += 1;
            Some(edge.node)
        } else {
            None
        }
    }

    fn current(&mut self) -> Option<Edge> {
        loop {
            let edge = self.node.children.get(self.index)?;

            match edge.label {
                // Skip trivia:
                EdgeLabel::LeadingTrivia | EdgeLabel::TrailingTrivia => {
                    self.index += 1;
                    continue;
                }
                // Otherwise, return the edge:
                _ => {
                    return Some(edge.clone());
                }
            }
        }
    }

    fn finalize(mut self) -> Result<()> {
        match self.current() {
            Some(edge) => {
                Err(format!("Unrecognized child with label '{label}'. Creating AST types from incorrect/incomplete CST nodes is not supported yet.", label = edge.label))
            }
            None => {
                Ok(())
            },
        }
    }
}
