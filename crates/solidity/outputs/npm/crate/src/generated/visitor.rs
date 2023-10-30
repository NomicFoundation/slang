// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::rc::Rc;

use super::{cst::*, cursor::Cursor};

/// A Visitor pattern for traversing the CST.
///
/// The trait supports fallible iteration, i.e. the visitor can early return an error from the visit.
pub trait Visitor<E> {
    /// Called when the [`Visitor`] enters a [`RuleNode`].
    fn rule_enter(
        &mut self,
        _node: &Rc<RuleNode>,
        _cursor: &Cursor,
    ) -> Result<VisitorEntryResponse, E> {
        Ok(VisitorEntryResponse::StepIn)
    }

    /// Called when the [`Visitor`] exits a [`RuleNode`].
    fn rule_exit(
        &mut self,
        _node: &Rc<RuleNode>,
        _cursor: &Cursor,
    ) -> Result<VisitorExitResponse, E> {
        Ok(VisitorExitResponse::Continue)
    }

    /// Called when the [`Visitor`] enters a [`TokenNode`].
    fn token(&mut self, _node: &Rc<TokenNode>, _cursor: &Cursor) -> Result<VisitorExitResponse, E> {
        Ok(VisitorExitResponse::Continue)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VisitorEntryResponse {
    Quit,
    StepIn,
    StepOver,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VisitorExitResponse {
    Quit,
    Continue,
}

impl Cursor {
    pub fn drive_visitor<E, V: Visitor<E>>(
        &mut self,
        visitor: &mut V,
    ) -> Result<VisitorExitResponse, E> {
        if self.is_completed() {
            return Ok(VisitorExitResponse::Continue);
        }
        loop {
            // Node clone is cheap because it's just an enum around an Rc
            match self.node() {
                Node::Rule(rule_node) => {
                    match visitor.rule_enter(&rule_node, self)? {
                        VisitorEntryResponse::Quit => return Ok(VisitorExitResponse::Quit),
                        VisitorEntryResponse::StepIn => {
                            if self.go_to_first_child() {
                                self.drive_visitor(visitor)?;
                                self.go_to_parent();
                            }
                        }
                        VisitorEntryResponse::StepOver => {}
                    }
                    if visitor.rule_exit(&rule_node, self)? == VisitorExitResponse::Quit {
                        return Ok(VisitorExitResponse::Quit);
                    }
                }

                Node::Token(token_node) => {
                    if visitor.token(&token_node, self)? == VisitorExitResponse::Quit {
                        return Ok(VisitorExitResponse::Quit);
                    }
                }
            }
            if !self.go_to_next_sibling() {
                break;
            }
        }
        Ok(VisitorExitResponse::Continue)
    }
}
