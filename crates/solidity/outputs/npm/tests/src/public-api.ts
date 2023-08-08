import * as slang from "@nomicfoundation/slang";
import { ProductionKind, RuleKind, TokenKind } from "@nomicfoundation/slang/syntax/nodes";

test("use namespace imports of the API", () => {
  expect(slang.syntax.nodes.ProductionKind.SourceUnit).toEqual("SourceUnit");
  expect(slang.syntax.nodes.RuleKind.ContractDefinition).toEqual("ContractDefinition");
  expect(slang.syntax.nodes.TokenKind.IfKeyword).toEqual("IfKeyword");
});

test("use nested imports of the API", () => {
  expect(ProductionKind.SourceUnit).toEqual("SourceUnit");
  expect(RuleKind.ContractDefinition).toEqual("ContractDefinition");
  expect(TokenKind.IfKeyword).toEqual("IfKeyword");
});
