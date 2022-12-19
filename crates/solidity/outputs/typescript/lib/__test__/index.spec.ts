import test from "ava";

import { Language, RuleKind, TokenKind, NodeType, RuleNode, TokenNode, ProductionKind } from "../index";

test("parse some token", (t) => {
  const l = new Language("0.8.1");
  const cst = l.getParser(ProductionKind.DecimalInteger).parse("5_286_981").parseTree();
  if (cst instanceof TokenNode) {
    t.is(cst.type, NodeType.Token);
    t.is(cst.kind, TokenKind.DecimalInteger);
  } else {
    t.fail("Expected TokenNode");
  }
});

test("parse some syntax", (t) => {
  const l = new Language("0.8.1");
  const cst = l.getParser(ProductionKind.SourceUnit).parse("int256 constant z = 1**2**3;").parseTree();
  if (cst instanceof RuleNode) {
    t.is(cst.type, NodeType.Rule);
    t.is(cst.kind, RuleKind.SourceUnit);
    t.is(cst.children().length, 1);
  } else {
    t.fail("Expected RuleNode");
  }
});
