import { Parser } from "@nomicfoundation/slang/parser";
import {
  Cursor,
  EdgeLabel,
  NodeType,
  assertNonterminalNode,
  assertTerminalNode,
  NonterminalKind,
  TerminalKind,
} from "@nomicfoundation/slang/cst";
import { LanguageFacts } from "@nomicfoundation/slang/utils";

test("use cursor goToNext()", () => {
  const source = "contract A {}";
  const parser = Parser.create(LanguageFacts.latestVersion());

  const parseOutput = parser.parseFileContents(source);
  const cursor: Cursor = parseOutput.createTreeCursor();

  assertNonterminalNode(cursor.node, NonterminalKind.SourceUnit);
  expect(cursor.goToNext()).toBe(true);

  assertNonterminalNode(cursor.node, NonterminalKind.SourceUnitMembers);
  expect(cursor.goToNext()).toBe(true);

  assertNonterminalNode(cursor.node, NonterminalKind.SourceUnitMember);
  expect(cursor.goToNext()).toBe(true);

  assertNonterminalNode(cursor.node, NonterminalKind.ContractDefinition);
  expect(cursor.goToNext()).toBe(true);

  assertTerminalNode(cursor.node, TerminalKind.ContractKeyword, "contract");
  expect(cursor.goToNext()).toBe(true);

  assertTerminalNode(cursor.node, TerminalKind.Whitespace, " ");
  expect(cursor.goToNext()).toBe(true);

  assertTerminalNode(cursor.node, TerminalKind.Identifier, "A");
  expect(cursor.goToNext()).toBe(true);

  assertNonterminalNode(cursor.node, NonterminalKind.ContractSpecifiers);
  expect(cursor.goToNext()).toBe(true);

  assertTerminalNode(cursor.node, TerminalKind.Whitespace, " ");
  expect(cursor.goToNext()).toBe(true);

  assertTerminalNode(cursor.node, TerminalKind.OpenBrace, "{");
  expect(cursor.goToNext()).toBe(true);

  assertNonterminalNode(cursor.node, NonterminalKind.ContractMembers);
  expect(cursor.goToNext()).toBe(true);

  assertTerminalNode(cursor.node, TerminalKind.CloseBrace, "}");
  expect(cursor.goToNext()).toBe(false);
});

test("access the node using its name", () => {
  const source = "contract A { function a() { }\n function b() { } }";
  const parser = Parser.create(LanguageFacts.latestVersion());
  const parseOutput = parser.parseFileContents(source);

  const cursor = parseOutput.createTreeCursor();
  let names: string[] = [];

  while (cursor.goToNextNonterminalWithKind(NonterminalKind.ContractDefinition)) {
    const innerCursor = cursor.spawn();
    while (innerCursor.goToNext()) {
      const node = innerCursor.node;
      const label = innerCursor.label;

      if (node.type == NodeType.TerminalNode && (label == EdgeLabel.OpenBrace || label == EdgeLabel.CloseBrace)) {
        names.push(node.unparse());
      }
    }
  }

  expect(names).toEqual(["{", "{", "}", "{", "}", "}"]);
});
