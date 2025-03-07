import assert from "node:assert";
import { Query } from "@nomicfoundation/slang/cst";
import { ContractDefinition, FunctionDefinition } from "@nomicfoundation/slang/ast";
import { CompilationUnit } from "@nomicfoundation/slang/compilation";

export function listFunctionsInContract(
  unit: CompilationUnit,
  contractName: string,
): FunctionDefinition[] | undefined {
  for (const file of unit.files()) {
    const cursor = file.createTreeCursor();
    const query = Query.create(`[ContractDefinition @name name: [Identifier]]`);
    const matches = cursor.query([query]);

    for (const match of matches) {
      const contractNode = match.root.node;
      assert(contractNode.isNonterminalNode());
      const name = match.captures["name"][0].node.unparse();
      if (name == contractName) {
        // found the contract
        const contract = new ContractDefinition(contractNode);
        const functions = contract.members.items
          .map((member) => member.variant)
          .filter((member) => member instanceof FunctionDefinition);
        return functions;
      }
    }
  }

  // no contracts found
  return undefined;
}
