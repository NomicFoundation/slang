import assert from "node:assert";
import { assertNonterminalNode, Cursor, NonterminalKind, Query } from "@nomicfoundation/slang/cst";
import { CompilationUnit } from "@nomicfoundation/slang/compilation";
import { Definition } from "@nomicfoundation/slang/bindings";
import { collectDefinitions } from "./collect-definitions.mjs";
import { followAllReferences } from "./follow-all-references.mjs";

export function visitDefinition(unit: CompilationUnit, definiens: Cursor): Definition[] {
  assertNonterminalNode(definiens.node);
  const kind = definiens.node.kind;

  if (kind == NonterminalKind.ContractDefinition) {
    // special case contracts; see below
    return visitContract(unit, definiens);
  } else if (
    kind == NonterminalKind.LibraryDefinition ||
    kind == NonterminalKind.StructDefinition ||
    kind == NonterminalKind.EnumDefinition
  ) {
    // members must be explicitly referenced
    return [];
  } else if (kind == NonterminalKind.FunctionDefinition || kind == NonterminalKind.ModifierDefinition) {
    // follow any references inside, but don't automatically reference any new
    // definitions (eg. a parameter)
    return followAllReferences(unit, definiens);
  } else {
    // anything else (events, errors, interfaces) should be considered fully
    // referenced (including inner definitions) and we need to follow any
    // references inside them
    const otherDefinitions = collectDefinitions(unit, definiens);
    return followAllReferences(unit, definiens).concat(otherDefinitions);
  }
}

function visitContract(unit: CompilationUnit, cursor: Cursor): Definition[] {
  // for a contract, we need to explicitly follow inheritance specifiers and constructors,
  // and visit state variables and public functions
  const visitedDefinitions = [];

  const inheritance = Query.create(`
    [InheritanceSpecifier]
  `);
  const unnamedFunctions = Query.create(`
    (
      [ConstructorDefinition]
    | [ReceiveFunctionDefinition]
    | [FallbackFunctionDefinition]
    | [UnnamedFunctionDefinition]
    | [FunctionDefinition [FunctionName [FallbackKeyword]]]
    | [FunctionDefinition [FunctionName [ReceiveKeyword]]]
    )
  `);
  const publicFunctions = Query.create(`
    [FunctionDefinition
      name: [FunctionName @name [Identifier]]
      attributes: [_ [FunctionAttribute [PublicKeyword]]]
    ]
  `);
  const publicStateVars = Query.create(`
    [StateVariableDefinition
      attributes: [_ [StateVariableAttribute [PublicKeyword]]]
      @name name: [Identifier]
    ]
  `);
  const matches = cursor.query([inheritance, unnamedFunctions, publicFunctions, publicStateVars]);
  for (const match of matches) {
    switch (match.queryIndex) {
      case 0:
      case 1:
        visitedDefinitions.push(...followAllReferences(unit, match.root));
        break;
      case 2:
      case 3:
        const innerDefinition = unit.bindingGraph.definitionAt(match.captures["name"][0]);
        assert(innerDefinition);
        visitedDefinitions.push(innerDefinition);
        break;
    }
  }
  return visitedDefinitions;
}
