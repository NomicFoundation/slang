import { Language } from "@nomicfoundation/slang/language";
import { RuleNode, TokenNode } from "@nomicfoundation/slang/cst";
import { ProductionKind, RuleKind, TokenKind } from "@nomicfoundation/slang/kinds";

test("simple contract", () => {
  const language = new Language("0.8.0");
  const parseOutput = language.parse(ProductionKind.ContractDefinition, "contract Foo {}");

  const parseTree = parseOutput.parseTree as RuleNode;
  expect(parseTree.kind).toEqual(RuleKind.ContractDefinition);

  const children = parseTree.children;
  expect(children).toHaveLength(6);

  expect((children[0] as TokenNode).kind).toEqual(TokenKind.ContractKeyword);
  expect((children[1] as RuleNode).kind).toEqual(RuleKind.LeadingTrivia);
  expect((children[2] as TokenNode).kind).toEqual(TokenKind.Identifier);
  expect((children[3] as RuleNode).kind).toEqual(RuleKind.LeadingTrivia);
  expect((children[4] as TokenNode).kind).toEqual(TokenKind.OpenBrace);
  expect((children[5] as TokenNode).kind).toEqual(TokenKind.CloseBrace);
});
