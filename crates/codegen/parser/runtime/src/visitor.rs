use std::ops::ControlFlow;
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
    ) -> Result<ControlFlow<(), Step>, E> {
        Ok(ControlFlow::Continue(Step::In))
    }

    /// Called when the [`Visitor`] exits a [`RuleNode`].
    fn rule_exit(&mut self, _node: &Rc<RuleNode>, _cursor: &Cursor) -> Result<ControlFlow<()>, E> {
        Ok(ControlFlow::Continue(()))
    }

    /// Called when the [`Visitor`] enters a [`TokenNode`].
    fn token(&mut self, _node: &Rc<TokenNode>, _cursor: &Cursor) -> Result<ControlFlow<()>, E> {
        Ok(ControlFlow::Continue(()))
    }
}

/// Whether the [`Visitor`] should should enter the children of a [`RuleNode`] or not.
pub enum Step {
    In,
    Over,
}

impl Cursor {
    pub fn drive_visitor<E, V: Visitor<E>>(
        &mut self,
        visitor: &mut V,
    ) -> Result<ControlFlow<()>, E> {
        if self.is_completed() {
            return Ok(ControlFlow::Continue(()));
        }

        loop {
            // Node clone is cheap because it's just an enum around an Rc
            match self.node() {
                Node::Rule(rule_node) => {
                    match visitor.rule_enter(&rule_node, self)? {
                        ControlFlow::Break(()) => return Ok(ControlFlow::Break(())),
                        ControlFlow::Continue(Step::In) => {
                            if self.go_to_first_child() {
                                self.drive_visitor(visitor)?;
                                self.go_to_parent();
                            }
                        }
                        ControlFlow::Continue(Step::Over) => {}
                    }
                    if visitor.rule_exit(&rule_node, self)? == ControlFlow::Break(()) {
                        return Ok(ControlFlow::Break(()));
                    }
                }

                Node::Token(token_node) => {
                    if visitor.token(&token_node, self)? == ControlFlow::Break(()) {
                        return Ok(ControlFlow::Break(()));
                    }
                }
            }

            if !self.go_to_next_sibling() {
                return Ok(ControlFlow::Continue(()));
            }
        }
    }
}
