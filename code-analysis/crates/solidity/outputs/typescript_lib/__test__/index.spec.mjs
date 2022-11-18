import test from "ava";

import { Language } from "../index.js";

test("parse some code", (t) => {
  const l = new Language("0.18.3");
  t.is(l.parseSourceUnit("int256 constant z = 1**2**3;"), "anything");
});
