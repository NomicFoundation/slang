import assert from "node:assert";
import { Cursor, NonterminalKind, Query } from "@nomicfoundation/slang/cst";
import { CompilationBuilder, CompilationUnit } from "@nomicfoundation/slang/compilation";
import { LanguageFacts } from "@nomicfoundation/slang/utils";

test("solidity builtins", async () => {
  const unit = await buildCompilationUnit();

  const query = Query.create(`
    [MemberAccessExpression
      [Expression @start ["tx"]]
      ["origin"]
    ]
  `);

  const cursor = unit.file("contract.sol")!.createTreeCursor();
  const matches = cursor.query([query]);
  for (const match of matches) {
    const txIdentifier = match.captures["start"][0];
    const reference = unit.bindingGraph.referenceAt(txIdentifier)!;
    const definitions = reference.definitions();

    if (definitions[0].nameLocation.isBuiltInLocation()) {
      // incorrect use of `tx.origin` detected!
      assert.strictEqual(txIdentifier.textRange.start.line, 13);
    } else {
      // this is ok: the `tx` builtin is shadowed by a parameter
      const definitionLocation = definitions[0].definiensLocation.asUserFileLocation()!;
      assert.strictEqual(definitionLocation.fileId, "contract.sol");
      assert.strictEqual(definitionLocation.cursor.node.kind, NonterminalKind.Parameter);
    }
  }
});

async function buildCompilationUnit(): Promise<CompilationUnit> {
  // we don't need to resolve imports for this example
  const resolveImport = async (_sourceId: string, _importPath: Cursor) => undefined;
  const readFile = async (fileId: string) => {
    assert(fileId == "contract.sol");
    return `
// SPDX-License-Identifier: GPL-3.0
pragma solidity >=0.7.0 <0.9.0;
// THIS CONTRACT CONTAINS A BUG - DO NOT USE
contract TxUserWallet {
    address owner;

    constructor() {
        owner = msg.sender;
    }

    function transferTo(address payable dest, uint amount) public {
        // THE BUG IS RIGHT HERE, you must use msg.sender instead of tx.origin
        require(tx.origin == owner);
        dest.transfer(amount);
    }

    struct UserTx {
        address origin;
    }

    function checkOwner(UserTx memory tx) public pure returns (bool) {
        // This use of tx.origin is fine
        return tx.origin == owner;
    }
}
    `;
  };

  const builder = CompilationBuilder.create({
    languageVersion: LanguageFacts.latestVersion(),
    readFile,
    resolveImport,
  });

  await builder.addFile("contract.sol");

  return builder.build();
}
