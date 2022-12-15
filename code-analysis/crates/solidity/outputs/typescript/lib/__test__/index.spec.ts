import test from "ava";

import { Language, Rule, Token, NodeType, RuleNode, TokenNode } from "../index";

test("parse some token", (t) => {
  const l = new Language("0.18.3");
  const cst = l.parseNumericLiteral("5_286_981").parseTree();
  if (cst instanceof TokenNode) {
    t.is(cst.type, NodeType.Token);
    t.is(cst.kind, Token.NumericLiteral);
  } else {
    t.fail("Expected TokenNode");
  }
});

test("parse some syntax", (t) => {
  const l = new Language("0.18.3");
  const cst = l.parseSourceUnit("int256 constant z = 1**2**3;").parseTree();
  if (cst instanceof RuleNode) {
    t.is(cst.type, NodeType.Rule);
    t.is(cst.kind, Rule.SourceUnit);
    t.is(cst.children().length, 1);
  } else {
    t.fail("Expected RuleNode");
  }
});
