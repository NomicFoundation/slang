import test from "ava";

import { Language, RuleKind, TokenKind, NodeType, RuleNode, TokenNode, ProductionKind } from "@slang/solidity";

test("parse some token", (t) => {
  const l = new Language("0.8.1");
  const cst = l.parse(ProductionKind.DecimalNumber, "5_286_981").parseTree();
  if (cst instanceof TokenNode) {
    t.is(cst.type, NodeType.Token);
    t.is(cst.kind, TokenKind.DecimalNumber);
  } else {
    t.fail("Expected TokenNode");
  }
});

test("parse some syntax", (t) => {
  const l = new Language("0.8.1");
  const cst = l.parse(ProductionKind.SourceUnit, "int256 constant z = 1**2**3;").parseTree();
  if (cst instanceof RuleNode) {
    t.is(cst.type, NodeType.Rule);
    t.is(cst.kind, RuleKind.SourceUnit);
    t.is(cst.children().length, 1);
  } else {
    t.fail("Expected RuleNode");
  }
});
