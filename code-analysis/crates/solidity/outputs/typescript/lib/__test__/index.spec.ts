import test from "ava";

import { Language, Rule, Token, CSTNodeType, CSTRuleNode, CSTTokenNode } from "../index";

test("parse some token", (t) => {
  const l = new Language("0.18.3");
  const cst = l.parseNumericLiteral("5_286_981").parseTree() as CSTTokenNode;
  t.is(cst.type, CSTNodeType.Token);
  t.is(cst.kind, Token.NumericLiteral);
});

test("parse some syntax", (t) => {
  const l = new Language("0.18.3");
  const cst = l.parseSourceUnit("int256 constant z = 1**2**3;").parseTree() as CSTRuleNode;
  t.is(cst.type, CSTNodeType.Rule);
  t.is(cst.kind, Rule.SourceUnit);
  t.is(cst.children().length, 1);
});
