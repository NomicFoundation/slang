import { Language } from "@nomicfoundation/slang/language";
import { NodeType, RuleNode, TokenNode } from "@nomicfoundation/slang/cst";
import { ProductionKind, TokenKind } from "@nomicfoundation/slang/kinds";
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

test("accessing the node in a cursor is slow", () => {
  const source = "int256 constant z = 1 + 2;";
  const language = new Language("0.8.1");

  const { parseTree } = language.parse(ProductionKind.SourceUnit, source);

  const cursor: Cursor = parseTree.cursor;
  expect(cursor.isCompleted).toBe(false);

  // This is slow.
  let access_time = 0;
  {
    const start = performance.now();
    for (let i = 0; i < 10; i++) {
      do {
        cursor.node;
      } while (cursor.goToNext());
      cursor.reset();
    }
    const end = performance.now();
    access_time = end - start;
  }

  cursor.reset();

  // This is faster
  let no_access_time = 0;
  {
    const start = performance.now();
    for (let i = 0; i < 10; i++) {
      do {} while (cursor.goToNext());
      cursor.reset();
    }
    const end = performance.now();
    no_access_time = end - start;
  }

  const ratio = access_time / no_access_time;
  console.log("cursor node access ratio", ratio);

  // This is not good.
  expect(ratio).toBeLessThan(10);
});

test("accessing the node in a cursor multiple times has a linear time cost", () => {
  const source = "int256 constant z = 1 + 2;";
  const language = new Language("0.8.1");

  const { parseTree } = language.parse(ProductionKind.SourceUnit, source);

  const cursor: Cursor = parseTree.cursor;

  // This is slow.
  let multiple_access_time = 0;
  {
    const start = performance.now();
    for (let i = 0; i < 100; i++) {
      do {
        cursor.node;
        cursor.node;
        cursor.node;
        cursor.node;
      } while (cursor.goToNext());
      cursor.reset();
    }
    const end = performance.now();
    multiple_access_time = end - start;
  }

  cursor.reset();

  // This is faster
  let single_access_time = 0;
  {
    const start = performance.now();
    for (let i = 0; i < 100; i++) {
      do {
        cursor.node;
      } while (cursor.goToNext());
      cursor.reset();
    }
    const end = performance.now();
    single_access_time = end - start;
  }

  const ratio = multiple_access_time / single_access_time;
  console.log("cursor node multiple access ratio", ratio);

  // This is not good.
  expect(ratio).toBeLessThan(4);
});

test("cursor find api is faster than testing the every node", () => {
  const source = "int256 constant z = 1 + 2;";
  const language = new Language("0.8.1");

  const { parseTree } = language.parse(ProductionKind.SourceUnit, source);

  const cursor: Cursor = parseTree.cursor;
  expect(cursor.isCompleted).toBe(false);

  // This is slow.
  let node_test_time = 0;
  {
    let count = 0;
    const start = performance.now();
    for (let i = 0; i < 100; i++) {
      do {
        const node = cursor.node;
        if (node.type === NodeType.Token && node.kind === TokenKind.Plus) {
          count++;
        }
      } while (cursor.goToNext());
      cursor.reset();
    }
    const end = performance.now();
    node_test_time = end - start;
    expect(count).toBe(100);
  }

  cursor.reset();

  // This is faster
  let cursor_find_time = 0;
  {
    let count = 0;
    const kinds = [TokenKind.Plus];
    const start = performance.now();
    for (let i = 0; i < 100; i++) {
      while (cursor.findTokenWithKind(kinds)) {
        count++;
        cursor.goToNext();
      }
      cursor.reset();
    }
    const end = performance.now();
    cursor_find_time = end - start;
    expect(count).toBe(100);
  }

  const ratio = node_test_time / cursor_find_time;
  console.log("cursor find ratio", ratio);

  // I expected this to be a lot better than 8x.
  expect(ratio).toBeGreaterThanOrEqual(8);
});
