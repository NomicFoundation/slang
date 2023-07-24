import test from "ava";
import { Language } from "@nomicfoundation/slang/language";
import { ProductionKind, RuleKind, RuleNode, TokenKind, TokenNode } from "@nomicfoundation/slang/syntax/nodes";

test("simple contract", (t) => {
  const language = new Language("0.8.0");
  const parseOutput = language.parse(ProductionKind.ContractDefinition, "contract Foo {}");

  const parseTree = parseOutput.parseTree as RuleNode;
  t.is(parseTree.kind, RuleKind.ContractDefinition);

  const children = parseTree.children;
  t.is(children.length, 6);

  t.is((children[0] as TokenNode).kind, TokenKind.ContractKeyword);
  t.is((children[1] as RuleNode).kind, RuleKind.LeadingTrivia);
  t.is((children[2] as TokenNode).kind, TokenKind.Identifier);
  t.is((children[3] as RuleNode).kind, RuleKind.LeadingTrivia);
  t.is((children[4] as TokenNode).kind, TokenKind.OpenBrace);
  t.is((children[5] as TokenNode).kind, TokenKind.CloseBrace);
});
