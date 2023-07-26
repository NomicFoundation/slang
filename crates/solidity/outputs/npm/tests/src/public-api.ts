import * as slang from "@nomicfoundation/slang";
import { ProductionKind, RuleKind, TokenKind } from "@nomicfoundation/slang/kinds";

test("use namespace imports of the API", () => {
  expect(slang.kinds.ProductionKind.SourceUnit).toEqual("SourceUnit");
  expect(slang.kinds.RuleKind.ContractDefinition).toEqual("ContractDefinition");
  expect(slang.kinds.TokenKind.IfKeyword).toEqual("IfKeyword");
});

test("use nested imports of the API", () => {
  expect(ProductionKind.SourceUnit).toEqual("SourceUnit");
  expect(RuleKind.ContractDefinition).toEqual("ContractDefinition");
  expect(TokenKind.IfKeyword).toEqual("IfKeyword");
});
