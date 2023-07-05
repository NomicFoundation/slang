use std::rc::Rc;

use anyhow::Result;
use slang_solidity::syntax::{
    nodes::{Node, RuleKind, RuleNode, TextRange, TokenKind, TokenNode},
    visitors::{Visitable, Visitor, VisitorEntryResponse, VisitorExitResponse},
};

#[derive(Debug)]
pub enum TestNodeKind {
    Rule(RuleKind),
    Token(TokenKind),
    Trivia(TokenKind),
}

pub struct TestNode {
    pub kind: TestNodeKind,
    pub range: TextRange,
    pub children: Vec<TestNode>,
}

struct TestNodeBuilder {
    stack: Vec<Vec<TestNode>>,
}

impl Visitor<()> for TestNodeBuilder {
    fn enter_rule(
        &mut self,
        _node: &Rc<RuleNode>,
        _path: &Vec<Rc<RuleNode>>,
        _range: &TextRange,
    ) -> std::result::Result<VisitorEntryResponse, ()> {
        self.stack.push(vec![]);
        Ok(VisitorEntryResponse::StepIn)
    }

    fn exit_rule(
        &mut self,
        node: &Rc<RuleNode>,
        _path: &Vec<Rc<RuleNode>>,
        range: &TextRange,
    ) -> std::result::Result<VisitorExitResponse, ()> {
        let children = self.stack.pop().unwrap();

        if (node.kind == RuleKind::LeadingTrivia) | (node.kind == RuleKind::TrailingTrivia) {
            if children.is_empty() {
                return Ok(VisitorExitResponse::Continue);
            }
        }

        let new_node = TestNode {
            kind: TestNodeKind::Rule(node.kind),
            range: range.clone(),
            children,
        };
        self.stack.last_mut().unwrap().push(new_node);

        Ok(VisitorExitResponse::Continue)
    }

    fn enter_token(
        &mut self,
        node: &Rc<TokenNode>,
        _path: &Vec<Rc<RuleNode>>,
        range: &TextRange,
    ) -> std::result::Result<VisitorEntryResponse, ()> {
        if Self::is_whitespace(node) {
            return Ok(VisitorEntryResponse::StepOver);
        }

        let kind = if Self::is_comment(node) {
            TestNodeKind::Trivia(node.kind)
        } else {
            TestNodeKind::Token(node.kind)
        };

        let new_node = TestNode {
            kind,
            range: range.clone(),
            children: vec![],
        };
        self.stack.last_mut().unwrap().push(new_node);

        Ok(VisitorEntryResponse::StepOver)
    }
}

impl TestNodeBuilder {
    fn is_whitespace(token_node: &Rc<TokenNode>) -> bool {
        (token_node.kind == TokenKind::Whitespace) | (token_node.kind == TokenKind::EndOfLine)
    }

    fn is_comment(token_node: &Rc<TokenNode>) -> bool {
        (token_node.kind == TokenKind::SingleLineComment)
            | (token_node.kind == TokenKind::MultilineComment)
    }
}

impl TestNode {
    pub fn from_cst(node: Node) -> Self {
        let mut visitor = TestNodeBuilder {
            stack: vec![vec![]],
        };
        node.accept_visitor(&mut visitor).unwrap();
        return visitor.stack.remove(0).remove(0);
    }

    pub fn render_preview(&self, source: &str, range: &TextRange) -> Result<String> {
        let max_length = 50;
        let length = range.end.utf8 - range.start.utf8;

        // Trim long values:
        let contents = source
            .bytes()
            .skip(range.start.utf8)
            .take(length.clamp(0, max_length))
            .collect();

        // Add terminator if trimmed:
        let mut contents = String::from_utf8(contents)?;
        if length > max_length {
            contents.push_str("...");
        }

        // Escape line breaks:
        let contents = contents
            .replace("\t", "\\t")
            .replace("\r", "\\r")
            .replace("\n", "\\n");

        // Surround by quotes for use in yaml:
        let contents = {
            if contents.contains("\"") {
                let contents = contents.replace("'", "''");
                format!("'{contents}'")
            } else {
                let contents = contents.replace("\"", "\\\"");
                format!("\"{contents}\"")
            }
        };

        return Ok(contents);
    }
}

impl std::fmt::Display for TestNodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return match self {
            TestNodeKind::Rule(kind) => write!(f, "{kind:?} (Rule)"),
            TestNodeKind::Token(kind) => write!(f, "{kind:?} (Token)"),
            TestNodeKind::Trivia(kind) => write!(f, "{kind:?} (Trivia)"),
        };
    }
}
