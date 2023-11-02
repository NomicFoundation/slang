import { Language } from "@nomicfoundation/slang/language";
import { NodeType, RuleNode, TokenNode } from "@nomicfoundation/slang/cst";
import { ProductionKind } from "@nomicfoundation/slang/kinds";
import { Cursor } from "@nomicfoundation/slang/cursor";

test("use cursor", () => {
  const source = "int256 constant z = 1 + 2;";
  const language = new Language("0.8.1");

  const { parseTree } = language.parse(ProductionKind.SourceUnit, source);
  const cursor: Cursor = parseTree.cursor;

  expect(cursor.node.type).toBe(NodeType.Rule);
  expect(cursor.node).toBeInstanceOf(RuleNode);
  expect(cursor.goToNext()).toBe(true);

  expect(cursor.node.type).toBe(NodeType.Rule);
  expect(cursor.node).toBeInstanceOf(RuleNode);
  expect(cursor.goToNext()).toBe(true);

  expect(cursor.node.type).toBe(NodeType.Rule);
  expect(cursor.node).toBeInstanceOf(RuleNode);
  expect(cursor.goToNext()).toBe(true);

  expect(cursor.node.type).toBe(NodeType.Rule);
  expect(cursor.node).toBeInstanceOf(RuleNode);
  expect(cursor.goToNext()).toBe(true);

  expect(cursor.node.type).toBe(NodeType.Token);
  expect(cursor.node).toBeInstanceOf(TokenNode);
  expect((cursor.node as TokenNode).text).toBe("int256");
  expect(cursor.goToNext()).toBe(true);

  expect(cursor.node.type).toBe(NodeType.Rule);
  expect(cursor.node).toBeInstanceOf(RuleNode);
  expect(cursor.goToNext()).toBe(true);

  expect(cursor.node.type).toBe(NodeType.Token);
  expect(cursor.node).toBeInstanceOf(TokenNode);
  expect((cursor.node as TokenNode).text).toBe(" ");
  expect(cursor.goToNext()).toBe(true);

  expect(cursor.node.type).toBe(NodeType.Token);
  expect(cursor.node).toBeInstanceOf(TokenNode);
  expect((cursor.node as TokenNode).text).toBe("constant");
  expect(cursor.goToNext()).toBe(true);

  expect(cursor.node.type).toBe(NodeType.Rule);
  expect(cursor.node).toBeInstanceOf(RuleNode);
  expect(cursor.goToNext()).toBe(true);

  expect(cursor.node.type).toBe(NodeType.Token);
  expect(cursor.node).toBeInstanceOf(TokenNode);
  expect((cursor.node as TokenNode).text).toBe(" ");
  expect(cursor.goToNext()).toBe(true);

  expect(cursor.node.type).toBe(NodeType.Token);
  expect(cursor.node).toBeInstanceOf(TokenNode);
  expect((cursor.node as TokenNode).text).toBe("z");
  expect(cursor.goToNext()).toBe(true);

  expect(cursor.node.type).toBe(NodeType.Rule);
  expect(cursor.node).toBeInstanceOf(RuleNode);
  expect(cursor.goToNext()).toBe(true);

  expect(cursor.node.type).toBe(NodeType.Token);
  expect(cursor.node).toBeInstanceOf(TokenNode);
  expect((cursor.node as TokenNode).text).toBe(" ");
  expect(cursor.goToNext()).toBe(true);

  expect(cursor.node.type).toBe(NodeType.Token);
  expect(cursor.node).toBeInstanceOf(TokenNode);
  expect((cursor.node as TokenNode).text).toBe("=");
  expect(cursor.goToNext()).toBe(true);

  expect(cursor.node.type).toBe(NodeType.Rule);
  expect(cursor.node).toBeInstanceOf(RuleNode);
  expect(cursor.goToNext()).toBe(true);

  expect(cursor.node.type).toBe(NodeType.Rule);
  expect(cursor.node).toBeInstanceOf(RuleNode);
  expect(cursor.goToNext()).toBe(true);

  expect(cursor.node.type).toBe(NodeType.Rule);
  expect(cursor.node).toBeInstanceOf(RuleNode);
  expect(cursor.goToNext()).toBe(true);

  expect(cursor.node.type).toBe(NodeType.Rule);
  expect(cursor.node).toBeInstanceOf(RuleNode);
  expect(cursor.goToNext()).toBe(true);

  expect(cursor.node.type).toBe(NodeType.Rule);
  expect(cursor.node).toBeInstanceOf(RuleNode);
  expect(cursor.goToNext()).toBe(true);

  expect(cursor.node.type).toBe(NodeType.Token);
  expect(cursor.node).toBeInstanceOf(TokenNode);
  expect((cursor.node as TokenNode).text).toBe(" ");
  expect(cursor.goToNext()).toBe(true);

  expect(cursor.node.type).toBe(NodeType.Token);
  expect(cursor.node).toBeInstanceOf(TokenNode);
  expect((cursor.node as TokenNode).text).toBe("1");
  expect(cursor.goToNext()).toBe(true);

  expect(cursor.node.type).toBe(NodeType.Rule);
  expect(cursor.node).toBeInstanceOf(RuleNode);
  expect(cursor.goToNext()).toBe(true);

  expect(cursor.node.type).toBe(NodeType.Token);
  expect(cursor.node).toBeInstanceOf(TokenNode);
  expect((cursor.node as TokenNode).text).toBe(" ");
  expect(cursor.goToNext()).toBe(true);

  expect(cursor.node.type).toBe(NodeType.Token);
  expect(cursor.node).toBeInstanceOf(TokenNode);
  expect((cursor.node as TokenNode).text).toBe("+");
  expect(cursor.goToNext()).toBe(true);

  expect(cursor.node.type).toBe(NodeType.Rule);
  expect(cursor.node).toBeInstanceOf(RuleNode);
  expect(cursor.goToNext()).toBe(true);

  expect(cursor.node.type).toBe(NodeType.Rule);
  expect(cursor.node).toBeInstanceOf(RuleNode);
  expect(cursor.goToNext()).toBe(true);

  expect(cursor.node.type).toBe(NodeType.Rule);
  expect(cursor.node).toBeInstanceOf(RuleNode);
  expect(cursor.goToNext()).toBe(true);

  expect(cursor.node.type).toBe(NodeType.Token);
  expect(cursor.node).toBeInstanceOf(TokenNode);
  expect((cursor.node as TokenNode).text).toBe(" ");
  expect(cursor.goToNext()).toBe(true);

  expect(cursor.node.type).toBe(NodeType.Token);
  expect(cursor.node).toBeInstanceOf(TokenNode);
  expect((cursor.node as TokenNode).text).toBe("2");
  expect(cursor.goToNext()).toBe(true);

  expect(cursor.node.type).toBe(NodeType.Token);
  expect(cursor.node).toBeInstanceOf(TokenNode);
  expect((cursor.node as TokenNode).text).toBe(";");
  expect(cursor.goToNext()).toBe(false);
});
