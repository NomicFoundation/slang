import { assertNonterminalNode, Query } from "@nomicfoundation/slang/cst";
import { ContractDefinition, FunctionDefinition } from "@nomicfoundation/slang/ast";
import { CompilationUnit } from "@nomicfoundation/slang/compilation";

export function listFunctionsInContract(unit: CompilationUnit, contractName: string): FunctionDefinition[] {
  for (const file of unit.files()) {
    const cursor = file.createTreeCursor();
    const query = Query.create(`
      [ContractDefinition
        @name name: [Identifier]
      ]
    `);
    const matches = cursor.query([query]);

    for (const match of matches) {
      const contractNode = match.root.node;
      assertNonterminalNode(contractNode);
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

  throw new Error(`Could not find contract named ${contractName}`);
}
