import test from "ava";
import { Language } from "@nomicfoundation/slang/language";
import {
  ProductionKind,
  NodeType,
  RuleKind,
  RuleNode,
  TokenKind,
  TokenNode,
} from "@nomicfoundation/slang/syntax/nodes";

test("parse token", (t) => {
  const source = "5_286_981";
  const language = new Language("0.8.1");

  const { parseTree } = language.parse(ProductionKind.DecimalLiteral, source);

  if (parseTree instanceof TokenNode) {
    t.is(parseTree.type, NodeType.Token);
    t.is(parseTree.kind, TokenKind.DecimalLiteral);
  } else {
    t.fail("Expected TokenNode");
  }
});

test("parse rule", (t) => {
  const source = "int256 constant z = 1**2**3;";
  const language = new Language("0.8.1");

  const { parseTree } = language.parse(ProductionKind.SourceUnit, source);

  if (parseTree instanceof RuleNode) {
    t.is(parseTree.type, NodeType.Rule);
    t.is(parseTree.kind, RuleKind.SourceUnit);
    t.is(parseTree.children.length, 1);
  } else {
    t.fail("Expected RuleNode");
  }
});

test("calculate both byte and char ranges", (t) => {
  const source = `unicode"some 😁 emoji"`;
  const language = new Language("0.8.1");

  const { parseTree } = language.parse(ProductionKind.UnicodeStringLiteral, source);

  if (parseTree instanceof TokenNode) {
    t.is(parseTree.kind, TokenKind.UnicodeStringLiteral);
    t.deepEqual(parseTree.textLength.char, 21);
    t.deepEqual(parseTree.textLength.utf8, 24);
  } else {
    t.fail("Expected TokenNode");
  }
});
