import test from "ava";

import * as slang from "@nomicfoundation/slang";

test("parse some token", (t) => {
  const l = new slang.Language("0.8.1");
  const cst = l.parse(slang.syntax.parser.ProductionKind.DecimalNumber, "5_286_981").parseTree();
  if (cst instanceof slang.syntax.nodes.TokenNode) {
    t.is(cst.type, slang.syntax.nodes.NodeType.Token);
    t.is(cst.kind, slang.syntax.nodes.TokenKind.DecimalNumber);
  } else {
    t.fail("Expected TokenNode");
  }
});

test("parse some syntax", (t) => {
  const l = new slang.Language("0.8.1");
  const cst = l.parse(slang.syntax.parser.ProductionKind.SourceUnit, "int256 constant z = 1**2**3;").parseTree();
  if (cst instanceof slang.syntax.nodes.RuleNode) {
    t.is(cst.type, slang.syntax.nodes.NodeType.Rule);
    t.is(cst.kind, slang.syntax.nodes.RuleKind.SourceUnit);
    t.is(cst.children().length, 1);
  } else {
    t.fail("Expected RuleNode");
  }
});
