import test from "ava";

import { Language, Rule, CSTNodeType } from "../index.js";

test("parse some code", (t) => {
  const l = new Language("0.18.3");
  const cst = l.parseSourceUnit("int256 constant z = 1**2**3;");
  t.is(cst.flavour, CSTNodeType.Rule);
  t.is(cst.kind, Rule.SourceUnit);
  console.log(cst);
});
