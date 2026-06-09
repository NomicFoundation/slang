import { CompilationUnit } from "@nomicfoundation/slang/compilation";
import { assertUserFileLocation, Definition } from "@nomicfoundation/slang/bindings";
import { findContractByName } from "./find-contract-by-name.mjs";
import { collectDefinitions, collectAllDefinitions } from "./collect-definitions.mjs";
import { visitDefinition } from "./visit-definition.mjs";

export function findUnusedDefinitions(unit: CompilationUnit, startingContractName: string): Definition[] {
  const allDefinitions = collectAllDefinitions(unit);
  const unusedDefinitions = new Map(allDefinitions.map((definition) => [definition.id, definition]));

  const visitQueue = [findContractByName(unit, startingContractName)];
  while (visitQueue.length > 0) {
    const toVisit = visitQueue.shift()!;
    if (!unusedDefinitions.has(toVisit.id)) continue;
    unusedDefinitions.delete(toVisit.id);

    const definiensLocation = toVisit.definiensLocation;
    assertUserFileLocation(definiensLocation);

    const followed = visitDefinition(unit, definiensLocation.cursor.spawn());
    visitQueue.push(...followed);
  }

  // for remaining unused definitions, remove any nested definitions inside them
  // to prevent reporting eg. a function and all its parameters
  visitQueue.push(...unusedDefinitions.values());
  while (visitQueue.length > 0) {
    const toVisit = visitQueue.shift()!;
    if (!unusedDefinitions.has(toVisit.id)) continue;

    const definiensLocation = toVisit.definiensLocation;
    assertUserFileLocation(definiensLocation);

    const innerDefinitions = collectDefinitions(unit, definiensLocation.cursor);
    for (const inner of innerDefinitions) {
      if (inner.id == toVisit.id) continue;
      unusedDefinitions.delete(inner.id);
    }
  }

  return Array.from(unusedDefinitions.values());
}
