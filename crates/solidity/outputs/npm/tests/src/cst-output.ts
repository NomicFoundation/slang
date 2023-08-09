import { Language } from "@nomicfoundation/slang/language";
import {
  ProductionKind,
  NodeType,
  RuleKind,
  RuleNode,
  TokenKind,
  TokenNode,
} from "@nomicfoundation/slang/syntax/nodes";

test("parse token", () => {
  const source = "5_286_981";
  const language = new Language("0.8.1");

  const { parseTree } = language.parse(ProductionKind.DecimalLiteral, source);

  if (parseTree instanceof TokenNode) {
    expect(parseTree.type).toEqual(NodeType.Token);
    expect(parseTree.kind).toEqual(TokenKind.DecimalLiteral);
  } else {
    fail("Expected TokenNode");
  }
});

test("parse rule", () => {
  const source = "int256 constant z = 1**2**3;";
  const language = new Language("0.8.1");

  const { parseTree } = language.parse(ProductionKind.SourceUnit, source);

  if (parseTree instanceof RuleNode) {
    expect(parseTree.type).toEqual(NodeType.Rule);
    expect(parseTree.kind).toEqual(RuleKind.SourceUnit);
    expect(parseTree.children).toHaveLength(1);
  } else {
    fail("Expected RuleNode");
  }
});

test("calculate unicode characters text length", () => {
  const source = `unicode"some 😁 emoji"`;
  const language = new Language("0.8.1");

  const { parseTree } = language.parse(ProductionKind.UnicodeStringLiteral, source);

  if (parseTree instanceof TokenNode) {
    expect(parseTree.kind).toEqual(TokenKind.UnicodeStringLiteral);
    expect(parseTree.textLength).toEqual({
      char: 21,
      utf16: 22,
      utf8: 24,
    });
  } else {
    fail("Expected TokenNode");
  }
});
