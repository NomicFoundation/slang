import test from "ava";
import * as slang from "@nomicfoundation/slang";
import { RuleKind } from "@nomicfoundation/slang/syntax/nodes";
import { ProductionKind } from "@nomicfoundation/slang/syntax/parser";

test("use namespace imports of the API", (t) => {
  t.is(typeof slang.syntax.nodes.RuleKind, "object");
  t.is(typeof slang.syntax.parser.ProductionKind, "object");
});

test("use nested imports of the API", (t) => {
  t.is(typeof RuleKind, "object");
  t.is(typeof ProductionKind, "object");
});
