import test from "ava";
import * as slang from "@nomicfoundation/slang";
import { ProductionKind, RuleKind, TokenKind } from "@nomicfoundation/slang/syntax/nodes";

test("use namespace imports of the API", (t) => {
  t.deepEqual(slang.syntax.nodes.ProductionKind.SourceUnit, "SourceUnit");
  t.deepEqual(slang.syntax.nodes.RuleKind.ContractDefinition, "ContractDefinition");
  t.deepEqual(slang.syntax.nodes.TokenKind.IfKeyword, "IfKeyword");
});

test("use nested imports of the API", (t) => {
  t.deepEqual(ProductionKind.SourceUnit, "SourceUnit");
  t.deepEqual(RuleKind.ContractDefinition, "ContractDefinition");
  t.deepEqual(TokenKind.IfKeyword, "IfKeyword");
});
