import { Language } from "@nomicfoundation/slang/language";
import { NodeType, RuleNode, TokenNode } from "@nomicfoundation/slang/cst";
import { ProductionKind, RuleKind, TokenKind } from "@nomicfoundation/slang/kinds";

test("parse token", () => {
  const source = "5_286_981";
  const language = new Language("0.8.1");

  const { parseTree } = language.parse(ProductionKind.NumericExpression, source);

  if (parseTree instanceof RuleNode) {
    expect(parseTree.type).toEqual(NodeType.Rule);
    expect(parseTree.kind).toEqual(RuleKind.NumericExpression);
    expect(parseTree.children).toHaveLength(1);
    const token = parseTree.children[0];
    if (token instanceof TokenNode) {
      expect(token.type).toEqual(NodeType.Token);
      expect(token.kind).toEqual(TokenKind.DecimalLiteral);
    } else {
      fail("Expected TokenNode");
    }
  } else {
    fail("Expected RuleNode");
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

test("trivial cursor access", () => {
  const source = "int256 constant z = 1**2**3;";
  const language = new Language("0.8.1");

  const { parseTree } = language.parse(ProductionKind.SourceUnit, source);

  let cursor = parseTree.cursor;
  let node = cursor.node;
  if (node instanceof RuleNode) {
    expect(node.type).toEqual(NodeType.Rule);
    expect(node.kind).toEqual(RuleKind.SourceUnit);
    expect(node.children).toHaveLength(1);
  } else {
    fail("Expected RuleNode");
  }
});

test("calculate unicode characters text length", () => {
  const source = `unicode"some 😁 emoji"`;
  const language = new Language("0.8.1");

  const { parseTree } = language.parse(ProductionKind.UnicodeStringLiteralsList, source);

  if (parseTree instanceof RuleNode) {
    expect(parseTree.type).toEqual(NodeType.Rule);
    expect(parseTree.kind).toEqual(RuleKind.UnicodeStringLiteralsList);
    expect(parseTree.children).toHaveLength(1);
    expect(parseTree.textLength).toEqual({
      char: 21,
      utf16: 22,
      utf8: 24,
    });
    const token = parseTree.children[0];
    if (token instanceof TokenNode) {
      expect(token.type).toEqual(NodeType.Token);
      expect(token.kind).toEqual(TokenKind.UnicodeStringLiteral);
      expect(token.textLength).toEqual({
        char: 21,
        utf16: 22,
        utf8: 24,
      });
    } else {
      fail("Expected TokenNode");
    }
  } else {
    fail("Expected RuleNode");
  }
});
