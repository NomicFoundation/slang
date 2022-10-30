import test from "ava";

import { Language, Rule, Token, CSTNodeType, LexNodeType, LexNamedNode, CSTRuleNode } from "../index";

test("parse some token", (t) => {
  const l = new Language("0.18.3");
  const lt: LexNamedNode = l.parseNumericLiteral("5_286_981");
  t.is(lt.type, LexNodeType.Named);
  t.is(lt.kind, Token.NumericLiteral);
});

test("parse some syntax", (t) => {
  const l = new Language("0.18.3");
  const cst: CSTRuleNode = l.parseSourceUnit("int256 constant z = 1**2**3;");
  t.is(cst.type, CSTNodeType.Rule);
  t.is(cst.kind, Rule.SourceUnit);
  t.is(cst.children().length, 3);
});
