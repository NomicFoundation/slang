import test from "ava";

import { Language, RuleKind, TokenKind, NodeType, RuleNode, TokenNode, ProductionKind } from "@nomicfoundation/slang";

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

test("render some error", (t) => {
  const l = new Language("0.8.1");
  const source = "int256 constant";
  const errors = l.parse(ProductionKind.SourceUnit, source).errors();

  t.is(errors.length, 1);

  const report = errors[0]?.toErrorReport("test.sol", source, /* withColor */ false);

  t.is(
    report,
    `
Error: Expected end of input.
   ╭─[test.sol:1:1]
   │
 1 │ int256 constant
   │ │ 
   │ ╰─ Error occurred here.
───╯
`.trim(),
  );
});
