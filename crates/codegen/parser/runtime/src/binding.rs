use std::{collections::BTreeMap, rc::Rc};

use crate::{
    cst::{NameLinkType, Node, Scope, ScopeLink, ScopeLinkType},
    kinds::RuleKind,
    tree_path::{RuleTreePath, TokenTreePath},
};

pub enum ScopeGraphAction {
    Visit(RuleKind),
    VisitAllChildren,
    // VisitInNewScope(RuleKind),
    PushScope,
    ReplaceScope,
    PopScope,
    AddDefinition(TokenTreePath),
    AddDefinitionAndPushAssociatedScope(TokenTreePath),
    AddReference(TokenTreePath),
    AddNameLink(NameLinkType, TokenTreePath),
    AddScopeLink(ScopeLinkType, RuleTreePath),
}

#[derive(Default)]
pub struct BindingSpecification {
    actions: BTreeMap<RuleKind, Vec<ScopeGraphAction>>,
}

impl BindingSpecification {
    pub fn insert<const N: usize, T: Into<RuleTreePath>>(
        &mut self,
        path: T,
        actions: [ScopeGraphAction; N],
    ) {
        let path: RuleTreePath = path.into();
        let existing_actions = self
            .actions
            .entry(path.head_rule_kind().expect("rule path has no head rule"))
            .or_default();
        for action in actions {
            existing_actions.push(action)
        }
    }

    pub fn apply_to_tree(&self, root_node: &Node) {
        Binder::compute_scope_graph(self, root_node);
    }
}

pub struct Binder<'a> {
    specification: &'a BindingSpecification,
    scope_stack: Vec<Rc<Scope>>,
    scopes: BTreeMap<usize, Rc<Scope>>,
}

impl<'a> Binder<'a> {
    pub fn compute_scope_graph(specification: &'a BindingSpecification, node: &Node) {
        let mut binder = {
            Binder {
                specification,
                scope_stack: Vec::new(),
                scopes: BTreeMap::new(),
            }
        };
        binder.scope_stack.push(Rc::new(Scope::default()));
        binder.process_node(node);
    }

    fn process_node(&mut self, node: &Node) {
        if let Some(rule) = node.as_rule() {
            if let Some(actions) = self.specification.actions.get(&rule.kind) {
                for action in actions {
                    match action {
                        ScopeGraphAction::VisitAllChildren => {
                            for node in rule.children.iter().filter(|child| child.is_rule()) {
                                self.process_node(node);
                            }
                        }
                        ScopeGraphAction::Visit(kind) => {
                            if let Some(child) = rule
                                .children
                                .iter()
                                .find_map(|child| child.as_rule_with_kind(&[*kind]))
                            {
                                self.process_node(&Node::Rule(child.clone()));
                            }
                        }
                        ScopeGraphAction::PushScope => {
                            let scope = Rc::new(Scope::default());
                            self.scope_stack.push(scope.clone());
                            self.scopes.insert(Rc::as_ptr(rule) as usize, scope);
                        }
                        ScopeGraphAction::ReplaceScope => {
                            let scope = Rc::new(Scope::default());
                            self.scope_stack.push(scope.clone());
                            self.scopes.insert(Rc::as_ptr(rule) as usize, scope);
                        }
                        ScopeGraphAction::PopScope => {
                            let old_scope = self.scope_stack.pop().unwrap();
                            let current_scope = self.scope_stack.last().unwrap();
                            let link = Rc::new(ScopeLink {
                                src: Rc::downgrade(&old_scope),
                                dest: current_scope.clone(),
                                link_type: ScopeLinkType::Parent,
                            });
                            // old_scope.scope_links.push(link);
                            // current_scope.scope_links.push(link);
                        }
                        ScopeGraphAction::AddDefinitionAndPushAssociatedScope(_) => todo!(),
                        ScopeGraphAction::AddDefinition(_) => todo!(),
                        ScopeGraphAction::AddReference(_) => todo!(),
                        ScopeGraphAction::AddNameLink(_, _) => todo!(),
                        ScopeGraphAction::AddScopeLink(_, _) => todo!(),
                    }
                }
            } else {
                for child in &rule.children {
                    self.process_node(child);
                }
            }
        }
    }
}
